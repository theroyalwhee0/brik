use super::BrikSelectors;
use cssparser::ToCss;
use selectors::parser::NonTSPseudoClass;
use std::fmt;

/// Supported CSS pseudo-classes for element matching.
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum PseudoClass {
    /// Matches `:any-link` (any link element).
    AnyLink,
    /// Matches `:link` (unvisited links).
    Link,
    /// Matches `:visited` (visited links).
    Visited,
    /// Matches `:active` (activated elements).
    Active,
    /// Matches `:focus` (focused elements).
    Focus,
    /// Matches `:hover` (hovered elements).
    Hover,
    /// Matches `:enabled` (enabled form elements).
    Enabled,
    /// Matches `:disabled` (disabled form elements).
    Disabled,
    /// Matches `:checked` (checked form elements).
    Checked,
    /// Matches `:indeterminate` (indeterminate form elements).
    Indeterminate,
}

impl NonTSPseudoClass for PseudoClass {
    type Impl = BrikSelectors;

    fn is_active_or_hover(&self) -> bool {
        matches!(*self, PseudoClass::Active | PseudoClass::Hover)
    }

    fn is_user_action_state(&self) -> bool {
        matches!(
            *self,
            PseudoClass::Active | PseudoClass::Hover | PseudoClass::Focus
        )
    }
}

impl ToCss for PseudoClass {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        dest.write_str(match *self {
            PseudoClass::AnyLink => ":any-link",
            PseudoClass::Link => ":link",
            PseudoClass::Visited => ":visited",
            PseudoClass::Active => ":active",
            PseudoClass::Focus => ":focus",
            PseudoClass::Hover => ":hover",
            PseudoClass::Enabled => ":enabled",
            PseudoClass::Disabled => ":disabled",
            PseudoClass::Checked => ":checked",
            PseudoClass::Indeterminate => ":indeterminate",
        })
    }
}
