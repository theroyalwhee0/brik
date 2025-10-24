use super::{Comments, ElementIterator, Elements, Select, TextNodes};
use crate::tree::NodeRef;

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

impl<I> NodeIterator for I where I: Iterator<Item = NodeRef> {}

#[cfg(test)]
mod tests {
    use crate::html5ever::tendril::TendrilSink;
    use crate::iter::NodeIterator;
    use crate::parse_html;

    /// Tests filtering iterator to text nodes.
    ///
    /// Verifies that text_nodes() correctly filters a node iterator to
    /// include only text nodes, finding all text content in the tree.
    #[test]
    fn text_nodes() {
        let html = "<div>text1<p>text2</p>text3</div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        let text_nodes: Vec<_> = div.as_node().descendants().text_nodes().collect();

        assert_eq!(text_nodes.len(), 3);
    }

    /// Tests filtering iterator to comment nodes.
    ///
    /// Verifies that comments() correctly filters a node iterator to
    /// include only comment nodes.
    #[test]
    fn comments() {
        let html = "<div><!-- comment1 --><p>text</p><!-- comment2 --></div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        let comments: Vec<_> = div.as_node().descendants().comments().collect();

        assert_eq!(comments.len(), 2);
    }

    /// Tests detaching all nodes in an iterator.
    ///
    /// Verifies that detach_all() removes all nodes in the iterator
    /// from their parents, leaving the parent element empty.
    #[test]
    fn detach_all() {
        let html = "<div><p>One</p><p>Two</p><p>Three</p></div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        let paragraphs: Vec<_> = div.as_node().descendants().select("p").unwrap().collect();

        paragraphs
            .into_iter()
            .map(|p| p.as_node().clone())
            .detach_all();

        assert_eq!(div.as_node().children().elements().count(), 0);
    }
}
