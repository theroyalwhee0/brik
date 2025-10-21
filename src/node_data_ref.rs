use crate::tree::{Doctype, DocumentData, ElementData, Node, NodeRef};
use std::cell::RefCell;
use std::fmt;
use std::ops::Deref;

#[cfg(not(feature = "unsafe"))]
use std::marker::PhantomData;

/// Discriminant for the type of node data being referenced (safe mode only).
#[cfg(not(feature = "unsafe"))]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NodeDataKind {
    /// Element node.
    Element = 0,
    /// Text node.
    Text = 1,
    /// Comment node.
    Comment = 2,
    /// Doctype node.
    Doctype = 3,
    /// Document node.
    Document = 4,
}

impl NodeRef {
    /// If this node is an element, return a strong reference to element-specific data.
    #[inline]
    pub fn into_element_ref(self) -> Option<NodeDataRef<ElementData>> {
        NodeDataRef::new_opt(self, Node::as_element)
    }

    /// If this node is a text node, return a strong reference to its contents.
    #[inline]
    pub fn into_text_ref(self) -> Option<NodeDataRef<RefCell<String>>> {
        NodeDataRef::new_opt(self, Node::as_text)
    }

    /// If this node is a comment, return a strong reference to its contents.
    #[inline]
    pub fn into_comment_ref(self) -> Option<NodeDataRef<RefCell<String>>> {
        NodeDataRef::new_opt(self, Node::as_comment)
    }

    /// If this node is a doctype, return a strong reference to doctype-specific data.
    #[inline]
    pub fn into_doctype_ref(self) -> Option<NodeDataRef<Doctype>> {
        NodeDataRef::new_opt(self, Node::as_doctype)
    }

    /// If this node is a document, return a strong reference to document-specific data.
    #[inline]
    pub fn into_document_ref(self) -> Option<NodeDataRef<DocumentData>> {
        NodeDataRef::new_opt(self, Node::as_document)
    }
}

/// Holds a strong reference to a node, but dereferences to some component inside of it.
#[derive(Eq)]
pub struct NodeDataRef<T> {
    /// Keeps the node alive while this reference exists.
    _keep_alive: NodeRef,
    /// Raw pointer to the data within the node (unsafe mode).
    #[cfg(feature = "unsafe")]
    _reference: *const T,
    /// Node data kind discriminant (safe mode).
    #[cfg(not(feature = "unsafe"))]
    _kind: NodeDataKind,
    /// Phantom data to maintain generic parameter (safe mode).
    #[cfg(not(feature = "unsafe"))]
    _phantom: PhantomData<T>,
}

impl<T> NodeDataRef<T> {
    /// Create a `NodeDataRef` for a component in a given node.
    #[inline]
    pub fn new<F>(rc: NodeRef, f: F) -> NodeDataRef<T>
    where
        F: FnOnce(&Node) -> &T,
    {
        #[cfg(feature = "unsafe")]
        {
            NodeDataRef {
                _reference: f(&rc),
                _keep_alive: rc,
            }
        }
        #[cfg(not(feature = "unsafe"))]
        {
            // Determine the node kind. Since every node must be one of the 5 types,
            // this should always succeed. The unreachable!() documents a logic bug.
            let kind = if rc.as_element().is_some() {
                NodeDataKind::Element
            } else if rc.as_text().is_some() {
                NodeDataKind::Text
            } else if rc.as_comment().is_some() {
                NodeDataKind::Comment
            } else if rc.as_doctype().is_some() {
                NodeDataKind::Doctype
            } else if rc.as_document().is_some() {
                NodeDataKind::Document
            } else {
                unreachable!("All node types are covered")
            };

            // We don't call f() because we trust the caller's function signature.
            // The infallible signature F: FnOnce(&Node) -> &T means the caller
            // guarantees this node has the correct type.
            let _ = f;

            NodeDataRef {
                _keep_alive: rc,
                _kind: kind,
                _phantom: PhantomData,
            }
        }
    }

