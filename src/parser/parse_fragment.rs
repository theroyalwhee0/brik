//! HTML fragment parsing functions.

use super::{ParseOpts, Sink};
use crate::tree::NodeRef;
use html5ever::{Attribute, QualName};
use std::cell::RefCell;

/// Parse an HTML fragment with html5ever and the default configuration.
///
/// Fragment parsing requires a context element (name and attributes) which
/// affects how the HTML5 parser interprets the fragment content.
///
/// # Examples
///
/// ```
/// use brik::parse_fragment;
/// use brik::traits::*;
///
/// # #[macro_use] extern crate html5ever;
/// # fn main() {
/// let ctx_name = html5ever::QualName::new(None, ns!(html), local_name!("tbody"));
/// let html = "<tr><td>Cell 1</td><td>Cell 2</td></tr>";
/// let document = parse_fragment(ctx_name, vec![]).one(html);
///
/// let td = document.select_first("td").unwrap();
/// assert_eq!(td.text_contents(), "Cell 1");
/// # }
/// ```
pub fn parse_fragment(ctx_name: QualName, ctx_attr: Vec<Attribute>) -> html5ever::Parser<Sink> {
    parse_fragment_with_options(ParseOpts::default(), ctx_name, ctx_attr)
}

/// Parse an HTML fragment with html5ever with custom configuration.
pub fn parse_fragment_with_options(
    opts: ParseOpts,
    ctx_name: QualName,
    ctx_attr: Vec<Attribute>,
) -> html5ever::Parser<Sink> {
    let sink = Sink {
        document_node: NodeRef::new_document(),
        on_parse_error: RefCell::new(opts.on_parse_error),
    };
    let html5opts = html5ever::ParseOpts {
        tokenizer: opts.tokenizer,
        tree_builder: opts.tree_builder,
    };
    html5ever::parse_fragment(sink, html5opts, ctx_name, ctx_attr, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::*;
    use html5ever::tree_builder::QuirksMode;

    /// Tests parsing an HTML fragment with a specific context.
    ///
    /// Verifies that fragment parsing respects the context element, which
    /// affects how the HTML5 parser interprets the fragment content.
    #[test]
    fn parse_and_serialize_fragment() {
        let html = r"<tbody><tr><td>Test case";

        let ctx_name = QualName::new(None, ns!(html), local_name!("tbody"));
        let document = parse_fragment(ctx_name, vec![]).one(html);
        assert_eq!(
            document.as_document().unwrap().quirks_mode(),
            QuirksMode::NoQuirks
        );
        assert_eq!(
            document.to_string(),
            r"<html><tr><td>Test case</td></tr></html>"
        );
    }
}
