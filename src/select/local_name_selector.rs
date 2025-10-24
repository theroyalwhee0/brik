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

#[cfg(test)]
mod tests {
    use super::*;
    use cssparser::ToCss;
    use std::borrow::Borrow;

    #[test]
    fn from_local_name() {
        let local_name = html5ever::local_name!("div");
        let selector = LocalNameSelector::from(local_name.clone());
        assert_eq!(&selector.0, &local_name);
    }

    #[test]
    fn from_str() {
        let selector = LocalNameSelector::from("span");
        assert_eq!(selector.0, html5ever::local_name!("span"));
    }

    #[test]
    fn deref() {
        let selector = LocalNameSelector::from("p");
        let name: &LocalName = &selector;
        assert_eq!(*name, html5ever::local_name!("p"));
    }

    #[test]
    fn borrow() {
        let selector = LocalNameSelector::from("div");
        let name: &LocalName = selector.borrow();
        assert_eq!(*name, html5ever::local_name!("div"));
    }

    #[test]
    fn as_ref() {
        let selector = LocalNameSelector::from("span");
        let name: &LocalName = selector.as_ref();
        assert_eq!(*name, html5ever::local_name!("span"));
    }

    #[test]
    fn to_css() {
        let selector = LocalNameSelector::from("div");
        let mut output = String::new();
        selector.to_css(&mut output).unwrap();
        assert_eq!(output, "div");
    }

    #[test]
    fn clone() {
        let selector1 = LocalNameSelector::from("div");
        let selector2 = selector1.clone();
        assert_eq!(selector1, selector2);
    }

    #[test]
    fn eq() {
        let selector1 = LocalNameSelector::from("div");
        let selector2 = LocalNameSelector::from("div");
        let selector3 = LocalNameSelector::from("span");
        assert_eq!(selector1, selector2);
        assert_ne!(selector1, selector3);
    }

    #[test]
    fn debug() {
        let selector = LocalNameSelector::from("div");
        let debug_str = format!("{selector:?}");
        assert!(debug_str.contains("LocalNameSelector"));
    }

    #[test]
    fn default() {
        let selector = LocalNameSelector::default();
        assert_eq!(selector.0, LocalName::default());
    }

    #[test]
    fn precomputed_hash() {
        let selector = LocalNameSelector::from("div");
        let hash = selector.precomputed_hash();
        assert!(hash > 0);
    }
}
