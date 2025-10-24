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

#[cfg(test)]
mod tests {
    use super::*;
    use cssparser::ToCss;
    use selectors::parser::NonTSPseudoClass;

    #[test]
    fn is_active_or_hover() {
        assert!(PseudoClass::Active.is_active_or_hover());
        assert!(PseudoClass::Hover.is_active_or_hover());
        assert!(!PseudoClass::Focus.is_active_or_hover());
        assert!(!PseudoClass::Link.is_active_or_hover());
        assert!(!PseudoClass::Visited.is_active_or_hover());
        assert!(!PseudoClass::AnyLink.is_active_or_hover());
        assert!(!PseudoClass::Enabled.is_active_or_hover());
        assert!(!PseudoClass::Disabled.is_active_or_hover());
        assert!(!PseudoClass::Checked.is_active_or_hover());
        assert!(!PseudoClass::Indeterminate.is_active_or_hover());
    }

    #[test]
    fn is_user_action_state() {
        assert!(PseudoClass::Active.is_user_action_state());
        assert!(PseudoClass::Hover.is_user_action_state());
        assert!(PseudoClass::Focus.is_user_action_state());
        assert!(!PseudoClass::Link.is_user_action_state());
        assert!(!PseudoClass::Visited.is_user_action_state());
        assert!(!PseudoClass::AnyLink.is_user_action_state());
        assert!(!PseudoClass::Enabled.is_user_action_state());
        assert!(!PseudoClass::Disabled.is_user_action_state());
        assert!(!PseudoClass::Checked.is_user_action_state());
        assert!(!PseudoClass::Indeterminate.is_user_action_state());
    }

    #[test]
    fn to_css_any_link() {
        let mut output = String::new();
        PseudoClass::AnyLink.to_css(&mut output).unwrap();
        assert_eq!(output, ":any-link");
    }

    #[test]
    fn to_css_link() {
        let mut output = String::new();
        PseudoClass::Link.to_css(&mut output).unwrap();
        assert_eq!(output, ":link");
    }

    #[test]
    fn to_css_visited() {
        let mut output = String::new();
        PseudoClass::Visited.to_css(&mut output).unwrap();
        assert_eq!(output, ":visited");
    }

    #[test]
    fn to_css_active() {
        let mut output = String::new();
        PseudoClass::Active.to_css(&mut output).unwrap();
        assert_eq!(output, ":active");
    }

    #[test]
    fn to_css_focus() {
        let mut output = String::new();
        PseudoClass::Focus.to_css(&mut output).unwrap();
        assert_eq!(output, ":focus");
    }

    #[test]
    fn to_css_hover() {
        let mut output = String::new();
        PseudoClass::Hover.to_css(&mut output).unwrap();
        assert_eq!(output, ":hover");
    }

    #[test]
    fn to_css_enabled() {
        let mut output = String::new();
        PseudoClass::Enabled.to_css(&mut output).unwrap();
        assert_eq!(output, ":enabled");
    }

    #[test]
    fn to_css_disabled() {
        let mut output = String::new();
        PseudoClass::Disabled.to_css(&mut output).unwrap();
        assert_eq!(output, ":disabled");
    }

    #[test]
    fn to_css_checked() {
        let mut output = String::new();
        PseudoClass::Checked.to_css(&mut output).unwrap();
        assert_eq!(output, ":checked");
    }

    #[test]
    fn to_css_indeterminate() {
        let mut output = String::new();
        PseudoClass::Indeterminate.to_css(&mut output).unwrap();
        assert_eq!(output, ":indeterminate");
    }

    #[test]
    fn clone() {
        let pc1 = PseudoClass::Active;
        let pc2 = pc1.clone();
        assert_eq!(pc1, pc2);
    }

    #[test]
    fn eq() {
        assert_eq!(PseudoClass::Active, PseudoClass::Active);
        assert_ne!(PseudoClass::Active, PseudoClass::Hover);
    }

    #[test]
    fn debug() {
        let pc = PseudoClass::Active;
        let debug_str = format!("{pc:?}");
        assert_eq!(debug_str, "Active");
    }

    #[test]
    fn hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        PseudoClass::Active.hash(&mut hasher1);
        PseudoClass::Active.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }
}