    /// Create a `NodeDataRef` for and a component that may or may not be in a given node.
    #[inline]
    pub fn new_opt<F>(rc: NodeRef, f: F) -> Option<NodeDataRef<T>>
    where
        F: FnOnce(&Node) -> Option<&T>,
    {
        #[cfg(feature = "unsafe")]
        {
            f(&rc).map(|r| r as *const T).map(move |r| NodeDataRef {
                _reference: r,
                _keep_alive: rc,
            })
        }
        #[cfg(not(feature = "unsafe"))]
        {
            // Determine the node kind by checking which variant matches.
            // This is safe because we're only storing the discriminant, not the pointer.
            let kind = if rc.as_element().is_some() {
                NodeDataKind::Element
            } else if rc.as_text().is_some() {
                NodeDataKind::Text
            } else if rc.as_comment().is_some() {
                NodeDataKind::Comment
            } else if rc.as_doctype().is_some() {
                NodeDataKind::Doctype
            } else if rc.as_document().is_some() {
                NodeDataKind::Document
            } else {
                return None;
            };

            // Verify that f returns Some for this node.
            if f(&rc).is_some() {
                Some(NodeDataRef {
                    _keep_alive: rc,
                    _kind: kind,
                    _phantom: PhantomData,
                })
            } else {
                None
            }
        }
    }

    /// Access the corresponding node.
    #[inline]
    pub fn as_node(&self) -> &NodeRef {
        &self._keep_alive
    }
}

// Generic Deref implementation for unsafe mode.
#[cfg(feature = "unsafe")]
impl<T> Deref for NodeDataRef<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        unsafe { &*self._reference }
    }
}

// Specialized Deref implementations for safe mode.
#[cfg(not(feature = "unsafe"))]
impl Deref for NodeDataRef<ElementData> {
    type Target = ElementData;
    #[inline]
    fn deref(&self) -> &ElementData {
        self._keep_alive.as_element().expect("NodeDataRef<ElementData> must contain Element")
    }
}

#[cfg(not(feature = "unsafe"))]
impl Deref for NodeDataRef<RefCell<String>> {
    type Target = RefCell<String>;
    #[inline]
    fn deref(&self) -> &RefCell<String> {
        match self._kind {
            NodeDataKind::Text => self._keep_alive.as_text().expect("NodeDataRef with Text kind must contain text"),
            NodeDataKind::Comment => self._keep_alive.as_comment().expect("NodeDataRef with Comment kind must contain comment"),
            _ => unreachable!("NodeDataRef<RefCell<String>> must be Text or Comment"),
        }
    }
}

#[cfg(not(feature = "unsafe"))]
impl Deref for NodeDataRef<Doctype> {
    type Target = Doctype;
    #[inline]
    fn deref(&self) -> &Doctype {
        self._keep_alive.as_doctype().expect("NodeDataRef<Doctype> must contain Doctype")
    }
}

#[cfg(not(feature = "unsafe"))]
impl Deref for NodeDataRef<DocumentData> {
    type Target = DocumentData;
    #[inline]
    fn deref(&self) -> &DocumentData {
        self._keep_alive.as_document().expect("NodeDataRef<DocumentData> must contain Document")
    }
}

// #[derive(PartialEq)] would compare both fields
impl<T> PartialEq for NodeDataRef<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self._keep_alive == other._keep_alive
    }
}

// #[derive(Clone)] would have an unnecessary `T: Clone` bound
impl<T> Clone for NodeDataRef<T> {
    #[inline]
    fn clone(&self) -> Self {
        #[cfg(feature = "unsafe")]
        {
            NodeDataRef {
                _keep_alive: self._keep_alive.clone(),
                _reference: self._reference,
            }
        }
        #[cfg(not(feature = "unsafe"))]
        {
            NodeDataRef {
                _keep_alive: self._keep_alive.clone(),
                _kind: self._kind,
                _phantom: PhantomData,
            }
        }
    }
}

// Generic Debug implementation for unsafe mode.
#[cfg(feature = "unsafe")]
impl<T: fmt::Debug> fmt::Debug for NodeDataRef<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(&**self, f)
    }
}

// Specialized Debug implementations for safe mode.
#[cfg(not(feature = "unsafe"))]
impl fmt::Debug for NodeDataRef<ElementData> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(&**self, f)
    }
}

#[cfg(not(feature = "unsafe"))]
impl fmt::Debug for NodeDataRef<RefCell<String>> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(&**self, f)
    }
}

#[cfg(not(feature = "unsafe"))]
impl fmt::Debug for NodeDataRef<Doctype> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(&**self, f)
    }
}

#[cfg(not(feature = "unsafe"))]
impl fmt::Debug for NodeDataRef<DocumentData> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(&**self, f)
    }
}

impl NodeDataRef<ElementData> {
    /// Return the concatenation of all text nodes in this subtree.
    pub fn text_contents(&self) -> String {
        self.as_node().text_contents()
    }
}
