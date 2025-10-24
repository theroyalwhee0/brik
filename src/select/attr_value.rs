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
