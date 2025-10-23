#![allow(clippy::print_stdout)]

//! Example demonstrating template processing workflow with namespace cleanup.
//!
//! This example shows how to:
//! 1. Parse HTML with template directives in a custom namespace
//! 2. Process template elements (simulated)
//! 3. Remove template elements by namespace
//! 4. Clean up xmlns namespace declarations
//! 5. Output clean HTML without template artifacts

#[macro_use]
extern crate html5ever;

use brik::parse_html;
use brik::traits::*;

fn main() {
    // Sample HTML with template directives in a custom namespace
    // Note: In real HTML parsing, custom namespaces in prefixed elements
    // are treated as regular HTML elements. This example demonstrates
    // the API pattern with SVG namespace (which is properly handled).
    let html = r#"<!DOCTYPE html>
<html>
<head>
    <title>Template Processing Example</title>
</head>
<body>
    <h1>Product List</h1>
    <div id="products">
        <!-- In a real template system, these might be processed server-side -->
        <svg xmlns="http://www.w3.org/2000/svg" class="icon" width="20" height="20">
            <circle r="10" cx="10" cy="10"/>
        </svg>
        <p>Regular HTML content</p>
        <svg xmlns="http://www.w3.org/2000/svg" class="icon" width="20" height="20">
            <rect width="20" height="20"/>
        </svg>
    </div>
</body>
</html>"#;

    let doc = parse_html().one(html);

    println!("=== Template Processing Workflow ===\n");

    // Step 1: Analyze the document
    println!("Step 1: Analyzing document structure");
    let all_elements: Vec<_> = doc.descendants().elements().collect();
    println!("  Total elements: {}", all_elements.len());

    let svg_elements: Vec<_> = doc
        .descendants()
        .elements()
        .elements_in_ns(ns!(svg))
        .collect();
    println!("  SVG elements (template artifacts): {}\n", svg_elements.len());

    // Step 2: Process template directives (simulated)
    println!("Step 2: Processing template directives");
    println!("  (In a real system, this would expand loops, conditionals, etc.)\n");

    // Step 3: Remove template elements by namespace
    println!("Step 3: Removing template elements");
    let svg_to_remove: Vec<_> = doc
        .descendants()
        .elements()
        .elements_in_ns(ns!(svg))
        .collect();

    svg_to_remove
        .into_iter()
        .map(|elem| elem.as_node().clone())
        .detach_all();

    println!("  Removed SVG elements\n");

    // Step 4: Clean up namespace declarations (if any)
    println!("Step 4: Cleaning up namespace declarations");
    // Note: HTML parser doesn't preserve xmlns:prefix declarations in the xmlns namespace
    // This step is included to demonstrate the API for XML-based workflows
    if let Ok(html_elem) = doc.select_first("html") {
        html_elem
            .attributes
            .borrow_mut()
            .remove_xmlns_for("http://www.w3.org/2000/svg");
        println!("  Cleaned up xmlns declarations\n");
    }

    // Step 5: Verify cleanup
    println!("Step 5: Verifying cleanup");
    let remaining_svg: Vec<_> = doc
        .descendants()
        .elements()
        .elements_in_ns(ns!(svg))
        .collect();
    println!("  Remaining SVG elements: {}", remaining_svg.len());

    let remaining_elements: Vec<_> = doc.descendants().elements().collect();
    println!("  Total remaining elements: {}\n", remaining_elements.len());

    // Step 6: Output clean HTML
    println!("Step 6: Generating clean HTML output");
    let mut output = Vec::new();
    doc.serialize(&mut output).unwrap();
    let html_output = String::from_utf8(output).unwrap();

    println!("---");
    println!("{html_output}");
    println!("---\n");

    println!("✓ Template processing complete!");
    println!("  Original: {} elements", all_elements.len());
    println!("  Final: {} elements", remaining_elements.len());
    println!("  Removed: {} elements", all_elements.len() - remaining_elements.len());
}
