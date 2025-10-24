use std::cell::{Cell, RefCell};
use std::fmt;
use std::rc::{Rc, Weak};

use crate::cell_extras::*;

use super::{Doctype, DocumentData, ElementData, NodeData, NodeRef};

/// A node inside a DOM-like tree.
pub struct Node {
    /// Weak reference to the parent node.
    pub(super) parent: Cell<Option<Weak<Node>>>,
    /// Weak reference to the previous sibling.
    pub(super) previous_sibling: Cell<Option<Weak<Node>>>,
    /// Strong reference to the next sibling.
    pub(super) next_sibling: Cell<Option<Rc<Node>>>,
    /// Strong reference to the first child.
    pub(super) first_child: Cell<Option<Rc<Node>>>,
    /// Weak reference to the last child.
    pub(super) last_child: Cell<Option<Weak<Node>>>,
    /// The data contained in this node.
    pub(super) data: NodeData,
}

impl fmt::Debug for Node {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?} @ {:?}", self.data, self as *const Node)
    }
}

/// Prevent implicit recursion when dropping nodes to avoid overflowing the stack.
///
/// The implicit drop is correct, but recursive.
/// In the worst case (where no node has both a next sibling and a child),
/// a tree of a few tens of thousands of nodes could cause a stack overflow.
///
/// This `Drop` implementations makes sure the recursion does not happen.
/// Instead, it has an explicit `Vec<Rc<Node>>` stack to traverse the subtree,
/// but only following `Rc<Node>` references that are "unique":
/// that have a strong reference count of 1.
/// Those are the nodes that would have been dropped recursively.
///
/// The stack holds ancestors of the current node rather than preceding siblings,
/// on the assumption that large document trees are typically wider than deep.
impl Drop for Node {
    fn drop(&mut self) {
        // `.take_if_unique_strong()` temporarily leaves the tree in an inconsistent state,
        // as the corresponding `Weak` reference in the other direction is not removed.
        // It is important that all `Some(_)` strong references it returns
        // are dropped by the end of this `drop` call,
        // and that no user code is invoked in-between.

        // Sharing `stack` between these two calls is not necessary,
        // but it allows re-using memory allocations.
        let mut stack = Vec::new();
        if let Some(rc) = self.first_child.take_if_unique_strong() {
            non_recursive_drop_unique_rc(rc, &mut stack);
        }
        if let Some(rc) = self.next_sibling.take_if_unique_strong() {
            non_recursive_drop_unique_rc(rc, &mut stack);
        }

        fn non_recursive_drop_unique_rc(mut rc: Rc<Node>, stack: &mut Vec<Rc<Node>>) {
            loop {
                if let Some(child) = rc.first_child.take_if_unique_strong() {
                    stack.push(rc);
                    rc = child;
                    continue;
                }
                if let Some(sibling) = rc.next_sibling.take_if_unique_strong() {
                    // The previous value of `rc: Rc<Node>` is dropped here.
                    // Since it was unique, the corresponding `Node` is dropped as well.
                    // `<Node as Drop>::drop` does not call `drop_rc`
                    // as both the first child and next sibling were already taken.
                    // Weak reference counts decremented here for `Cell`s that are `Some`:
                    // * `rc.parent`: still has a strong reference in `stack` or elsewhere
                    // * `rc.last_child`: this is the last weak ref. Deallocated now.
                    // * `rc.previous_sibling`: this is the last weak ref. Deallocated now.
                    rc = sibling;
                    continue;
                }
                if let Some(parent) = stack.pop() {
                    // Same as in the above comment.
                    rc = parent;
                    continue;
                }
                return;
            }
        }
    }
}

impl Node {
    /// Return a reference to this node's node-type-specific data.
    #[inline]
    pub fn data(&self) -> &NodeData {
        &self.data
    }

    /// If this node is an element, return a reference to element-specific data.
    #[inline]
    pub fn as_element(&self) -> Option<&ElementData> {
        match self.data {
            NodeData::Element(ref value) => Some(value),
            _ => None,
        }
    }

    /// If this node is a text node, return a reference to its contents.
    #[inline]
    pub fn as_text(&self) -> Option<&RefCell<String>> {
        match self.data {
            NodeData::Text(ref value) => Some(value),
            _ => None,
        }
    }

    /// If this node is a comment, return a reference to its contents.
    #[inline]
    pub fn as_comment(&self) -> Option<&RefCell<String>> {
        match self.data {
            NodeData::Comment(ref value) => Some(value),
            _ => None,
        }
    }

    /// If this node is a document, return a reference to doctype-specific data.
    #[inline]
    pub fn as_doctype(&self) -> Option<&Doctype> {
        match self.data {
            NodeData::Doctype(ref value) => Some(value),
            _ => None,
        }
    }

    /// If this node is a document, return a reference to document-specific data.
    #[inline]
    pub fn as_document(&self) -> Option<&DocumentData> {
        match self.data {
            NodeData::Document(ref value) => Some(value),
            _ => None,
        }
    }

