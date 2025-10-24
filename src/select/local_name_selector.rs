use cssparser::ToCss;
use html5ever::LocalName;
use precomputed_hash::PrecomputedHash;
use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;

/// Wrapper for LocalName that implements ToCss
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct LocalNameSelector(LocalName);

impl ToCss for LocalNameSelector {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        cssparser::serialize_identifier(&self.0, dest)
    }
}

impl From<LocalName> for LocalNameSelector {
    fn from(name: LocalName) -> Self {
        LocalNameSelector(name)
    }
}

impl<'a> From<&'a str> for LocalNameSelector {
    fn from(s: &'a str) -> Self {
        LocalNameSelector(LocalName::from(s))
    }
}

impl Deref for LocalNameSelector {
    type Target = LocalName;
    fn deref(&self) -> &LocalName {
        &self.0
    }
}

impl Borrow<LocalName> for LocalNameSelector {
    fn borrow(&self) -> &LocalName {
        &self.0
    }
}

impl AsRef<LocalName> for LocalNameSelector {
    fn as_ref(&self) -> &LocalName {
        &self.0
    }
}

impl PrecomputedHash for LocalNameSelector {
    fn precomputed_hash(&self) -> u32 {
        self.0.precomputed_hash()
    }
}
