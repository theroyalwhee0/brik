use super::{BrikSelectors, Selector, SelectorContext};
use crate::iter::Select;
use crate::node_data_ref::NodeDataRef;
use crate::tree::ElementData;
use selectors::parser::{Parser, SelectorList};
use std::fmt;

/// Parser for CSS selectors.
struct BrikParser<'a> {
    /// Selector context containing namespace mappings and other configuration.
    context: &'a SelectorContext,
}

impl<'a> BrikParser<'a> {
    /// Create a new parser with the given selector context.
    fn new(context: &'a SelectorContext) -> Self {
        BrikParser { context }
    }
}

impl<'i, 'a> Parser<'i> for BrikParser<'a> {
    type Impl = BrikSelectors;
    type Error = selectors::parser::SelectorParseErrorKind<'i>;

    fn parse_non_ts_pseudo_class(
        &self,
        location: cssparser::SourceLocation,
        name: cssparser::CowRcStr<'i>,
    ) -> Result<
        super::PseudoClass,
        cssparser::ParseError<'i, selectors::parser::SelectorParseErrorKind<'i>>,
    > {
        use super::PseudoClass::*;
        use selectors::parser::SelectorParseErrorKind;
        if name.eq_ignore_ascii_case("any-link") {
            Ok(AnyLink)
        } else if name.eq_ignore_ascii_case("link") {
            Ok(Link)
        } else if name.eq_ignore_ascii_case("visited") {
            Ok(Visited)
        } else if name.eq_ignore_ascii_case("active") {
            Ok(Active)
        } else if name.eq_ignore_ascii_case("focus") {
            Ok(Focus)
        } else if name.eq_ignore_ascii_case("hover") {
            Ok(Hover)
        } else if name.eq_ignore_ascii_case("enabled") {
            Ok(Enabled)
        } else if name.eq_ignore_ascii_case("disabled") {
            Ok(Disabled)
        } else if name.eq_ignore_ascii_case("checked") {
            Ok(Checked)
        } else if name.eq_ignore_ascii_case("indeterminate") {
            Ok(Indeterminate)
        } else {
            Err(
                location.new_custom_error(SelectorParseErrorKind::UnsupportedPseudoClassOrElement(
                    name,
                )),
            )
        }
    }

    fn default_namespace(&self) -> Option<html5ever::Namespace> {
        self.context.default_namespace.clone()
    }

    fn namespace_for_prefix(
        &self,
        prefix: &super::LocalNameSelector,
    ) -> Option<html5ever::Namespace> {
        self.context
            .namespaces
            .get(prefix.as_ref().as_ref())
            .cloned()
    }
}

/// A pre-compiled list of CSS Selectors.
pub struct Selectors(pub Vec<Selector>);

impl Selectors {
    /// Compile a list of selectors. This may fail on syntax errors or unsupported selectors.
    ///
    /// # Errors
    ///
    /// Returns `Err(())` if the selector string contains syntax errors or unsupported selectors.
    #[inline]
    pub fn compile(s: &str) -> Result<Selectors, ()> {
        let context = SelectorContext::default();
        Self::compile_with_context(s, &context)
    }

