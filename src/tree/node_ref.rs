use super::{Doctype, DocumentData, ElementData, Node, NodeData};
use crate::attributes::{Attribute, Attributes, ExpandedName};
use crate::cell_extras::*;
use crate::iter::NodeIterator;
use html5ever::tree_builder::QuirksMode;
use html5ever::QualName;
use std::cell::{Cell, RefCell};
use std::ops::Deref;
use std::rc::Rc;

/// A strong reference to a node.
///
/// A node is destroyed when the last strong reference to it dropped.
///
/// Each node holds a strong reference to its first child and next sibling (if any),
/// but only a weak reference to its last child, previous sibling, and parent.
/// This is to avoid strong reference cycles, which would cause memory leaks.
///
/// As a result, a single `NodeRef` is sufficient to keep alive a node
/// and nodes that are after it in tree order
/// (its descendants, its following siblings, and their descendants)
/// but not other nodes in a tree.
///
/// To avoid detroying nodes prematurely,
/// programs typically hold a strong reference to the root of a document
/// until they're done with that document.
#[derive(Clone, Debug)]
pub struct NodeRef(pub(super) Rc<Node>);

/// Implements Deref to allow transparent access to the underlying Node.
///
/// This allows NodeRef to be used like a reference to Node, automatically
/// dereferencing to access Node's methods and fields.
impl Deref for NodeRef {
    type Target = Node;
    #[inline]
    fn deref(&self) -> &Node {
        &self.0
    }
}

/// Implements Eq for NodeRef.
///
/// Two NodeRefs are equal if they point to the same Node instance
/// (pointer equality), not if their data is equivalent.
impl Eq for NodeRef {}

/// Implements PartialEq for NodeRef using pointer equality.
///
/// Compares the memory addresses of the underlying Node instances.
/// Returns true only if both NodeRefs point to the exact same Node.
impl PartialEq for NodeRef {
    #[inline]
    fn eq(&self, other: &NodeRef) -> bool {
        let a: *const Node = &*self.0;
        let b: *const Node = &*other.0;
        a == b
    }
}

/// Factory methods and tree manipulation for NodeRef.
///
/// Provides constructors for all node types (elements, text, comments, etc.)
/// and methods for reading and modifying the tree structure.
impl NodeRef {
    /// Create a new node.
    #[inline]
    pub fn new(data: NodeData) -> NodeRef {
        NodeRef(Rc::new(Node {
            parent: Cell::new(None),
            first_child: Cell::new(None),
            last_child: Cell::new(None),
            previous_sibling: Cell::new(None),
            next_sibling: Cell::new(None),
            data,
        }))
    }

    /// Create a new element node.
    #[inline]
    pub fn new_element<I>(name: QualName, attributes: I) -> NodeRef
    where
        I: IntoIterator<Item = (ExpandedName, Attribute)>,
    {
        NodeRef::new(NodeData::Element(ElementData {
            template_contents: if name.expanded() == expanded_name!(html "template") {
                Some(NodeRef::new(NodeData::DocumentFragment))
            } else {
                None
            },
            name,
            attributes: RefCell::new(Attributes {
                map: attributes.into_iter().collect(),
            }),
        }))
    }

    /// Create a new text node.
    #[inline]
    pub fn new_text<T: Into<String>>(value: T) -> NodeRef {
        NodeRef::new(NodeData::Text(RefCell::new(value.into())))
    }

    /// Create a new comment node.
    #[inline]
    pub fn new_comment<T: Into<String>>(value: T) -> NodeRef {
        NodeRef::new(NodeData::Comment(RefCell::new(value.into())))
    }

    /// Create a new processing instruction node.
    #[inline]
    pub fn new_processing_instruction<T1, T2>(target: T1, data: T2) -> NodeRef
    where
        T1: Into<String>,
        T2: Into<String>,
    {
        NodeRef::new(NodeData::ProcessingInstruction(RefCell::new((
            target.into(),
            data.into(),
        ))))
    }

    /// Create a new doctype node.
    #[inline]
    pub fn new_doctype<T1, T2, T3>(name: T1, public_id: T2, system_id: T3) -> NodeRef
    where
        T1: Into<String>,
        T2: Into<String>,
        T3: Into<String>,
    {
        NodeRef::new(NodeData::Doctype(Doctype {
            name: name.into(),
            public_id: public_id.into(),
            system_id: system_id.into(),
        }))
    }

