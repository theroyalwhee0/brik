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
