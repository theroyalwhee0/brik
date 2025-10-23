#![allow(clippy::print_stdout)]

//! Example demonstrating namespace support with HTML and SVG.
//!
//! This example shows how to work with namespaces when parsing HTML documents
//! that contain embedded SVG elements.

#[macro_use]
extern crate html5ever;

use brik::parse_html;
use brik::traits::*;

fn main() {
    // Sample HTML with embedded SVG
    let html = r#"<!DOCTYPE html>
<html>
<head>
    <title>Namespace Example</title>
</head>
<body>
    <h1>SVG in HTML</h1>
    <svg xmlns="http://www.w3.org/2000/svg" width="200" height="100">
        <rect x="10" y="10" width="180" height="80" fill="blue"/>
        <circle cx="100" cy="50" r="30" fill="red"/>
    </svg>
    <p>SVG elements have different namespaces than HTML elements.</p>
</body>
</html>"#;

    let document = parse_html().one(html);

    println!("=== Namespace Support Example ===\n");

    // Example 1: Check element namespaces
    println!("1. Element Namespaces:");
    let h1 = document.select_first("h1").unwrap();
    println!("  <h1> namespace: {}", h1.namespace_uri().as_ref());
    println!("  <h1> local name: {}", h1.local_name().as_ref());

    let svg = document.select_first("svg").unwrap();
    println!("  <svg> namespace: {}", svg.namespace_uri().as_ref());
    println!("  <svg> local name: {}", svg.local_name().as_ref());

    let rect = document.select_first("rect").unwrap();
    println!("  <rect> namespace: {}", rect.namespace_uri().as_ref());
    println!("  <rect> local name: {}\n", rect.local_name().as_ref());

    // Example 2: Working with SVG attributes
    println!("2. SVG Attributes:");
    let attrs = svg.attributes.borrow();

    // All these attributes are in the null namespace
    println!(
        "  SVG width: {}",
        attrs.get_ns(ns!(), "width").unwrap_or("N/A")
    );
    println!(
        "  SVG height: {}",
        attrs.get_ns(ns!(), "height").unwrap_or("N/A")
    );
    println!("  Has xmlns? {}\n", attrs.has_ns(ns!(), "xmlns"));

    // Example 3: Iterating attributes by namespace
    println!("3. All null namespace attributes on <svg>:");
    let mut null_attrs: Vec<_> = attrs.attrs_in_ns(ns!()).collect();
    null_attrs.sort_by(|(a, _), (b, _)| a.as_ref().cmp(b.as_ref()));

    for (name, value) in null_attrs {
        println!("  {}: {}", name.as_ref(), value);
    }
    println!();

    // Example 4: Comparing HTML and SVG elements
    println!("4. Namespace Comparison:");
    let p = document.select_first("p").unwrap();

    println!("  HTML <p>:");
    println!("    namespace: {}", p.namespace_uri().as_ref());
    println!(
        "    Is XHTML? {}",
        p.namespace_uri().as_ref() == "http://www.w3.org/1999/xhtml"
    );

    println!("  SVG <circle>:");
    let circle = document.select_first("circle").unwrap();
    println!("    namespace: {}", circle.namespace_uri().as_ref());
    println!(
        "    Is SVG? {}",
        circle.namespace_uri().as_ref() == "http://www.w3.org/2000/svg"
    );
    println!();

    // Example 5: Working with rect attributes
    println!("5. Rectangle Attributes:");
    let rect_attrs = rect.attributes.borrow();

    if let Some(x) = rect_attrs.get_ns(ns!(), "x") {
        println!("  x position: {x}");
    }
    if let Some(y) = rect_attrs.get_ns(ns!(), "y") {
        println!("  y position: {y}");
    }
    if let Some(width) = rect_attrs.get_ns(ns!(), "width") {
        println!("  width: {width}");
    }
    if let Some(height) = rect_attrs.get_ns(ns!(), "height") {
        println!("  height: {height}");
    }
    if let Some(fill) = rect_attrs.get_ns(ns!(), "fill") {
        println!("  fill color: {fill}");
    }
}