    /// Create a new document node.
    #[inline]
    pub fn new_document() -> NodeRef {
        NodeRef::new(NodeData::Document(DocumentData {
            _quirks_mode: Cell::new(QuirksMode::NoQuirks),
        }))
    }

    /// Return the concatenation of all text nodes in this subtree.
    pub fn text_contents(&self) -> String {
        let mut s = String::new();
        for text_node in self.inclusive_descendants().text_nodes() {
            s.push_str(&text_node.borrow());
        }
        s
    }

    /// Append a new child to this node, after existing children.
    ///
    /// The new child is detached from its previous position.
    pub fn append(&self, new_child: NodeRef) {
        new_child.detach();
        new_child.parent.replace(Some(Rc::downgrade(&self.0)));
        if let Some(last_child_weak) = self.last_child.replace(Some(Rc::downgrade(&new_child.0))) {
            if let Some(last_child) = last_child_weak.upgrade() {
                new_child.previous_sibling.replace(Some(last_child_weak));
                debug_assert!(last_child.next_sibling.is_none());
                last_child.next_sibling.replace(Some(new_child.0));
                return;
            }
        }
        debug_assert!(self.first_child.is_none());
        self.first_child.replace(Some(new_child.0));
    }

    /// Prepend a new child to this node, before existing children.
    ///
    /// The new child is detached from its previous position.
    pub fn prepend(&self, new_child: NodeRef) {
        new_child.detach();
        new_child.parent.replace(Some(Rc::downgrade(&self.0)));
        if let Some(first_child) = self.first_child.take() {
            debug_assert!(first_child.previous_sibling.is_none());
            first_child
                .previous_sibling
                .replace(Some(Rc::downgrade(&new_child.0)));
            new_child.next_sibling.replace(Some(first_child));
        } else {
            debug_assert!(self.first_child.is_none());
            self.last_child.replace(Some(Rc::downgrade(&new_child.0)));
        }
        self.first_child.replace(Some(new_child.0));
    }

    /// Insert a new sibling after this node.
    ///
    /// The new sibling is detached from its previous position.
    ///
    /// # Panics
    ///
    /// Panics in debug mode if internal tree invariants are violated.
    pub fn insert_after(&self, new_sibling: NodeRef) {
        new_sibling.detach();
        new_sibling.parent.replace(self.parent.clone_inner());
        new_sibling
            .previous_sibling
            .replace(Some(Rc::downgrade(&self.0)));
        if let Some(next_sibling) = self.next_sibling.take() {
            debug_assert!(next_sibling.previous_sibling().unwrap() == *self);
            next_sibling
                .previous_sibling
                .replace(Some(Rc::downgrade(&new_sibling.0)));
            new_sibling.next_sibling.replace(Some(next_sibling));
        } else if let Some(parent) = self.parent() {
            debug_assert!(parent.last_child().unwrap() == *self);
            parent
                .last_child
                .replace(Some(Rc::downgrade(&new_sibling.0)));
        }
        self.next_sibling.replace(Some(new_sibling.0));
    }

