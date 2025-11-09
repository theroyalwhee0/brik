#![allow(clippy::print_stdout)]

//! Example demonstrating apply_xmlns functionality.
//!
//! This example shows how to use apply_xmlns and apply_xmlns_strict to process
//! HTML documents with namespace-prefixed elements and attributes.

use brik::ns::NsError;
use brik::parse_html;
use brik::traits::*;

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

    // Example 3: Document with undefined namespaces (strict)
    println!("3. Document with undefined namespace (strict mode):");
    let doc3 = parse_html().one(html_no_ns);

    match doc3.apply_xmlns_strict() {
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

    // Example 4: Mixed defined and undefined prefixes
    println!("4. Mixed defined and undefined prefixes:");
    let html_mixed = r#"<html xmlns:good="https://example.com/good">
    <body>
        <good:element>This one is defined</good:element>
        <bad:element>This one is NOT defined</bad:element>
    </body>
</html>"#;

    let doc4 = parse_html().one(html_mixed);

    match doc4.apply_xmlns_strict() {
        Ok(_) => println!("   All namespaces defined"),
        Err(NsError::UndefinedPrefix(result_doc, prefixes)) => {
            println!(
                "   Found {} undefined prefix(es): {:?}",
                prefixes.len(),
                prefixes
            );

            // Good element should have namespace
            if let Ok(good) = result_doc.select_first("element") {
                println!("   First <element>:");
                println!("     - Prefix: {:?}", good.prefix().map(|p| p.as_ref()));
                println!("     - Namespace: {}", good.namespace_uri().as_ref());
            }

            // Find the bad element (second one)
            let mut count = 0;
            for node in result_doc.descendants() {
                if let Some(elem) = node.as_element() {
                    if elem.name.local.as_ref() == "element" {
                        count += 1;
                        if count == 2 {
                            println!("   Second <element>:");
                            if let Some(prefix) = &elem.name.prefix {
                                println!("     - Prefix: {:?}", prefix.as_ref());
                            }
                            println!("     - Namespace: '{}' (null)", elem.name.ns.as_ref());
                        }
                    }
                }
            }
        }
        Err(e) => println!("   Unexpected error: {e}"),
    }
}
