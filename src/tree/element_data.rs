use html5ever::QualName;
use std::cell::RefCell;

use crate::attributes::Attributes;

use super::NodeRef;

/// Data specific to element nodes.
#[derive(Debug, PartialEq, Clone)]
pub struct ElementData {
    /// The namespace and local name of the element, such as `ns!(html)` and `body`.
    pub name: QualName,

    /// The attributes of the elements.
    pub attributes: RefCell<Attributes>,

    /// If the element is an HTML `<template>` element,
    /// the document fragment node that is the root of template contents.
    pub template_contents: Option<NodeRef>,
}

/// Methods for ElementData.
///
/// Provides accessors for element name components including
/// namespace URI, local name, and prefix.
impl ElementData {
    /// Returns the namespace URI of the element.
    ///
    /// **Note:** This method requires the `namespaces` feature to be enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// #[cfg(feature = "namespaces")]
    /// {
    /// use brik::parse_html;
    /// use brik::traits::*;
    ///
    /// let doc = parse_html().one("<div>Hello</div>");
    /// let div = doc.select_first("div").unwrap();
    /// // HTML elements use the XHTML namespace
    /// assert_eq!(div.namespace_uri().as_ref(), "http://www.w3.org/1999/xhtml");
    /// }
    /// ```
    #[inline]
    #[cfg(feature = "namespaces")]
    pub fn namespace_uri(&self) -> &html5ever::Namespace {
        &self.name.ns
    }

    /// Returns the local name of the element without any namespace prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// use brik::parse_html;
    /// use brik::traits::*;
    ///
    /// let doc = parse_html().one("<div>Hello</div>");
    /// let div = doc.select_first("div").unwrap();
    /// assert_eq!(div.local_name().as_ref(), "div");
    /// ```
    #[inline]
    pub fn local_name(&self) -> &html5ever::LocalName {
        &self.name.local
    }

    /// Returns the namespace prefix of the element, if any.
    ///
    /// **Note:** This method requires the `namespaces` feature to be enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// #[cfg(feature = "namespaces")]
    /// {
    /// use brik::parse_html;
    /// use brik::traits::*;
    ///
    /// let doc = parse_html().one("<div>Hello</div>");
    /// let div = doc.select_first("div").unwrap();
    /// // HTML elements typically have no prefix
    /// assert_eq!(div.prefix(), None);
    /// }
    /// ```
    #[inline]
    #[cfg(feature = "namespaces")]
    pub fn prefix(&self) -> Option<&html5ever::Prefix> {
        self.name.prefix.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_html;
    use crate::traits::*;

    /// Tests that `namespace_uri()` returns the correct namespace for elements.
    ///
    /// Verifies both HTML elements (XHTML namespace) and SVG elements
    /// (SVG namespace) return their correct namespace URIs.
    #[test]
    #[cfg(feature = "namespaces")]
    fn element_namespace_uri() {
        // Test HTML element namespace
        let html = r"<!DOCTYPE html><html><body><div>Test</div></body></html>";
        let document = parse_html().one(html);
        let div = document.select_first("div").unwrap();
        assert_eq!(div.namespace_uri().as_ref(), "http://www.w3.org/1999/xhtml");

        // Test SVG element namespace
        let svg_html = r#"<!DOCTYPE html>
<html>
<body>
<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
  <circle cx="50" cy="50" r="40"/>
</svg>
</body>
</html>"#;
        let document = parse_html().one(svg_html);
        let svg = document.select_first("svg").unwrap();
        assert_eq!(svg.namespace_uri().as_ref(), "http://www.w3.org/2000/svg");

        let circle = document.select_first("circle").unwrap();
        assert_eq!(
            circle.namespace_uri().as_ref(),
            "http://www.w3.org/2000/svg"
        );
    }

    /// Tests that `local_name()` returns the element tag name without namespace.
    ///
    /// Verifies that local_name returns just the tag name (e.g., "div", "body")
    /// without any namespace prefix or URI.
    #[test]
    fn element_local_name() {
        let html = r"<!DOCTYPE html><html><body><div class='test'>Content</div></body></html>";
        let document = parse_html().one(html);
        let div = document.select_first("div").unwrap();
        assert_eq!(div.local_name().as_ref(), "div");

        let body = document.select_first("body").unwrap();
        assert_eq!(body.local_name().as_ref(), "body");
    }

    /// Tests that `prefix()` returns None for elements without namespace prefixes.
    ///
    /// In HTML5, elements typically don't have namespace prefixes even when
    /// they're in specific namespaces (like SVG or MathML).
    #[test]
    #[cfg(feature = "namespaces")]
    fn element_prefix() {
        // Regular HTML elements have no prefix
        let html = r"<!DOCTYPE html><html><body><div>Test</div></body></html>";
        let document = parse_html().one(html);
        let div = document.select_first("div").unwrap();
        assert_eq!(div.prefix(), None);

        // SVG elements typically have no prefix when embedded in HTML5
        let svg_html = r#"<!DOCTYPE html>
<html>
<body>
<svg xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="100"/>
</svg>
</body>
</html>"#;
        let document = parse_html().one(svg_html);
        let rect = document.select_first("rect").unwrap();
        assert_eq!(rect.prefix(), None);
    }
}
