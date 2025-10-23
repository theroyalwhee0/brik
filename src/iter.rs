//! Node iterators

// Addressing this lint is a semver-breaking change.
// Remove this once the issue has been addressed.
#![allow(clippy::result_unit_err)]

use std::borrow::Borrow;
use std::cell::RefCell;
use std::iter::Rev;

use crate::node_data_ref::NodeDataRef;
use crate::select::Selectors;
use crate::tree::{ElementData, NodeRef};

impl NodeRef {
    /// Return an iterator of references to this node and its ancestors.
    #[inline]
    pub fn inclusive_ancestors(&self) -> Ancestors {
        Ancestors(Some(self.clone()))
    }

    /// Return an iterator of references to this node’s ancestors.
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

    /// Return an iterator of references to this node’s children.
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

    /// Return an iterator of references to this node’s descendants, in tree order.
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

    /// Return an iterator of the start and end edges of this node’s descendants,
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

/// Internal state for double-ended iterators.
#[derive(Debug, Clone)]
struct State<T> {
    /// The next item to be returned from the front of the iterator.
    next: T,
    /// The next item to be returned from the back of the iterator.
    next_back: T,
}

/// A double-ended iterator of sibling nodes.
#[derive(Debug, Clone)]
pub struct Siblings(Option<State<NodeRef>>);

/// Macro to implement iterator methods for sibling traversal.
macro_rules! siblings_next {
    ($next: ident, $next_back: ident, $next_sibling: ident) => {
        fn $next(&mut self) -> Option<NodeRef> {
            #![allow(non_shorthand_field_patterns)]
            self.0.take().map(
                |State {
                     $next: next,
                     $next_back: next_back,
                 }| {
                    if let Some(sibling) = next.$next_sibling() {
                        if next != next_back {
                            self.0 = Some(State {
                                $next: sibling,
                                $next_back: next_back,
                            })
                        }
                    }
                    next
                },
            )
        }
    };
}

impl Iterator for Siblings {
    type Item = NodeRef;
    siblings_next!(next, next_back, next_sibling);
}

impl DoubleEndedIterator for Siblings {
    siblings_next!(next_back, next, previous_sibling);
}

/// An iterator on ancestor nodes.
#[derive(Debug, Clone)]
pub struct Ancestors(Option<NodeRef>);

impl Iterator for Ancestors {
    type Item = NodeRef;

    #[inline]
    fn next(&mut self) -> Option<NodeRef> {
        self.0.take().inspect(|node| {
            self.0 = node.parent();
        })
    }
}

/// An iterator of references to a given node and its descendants, in tree order.
#[derive(Debug, Clone)]
pub struct Descendants(Traverse);

/// Macro to implement iterator methods for descendant traversal.
macro_rules! descendants_next {
    ($next: ident) => {
        #[inline]
        fn $next(&mut self) -> Option<NodeRef> {
            loop {
                match (self.0).$next() {
                    Some(NodeEdge::Start(node)) => return Some(node),
                    Some(NodeEdge::End(_)) => {}
                    None => return None,
                }
            }
        }
    };
}

impl Iterator for Descendants {
    type Item = NodeRef;
    descendants_next!(next);
}

impl DoubleEndedIterator for Descendants {
    descendants_next!(next_back);
}

/// Marks either the start or the end of a node.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NodeEdge<T> {
    /// Indicates that start of a node that has children.
    /// Yielded by `Traverse::next` before the node’s descendants.
    /// In HTML or XML, this corresponds to an opening tag like `<div>`
    Start(T),

    /// Indicates that end of a node that has children.
    /// Yielded by `Traverse::next` after the node’s descendants.
    /// In HTML or XML, this corresponds to a closing tag like `</div>`
    End(T),
}

/// An iterator of the start and end edges of the nodes in a given subtree.
#[derive(Debug, Clone)]
pub struct Traverse(Option<State<NodeEdge<NodeRef>>>);

