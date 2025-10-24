use super::{BrikSelectors, Specificity};
use crate::node_data_ref::NodeDataRef;
use crate::tree::ElementData;
use selectors::context::QuirksMode;
use selectors::matching;
use selectors::parser::Selector as GenericSelector;
use std::fmt;

/// A pre-compiled CSS Selector.
pub struct Selector(pub(super) GenericSelector<BrikSelectors>);

impl Selector {
    /// Returns whether the given element matches this selector.
    #[inline]
    pub fn matches(&self, element: &NodeDataRef<ElementData>) -> bool {
        let mut selector_caches = matching::SelectorCaches::default();
        let mut context = matching::MatchingContext::new(
            matching::MatchingMode::Normal,
            None,
            &mut selector_caches,
            QuirksMode::NoQuirks,
            matching::NeedsSelectorFlags::No,
            matching::MatchingForInvalidation::No,
        );
        matching::matches_selector(&self.0, 0, None, element, &mut context)
    }

    /// Return the specificity of this selector.
    pub fn specificity(&self) -> Specificity {
        Specificity(self.0.specificity())
    }
}

impl fmt::Display for Selector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use cssparser::ToCss;
        self.0.to_css(f)
    }
}

impl fmt::Debug for Selector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use crate::html5ever::tendril::TendrilSink;
    use crate::parse_html;
    use crate::select::Selectors;

    #[test]
    fn matches_true() {
        let html = r#"<div class="test" id="myDiv">content</div>"#;
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        let selectors = Selectors::compile(".test").unwrap();
        assert!(selectors.0.first().unwrap().matches(&div));
    }

    #[test]
    fn matches_false() {
        let html = r#"<div class="test">content</div>"#;
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        let selectors = Selectors::compile(".other").unwrap();
        assert!(!selectors.0.first().unwrap().matches(&div));
    }

    #[test]
    fn specificity_id() {
        let selectors = Selectors::compile("#myId").unwrap();
        let spec = selectors.0.first().unwrap().specificity();
        // ID selector has higher specificity than class
        assert!(spec.0 > 0);
    }

    #[test]
    fn specificity_class() {
        let selectors = Selectors::compile(".myClass").unwrap();
        let spec = selectors.0.first().unwrap().specificity();
        assert!(spec.0 > 0);
    }

    #[test]
    fn display() {
        let selectors = Selectors::compile("div.test").unwrap();
        let display = format!("{}", selectors.0.first().unwrap());
        assert!(display.contains("div"));
        assert!(display.contains("test"));
    }

    #[test]
    fn debug() {
        let selectors = Selectors::compile("div#myId").unwrap();
        let debug = format!("{:?}", selectors.0.first().unwrap());
        assert!(debug.contains("div"));
        assert!(debug.contains("myId"));
    }
}
