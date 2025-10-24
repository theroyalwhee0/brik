use super::filter_iterators::Elements;
use super::node_edge::NodeEdge;
use super::siblings::State;
use super::{Ancestors, Descendants, NodeIterator, Select, Siblings, Traverse};
use crate::node_data_ref::NodeDataRef;
use crate::tree::{ElementData, NodeRef};
use std::iter::Rev;

impl NodeRef {
    /// Return an iterator of references to this node and its ancestors.
    #[inline]
    pub fn inclusive_ancestors(&self) -> Ancestors {
        Ancestors(Some(self.clone()))
    }

    /// Return an iterator of references to this node's ancestors.
    #[inline]
    pub fn ancestors(&self) -> Ancestors {
        Ancestors(self.parent())
    }

    /// Return an iterator of references to this node and the siblings before it.
    ///
    /// # Panics
    ///
    /// Panics if the node has a parent but that parent has no first child (internal tree inconsistency).
    #[inline]
    pub fn inclusive_preceding_siblings(&self) -> Rev<Siblings> {
        match self.parent() {
            Some(parent) => {
                let first_sibling = parent.first_child().unwrap();
                debug_assert!(self.previous_sibling().is_some() || *self == first_sibling);
                Siblings(Some(State {
                    next: first_sibling,
                    next_back: self.clone(),
                }))
            }
            None => {
                debug_assert!(self.previous_sibling().is_none());
                Siblings(Some(State {
                    next: self.clone(),
                    next_back: self.clone(),
                }))
            }
        }
        .rev()
    }

    /// Return an iterator of references to this node's siblings before it.
    ///
    /// # Panics
    ///
    /// Panics if the node has a parent but that parent has no first child (internal tree inconsistency).
    #[inline]
    pub fn preceding_siblings(&self) -> Rev<Siblings> {
        match (self.parent(), self.previous_sibling()) {
            (Some(parent), Some(previous_sibling)) => {
                let first_sibling = parent.first_child().unwrap();
                Siblings(Some(State {
                    next: first_sibling,
                    next_back: previous_sibling,
                }))
            }
            _ => Siblings(None),
        }
        .rev()
    }

    /// Return an iterator of references to this node and the siblings after it.
    ///
    /// # Panics
    ///
    /// Panics if the node has a parent but that parent has no last child (internal tree inconsistency).
    #[inline]
    pub fn inclusive_following_siblings(&self) -> Siblings {
        match self.parent() {
            Some(parent) => {
                let last_sibling = parent.last_child().unwrap();
                debug_assert!(self.next_sibling().is_some() || *self == last_sibling);
                Siblings(Some(State {
                    next: self.clone(),
                    next_back: last_sibling,
                }))
            }
            None => {
                debug_assert!(self.next_sibling().is_none());
                Siblings(Some(State {
                    next: self.clone(),
                    next_back: self.clone(),
                }))
            }
        }
    }

    /// Return an iterator of references to this node's siblings after it.
    ///
    /// # Panics
    ///
    /// Panics if the node has a parent but that parent has no last child (internal tree inconsistency).
    #[inline]
    pub fn following_siblings(&self) -> Siblings {
        match (self.parent(), self.next_sibling()) {
            (Some(parent), Some(next_sibling)) => {
                let last_sibling = parent.last_child().unwrap();
                Siblings(Some(State {
                    next: next_sibling,
                    next_back: last_sibling,
                }))
            }
            _ => Siblings(None),
        }
    }

    /// Return an iterator of references to this node's children.
    #[inline]
    pub fn children(&self) -> Siblings {
        match (self.first_child(), self.last_child()) {
            (Some(first_child), Some(last_child)) => Siblings(Some(State {
                next: first_child,
                next_back: last_child,
            })),
            (None, None) => Siblings(None),
            _ => unreachable!(),
        }
    }

    /// Return an iterator of references to this node and its descendants, in tree order.
    ///
    /// Parent nodes appear before the descendants.
    ///
    /// Note: this is the `NodeEdge::Start` items from `traverse()`.
    #[inline]
    pub fn inclusive_descendants(&self) -> Descendants {
        Descendants(self.traverse_inclusive())
    }

    /// Return an iterator of references to this node's descendants, in tree order.
    ///
    /// Parent nodes appear before the descendants.
    ///
    /// Note: this is the `NodeEdge::Start` items from `traverse()`.
    #[inline]
    pub fn descendants(&self) -> Descendants {
        Descendants(self.traverse())
    }

    /// Return an iterator of the start and end edges of this node and its descendants,
    /// in tree order.
    #[inline]
    pub fn traverse_inclusive(&self) -> Traverse {
        Traverse(Some(State {
            next: NodeEdge::Start(self.clone()),
            next_back: NodeEdge::End(self.clone()),
        }))
    }

    /// Return an iterator of the start and end edges of this node's descendants,
    /// in tree order.
    #[inline]
    pub fn traverse(&self) -> Traverse {
        match (self.first_child(), self.last_child()) {
            (Some(first_child), Some(last_child)) => Traverse(Some(State {
                next: NodeEdge::Start(first_child),
                next_back: NodeEdge::End(last_child),
            })),
            (None, None) => Traverse(None),
            _ => unreachable!(),
        }
    }

    /// Return an iterator of the inclusive descendants element that match the given selector list.
    ///
    /// # Errors
    ///
    /// Returns `Err(())` if the selector string fails to parse.
    #[inline]
    pub fn select(&self, selectors: &str) -> Result<Select<Elements<Descendants>>, ()> {
        self.inclusive_descendants().select(selectors)
    }

    /// Return the first inclusive descendants element that match the given selector list.
    ///
    /// # Errors
    ///
    /// Returns `Err(())` if the selector string fails to parse or if no element matches.
    #[inline]
    pub fn select_first(&self, selectors: &str) -> Result<NodeDataRef<ElementData>, ()> {
        let mut elements = self.select(selectors)?;
        elements.next().ok_or(())
    }
}