/// Macro to implement iterator methods for tree traversal with start/end edges.
macro_rules! traverse_next {
    ($next: ident, $next_back: ident, $first_child: ident, $next_sibling: ident, $Start: ident, $End: ident) => {
        fn $next(&mut self) -> Option<NodeEdge<NodeRef>> {
            #![allow(non_shorthand_field_patterns)]
            self.0.take().map(
                |State {
                     $next: next,
                     $next_back: next_back,
                 }| {
                    if next != next_back {
                        self.0 = match next {
                            NodeEdge::$Start(ref node) => match node.$first_child() {
                                Some(child) => Some(State {
                                    $next: NodeEdge::$Start(child),
                                    $next_back: next_back,
                                }),
                                None => Some(State {
                                    $next: NodeEdge::$End(node.clone()),
                                    $next_back: next_back,
                                }),
                            },
                            NodeEdge::$End(ref node) => match node.$next_sibling() {
                                Some(sibling) => Some(State {
                                    $next: NodeEdge::$Start(sibling),
                                    $next_back: next_back,
                                }),
                                None => node.parent().map(|parent| State {
                                    $next: NodeEdge::$End(parent),
                                    $next_back: next_back,
                                }),
                            },
                        };
                    }
                    next
                },
            )
        }
    };
}

impl Iterator for Traverse {
    type Item = NodeEdge<NodeRef>;
    traverse_next!(next, next_back, first_child, next_sibling, Start, End);
}

impl DoubleEndedIterator for Traverse {
    traverse_next!(next_back, next, last_child, previous_sibling, End, Start);
}

/// Macro to create filter-map-like iterator wrappers.
macro_rules! filter_map_like_iterator {
    (#[$doc: meta] $name: ident: $f: expr, $from: ty => $to: ty) => {
        #[$doc]
        #[derive(Debug, Clone)]
        pub struct $name<I>(pub I);

        impl<I> Iterator for $name<I>
        where
            I: Iterator<Item = $from>,
        {
            type Item = $to;

            #[inline]
            fn next(&mut self) -> Option<$to> {
                for x in self.0.by_ref() {
                    if let Some(y) = ($f)(x) {
                        return Some(y);
                    }
                }
                None
            }
        }

        impl<I> DoubleEndedIterator for $name<I>
        where
            I: DoubleEndedIterator<Item = $from>,
        {
            #[inline]
            fn next_back(&mut self) -> Option<$to> {
                for x in self.0.by_ref().rev() {
                    if let Some(y) = ($f)(x) {
                        return Some(y);
                    }
                }
                None
            }
        }
    };
}

filter_map_like_iterator! {
    /// A node iterator adaptor that yields element nodes.
    Elements: NodeRef::into_element_ref, NodeRef => NodeDataRef<ElementData>
}

filter_map_like_iterator! {
    /// A node iterator adaptor that yields comment nodes.
    Comments: NodeRef::into_comment_ref, NodeRef => NodeDataRef<RefCell<String>>
}

filter_map_like_iterator! {
    /// A node iterator adaptor that yields text nodes.
    TextNodes: NodeRef::into_text_ref, NodeRef => NodeDataRef<RefCell<String>>
}

/// An element iterator adaptor that yields elements in a specific namespace.
#[derive(Debug, Clone)]
pub struct ElementsInNamespace<I> {
    /// The underlying iterator.
    iter: I,
    /// The namespace to filter by.
    namespace: html5ever::Namespace,
}

impl<I> Iterator for ElementsInNamespace<I>
where
    I: Iterator<Item = NodeDataRef<ElementData>>,
{
    type Item = NodeDataRef<ElementData>;

    #[inline]
    fn next(&mut self) -> Option<NodeDataRef<ElementData>> {
        let namespace = &self.namespace;
        self.iter
            .by_ref()
            .find(|element| element.namespace_uri() == namespace)
    }
}

impl<I> DoubleEndedIterator for ElementsInNamespace<I>
where
    I: DoubleEndedIterator<Item = NodeDataRef<ElementData>>,
{
    #[inline]
    fn next_back(&mut self) -> Option<NodeDataRef<ElementData>> {
        let namespace = &self.namespace;
        self.iter
            .by_ref()
            .rev()
            .find(|element| element.namespace_uri() == namespace)
    }
}

/// An element iterator adaptor that yields elements maching given selectors.
pub struct Select<I, S = Selectors>
where
    I: Iterator<Item = NodeDataRef<ElementData>>,
    S: Borrow<Selectors>,
{
    /// The underlying iterator.
    pub iter: I,

    /// The selectors to be matched.
    pub selectors: S,
}

impl<I, S> Iterator for Select<I, S>
where
    I: Iterator<Item = NodeDataRef<ElementData>>,
    S: Borrow<Selectors>,
{
    type Item = NodeDataRef<ElementData>;

    #[inline]
    fn next(&mut self) -> Option<NodeDataRef<ElementData>> {
        let selectors = self.selectors.borrow();
        self.iter
            .by_ref()
            .find(|element| selectors.matches(element))
    }
}

