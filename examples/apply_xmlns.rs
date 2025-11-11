#![allow(clippy::print_stdout)]

//! Example demonstrating apply_xmlns functionality.
//!
//! This example shows how to use apply_xmlns and apply_xmlns_opts to process
//! HTML documents with namespace-prefixed elements and attributes.

use brik::ns::{NsError, NsOptions};
use brik::parse_html;
use brik::traits::*;
use html5ever::ns;
use std::collections::HashMap;

fn main() {
    println!("=== apply_xmlns Example ===\n");

    // Example 1: Document with defined namespaces
    println!("1. Document with properly defined namespaces:");
    let html_with_ns = r#"<html xmlns:c="https://example.com/custom" xmlns:foo="https://example.com/foo">
    <head><title>Example</title></head>
    <body>
        <c:widget id="main" foo:attr="test">
            <c:child>Content</c:child>
        </c:widget>
    </body>
</html>"#;

    let doc = parse_html().one(html_with_ns);
    println!("   Original HTML parsed (element names not yet split)");

    let corrected = doc.apply_xmlns().unwrap();
    println!("   ✓ Applied xmlns declarations");

    // Check the widget element
    if let Ok(widget) = corrected.select_first("widget") {
        println!("   Found <widget> element:");
        println!("     - Namespace: {}", widget.namespace_uri().as_ref());
        println!("     - Prefix: {:?}", widget.prefix().map(|p| p.as_ref()));
        println!("     - Local name: {}", widget.local_name().as_ref());

        let attrs = widget.attributes.borrow();
        for (name, attr) in &attrs.map {
            if name.local.as_ref() == "attr" {
                println!("     - Attribute 'foo:attr':");
                println!("       - Namespace: {}", name.ns.as_ref());
                println!(
                    "       - Prefix: {:?}",
                    attr.prefix.as_ref().map(|p| p.as_ref())
                );
                println!("       - Value: {}", attr.value);
            }
        }
    }
    println!();

    // Example 2: Document with undefined namespaces (lenient)
    println!("2. Document with undefined namespace (lenient mode):");
    let html_no_ns = r#"<html>
    <body>
        <c:widget>This prefix 'c' is not defined!</c:widget>
    </body>
</html>"#;

    let doc2 = parse_html().one(html_no_ns);
    let corrected2 = doc2.apply_xmlns().unwrap();
    println!("   ✓ Processed with apply_xmlns() - no error");

    if let Ok(widget) = corrected2.select_first("widget") {
        println!("   Found <widget> element:");
        println!(
            "     - Namespace: '{}' (empty = null)",
            widget.namespace_uri().as_ref()
        );
        println!("     - Prefix: {:?}", widget.prefix().map(|p| p.as_ref()));
        println!("     - Local name: {}", widget.local_name().as_ref());
    }
    println!();

    // Example 3: Document with undefined namespaces (strict mode using NsOptions)
    println!("3. Document with undefined namespace (strict mode using NsOptions):");
    let doc3 = parse_html().one(html_no_ns);

    let strict_options = NsOptions {
        namespaces: HashMap::new(),
        strict: true,
    };

    match doc3.apply_xmlns_opts(&strict_options) {
        Ok(_) => println!("   All namespaces defined (unexpected)"),
        Err(NsError::UndefinedPrefix(result_doc, prefixes)) => {
            println!("   ✗ Error: Found undefined prefixes: {prefixes:?}");
            println!("   But the processed document is still available:");

            if let Ok(widget) = result_doc.select_first("widget") {
                println!("     - Element <widget> was still processed");
                println!("     - Prefix: {:?}", widget.prefix().map(|p| p.as_ref()));
                println!(
                    "     - Namespace: '{}' (null)",
                    widget.namespace_uri().as_ref()
                );
            }
        }
        Err(e) => println!("   Unexpected error: {e}"),
    }
    println!();

    // Example 4: Providing additional namespaces via NsOptions
    println!("4. Providing additional namespaces via NsOptions:");
    let html_svg = r#"<html>
    <body>
        <svg:rect width="100" height="100" />
        <c:widget>Custom widget</c:widget>
    </body>
</html>"#;

    let doc4 = parse_html().one(html_svg);

    // Provide SVG namespace via options
    let mut namespaces = HashMap::new();
    namespaces.insert("svg".to_string(), ns!(svg));

    let options_with_svg = NsOptions {
        namespaces,
        strict: true, // Strict mode - will error on undefined 'c' prefix
    };

    match doc4.apply_xmlns_opts(&options_with_svg) {
        Ok(_) => println!("   All namespaces defined"),
        Err(NsError::UndefinedPrefix(result_doc, prefixes)) => {
            println!(
                "   Found {} undefined prefix(es): {:?}",
                prefixes.len(),
                prefixes
            );
            println!("   Note: 'svg' was provided via options, but 'c' was not defined");

            // SVG rect should have proper namespace
            if let Ok(rect) = result_doc.select_first("rect") {
                println!("   <rect> element:");
                println!("     - Prefix: {:?}", rect.prefix().map(|p| p.as_ref()));
                println!("     - Namespace: {}", rect.namespace_uri().as_ref());
            }

            // Widget should have null namespace
            if let Ok(widget) = result_doc.select_first("widget") {
                println!("   <widget> element:");
                println!("     - Prefix: {:?}", widget.prefix().map(|p| p.as_ref()));
                println!(
                    "     - Namespace: '{}' (null)",
                    widget.namespace_uri().as_ref()
                );
            }
        }
        Err(e) => println!("   Unexpected error: {e}"),
    }
}
