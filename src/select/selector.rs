use super::{BrikSelectors, Specificity};
use crate::node_data_ref::NodeDataRef;
use crate::tree::ElementData;
use selectors::context::QuirksMode;
use selectors::matching;
use selectors::parser::Selector as GenericSelector;
use std::fmt;

/// A pre-compiled CSS Selector.
pub struct Selector(pub(super) GenericSelector<BrikSelectors>);

/// Methods for Selector.
///
/// Provides selector matching and specificity calculation functionality.
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

/// Implements Display for Selector.
///
/// Formats the selector as a CSS selector string using the cssparser
/// serialization rules.
impl fmt::Display for Selector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use cssparser::ToCss;
        self.0.to_css(f)
    }
}

/// Implements Debug for Selector.
///
/// Delegates to Display to show the selector as a CSS string for
/// debugging purposes.
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

    /// Tests selector matching when the selector matches the element.
    ///
    /// Verifies that matches() returns true when an element has the
    /// class specified in the selector.
    #[test]
    fn matches_true() {
        let html = r#"<div class="test" id="myDiv">content</div>"#;
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        let selectors = Selectors::compile(".test").unwrap();
        assert!(selectors.0.first().unwrap().matches(&div));
    }

    /// Tests selector matching when the selector does not match the element.
    ///
    /// Verifies that matches() returns false when an element does not
    /// have the class specified in the selector.
    #[test]
    fn matches_false() {
        let html = r#"<div class="test">content</div>"#;
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        let selectors = Selectors::compile(".other").unwrap();
        assert!(!selectors.0.first().unwrap().matches(&div));
    }

    /// Tests specificity calculation for ID selectors.
    ///
    /// Verifies that an ID selector produces a non-zero specificity value,
    /// which is used for CSS cascade resolution.
    #[test]
    fn specificity_id() {
        let selectors = Selectors::compile("#myId").unwrap();
        let spec = selectors.0.first().unwrap().specificity();
        // ID selector has higher specificity than class
        assert!(spec.0 > 0);
    }

    /// Tests specificity calculation for class selectors.
    ///
    /// Verifies that a class selector produces a non-zero specificity value.
    #[test]
    fn specificity_class() {
        let selectors = Selectors::compile(".myClass").unwrap();
        let spec = selectors.0.first().unwrap().specificity();
        assert!(spec.0 > 0);
    }

    /// Tests Display formatting of selectors.
    ///
    /// Verifies that the Display implementation produces a CSS selector
    /// string containing the element and class components.
    #[test]
    fn display() {
        let selectors = Selectors::compile("div.test").unwrap();
        let display = format!("{}", selectors.0.first().unwrap());
        assert!(display.contains("div"));
        assert!(display.contains("test"));
    }

    /// Tests Debug formatting of selectors.
    ///
    /// Verifies that the Debug implementation produces output containing
    /// the element and ID components of the selector.
    #[test]
    fn debug() {
        let selectors = Selectors::compile("div#myId").unwrap();
        let debug = format!("{:?}", selectors.0.first().unwrap());
        assert!(debug.contains("div"));
        assert!(debug.contains("myId"));
    }
}
