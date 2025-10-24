use super::BrikSelectors;
use cssparser::ToCss;
use std::fmt;

/// CSS pseudo-elements (currently none are supported).
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum PseudoElement {}

impl ToCss for PseudoElement {
    fn to_css<W>(&self, _dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        match *self {}
    }
}

impl selectors::parser::PseudoElement for PseudoElement {
    type Impl = BrikSelectors;
}
