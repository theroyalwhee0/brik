use cssparser::ToCss;
use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;

/// Wrapper for String that implements ToCss
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AttrValue(String);

impl ToCss for AttrValue {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        cssparser::serialize_string(&self.0, dest)
    }
}

impl From<String> for AttrValue {
    fn from(s: String) -> Self {
        AttrValue(s)
    }
}

impl<'a> From<&'a str> for AttrValue {
    fn from(s: &'a str) -> Self {
        AttrValue(s.to_string())
    }
}

impl Deref for AttrValue {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl Borrow<str> for AttrValue {
    fn borrow(&self) -> &str {
        &self.0
    }
}

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

    #[test]
    fn from_string() {
        let s = String::from("test");
        let attr_val = AttrValue::from(s);
        assert_eq!(attr_val.0, "test");
    }

    #[test]
    fn from_str() {
        let attr_val = AttrValue::from("test");
        assert_eq!(attr_val.0, "test");
    }

    #[test]
    fn deref() {
        let attr_val = AttrValue::from("test");
        let s: &String = &attr_val;
        assert_eq!(s, "test");
    }

    #[test]
    fn borrow() {
        let attr_val = AttrValue::from("test");
        let s: &str = attr_val.borrow();
        assert_eq!(s, "test");
    }

    #[test]
    fn as_ref() {
        let attr_val = AttrValue::from("test");
        let s: &str = attr_val.as_ref();
        assert_eq!(s, "test");
    }

    #[test]
    fn to_css_simple() {
        let attr_val = AttrValue::from("test");
        let mut output = String::new();
        attr_val.to_css(&mut output).unwrap();
        assert_eq!(output, "\"test\"");
    }

    #[test]
    fn to_css_with_quotes() {
        let attr_val = AttrValue::from("test\"value");
        let mut output = String::new();
        attr_val.to_css(&mut output).unwrap();
        // Should escape the quote
        assert_eq!(output, "\"test\\\"value\"");
    }

    #[test]
    fn to_css_empty() {
        let attr_val = AttrValue::from("");
        let mut output = String::new();
        attr_val.to_css(&mut output).unwrap();
        assert_eq!(output, "\"\"");
    }

    #[test]
    fn clone() {
        let attr_val1 = AttrValue::from("test");
        let attr_val2 = attr_val1.clone();
        assert_eq!(attr_val1, attr_val2);
    }

    #[test]
    fn eq() {
        let attr_val1 = AttrValue::from("test");
        let attr_val2 = AttrValue::from("test");
        let attr_val3 = AttrValue::from("other");
        assert_eq!(attr_val1, attr_val2);
        assert_ne!(attr_val1, attr_val3);
    }

    #[test]
    fn debug() {
        let attr_val = AttrValue::from("test");
        let debug_str = format!("{attr_val:?}");
        assert_eq!(debug_str, "AttrValue(\"test\")");
    }
}
