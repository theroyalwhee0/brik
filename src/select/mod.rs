// Addressing this lint is a semver-breaking change.
// Remove this once the issue has been addressed.
#![allow(clippy::result_unit_err)]

/// CSS attribute value wrapper.
mod attr_value;
/// Brik's selector implementation.
mod brik_selectors;
/// Element trait implementation for selector matching.
mod element_impl;
/// CSS local name selector wrapper.
mod local_name_selector;
/// CSS pseudo-class support.
mod pseudo_class;
/// CSS pseudo-element support.
mod pseudo_element;
/// Compiled CSS selector.
mod selector;
/// Selector compilation context.
mod selector_context;
/// Compiled list of CSS selectors.
mod selectors;
/// Selector specificity.
mod specificity;

pub use attr_value::AttrValue;
pub use brik_selectors::BrikSelectors;
pub use local_name_selector::LocalNameSelector;
pub use pseudo_class::PseudoClass;
pub use pseudo_element::PseudoElement;
pub use selector::Selector;
pub use selector_context::SelectorContext;
pub use selectors::Selectors;
pub use specificity::Specificity;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_html;
    use crate::traits::*;
    use html5ever::local_name;
    #[cfg(feature = "namespaces")]
    use html5ever::{ns, Namespace};

    /// Tests namespace-qualified type selectors.
    ///
    /// Verifies that selectors can match elements in specific namespaces
    /// using the namespace prefix syntax (e.g., `svg|rect`, `svg|*`).
    #[test]
    #[cfg(feature = "namespaces")]
    fn namespace_type_selector() {
        let html = r#"<!DOCTYPE html>
<html>
<body>
    <div>HTML div</div>
    <svg xmlns="http://www.w3.org/2000/svg">
        <rect width="100" height="100"/>
        <circle cx="50" cy="50" r="40"/>
    </svg>
</body>
</html>"#;

        let document = parse_html().one(html);

        // Create context with SVG namespace
        let mut context = SelectorContext::new();
        context.add_namespace("svg".to_string(), ns!(svg));

        // Select SVG rect elements using namespace selector
        let selectors = Selectors::compile_with_context("svg|rect", &context).unwrap();
        let rects = selectors
            .filter(document.descendants().elements())
            .collect::<Vec<_>>();
        assert_eq!(rects.len(), 1);
        assert_eq!(rects[0].name.local, local_name!("rect"));
        assert_eq!(rects[0].name.ns, ns!(svg));

        // Select all SVG elements using namespace wildcard
        let selectors = Selectors::compile_with_context("svg|*", &context).unwrap();
        let svg_elements = selectors
            .filter(document.descendants().elements())
            .collect::<Vec<_>>();
        assert_eq!(svg_elements.len(), 3); // svg, rect, circle
    }

    /// Tests namespace-qualified attribute selectors.
    ///
    /// Verifies that selectors can match elements with attributes in
    /// specific namespaces using namespace prefix syntax (e.g., `[xlink|href]`).
    #[test]
    #[cfg(feature = "namespaces")]
    fn namespace_attribute_selector() {
        let html = r##"<!DOCTYPE html>
<html xmlns:custom="http://example.com/custom">
<body>
    <div>Regular div without custom attribute</div>
    <div custom:attr="value">Div with custom:attr</div>
    <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
        <use xlink:href="#icon"/>
    </svg>
</body>
</html>"##;

        let document = parse_html().one(html);

        // Create context with xlink namespace
        let mut context = SelectorContext::new();
        context.add_namespace(
            "xlink".to_string(),
            Namespace::from("http://www.w3.org/1999/xlink"),
        );

        // Select elements with xlink:href attribute
        let selectors = Selectors::compile_with_context("[xlink|href]", &context).unwrap();
        let elements = selectors
            .filter(document.descendants().elements())
            .collect::<Vec<_>>();
        assert_eq!(elements.len(), 1);
        assert_eq!(elements[0].name.local, local_name!("use"));
    }

    /// Tests error handling for undefined namespace prefixes.
    ///
    /// Verifies that compiling a selector with an undefined namespace
    /// prefix returns an error rather than panicking or silently failing.
    #[test]
    #[cfg(feature = "namespaces")]
    fn namespace_selector_undefined_prefix() {
        let context = SelectorContext::new(); // Empty context

        // Try to compile selector with undefined namespace prefix
        let result = Selectors::compile_with_context("undefined|div", &context);
        assert!(
            result.is_err(),
            "Should fail with undefined namespace prefix"
        );
    }

    /// Tests backward compatibility of selector compilation.
    ///
    /// Verifies that the original compile() method continues to work
    /// for non-namespaced selectors, maintaining API compatibility.
    #[test]
    fn namespace_selector_backward_compatibility() {
        let html = r#"<div class="test">Content</div>"#;
        let document = parse_html().one(html);

        // Old compile() method should still work
        let selectors = Selectors::compile("div.test").unwrap();
        let elements = selectors
            .filter(document.descendants().elements())
            .collect::<Vec<_>>();
        assert_eq!(elements.len(), 1);
    }

    /// Tests SelectorContext builder pattern.
    ///
    /// Verifies that SelectorContext supports fluent builder-style
    /// method chaining for adding namespaces and setting defaults.
    #[test]
    #[cfg(feature = "namespaces")]
    fn namespace_context_builder_pattern() {
        let mut context = SelectorContext::new();
        context
            .add_namespace("svg".to_string(), ns!(svg))
            .add_namespace("html".to_string(), ns!(html))
            .set_default_namespace(ns!(html));

        // Verify the builder pattern works
        let html = r#"<!DOCTYPE html>
<html>
<body>
    <svg xmlns="http://www.w3.org/2000/svg">
        <rect/>
    </svg>
</body>
</html>"#;

        let document = parse_html().one(html);
        let selectors = Selectors::compile_with_context("svg|rect", &context).unwrap();
        let rects = selectors
            .filter(document.descendants().elements())
            .collect::<Vec<_>>();
        assert_eq!(rects.len(), 1);
    }

    /// Tests basic selector matching functionality.
    ///
    /// Verifies that select() correctly finds elements matching a CSS
    /// selector and that attribute access works properly.
    #[test]
    fn select() {
        let html = r"
<title>Test case</title>
<p class=foo>Foo
<p>Bar
<p class=foo>Foo
";

        let document = parse_html().one(html);
        let matching = document.select("p.foo").unwrap().collect::<Vec<_>>();
        assert_eq!(matching.len(), 2);
        let child = matching[0].as_node().first_child().unwrap();
        assert_eq!(&**child.as_text().unwrap().borrow(), "Foo\n");
        assert_eq!(matching[0].attributes.borrow().get("class"), Some("foo"));
        assert_eq!(
            matching[0].attributes.borrow().get(local_name!("class")),
            Some("foo")
        );

        let selectors = Selectors::compile("p.foo").unwrap();
        let matching2 = selectors
            .filter(document.descendants().elements())
            .collect::<Vec<_>>();
        assert_eq!(matching, matching2);
    }

    /// Tests select_first convenience method.
    ///
    /// Verifies that select_first() returns the first matching element
    /// and returns an error when no elements match.
    #[test]
    fn select_first() {
        let html = r"
<title>Test case</title>
<p class=foo>Foo
<p>Bar
<p class=foo>Baz
";

        let document = parse_html().one(html);
        let matching = document.select_first("p.foo").unwrap();
        let child = matching.as_node().first_child().unwrap();
        assert_eq!(&**child.as_text().unwrap().borrow(), "Foo\n");
        assert_eq!(matching.attributes.borrow().get("class"), Some("foo"));
        assert_eq!(
            matching.attributes.borrow().get(local_name!("class")),
            Some("foo")
        );

        assert!(document.select_first("p.bar").is_err());
    }

    /// Tests CSS selector specificity comparison.
    ///
    /// Verifies that selector specificity is calculated correctly and
    /// can be compared to determine cascade precedence.
    #[test]
    fn specificity() {
        let selectors = Selectors::compile(".example, :first-child, div").unwrap();
        let specificities = selectors
            .0
            .iter()
            .map(|s| s.specificity())
            .collect::<Vec<_>>();
        assert_eq!(specificities.len(), 3);
        assert!(specificities[0] == specificities[1]);
        assert!(specificities[0] > specificities[2]);
        assert!(specificities[1] > specificities[2]);
    }
}
