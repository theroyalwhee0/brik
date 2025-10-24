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
