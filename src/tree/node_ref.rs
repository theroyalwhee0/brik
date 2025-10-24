use html5ever::tree_builder::QuirksMode;
use html5ever::QualName;
use std::cell::{Cell, RefCell};
use std::ops::Deref;
use std::rc::Rc;

use crate::attributes::{Attribute, Attributes, ExpandedName};
use crate::cell_extras::*;
use crate::iter::NodeIterator;

use super::{Doctype, DocumentData, ElementData, Node, NodeData};

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

impl Deref for NodeRef {
    type Target = Node;
    #[inline]
    fn deref(&self) -> &Node {
        &self.0
    }
}

impl Eq for NodeRef {}
impl PartialEq for NodeRef {
    #[inline]
    fn eq(&self, other: &NodeRef) -> bool {
        let a: *const Node = &*self.0;
        let b: *const Node = &*other.0;
        a == b
    }
}

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
