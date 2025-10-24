#![allow(clippy::print_stdout)]

//! Example demonstrating namespace-aware CSS selector matching.
//!
//! This example shows how to use the `SelectorContext` to compile and match
//! selectors that include namespace prefixes for both element types and attributes.

#[macro_use]
extern crate html5ever;

use brik::traits::*;
use brik::{parse_html, SelectorContext, Selectors};

fn main() {
    // Sample HTML with SVG embedded in HTML, plus custom namespaced attributes
    let html = r##"<!DOCTYPE html>
<html xmlns:tmpl="http://example.com/template">
<head>
    <title>Namespace Selector Example</title>
</head>
<body>
    <h1>Namespace-Aware Selectors</h1>

    <!-- Regular HTML elements -->
    <div class="content">
        <p>This is a regular paragraph.</p>
    </div>

    <!-- SVG elements in the SVG namespace -->
    <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="200" height="200">
        <rect x="10" y="10" width="80" height="80" fill="blue"/>
        <circle cx="150" cy="50" r="30" fill="red"/>
        <use xlink:href="#icon"/>
    </svg>

    <!-- Template elements with custom namespace attributes -->
    <div tmpl:if="user.loggedIn">
        <p tmpl:text="user.name">Username</p>
    </div>
</body>
</html>"##;

    let document = parse_html().one(html);

    println!("=== Namespace-Aware Selector Examples ===\n");

    // Example 1: Select SVG elements using namespace type selectors
    println!("1. Type Selectors with Namespaces:");

    let mut context = SelectorContext::new();
    context.add_namespace("svg".to_string(), ns!(svg));

    // Select all SVG rect elements
    let selectors = Selectors::compile_with_context("svg|rect", &context).unwrap();
    let rects = selectors
        .filter(document.descendants().elements())
        .collect::<Vec<_>>();

    println!("  Found {} <rect> elements in SVG namespace", rects.len());
    for rect in &rects {
        if let Some(fill) = rect.attributes.borrow().get("fill") {
            println!("    - rect with fill=\"{fill}\"");
        }
    }
    println!();

    // Example 2: Select all elements in SVG namespace using wildcard
    println!("2. Namespace Wildcard Selector:");

    let selectors = Selectors::compile_with_context("svg|*", &context).unwrap();
    let svg_elements = selectors
        .filter(document.descendants().elements())
        .collect::<Vec<_>>();

    println!("  Found {} elements in SVG namespace:", svg_elements.len());
    for elem in &svg_elements {
        println!("    - <{}>", elem.name.local);
    }
    println!();

    // Example 3: Attribute selectors with namespaces
    println!("3. Attribute Selectors with Namespaces:");

    let mut context = SelectorContext::new();
    context.add_namespace(
        "xlink".to_string(),
        html5ever::Namespace::from("http://www.w3.org/1999/xlink"),
    );

    // Select elements with xlink:href attribute
    let selectors = Selectors::compile_with_context("[xlink|href]", &context).unwrap();
    let xlink_elements = selectors
        .filter(document.descendants().elements())
        .collect::<Vec<_>>();

    println!(
        "  Found {} elements with xlink:href attribute",
        xlink_elements.len()
    );
    for elem in &xlink_elements {
        println!("    - <{}>", elem.name.local);
    }
    println!();

    // Example 4: Builder pattern for context configuration
    println!("4. Builder Pattern:");

    let mut context = SelectorContext::new();
    context
        .add_namespace("svg".to_string(), ns!(svg))
        .add_namespace("html".to_string(), ns!(html))
        .set_default_namespace(ns!(html));

    // Now we can use both prefixes
    let selectors = Selectors::compile_with_context("svg|circle", &context).unwrap();
    let circles = selectors
        .filter(document.descendants().elements())
        .collect::<Vec<_>>();

    println!(
        "  Found {} <circle> elements using chained configuration",
        circles.len()
    );
    println!();

    // Example 5: Regular selectors still work without namespace context
    println!("5. Backward Compatibility:");

    let selectors = Selectors::compile("p").unwrap();
    let paragraphs = selectors
        .filter(document.descendants().elements())
        .collect::<Vec<_>>();

    println!(
        "  Found {} <p> elements using regular selector",
        paragraphs.len()
    );
    println!("  (Regular selectors work without namespace context)");
}
