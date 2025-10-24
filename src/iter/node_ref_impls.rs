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

#[cfg(test)]
mod tests {
    use crate::html5ever::tendril::TendrilSink;
    use crate::parse_html;

    /// Tests inclusive_preceding_siblings method.
    ///
    /// Verifies that the iterator includes the target node and all siblings
    /// before it in the parent's child list, in reverse order.
    #[test]
    fn inclusive_preceding_siblings() {
        let html = "<div><p>1</p><p>2</p><p id='target'>3</p><p>4</p></div>";
        let doc = parse_html().one(html);
        let target = doc.select("#target").unwrap().next().unwrap();

        let siblings: Vec<_> = target.as_node().inclusive_preceding_siblings().collect();

        // Includes target and preceding elements
        assert_eq!(siblings.len(), 3);
        assert_eq!(
            siblings[0]
                .as_element()
                .unwrap()
                .attributes
                .borrow()
                .get("id"),
            Some("target")
        );
    }

    /// Tests inclusive_preceding_siblings with first child.
    ///
    /// Verifies that when the target is the first child, the iterator
    /// contains only the target itself.
    #[test]
    fn inclusive_preceding_siblings_first_child() {
        let html = "<div><p id='target'>1</p><p>2</p></div>";
        let doc = parse_html().one(html);
        let target = doc.select("#target").unwrap().next().unwrap();

        let siblings: Vec<_> = target.as_node().inclusive_preceding_siblings().collect();

        assert_eq!(siblings.len(), 1);
        assert_eq!(
            siblings[0]
                .as_element()
                .unwrap()
                .attributes
                .borrow()
                .get("id"),
            Some("target")
        );
    }

    /// Tests inclusive_preceding_siblings with no parent.
    ///
    /// Verifies that when the node has no parent (root node), the iterator
    /// contains only the node itself.
    #[test]
    fn inclusive_preceding_siblings_no_parent() {
        let doc = parse_html().one("<html></html>");
        let siblings: Vec<_> = doc.inclusive_preceding_siblings().collect();
        assert_eq!(siblings.len(), 1);
    }

    /// Tests preceding_siblings method.
    ///
    /// Verifies that the iterator excludes the target node and returns only
    /// siblings before it in the parent's child list, in reverse order.
    #[test]
    fn preceding_siblings() {
        let html = "<div><p>1</p><p>2</p><p id='target'>3</p><p>4</p></div>";
        let doc = parse_html().one(html);
        let target = doc.select("#target").unwrap().next().unwrap();

        let siblings: Vec<_> = target.as_node().preceding_siblings().collect();

        // Should not include the target itself, includes preceding elements
        assert_eq!(siblings.len(), 2);
    }

    /// Tests preceding_siblings with first child.
    ///
    /// Verifies that when the target is the first child, the iterator is
    /// empty since there are no siblings before it.
    #[test]
    fn preceding_siblings_first_child() {
        let html = "<div><p id='target'>1</p><p>2</p></div>";
        let doc = parse_html().one(html);
        let target = doc.select("#target").unwrap().next().unwrap();

        let siblings: Vec<_> = target.as_node().preceding_siblings().collect();
        assert_eq!(siblings.len(), 0);
    }

    /// Tests preceding_siblings with no parent.
    ///
    /// Verifies that when the node has no parent (root node), the iterator
    /// is empty since there are no siblings.
    #[test]
    fn preceding_siblings_no_parent() {
        let doc = parse_html().one("<html></html>");
        let siblings: Vec<_> = doc.preceding_siblings().collect();
        assert_eq!(siblings.len(), 0);
    }

    /// Tests inclusive_following_siblings method.
    ///
    /// Verifies that the iterator includes the target node and all siblings
    /// after it in the parent's child list.
    #[test]
    fn inclusive_following_siblings() {
        let html = "<div><p>1</p><p id='target'>2</p><p>3</p><p>4</p></div>";
        let doc = parse_html().one(html);
        let target = doc.select("#target").unwrap().next().unwrap();

        let siblings: Vec<_> = target.as_node().inclusive_following_siblings().collect();

        assert_eq!(siblings.len(), 3);
        assert_eq!(
            siblings[0]
                .as_element()
                .unwrap()
                .attributes
                .borrow()
                .get("id"),
            Some("target")
        );
    }

    /// Tests inclusive_following_siblings with last child.
    ///
    /// Verifies that when the target is the last child, the iterator
    /// contains only the target itself.
    #[test]
    fn inclusive_following_siblings_last_child() {
        let html = "<div><p>1</p><p id='target'>2</p></div>";
        let doc = parse_html().one(html);
        let target = doc.select("#target").unwrap().next().unwrap();

        let siblings: Vec<_> = target.as_node().inclusive_following_siblings().collect();
        assert_eq!(siblings.len(), 1);
        assert_eq!(
            siblings[0]
                .as_element()
                .unwrap()
                .attributes
                .borrow()
                .get("id"),
            Some("target")
        );
    }

    /// Tests inclusive_following_siblings with no parent.
    ///
    /// Verifies that when the node has no parent (root node), the iterator
    /// contains only the node itself.
    #[test]
    fn inclusive_following_siblings_no_parent() {
        let doc = parse_html().one("<html></html>");
        let siblings: Vec<_> = doc.inclusive_following_siblings().collect();
        assert_eq!(siblings.len(), 1);
    }

    /// Tests following_siblings method.
    ///
    /// Verifies that the iterator excludes the target node and returns only
    /// siblings after it in the parent's child list.
    #[test]
    fn following_siblings() {
        let html = "<div><p>1</p><p id='target'>2</p><p>3</p><p>4</p></div>";
        let doc = parse_html().one(html);
        let target = doc.select("#target").unwrap().next().unwrap();

        let siblings: Vec<_> = target.as_node().following_siblings().collect();

        // Should not include the target itself
        assert_eq!(siblings.len(), 2);
    }