impl<I, S> DoubleEndedIterator for Select<I, S>
where
    I: DoubleEndedIterator<Item = NodeDataRef<ElementData>>,
    S: Borrow<Selectors>,
{
    #[inline]
    fn next_back(&mut self) -> Option<NodeDataRef<ElementData>> {
        let selectors = self.selectors.borrow();
        self.iter
            .by_ref()
            .rev()
            .find(|element| selectors.matches(element))
    }
}

/// Convenience methods for node iterators.
pub trait NodeIterator: Sized + Iterator<Item = NodeRef> {
    /// Filter this element iterator to elements.
    #[inline]
    fn elements(self) -> Elements<Self> {
        Elements(self)
    }

    /// Filter this node iterator to text nodes.
    #[inline]
    fn text_nodes(self) -> TextNodes<Self> {
        TextNodes(self)
    }

    /// Filter this node iterator to comment nodes.
    #[inline]
    fn comments(self) -> Comments<Self> {
        Comments(self)
    }

    /// Filter this node iterator to elements maching the given selectors.
    ///
    /// # Errors
    ///
    /// Returns `Err(())` if the selector string fails to parse.
    #[inline]
    fn select(self, selectors: &str) -> Result<Select<Elements<Self>>, ()> {
        self.elements().select(selectors)
    }

    /// Detach all nodes in this iterator from their parents.
    ///
    /// # Examples
    ///
    /// ```
    /// use brik::parse_html;
    /// use brik::traits::*;
    ///
    /// let html = r#"<div><p>One</p><p>Two</p><p>Three</p></div>"#;
    ///
    /// let doc = parse_html().one(html);
    /// let div = doc.select_first("div").unwrap();
    ///
    /// // Detach all paragraph elements
    /// let paragraphs: Vec<_> = div
    ///     .as_node()
    ///     .descendants()
    ///     .select("p")
    ///     .unwrap()
    ///     .collect();
    ///
    /// paragraphs
    ///     .into_iter()
    ///     .map(|p| p.as_node().clone())
    ///     .detach_all();
    ///
    /// assert_eq!(div.as_node().children().elements().count(), 0);
    /// ```
    #[inline]
    fn detach_all(self) {
        for node in self {
            node.detach();
        }
    }
}

/// Convenience methods for element iterators.
pub trait ElementIterator: Sized + Iterator<Item = NodeDataRef<ElementData>> {
    /// Filter this element iterator to elements maching the given selectors.
    ///
    /// # Errors
    ///
    /// Returns `Err(())` if the selector string fails to parse.
    #[inline]
    fn select(self, selectors: &str) -> Result<Select<Self>, ()> {
        Selectors::compile(selectors).map(|s| Select {
            iter: self,
            selectors: s,
        })
    }

    /// Filter this element iterator to elements in the given namespace.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate html5ever;
    /// use brik::parse_html;
    /// use brik::traits::*;
    ///
    /// let html = r#"<!DOCTYPE html>
    /// <html>
    /// <body>
    ///   <div>HTML element</div>
    ///   <svg xmlns="http://www.w3.org/2000/svg">
    ///     <circle r="10"/>
    ///     <rect width="20" height="20"/>
    ///   </svg>
    /// </body>
    /// </html>
    /// "#;
    ///
    /// let doc = parse_html().one(html);
    ///
    /// // Find all SVG elements
    /// let svg_elements: Vec<_> = doc
    ///     .descendants()
    ///     .elements()
    ///     .elements_in_ns(ns!(svg))
    ///     .collect();
    ///
    /// assert_eq!(svg_elements.len(), 3); // svg, circle, rect
    /// ```
    #[inline]
    fn elements_in_ns(self, namespace: html5ever::Namespace) -> ElementsInNamespace<Self> {
        ElementsInNamespace {
            iter: self,
            namespace,
        }
    }
}

impl<I> NodeIterator for I where I: Iterator<Item = NodeRef> {}
impl<I> ElementIterator for I where I: Iterator<Item = NodeDataRef<ElementData>> {}

#[cfg(test)]
mod tests {
    use crate::parser::parse_html;
    use crate::traits::*;

