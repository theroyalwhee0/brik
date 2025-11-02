#![allow(clippy::print_stdout)]

//! Example demonstrating NamespaceDefaults for injecting missing namespace declarations.
//!
//! This example shows how NamespaceDefaults can be used to automatically inject
//! missing namespace declarations into an HTML document. The example uses SVG
//! embedded in HTML without a proper xmlns:svg declaration.
//!
//! NOTE: This example demonstrates the intended API, but the implementation is
//! not yet complete, so it won't actually inject the namespaces yet.

#[macro_use]
extern crate html5ever;

use brik::ns::NamespaceDefaults;

fn main() {
    println!("=== NamespaceDefaults Example ===\n");

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

    // Create a NamespaceDefaults provider and register the SVG namespace.
    // The API supports method chaining for a fluent interface.
    let ns_defaults = NamespaceDefaults::new()
        .namespace("svg", ns!(svg))
        .namespace("custom", "http://example.com/custom-namespace")
        .from_string(html)
        .build();

    println!("Registered namespaces:");
    println!("  - svg: http://www.w3.org/2000/svg");
    println!("  - custom: http://example.com/custom-namespace\n");

    // Once implemented, you would parse the modified HTML with:
    // let document = parse_html().one(provider.into());
    // NOTE: The .into() conversion is not yet implemented.
    let _ = ns_defaults; // Silence unused variable warning for now.

    println!("Expected result (when implemented):");
    println!("The <html> tag should be modified to include:");
    println!("  <html lang=\"en\" xmlns:svg=\"http://www.w3.org/2000/svg\" xmlns:custom=\"http://example.com/custom-namespace\">");
    println!("\nThis would ensure that:");
    println!("  1. SVG namespace is properly declared at the document level");
    println!("  2. Custom namespaces are available throughout the document");
    println!("  3. Namespace-aware tools can properly process the document");

    println!("\n⚠️  Note: The actual injection is not yet implemented.");
    println!("This example demonstrates the intended API only.");
}
