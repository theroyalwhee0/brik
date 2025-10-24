use super::BrikSelectors;
use cssparser::ToCss;
use std::fmt;

/// CSS pseudo-elements (currently none are supported).
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum PseudoElement {}

/// Implements ToCss for PseudoElement.
///
/// Since no pseudo-elements are currently supported, the match is exhaustive
/// over the empty enum and never executes.
impl ToCss for PseudoElement {
    fn to_css<W>(&self, _dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        match *self {}
    }
}

/// Implements selectors::parser::PseudoElement for PseudoElement.
///
/// Associates this pseudo-element type with the BrikSelectors implementation
/// for CSS selector parsing and matching.
impl selectors::parser::PseudoElement for PseudoElement {
    type Impl = BrikSelectors;
}