    #[test]
    fn elements_in_ns_filters_by_namespace() {
        let html = r#"<!DOCTYPE html>
<html>
<body>
  <div>HTML element 1</div>
  <svg xmlns="http://www.w3.org/2000/svg">
    <circle r="10"/>
    <rect width="20" height="20"/>
  </svg>
  <p>HTML element 2</p>
</body>
</html>"#;

        let doc = parse_html().one(html);

        // Find all SVG elements
        let svg_elements: Vec<_> = doc
            .descendants()
            .elements()
            .elements_in_ns(ns!(svg))
            .collect();

        assert_eq!(svg_elements.len(), 3); // svg, circle, rect
        assert!(svg_elements.iter().all(|e| e.namespace_uri() == &ns!(svg)));
    }

    #[test]
    fn elements_in_ns_empty_when_no_match() {
        let html = r#"<div><p>Only HTML elements</p></div>"#;
        let doc = parse_html().one(html);

        let svg_elements: Vec<_> = doc
            .descendants()
            .elements()
            .elements_in_ns(ns!(svg))
            .collect();

        assert_eq!(svg_elements.len(), 0);
    }

    #[test]
    fn elements_in_ns_works_with_nested_elements() {
        let html = r#"<!DOCTYPE html>
<html>
<body>
  <svg xmlns="http://www.w3.org/2000/svg">
    <g>
      <circle r="10"/>
      <circle r="20"/>
    </g>
  </svg>
</body>
</html>"#;

        let doc = parse_html().one(html);

        let svg_elements: Vec<_> = doc
            .descendants()
            .elements()
            .elements_in_ns(ns!(svg))
            .collect();

        // svg, g, circle, circle
        assert_eq!(svg_elements.len(), 4);
    }

    #[test]
    fn elements_in_ns_double_ended_iteration() {
        let html = r#"<!DOCTYPE html>
<html>
<body>
  <svg xmlns="http://www.w3.org/2000/svg">
    <circle r="10"/>
    <rect width="20" height="20"/>
    <line x1="0" y1="0" x2="10" y2="10"/>
  </svg>
</body>
</html>"#;

        let doc = parse_html().one(html);

        let mut svg_elements = doc
            .descendants()
            .elements()
            .elements_in_ns(ns!(svg));

        // Test forward iteration
        let first = svg_elements.next().unwrap();
        assert_eq!(first.local_name().as_ref(), "svg");

        // Test reverse iteration
        let last = svg_elements.next_back().unwrap();
        assert_eq!(last.local_name().as_ref(), "line");
    }

    #[test]
    fn detach_all_removes_elements() {
        let html = r#"<div><p>One</p><p>Two</p><p>Three</p></div>"#;
        let doc = parse_html().one(html);
        let div = doc.select_first("div").unwrap();

        let initial_count = div.as_node().children().elements().count();
        assert_eq!(initial_count, 3);

        // Detach all paragraph elements
        let paragraphs: Vec<_> = div
            .as_node()
            .descendants()
            .elements()
            .filter(|e| e.local_name().as_ref() == "p")
            .collect();

        paragraphs
            .into_iter()
            .map(|p| p.as_node().clone())
            .detach_all();

        assert_eq!(div.as_node().children().elements().count(), 0);
    }

    #[test]
    fn detach_all_with_empty_iterator() {
        let html = r#"<div><p>Test</p></div>"#;
        let doc = parse_html().one(html);

        // Try to detach elements that don't exist - should not panic
        doc
            .descendants()
            .elements()
            .filter(|e| e.local_name().as_ref() == "nonexistent")
            .map(|e| e.as_node().clone())
            .detach_all();
    }

    #[test]
    fn detach_all_with_mixed_namespaces() {
        let html = r#"<!DOCTYPE html>
<html>
<body>
  <div>HTML</div>
  <svg xmlns="http://www.w3.org/2000/svg">
    <circle r="10"/>
  </svg>
  <p>More HTML</p>
</body>
</html>"#;

        let doc = parse_html().one(html);
        let body = doc.select_first("body").unwrap();

        // Detach only SVG elements
        let svg_elements: Vec<_> = body
            .as_node()
            .descendants()
            .elements()
            .elements_in_ns(ns!(svg))
            .collect();

        svg_elements
            .into_iter()
            .map(|e| e.as_node().clone())
            .detach_all();

        // HTML elements should still be present
        let remaining_html: Vec<_> = body
            .as_node()
            .descendants()
            .elements()
            .elements_in_ns(ns!(html))
            .collect();

        assert_eq!(remaining_html.len(), 2); // div, p

        // SVG elements should be gone
        let remaining_svg: Vec<_> = body
            .as_node()
            .descendants()
            .elements()
            .elements_in_ns(ns!(svg))
            .collect();

        assert_eq!(remaining_svg.len(), 0);
    }
}
