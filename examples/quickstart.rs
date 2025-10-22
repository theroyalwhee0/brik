//! Quick start example showing basic HTML parsing, querying, and manipulation.

#![allow(clippy::print_stdout)]

use kuchikiki::parse_html;
use kuchikiki::traits::*;

fn main() {
    // Parse an HTML document
    let document = parse_html().one(r#"
        <html>
            <body>
                <p class="greeting">Hello, world!</p>
            </body>
        </html>
    "#);

    // Query with CSS selectors
    if let Ok(match_) = document.select_first(".greeting") {
        let node = match_.as_node();
        println!("{}", node.text_contents());  // Prints: Hello, world!
    }

    // Manipulate the tree
    let new_paragraph = parse_html().one("<p>New content</p>");
    document
        .select_first("body")
        .unwrap()
        .as_node()
        .append(new_paragraph.first_child().unwrap());

    // Serialize back to HTML
    let mut html_bytes = Vec::new();
    document.serialize(&mut html_bytes).unwrap();
    println!("\nSerialized HTML ({} bytes):", html_bytes.len());
    let html_string = String::from_utf8(html_bytes).expect("HTML should be valid UTF-8");
    println!("{html_string}");
}
