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

/// Implements NonTSPseudoClass for PseudoClass.
///
/// Provides the selectors crate interface for CSS pseudo-class matching,
/// including classification of pseudo-classes by type (user action states,
/// active/hover states, etc.) for selector matching logic.
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

/// Implements ToCss for PseudoClass.
///
/// Serializes pseudo-class selectors to their CSS representation
/// (e.g., `:hover`, `:active`, `:checked`).
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

    /// Tests is_active_or_hover classification.
    ///
    /// Verifies that only :active and :hover pseudo-classes are classified
    /// as active-or-hover states for selector matching.
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

    /// Tests is_user_action_state classification.
    ///
    /// Verifies that :active, :hover, and :focus pseudo-classes are classified
    /// as user action states for selector matching.
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

    /// Tests CSS serialization of :any-link pseudo-class.
    ///
    /// Verifies that the :any-link pseudo-class serializes correctly.
    #[test]
    fn to_css_any_link() {
        let mut output = String::new();
        PseudoClass::AnyLink.to_css(&mut output).unwrap();
        assert_eq!(output, ":any-link");
    }

    /// Tests CSS serialization of :link pseudo-class.
    ///
    /// Verifies that the :link pseudo-class serializes correctly.
    #[test]
    fn to_css_link() {
        let mut output = String::new();
        PseudoClass::Link.to_css(&mut output).unwrap();
        assert_eq!(output, ":link");
    }

    /// Tests CSS serialization of :visited pseudo-class.
    ///
    /// Verifies that the :visited pseudo-class serializes correctly.
    #[test]
    fn to_css_visited() {
        let mut output = String::new();
        PseudoClass::Visited.to_css(&mut output).unwrap();
        assert_eq!(output, ":visited");
    }

    /// Tests CSS serialization of :active pseudo-class.
    ///
    /// Verifies that the :active pseudo-class serializes correctly.
    #[test]
    fn to_css_active() {
        let mut output = String::new();
        PseudoClass::Active.to_css(&mut output).unwrap();
        assert_eq!(output, ":active");
    }

    /// Tests CSS serialization of :focus pseudo-class.
    ///
    /// Verifies that the :focus pseudo-class serializes correctly.
    #[test]
    fn to_css_focus() {
        let mut output = String::new();
        PseudoClass::Focus.to_css(&mut output).unwrap();
        assert_eq!(output, ":focus");
    }

    /// Tests CSS serialization of :hover pseudo-class.
    ///
    /// Verifies that the :hover pseudo-class serializes correctly.
    #[test]
    fn to_css_hover() {
        let mut output = String::new();
        PseudoClass::Hover.to_css(&mut output).unwrap();
        assert_eq!(output, ":hover");
    }

    /// Tests CSS serialization of :enabled pseudo-class.
    ///
    /// Verifies that the :enabled pseudo-class serializes correctly.
    #[test]
    fn to_css_enabled() {
        let mut output = String::new();
        PseudoClass::Enabled.to_css(&mut output).unwrap();
        assert_eq!(output, ":enabled");
    }

    /// Tests CSS serialization of :disabled pseudo-class.
    ///
    /// Verifies that the :disabled pseudo-class serializes correctly.
    #[test]
    fn to_css_disabled() {
        let mut output = String::new();
        PseudoClass::Disabled.to_css(&mut output).unwrap();
        assert_eq!(output, ":disabled");
    }

    /// Tests CSS serialization of :checked pseudo-class.
    ///
    /// Verifies that the :checked pseudo-class serializes correctly.
    #[test]
    fn to_css_checked() {
        let mut output = String::new();
        PseudoClass::Checked.to_css(&mut output).unwrap();
        assert_eq!(output, ":checked");
    }

    /// Tests CSS serialization of :indeterminate pseudo-class.
    ///
    /// Verifies that the :indeterminate pseudo-class serializes correctly.
    #[test]
    fn to_css_indeterminate() {
        let mut output = String::new();
        PseudoClass::Indeterminate.to_css(&mut output).unwrap();
        assert_eq!(output, ":indeterminate");
    }

    /// Tests cloning PseudoClass instances.
    ///
    /// Verifies that the Clone implementation produces an independent
    /// copy with identical value.
    #[test]
    fn clone() {
        let pc1 = PseudoClass::Active;
        let pc2 = pc1.clone();
        assert_eq!(pc1, pc2);
    }

    /// Tests equality comparison of PseudoClass instances.
    ///
    /// Verifies that the PartialEq and Eq implementations correctly
    /// compare values for equality and inequality.
    #[test]
    fn eq() {
        assert_eq!(PseudoClass::Active, PseudoClass::Active);
        assert_ne!(PseudoClass::Active, PseudoClass::Hover);
    }

    /// Tests debug formatting of PseudoClass.
    ///
    /// Verifies that the Debug implementation produces readable
    /// output showing the variant name.
    #[test]
    fn debug() {
        let pc = PseudoClass::Active;
        let debug_str = format!("{pc:?}");
        assert_eq!(debug_str, "Active");
    }

    /// Tests hashing of PseudoClass instances.
    ///
    /// Verifies that the Hash implementation produces consistent hash
    /// values for identical pseudo-classes.
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