    /// Compile a list of selectors with a selector context.
    ///
    /// This method allows selectors to use namespace prefixes in both type selectors
    /// (e.g., `svg|rect`) and attribute selectors (e.g., `[tmpl|if]`).
    ///
    /// **Note:** Namespace-aware selector features require the `namespaces` feature to be enabled.
    /// Without the feature, namespace prefixes in selectors will fail to parse or match.
    ///
    /// This is the recommended method when using namespace-aware selectors.
    ///
    /// # Arguments
    ///
    /// * `s` - The selector string to compile
    /// * `context` - A selector context containing namespace mappings
    ///
    /// # Examples
    ///
    /// ```
    /// #[cfg(feature = "namespaces")]
    /// {
    /// use brik::{Selectors, SelectorContext};
    /// use html5ever::ns;
    ///
    /// let mut context = SelectorContext::new();
    /// context.add_namespace("svg".to_string(), ns!(svg));
    ///
    /// // Select SVG rect elements
    /// let selectors = Selectors::compile_with_context("svg|rect", &context).unwrap();
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `Err(())` if the selector string contains syntax errors, unsupported selectors,
    /// or references undefined namespace prefixes.
    #[inline]
    pub fn compile_with_context(s: &str, context: &SelectorContext) -> Result<Selectors, ()> {
        let mut input = cssparser::ParserInput::new(s);
        match SelectorList::parse(
            &BrikParser::new(context),
            &mut cssparser::Parser::new(&mut input),
            selectors::parser::ParseRelative::No,
        ) {
            Ok(list) => Ok(Selectors(
                list.slice().iter().cloned().map(Selector).collect(),
            )),
            Err(_) => Err(()),
        }
    }

    /// Returns whether the given element matches this list of selectors.
    #[inline]
    pub fn matches(&self, element: &NodeDataRef<ElementData>) -> bool {
        self.0.iter().any(|s| s.matches(element))
    }

    /// Filter an element iterator, yielding those matching this list of selectors.
    #[inline]
    pub fn filter<I>(&self, iter: I) -> Select<I, &Selectors>
    where
        I: Iterator<Item = NodeDataRef<ElementData>>,
    {
        Select {
            iter,
            selectors: self,
        }
    }
}

impl ::std::str::FromStr for Selectors {
    type Err = ();
    #[inline]
    fn from_str(s: &str) -> Result<Selectors, ()> {
        Selectors::compile(s)
    }
}

