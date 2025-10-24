use cssparser::ToCss;
use html5ever::LocalName;
use precomputed_hash::PrecomputedHash;
use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;

/// Wrapper for LocalName that implements ToCss
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct LocalNameSelector(LocalName);

/// Implements ToCss for LocalNameSelector.
///
/// Serializes the element name as a CSS identifier, properly escaping
/// any special characters according to CSS syntax rules.
impl ToCss for LocalNameSelector {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        cssparser::serialize_identifier(&self.0, dest)
    }
}

/// Implements From<LocalName> for LocalNameSelector.
///
/// Allows creating LocalNameSelector from an existing LocalName.
impl From<LocalName> for LocalNameSelector {
    fn from(name: LocalName) -> Self {
        LocalNameSelector(name)
    }
}

/// Implements From<&str> for LocalNameSelector.
///
/// Allows creating LocalNameSelector from a string slice by converting
/// to LocalName first.
impl<'a> From<&'a str> for LocalNameSelector {
    fn from(s: &'a str) -> Self {
        LocalNameSelector(LocalName::from(s))
    }
}

/// Implements Deref for LocalNameSelector.
///
/// Allows LocalNameSelector to be dereferenced to access LocalName methods.
impl Deref for LocalNameSelector {
    type Target = LocalName;
    fn deref(&self) -> &LocalName {
        &self.0
    }
}

/// Implements Borrow<LocalName> for LocalNameSelector.
///
/// Allows borrowing the inner LocalName.
impl Borrow<LocalName> for LocalNameSelector {
    fn borrow(&self) -> &LocalName {
        &self.0
    }
}

/// Implements AsRef<LocalName> for LocalNameSelector.
///
/// Provides a generic reference conversion to LocalName.
impl AsRef<LocalName> for LocalNameSelector {
    fn as_ref(&self) -> &LocalName {
        &self.0
    }
}

/// Implements PrecomputedHash for LocalNameSelector.
///
/// Delegates to the inner LocalName's precomputed hash for efficient
/// selector matching and hash-based operations.
impl PrecomputedHash for LocalNameSelector {
    fn precomputed_hash(&self) -> u32 {
        self.0.precomputed_hash()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cssparser::ToCss;
    use std::borrow::Borrow;

    /// Tests creating LocalNameSelector from a LocalName.
    ///
    /// Verifies that the From<LocalName> trait implementation correctly
    /// wraps the LocalName value.
    #[test]
    fn from_local_name() {
        let local_name = html5ever::local_name!("div");
        let selector = LocalNameSelector::from(local_name.clone());
        assert_eq!(&selector.0, &local_name);
    }

    /// Tests creating LocalNameSelector from a string slice.
    ///
    /// Verifies that the From<&str> trait implementation correctly
    /// converts and wraps the string value.
    #[test]
    fn from_str() {
        let selector = LocalNameSelector::from("span");
        assert_eq!(selector.0, html5ever::local_name!("span"));
    }

    /// Tests dereferencing LocalNameSelector to access LocalName methods.
    ///
    /// Verifies that the Deref implementation allows transparent
    /// access to the underlying LocalName.
    #[test]
    fn deref() {
        let selector = LocalNameSelector::from("p");
        let name: &LocalName = &selector;
        assert_eq!(*name, html5ever::local_name!("p"));
    }

    /// Tests borrowing LocalNameSelector as LocalName.
    ///
    /// Verifies that the Borrow<LocalName> implementation allows
    /// borrowing the inner value.
    #[test]
    fn borrow() {
        let selector = LocalNameSelector::from("div");
        let name: &LocalName = selector.borrow();
        assert_eq!(*name, html5ever::local_name!("div"));
    }

    /// Tests converting LocalNameSelector to a LocalName reference.
    ///
    /// Verifies that the AsRef<LocalName> implementation provides
    /// generic reference conversion.
    #[test]
    fn as_ref() {
        let selector = LocalNameSelector::from("span");
        let name: &LocalName = selector.as_ref();
        assert_eq!(*name, html5ever::local_name!("span"));
    }

    /// Tests CSS serialization of element names.
    ///
    /// Verifies that ToCss correctly serializes the element name
    /// as a valid CSS identifier.
    #[test]
    fn to_css() {
        let selector = LocalNameSelector::from("div");
        let mut output = String::new();
        selector.to_css(&mut output).unwrap();
        assert_eq!(output, "div");
    }

    /// Tests cloning LocalNameSelector instances.
    ///
    /// Verifies that the Clone implementation produces an independent
    /// copy with identical contents.
    #[test]
    fn clone() {
        let selector1 = LocalNameSelector::from("div");
        let selector2 = selector1.clone();
        assert_eq!(selector1, selector2);
    }

    /// Tests equality comparison of LocalNameSelector instances.
    ///
    /// Verifies that the PartialEq and Eq implementations correctly
    /// compare values for equality and inequality.
    #[test]
    fn eq() {
        let selector1 = LocalNameSelector::from("div");
        let selector2 = LocalNameSelector::from("div");
        let selector3 = LocalNameSelector::from("span");
        assert_eq!(selector1, selector2);
        assert_ne!(selector1, selector3);
    }

    /// Tests debug formatting of LocalNameSelector.
    ///
    /// Verifies that the Debug implementation produces readable
    /// output including the type name.
    #[test]
    fn debug() {
        let selector = LocalNameSelector::from("div");
        let debug_str = format!("{selector:?}");
        assert!(debug_str.contains("LocalNameSelector"));
    }

    /// Tests the default value of LocalNameSelector.
    ///
    /// Verifies that Default produces a LocalNameSelector wrapping
    /// the default LocalName value.
    #[test]
    fn default() {
        let selector = LocalNameSelector::default();
        assert_eq!(selector.0, LocalName::default());
    }

    /// Tests precomputed hash functionality.
    ///
    /// Verifies that precomputed_hash returns a non-zero hash value
    /// for efficient selector matching operations.
    #[test]
    fn precomputed_hash() {
        let selector = LocalNameSelector::from("div");
        let hash = selector.precomputed_hash();
        assert!(hash > 0);
    }
}