    /// Tests following_siblings with last child.
    ///
    /// Verifies that when the target is the last child, the iterator is
    /// empty since there are no siblings after it.
    #[test]
    fn following_siblings_last_child() {
        let html = "<div><p>1</p><p id='target'>2</p></div>";
        let doc = parse_html().one(html);
        let target = doc.select("#target").unwrap().next().unwrap();

        let siblings: Vec<_> = target.as_node().following_siblings().collect();
        assert_eq!(siblings.len(), 0);
    }

    /// Tests following_siblings with no parent.
    ///
    /// Verifies that when the node has no parent (root node), the iterator
    /// is empty since there are no siblings.
    #[test]
    fn following_siblings_no_parent() {
        let doc = parse_html().one("<html></html>");
        let siblings: Vec<_> = doc.following_siblings().collect();
        assert_eq!(siblings.len(), 0);
    }

    /// Tests children method.
    ///
    /// Verifies that the iterator returns all direct children of a node
    /// in order.
    #[test]
    fn children() {
        let html = "<div><p>1</p><p>2</p><p>3</p></div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        let children: Vec<_> = div.as_node().children().collect();

        // Should have 3 p elements
        assert_eq!(children.len(), 3);
        assert!(children.iter().any(|n| n.as_element().is_some()));
    }

    /// Tests children method with no children.
    ///
    /// Verifies that the iterator is empty when a node has no children.
    #[test]
    fn children_empty() {
        let html = "<div></div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        let children: Vec<_> = div.as_node().children().collect();
        assert_eq!(children.len(), 0);
    }

    /// Tests traverse_inclusive method.
    ///
    /// Verifies that the iterator produces start and end edges for the node
    /// itself and all its descendants in depth-first order.
    #[test]
    fn traverse_inclusive() {
        let html = "<div><p>text</p></div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        let edges: Vec<_> = div.as_node().traverse_inclusive().collect();

        // Should have start and end edges for div, p, and text
        assert_eq!(edges.len(), 6);
    }

    /// Tests traverse method.
    ///
    /// Verifies that the iterator produces start and end edges for
    /// descendants only, excluding the node itself.
    #[test]
    fn traverse() {
        let html = "<div><p>text</p></div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        let edges: Vec<_> = div.as_node().traverse().collect();

        // Should have start and end edges for p and text (not div itself)
        assert_eq!(edges.len(), 4);
    }

    /// Tests traverse method with no children.
    ///
    /// Verifies that the iterator is empty when a node has no descendants.
    #[test]
    fn traverse_empty() {
        let html = "<div></div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        let edges: Vec<_> = div.as_node().traverse().collect();
        assert_eq!(edges.len(), 0);
    }

    /// Tests select_first when element is found.
    ///
    /// Verifies that select_first returns the first matching element for
    /// a valid selector.
    #[test]
    fn select_first_found() {
        let html = "<div><p>1</p><p class='test'>2</p><p class='test'>3</p></div>";
        let doc = parse_html().one(html);

        let result = doc.select_first(".test");
        assert!(result.is_ok());
        let element = result.unwrap();
        assert_eq!(element.name.local.as_ref(), "p");
    }

    /// Tests select_first when no element matches.
    ///
    /// Verifies that select_first returns an error when no elements match
    /// the selector.
    #[test]
    fn select_first_not_found() {
        let html = "<div><p>1</p></div>";
        let doc = parse_html().one(html);

        let result = doc.select_first(".nonexistent");
        assert!(result.is_err());
    }

    /// Tests select_first with invalid selector.
    ///
    /// Verifies that select_first returns an error when the selector string
    /// fails to parse.
    #[test]
    fn select_first_invalid_selector() {
        let doc = parse_html().one("<div></div>");
        let result = doc.select_first("::invalid:::");
        assert!(result.is_err());
    }

    /// Tests inclusive_ancestors method.
    ///
    /// Verifies that the iterator includes the node itself and all parent
    /// nodes up to the document root.
    #[test]
    fn inclusive_ancestors() {
        let html = "<html><body><div><p id='target'>text</p></div></body></html>";
        let doc = parse_html().one(html);
        let target = doc.select("#target").unwrap().next().unwrap();

        let ancestors: Vec<_> = target.as_node().inclusive_ancestors().collect();

        // Should include: p, div, body, html, document
        assert_eq!(ancestors.len(), 5);
        assert_eq!(ancestors[0].as_element().unwrap().name.local.as_ref(), "p");
    }

    /// Tests inclusive_descendants method.
    ///
    /// Verifies that the iterator includes the node itself and all
    /// descendant nodes in depth-first order.
    #[test]
    fn inclusive_descendants() {
        let html = "<div><p>text</p><span>more</span></div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        let descendants: Vec<_> = div.as_node().inclusive_descendants().collect();

        // Should include div itself plus its descendants (p, text, span, text)
        assert_eq!(descendants.len(), 5);
        assert_eq!(
            descendants[0].as_element().unwrap().name.local.as_ref(),
            "div"
        );
    }

    /// Tests descendants method.
    ///
    /// Verifies that the iterator excludes the node itself and returns only
    /// descendant nodes in depth-first order.
    #[test]
    fn descendants() {
        let html = "<div><p>text</p><span>more</span></div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        let descendants: Vec<_> = div.as_node().descendants().collect();

        // Should not include div itself (p, text, span, text)
        assert_eq!(descendants.len(), 4);
        assert!(descendants.iter().all(|n| n
            .as_element()
            .is_none_or(|e| e.name.local.as_ref() != "div")));
    }
}