    /// If this node is a processing instruction, return a reference to its contents.
    #[inline]
    pub fn as_processing_instruction(&self) -> Option<&RefCell<(String, String)>> {
        match self.data {
            NodeData::ProcessingInstruction(ref value) => Some(value),
            _ => None,
        }
    }

    /// If this node is a document fragment, return a reference to the unit value.
    #[inline]
    pub fn as_document_fragment(&self) -> Option<&()> {
        match self.data {
            NodeData::DocumentFragment => Some(&()),
            _ => None,
        }
    }

    /// Return a reference to the parent node, unless this node is the root of the tree.
    #[inline]
    pub fn parent(&self) -> Option<NodeRef> {
        self.parent.upgrade().map(NodeRef)
    }

    /// Return a reference to the first child of this node, unless it has no child.
    #[inline]
    pub fn first_child(&self) -> Option<NodeRef> {
        self.first_child.clone_inner().map(NodeRef)
    }

    /// Return a reference to the last child of this node, unless it has no child.
    #[inline]
    pub fn last_child(&self) -> Option<NodeRef> {
        self.last_child.upgrade().map(NodeRef)
    }

    /// Return a reference to the previous sibling of this node, unless it is a first child.
    #[inline]
    pub fn previous_sibling(&self) -> Option<NodeRef> {
        self.previous_sibling.upgrade().map(NodeRef)
    }

    /// Return a reference to the next sibling of this node, unless it is a last child.
    #[inline]
    pub fn next_sibling(&self) -> Option<NodeRef> {
        self.next_sibling.clone_inner().map(NodeRef)
    }

    /// Detach a node from its parent and siblings. Children are not affected.
    ///
    /// To remove a node and its descendants, detach it and drop any strong reference to it.
    pub fn detach(&self) {
        let parent_weak = self.parent.take();
        let previous_sibling_weak = self.previous_sibling.take();
        let next_sibling_strong = self.next_sibling.take();

        let previous_sibling_opt = previous_sibling_weak
            .as_ref()
            .and_then(|weak| weak.upgrade());

        if let Some(next_sibling_ref) = next_sibling_strong.as_ref() {
            next_sibling_ref
                .previous_sibling
                .replace(previous_sibling_weak);
        } else if let Some(parent_ref) = parent_weak.as_ref() {
            if let Some(parent_strong) = parent_ref.upgrade() {
                parent_strong.last_child.replace(previous_sibling_weak);
            }
        }

        if let Some(previous_sibling_strong) = previous_sibling_opt {
            previous_sibling_strong
                .next_sibling
                .replace(next_sibling_strong);
        } else if let Some(parent_ref) = parent_weak.as_ref() {
            if let Some(parent_strong) = parent_ref.upgrade() {
                parent_strong.first_child.replace(next_sibling_strong);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::html5ever::tendril::TendrilSink;
    use crate::parse_html;

    #[test]
    fn as_text() {
        let html = "<div>text content</div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        let text_node = div.as_node().first_child().unwrap();
        assert!(text_node.as_text().is_some());
        assert_eq!(&*text_node.as_text().unwrap().borrow(), "text content");
    }

    #[test]
    fn as_comment() {
        let html = "<!-- comment text --><div></div>";
        let doc = parse_html().one(html);

        let comment_node = doc.first_child().unwrap();
        assert!(comment_node.as_comment().is_some());
        assert_eq!(
            &*comment_node.as_comment().unwrap().borrow(),
            " comment text "
        );
    }

    #[test]
    fn as_doctype() {
        let html = "<!DOCTYPE html><html></html>";
        let doc = parse_html().one(html);

        let doctype_node = doc.first_child().unwrap();
        let doctype = doctype_node.as_doctype();
        assert!(doctype.is_some());
        assert_eq!(&*doctype.unwrap().name, "html");
    }

    #[test]
    fn as_document() {
        let html = "<html></html>";
        let doc = parse_html().one(html);

        assert!(doc.as_document().is_some());
    }

    #[test]
    fn as_processing_instruction() {
        let html = r#"<?xml-stylesheet href="style.css"?><div></div>"#;
        let doc = parse_html().one(html);

        // HTML parser doesn't create PI nodes, so test None case
        let div = doc.select("div").unwrap().next().unwrap();
        assert!(div.as_node().as_processing_instruction().is_none());
    }

    #[test]
    fn as_document_fragment() {
        let html = "<div></div>";
        let doc = parse_html().one(html);

        // Document nodes are not fragments
        assert!(doc.as_document_fragment().is_none());
    }

    #[test]
    fn previous_sibling() {
        let html = "<div><p>1</p><span>2</span></div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        let span = div.as_node().last_child().unwrap();
        let previous = span.previous_sibling();
        assert!(previous.is_some());
        assert_eq!(
            previous.unwrap().as_element().unwrap().name.local.as_ref(),
            "p"
        );
    }

    #[test]
    fn previous_sibling_none() {
        let html = "<div><p>first</p></div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        let first_child = div.as_node().first_child().unwrap();
        assert!(first_child.previous_sibling().is_none());
    }

    #[test]
    fn debug_format() {
        let html = "<div></div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        let debug_str = format!("{:?}", div.as_node());
        assert!(debug_str.contains("Element"));
    }
}