impl fmt::Display for Selectors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use cssparser::ToCss;
        let mut iter = self.0.iter();
        let first = iter
            .next()
            .expect("Empty Selectors, should contain at least one selector");
        first.0.to_css(f)?;
        for selector in iter {
            f.write_str(", ")?;
            selector.0.to_css(f)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Selectors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::html5ever::tendril::TendrilSink;
    use crate::iter::NodeIterator;
    use crate::parse_html;

    #[test]
    fn compile_simple_selector() {
        let selectors = Selectors::compile("div").unwrap();
        assert_eq!(selectors.0.len(), 1);
    }

    #[test]
    fn compile_multiple_selectors() {
        let selectors = Selectors::compile("div, p, span").unwrap();
        assert_eq!(selectors.0.len(), 3);
    }

    #[test]
    fn compile_class_selector() {
        let selectors = Selectors::compile(".myClass").unwrap();
        assert_eq!(selectors.0.len(), 1);
    }

    #[test]
    fn compile_id_selector() {
        let selectors = Selectors::compile("#myId").unwrap();
        assert_eq!(selectors.0.len(), 1);
    }

    #[test]
    fn compile_pseudo_class_any_link() {
        let selectors = Selectors::compile("a:any-link").unwrap();
        assert_eq!(selectors.0.len(), 1);
    }

    #[test]
    fn compile_pseudo_class_link() {
        let selectors = Selectors::compile("a:link").unwrap();
        assert_eq!(selectors.0.len(), 1);
    }

    #[test]
    fn compile_pseudo_class_visited() {
        let selectors = Selectors::compile("a:visited").unwrap();
        assert_eq!(selectors.0.len(), 1);
    }

    #[test]
    fn compile_pseudo_class_active() {
        let selectors = Selectors::compile("button:active").unwrap();
        assert_eq!(selectors.0.len(), 1);
    }

    #[test]
    fn compile_pseudo_class_focus() {
        let selectors = Selectors::compile("input:focus").unwrap();
        assert_eq!(selectors.0.len(), 1);
    }

    #[test]
    fn compile_pseudo_class_hover() {
        let selectors = Selectors::compile("div:hover").unwrap();
        assert_eq!(selectors.0.len(), 1);
    }

    #[test]
    fn compile_pseudo_class_enabled() {
        let selectors = Selectors::compile("input:enabled").unwrap();
        assert_eq!(selectors.0.len(), 1);
    }

    #[test]
    fn compile_pseudo_class_disabled() {
        let selectors = Selectors::compile("input:disabled").unwrap();
        assert_eq!(selectors.0.len(), 1);
    }

    #[test]
    fn compile_pseudo_class_checked() {
        let selectors = Selectors::compile("input:checked").unwrap();
        assert_eq!(selectors.0.len(), 1);
    }

    #[test]
    fn compile_pseudo_class_indeterminate() {
        let selectors = Selectors::compile("input:indeterminate").unwrap();
        assert_eq!(selectors.0.len(), 1);
    }

    #[test]
    fn compile_unsupported_pseudo_class() {
        let result = Selectors::compile(":unsupported");
        assert!(result.is_err());
    }

    #[test]
    fn compile_invalid_syntax() {
        let result = Selectors::compile(":::");
        assert!(result.is_err());
    }

    #[test]
    fn matches_true() {
        let html = r#"<div class="test">content</div>"#;
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        let selectors = Selectors::compile(".test").unwrap();
        assert!(selectors.matches(&div));
    }

    #[test]
    fn matches_false() {
        let html = r#"<div class="test">content</div>"#;
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        let selectors = Selectors::compile(".other").unwrap();
        assert!(!selectors.matches(&div));
    }

    #[test]
    fn matches_multiple_selectors() {
        let html = r#"<div class="test">content</div>"#;
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        let selectors = Selectors::compile(".other, .test").unwrap();
        assert!(selectors.matches(&div));
    }

    #[test]
    fn filter() {
        let html = r#"<div><p class="keep">1</p><span>2</span><p class="keep">3</p></div>"#;
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        let selectors = Selectors::compile(".keep").unwrap();
        let filtered: Vec<_> = selectors
            .filter(div.as_node().descendants().elements())
            .collect();

        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().all(|e| e.name.local.as_ref() == "p"));
    }

    #[test]
    fn from_str() {
        let selectors: Selectors = "div.test".parse().unwrap();
        assert_eq!(selectors.0.len(), 1);
    }

    #[test]
    fn from_str_error() {
        let result: Result<Selectors, ()> = ":::".parse();
        assert!(result.is_err());
    }

    #[test]
    fn display_single() {
        let selectors = Selectors::compile("div").unwrap();
        let display = format!("{selectors}");
        assert_eq!(display, "div");
    }

    #[test]
    fn display_multiple() {
        let selectors = Selectors::compile("div, p").unwrap();
        let display = format!("{selectors}");
        assert!(display.contains("div"));
        assert!(display.contains("p"));
        assert!(display.contains(", "));
    }

    #[test]
    fn debug() {
        let selectors = Selectors::compile("div.test").unwrap();
        let debug = format!("{selectors:?}");
        assert!(debug.contains("div"));
        assert!(debug.contains("test"));
    }

    #[test]
    #[cfg(feature = "namespaces")]
    fn compile_with_context_namespace() {
        let mut context = SelectorContext::new();
        context.add_namespace("svg".to_string(), html5ever::ns!(svg));

        let selectors = Selectors::compile_with_context("svg|rect", &context).unwrap();
        assert_eq!(selectors.0.len(), 1);
    }

    #[test]
    #[cfg(feature = "namespaces")]
    fn compile_with_context_undefined_namespace() {
        let context = SelectorContext::new();

        let result = Selectors::compile_with_context("svg|rect", &context);
        assert!(result.is_err());
    }

    #[test]
    fn compile_with_context_no_namespace() {
        let context = SelectorContext::default();

        let selectors = Selectors::compile_with_context("div", &context).unwrap();
        assert_eq!(selectors.0.len(), 1);
    }
}