    /// Insert a new sibling before this node.
    ///
    /// The new sibling is detached from its previous position.
    ///
    /// # Panics
    ///
    /// Panics in debug mode if internal tree invariants are violated.
    pub fn insert_before(&self, new_sibling: NodeRef) {
        new_sibling.detach();
        new_sibling.parent.replace(self.parent.clone_inner());
        new_sibling.next_sibling.replace(Some(self.0.clone()));
        if let Some(previous_sibling_weak) = self
            .previous_sibling
            .replace(Some(Rc::downgrade(&new_sibling.0)))
        {
            if let Some(previous_sibling) = previous_sibling_weak.upgrade() {
                new_sibling
                    .previous_sibling
                    .replace(Some(previous_sibling_weak));
                debug_assert!(previous_sibling.next_sibling().unwrap() == *self);
                previous_sibling.next_sibling.replace(Some(new_sibling.0));
                return;
            }
        }
        if let Some(parent) = self.parent() {
            debug_assert!(parent.first_child().unwrap() == *self);
            parent.first_child.replace(Some(new_sibling.0));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::html5ever::tendril::TendrilSink;
    use crate::parse_html;

    /// Tests that `new_element()` creates an element node with the correct tag name.
    ///
    /// Verifies both that the node is recognized as an element and that
    /// the local name matches the specified tag.
    #[test]
    fn new_element() {
        let element =
            NodeRef::new_element(QualName::new(None, ns!(html), local_name!("div")), vec![]);

        assert!(element.as_element().is_some());
        assert_eq!(element.as_element().unwrap().name.local.as_ref(), "div");
    }

    /// Tests that `new_text()` creates a text node with the specified content.
    ///
    /// Verifies both that the node is recognized as a text node and that
    /// the text content is stored correctly.
    #[test]
    fn new_text() {
        let text = NodeRef::new_text("Hello World");

        assert!(text.as_text().is_some());
        assert_eq!(&*text.as_text().unwrap().borrow(), "Hello World");
    }

    /// Tests that `new_comment()` creates a comment node with the specified content.
    ///
    /// Verifies both that the node is recognized as a comment and that
    /// the comment text is stored correctly.
    #[test]
    fn new_comment() {
        let comment = NodeRef::new_comment("This is a comment");

        assert!(comment.as_comment().is_some());
        assert_eq!(
            &*comment.as_comment().unwrap().borrow(),
            "This is a comment"
        );
    }

    /// Tests that `new_processing_instruction()` creates a PI node with target and data.
    ///
    /// Verifies that both the target and data portions of the processing instruction
    /// are stored and accessible.
    #[test]
    fn new_processing_instruction() {
        let pi = NodeRef::new_processing_instruction("xml-stylesheet", "href='style.css'");

        assert!(pi.as_processing_instruction().is_some());
        let pi_data = pi.as_processing_instruction().unwrap().borrow();
        assert_eq!(pi_data.0, "xml-stylesheet");
        assert_eq!(pi_data.1, "href='style.css'");
    }

    /// Tests that `new_doctype()` creates a doctype node with the specified name.
    ///
    /// Verifies both that the node is recognized as a doctype and that
    /// the name field is accessible.
    #[test]
    fn new_doctype() {
        let doctype = NodeRef::new_doctype("html", "", "");

        assert!(doctype.as_doctype().is_some());
        assert_eq!(&*doctype.as_doctype().unwrap().name, "html");
    }

    /// Tests that `new_document()` creates a document node.
    ///
    /// Verifies that the node is recognized as a document type.
    #[test]
    fn new_document() {
        let doc = NodeRef::new_document();

        assert!(doc.as_document().is_some());
    }

    /// Tests that `text_contents()` concatenates all text from descendant nodes.
    ///
    /// Parses HTML with text in multiple elements and verifies that
    /// all text is extracted and concatenated correctly.
    #[test]
    fn text_contents() {
        let doc = parse_html().one(r#"<div>Hello <b>World</b>!</div>"#);
        let div = doc.select("div").unwrap().next().unwrap();

        assert_eq!(div.as_node().text_contents(), "Hello World!");
    }

    /// Tests that `append()` adds children in the correct order.
    ///
    /// Appends two text nodes and verifies that first_child, last_child,
    /// and next_sibling relationships are established correctly.
    #[test]
    fn append() {
        let parent =
            NodeRef::new_element(QualName::new(None, ns!(html), local_name!("div")), vec![]);
        let child1 = NodeRef::new_text("First");
        let child2 = NodeRef::new_text("Second");

        parent.append(child1.clone());
        parent.append(child2.clone());

        assert_eq!(parent.first_child().unwrap(), child1);
        assert_eq!(parent.last_child().unwrap(), child2);
        assert_eq!(child1.next_sibling().unwrap(), child2);
    }

    /// Tests that `prepend()` adds children at the beginning.
    ///
    /// Appends one child, then prepends another, and verifies that
    /// the prepended child becomes the first child.
    #[test]
    fn prepend() {
        let parent =
            NodeRef::new_element(QualName::new(None, ns!(html), local_name!("div")), vec![]);
        let child1 = NodeRef::new_text("First");
        let child2 = NodeRef::new_text("Second");

        parent.append(child1.clone());
        parent.prepend(child2.clone());

        assert_eq!(parent.first_child().unwrap(), child2);
        assert_eq!(parent.last_child().unwrap(), child1);
        assert_eq!(child2.next_sibling().unwrap(), child1);
    }

    /// Tests that `insert_after()` inserts a sibling in the middle of children.
    ///
    /// Creates three children with one inserted between two existing children,
    /// and verifies the final order is correct.
    #[test]
    fn insert_after() {
        let parent =
            NodeRef::new_element(QualName::new(None, ns!(html), local_name!("div")), vec![]);
        let child1 = NodeRef::new_text("First");
        let child2 = NodeRef::new_text("Second");
        let child3 = NodeRef::new_text("Third");

        parent.append(child1.clone());
        parent.append(child3.clone());
        child1.insert_after(child2.clone());

        let children: Vec<_> = parent.children().collect();
        assert_eq!(children.len(), 3);
        assert_eq!(children[0], child1);
        assert_eq!(children[1], child2);
        assert_eq!(children[2], child3);
    }

    /// Tests that `insert_before()` inserts a sibling in the middle of children.
    ///
    /// Creates three children with one inserted between two existing children,
    /// and verifies the final order is correct.
    #[test]
    fn insert_before() {
        let parent =
            NodeRef::new_element(QualName::new(None, ns!(html), local_name!("div")), vec![]);
        let child1 = NodeRef::new_text("First");
        let child2 = NodeRef::new_text("Second");
        let child3 = NodeRef::new_text("Third");

        parent.append(child1.clone());
        parent.append(child3.clone());
        child3.insert_before(child2.clone());

        let children: Vec<_> = parent.children().collect();
        assert_eq!(children.len(), 3);
        assert_eq!(children[0], child1);
        assert_eq!(children[1], child2);
        assert_eq!(children[2], child3);
    }

    /// Tests that `detach()` removes a child from its parent.
    ///
    /// Creates three children, detaches the middle one, and verifies that
    /// the parent's children list no longer includes it and that the child
    /// has no parent.
    #[test]
    fn detach() {
        let parent =
            NodeRef::new_element(QualName::new(None, ns!(html), local_name!("div")), vec![]);
        let child1 = NodeRef::new_text("First");
        let child2 = NodeRef::new_text("Second");
        let child3 = NodeRef::new_text("Third");

        parent.append(child1.clone());
        parent.append(child2.clone());
        parent.append(child3.clone());

        child2.detach();

        let children: Vec<_> = parent.children().collect();
        assert_eq!(children.len(), 2);
        assert_eq!(children[0], child1);
        assert_eq!(children[1], child3);
        assert!(child2.parent().is_none());
    }

    /// Tests that `prepend()` works correctly on an empty parent.
    ///
    /// Edge case: when prepending to a parent with no children,
    /// the child should become both first_child and last_child.
    #[test]
    fn prepend_to_empty() {
        let parent =
            NodeRef::new_element(QualName::new(None, ns!(html), local_name!("div")), vec![]);
        let child = NodeRef::new_text("Only child");

        parent.prepend(child.clone());

        assert_eq!(parent.first_child().unwrap(), child);
        assert_eq!(parent.last_child().unwrap(), child);
    }

    /// Tests that `insert_after()` correctly updates parent's last_child.
    ///
    /// Edge case: when inserting after the current last child,
    /// the parent's last_child pointer must be updated.
    #[test]
    fn insert_after_as_last_child() {
        let parent =
            NodeRef::new_element(QualName::new(None, ns!(html), local_name!("div")), vec![]);
        let child1 = NodeRef::new_text("First");
        let child2 = NodeRef::new_text("Last");

        parent.append(child1.clone());
        child1.insert_after(child2.clone());

        assert_eq!(parent.last_child().unwrap(), child2);
        assert!(child2.next_sibling().is_none());
    }

    /// Tests that `insert_before()` correctly updates parent's first_child.
    ///
    /// Edge case: when inserting before the current first child,
    /// the parent's first_child pointer must be updated.
    #[test]
    fn insert_before_as_first_child() {
        let parent =
            NodeRef::new_element(QualName::new(None, ns!(html), local_name!("div")), vec![]);
        let child1 = NodeRef::new_text("Second");
        let child2 = NodeRef::new_text("First");

        parent.append(child1.clone());
        child1.insert_before(child2.clone());

        assert_eq!(parent.first_child().unwrap(), child2);
        assert!(child2.previous_sibling().is_none());
    }
}
