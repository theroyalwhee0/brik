use crate::node_data_ref::NodeDataRef;
use crate::select::Selectors;
use crate::tree::ElementData;
use std::borrow::Borrow;

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

#[cfg(test)]
mod tests {
    use crate::html5ever::tendril::TendrilSink;
    use crate::iter::NodeIterator;
    use crate::parse_html;

    /// Tests forward iteration through selected elements.
    ///
    /// Verifies that Select iterator correctly filters elements matching
    /// a CSS selector in forward order.
    #[test]
    fn select_forward() {
        let html = r#"<div><p class="test">1</p><span>2</span><p class="test">3</p></div>"#;
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        let mut select = div.as_node().descendants().select(".test").unwrap();

        let first = select.next().unwrap();
        assert_eq!(first.name.local.as_ref(), "p");

        let second = select.next().unwrap();
        assert_eq!(second.name.local.as_ref(), "p");

        assert!(select.next().is_none());
    }

    /// Tests backward iteration through selected elements.
    ///
    /// Verifies that Select iterator correctly filters elements matching
    /// a CSS selector in reverse order using next_back().
    #[test]
    fn select_backward() {
        let html = r#"<div><p class="test">1</p><span>2</span><p class="test">3</p></div>"#;
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        let mut select = div.as_node().descendants().select(".test").unwrap();

        let last = select.next_back().unwrap();
        assert_eq!(last.name.local.as_ref(), "p");

        let first = select.next_back().unwrap();
        assert_eq!(first.name.local.as_ref(), "p");

        assert!(select.next_back().is_none());
    }

    /// Tests select iterator with no matching elements.
    ///
    /// Verifies that Select iterator returns None when no elements
    /// match the given selector.
    #[test]
    fn select_no_matches() {
        let html = "<div><p>1</p><span>2</span></div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        let mut select = div.as_node().descendants().select(".nonexistent").unwrap();

        assert!(select.next().is_none());
    }
}
