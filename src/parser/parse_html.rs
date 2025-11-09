//! HTML document parsing functions.

use super::{ParseOpts, Sink};
use crate::tree::NodeRef;
use std::cell::RefCell;

/// Parse an HTML document with html5ever and the default configuration.
///
/// Returns an html5ever Parser that can be used with TendrilSink methods
/// to parse HTML from various sources.
///
/// # Examples
///
/// ```
/// use brik::parse_html;
/// use brik::traits::*;
///
/// let html = "<html><body><div>Hello, world!</div></body></html>";
/// let document = parse_html().one(html);
///
/// let div = document.select_first("div").unwrap();
/// assert_eq!(div.text_contents(), "Hello, world!");
/// ```
pub fn parse_html() -> html5ever::Parser<Sink> {
    parse_html_with_options(ParseOpts::default())
}

/// Parse an HTML document with html5ever with custom configuration.
pub fn parse_html_with_options(opts: ParseOpts) -> html5ever::Parser<Sink> {
    let sink = Sink {
        document_node: NodeRef::new_document(),
        on_parse_error: RefCell::new(opts.on_parse_error),
    };
    let html5opts = html5ever::ParseOpts {
        tokenizer: opts.tokenizer,
        tree_builder: opts.tree_builder,
    };
    html5ever::parse_document(sink, html5opts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::*;
    use html5ever::tree_builder::QuirksMode;
    use std::path::Path;

    /// Tests parsing HTML and serializing back to a string.
    ///
    /// Verifies that the parser correctly constructs a DOM tree from HTML input,
    /// detects quirks mode, and can serialize the result back to normalized HTML.
    #[test]
    fn parse_and_serialize() {
        let html = r"
<!doctype html>
<title>Test case</title>
<p>Content";
        let document = parse_html().one(html);
        assert_eq!(
            document.as_document().unwrap().quirks_mode(),
            QuirksMode::NoQuirks
        );
        assert_eq!(
            document.to_string(),
            r"<!DOCTYPE html><html><head><title>Test case</title>
</head><body><p>Content</p></body></html>"
        );
    }

    /// Tests parsing HTML with template elements.
    ///
    /// Verifies that the parser correctly handles HTML template elements,
    /// which have special parsing rules and maintain separate content trees.
    #[test]
    fn parse_and_serialize_with_template() {
        let html = r"
<!doctype html>
<title>Test case</title>
<template><p>Content</p></template>";
        let document = parse_html().one(html);
        assert_eq!(
            document.as_document().unwrap().quirks_mode(),
            QuirksMode::NoQuirks
        );
        assert_eq!(
            document.to_string(),
            r"<!DOCTYPE html><html><head><title>Test case</title>
<template><p>Content</p></template></head><body></body></html>"
        );
    }

    /// Tests parsing HTML from a file.
    ///
    /// Verifies that the parser can read and parse HTML content from
    /// a file path, producing the expected DOM structure.
    #[test]
    fn parse_file() {
        let mut path = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
        path.push("test_data");
        path.push("foo.html");

        let html = r"<!DOCTYPE html><html><head>
        <title>Test case</title>
    </head>
    <body>
        <p>Foo</p>
    

</body></html>";
        let document = parse_html().from_utf8().from_file(&path).unwrap();
        assert_eq!(document.to_string(), html);
    }
}
