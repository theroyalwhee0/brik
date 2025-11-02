#![allow(clippy::print_stdout)]

//! Example demonstrating NsDefaults for injecting missing namespace declarations.
//!
//! This example shows how NsDefaults can be used to automatically inject
//! missing namespace declarations into an HTML document. The example uses SVG
//! embedded in HTML without a proper xmlns:svg declaration, and demonstrates
//! how NsDefaults adds the necessary xmlns attributes to the `<html>` tag.
//!
//! This also serves as an integration test, verifying that the output from
//! NsDefaults can be successfully parsed by html5ever and that the namespace
//! declarations are correctly recognized.

#[macro_use]
extern crate html5ever;

use brik::ns::NsDefaultsBuilder;
use brik::parse_html;
use brik::traits::*;

fn main() {
    println!("=== NsDefaults Example ===\n");

    // Sample HTML with SVG elements using the svg: prefix, but missing the
    // xmlns:svg declaration on the <html> tag. Without the namespace declaration,
    // the svg: prefix is invalid and the elements won't be recognized as SVG.
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <title>SVG Example</title>
</head>
<body>
    <h1>Embedded SVG with prefix but no namespace declaration</h1>
    <svg:svg width="200" height="100">
        <svg:rect x="10" y="10" width="180" height="80" fill="blue"/>
        <svg:circle cx="100" cy="50" r="30" fill="red"/>
    </svg:svg>
    <p>The svg: prefix above won't work without xmlns:svg on the html tag.</p>
</body>
</html>"#;

    println!("Original HTML:");
    println!("{html}\n");

    // Create a NsDefaultsBuilder and register the SVG namespace.
    // The builder pattern separates configuration from the processed result.
    let ns_defaults = NsDefaultsBuilder::new()
        .namespace("svg", ns!(svg))
        .namespace("custom", "http://example.com/custom-namespace")
        .from_string(html)
        .expect("Failed to parse HTML");

    println!("Registered namespaces:");
    println!("  - svg: http://www.w3.org/2000/svg");
    println!("  - custom: http://example.com/custom-namespace\n");

    println!("Processed HTML:");
    println!("{ns_defaults}\n");

    println!("Result:");
    println!("The <html> tag has been modified to include the missing namespace declarations:");
    println!("  <html lang=\"en\" xmlns:svg=\"http://www.w3.org/2000/svg\" xmlns:custom=\"http://example.com/custom-namespace\">");
    println!("\nThis ensures that:");
    println!("  1. SVG namespace is properly declared at the document level");
    println!("  2. Custom namespaces are available throughout the document");
    println!("  3. Namespace-aware tools can properly process the document");

    // Integration test: Parse the processed HTML with html5ever to verify it works
    println!("\n=== Integration Test with html5ever ===\n");

    let processed_html: String = ns_defaults.into();
    let document = parse_html().one(processed_html);

    // Verify the HTML element has the namespace attributes
    let html_elem = document
        .select_first("html")
        .expect("Should find <html> element");
    let attrs = html_elem.attributes.borrow();

    println!("Verifying namespace attributes on <html> element:");

    // Check for svg namespace
    if let Some(svg_ns) = attrs.get("xmlns:svg") {
        println!("  ✓ xmlns:svg=\"{svg_ns}\"");
        assert_eq!(
            svg_ns, "http://www.w3.org/2000/svg",
            "SVG namespace should match"
        );
    } else {
        panic!("✗ xmlns:svg attribute not found!");
    }

    // Check for custom namespace
    if let Some(custom_ns) = attrs.get("xmlns:custom") {
        println!("  ✓ xmlns:custom=\"{custom_ns}\"");
        assert_eq!(
            custom_ns, "http://example.com/custom-namespace",
            "Custom namespace should match"
        );
    } else {
        panic!("✗ xmlns:custom attribute not found!");
    }

    // Verify original attributes are preserved
    if let Some(lang) = attrs.get("lang") {
        println!("  ✓ lang=\"{lang}\" (original attribute preserved)");
        assert_eq!(lang, "en", "lang attribute should be preserved");
    } else {
        panic!("✗ lang attribute was not preserved!");
    }

    println!("\n✓ Integration test passed!");
    println!("  - NsDefaults successfully injected namespace declarations");
    println!("  - html5ever successfully parsed the modified HTML");
    println!("  - All namespace attributes are correctly present");
    println!("  - Original attributes were preserved");
}
