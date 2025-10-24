use cssparser::ToCss;
use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;

/// Wrapper for String that implements ToCss
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AttrValue(String);

/// Implements ToCss for AttrValue.
///
/// Serializes the attribute value as a CSS-escaped string with quotes,
/// properly escaping any special characters.
impl ToCss for AttrValue {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        cssparser::serialize_string(&self.0, dest)
    }
}

/// Implements From<String> for AttrValue.
///
/// Allows creating AttrValue from an owned String.
impl From<String> for AttrValue {
    fn from(s: String) -> Self {
        AttrValue(s)
    }
}

/// Implements From<&str> for AttrValue.
///
/// Allows creating AttrValue from a string slice.
impl<'a> From<&'a str> for AttrValue {
    fn from(s: &'a str) -> Self {
        AttrValue(s.to_string())
    }
}

/// Implements Deref for AttrValue.
///
/// Allows AttrValue to be dereferenced to access String methods.
impl Deref for AttrValue {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

/// Implements Borrow<str> for AttrValue.
///
/// Allows borrowing the inner string as a `&str`.
impl Borrow<str> for AttrValue {
    fn borrow(&self) -> &str {
        &self.0
    }
}

/// Implements AsRef<str> for AttrValue.
///
/// Provides a generic reference conversion to `&str`.
impl AsRef<str> for AttrValue {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cssparser::ToCss;
    use std::borrow::Borrow;

    /// Tests creating AttrValue from an owned String.
    ///
    /// Verifies that the From<String> trait implementation correctly
    /// wraps the string value.
    #[test]
    fn from_string() {
        let s = String::from("test");
        let attr_val = AttrValue::from(s);
        assert_eq!(attr_val.0, "test");
    }

    /// Tests creating AttrValue from a string slice.
    ///
    /// Verifies that the From<&str> trait implementation correctly
    /// converts and wraps the string value.
    #[test]
    fn from_str() {
        let attr_val = AttrValue::from("test");
        assert_eq!(attr_val.0, "test");
    }

    /// Tests dereferencing AttrValue to access String methods.
    ///
    /// Verifies that the Deref implementation allows transparent
    /// access to the underlying String.
    #[test]
    fn deref() {
        let attr_val = AttrValue::from("test");
        let s: &String = &attr_val;
        assert_eq!(s, "test");
    }

    /// Tests borrowing AttrValue as a string slice.
    ///
    /// Verifies that the Borrow<str> implementation allows
    /// borrowing the inner value as &str.
    #[test]
    fn borrow() {
        let attr_val = AttrValue::from("test");
        let s: &str = attr_val.borrow();
        assert_eq!(s, "test");
    }

    /// Tests converting AttrValue to a string reference.
    ///
    /// Verifies that the AsRef<str> implementation provides
    /// generic reference conversion to &str.
    #[test]
    fn as_ref() {
        let attr_val = AttrValue::from("test");
        let s: &str = attr_val.as_ref();
        assert_eq!(s, "test");
    }

    /// Tests CSS serialization of a simple attribute value.
    ///
    /// Verifies that ToCss correctly wraps the value in quotes
    /// for use in CSS selectors.
    #[test]
    fn to_css_simple() {
        let attr_val = AttrValue::from("test");
        let mut output = String::new();
        attr_val.to_css(&mut output).unwrap();
        assert_eq!(output, "\"test\"");
    }

    /// Tests CSS serialization of an attribute value containing quotes.
    ///
    /// Verifies that ToCss properly escapes embedded quote characters
    /// to produce valid CSS string syntax.
    #[test]
    fn to_css_with_quotes() {
        let attr_val = AttrValue::from("test\"value");
        let mut output = String::new();
        attr_val.to_css(&mut output).unwrap();
        // Should escape the quote
        assert_eq!(output, "\"test\\\"value\"");
    }

    /// Tests CSS serialization of an empty attribute value.
    ///
    /// Verifies that ToCss produces an empty quoted string for
    /// empty input, maintaining valid CSS syntax.
    #[test]
    fn to_css_empty() {
        let attr_val = AttrValue::from("");
        let mut output = String::new();
        attr_val.to_css(&mut output).unwrap();
        assert_eq!(output, "\"\"");
    }

    /// Tests cloning AttrValue instances.
    ///
    /// Verifies that the Clone implementation produces an independent
    /// copy with identical contents.
    #[test]
    fn clone() {
        let attr_val1 = AttrValue::from("test");
        let attr_val2 = attr_val1.clone();
        assert_eq!(attr_val1, attr_val2);
    }

    /// Tests equality comparison of AttrValue instances.
    ///
    /// Verifies that the PartialEq and Eq implementations correctly
    /// compare values for equality and inequality.
    #[test]
    fn eq() {
        let attr_val1 = AttrValue::from("test");
        let attr_val2 = AttrValue::from("test");
        let attr_val3 = AttrValue::from("other");
        assert_eq!(attr_val1, attr_val2);
        assert_ne!(attr_val1, attr_val3);
    }

    /// Tests debug formatting of AttrValue.
    ///
    /// Verifies that the Debug implementation produces readable
    /// output showing the type name and inner string value.
    #[test]
    fn debug() {
        let attr_val = AttrValue::from("test");
        let debug_str = format!("{attr_val:?}");
        assert_eq!(debug_str, "AttrValue(\"test\")");
    }
}
