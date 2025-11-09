//! HTML parser configuration options.

use std::borrow::Cow;

/// Options for the HTML parser.
#[derive(Default)]
pub struct ParseOpts {
    /// Options for the HTML tokenizer.
    pub tokenizer: html5ever::tokenizer::TokenizerOpts,

    /// Options for the HTML tree builder.
    pub tree_builder: html5ever::tree_builder::TreeBuilderOpts,

    /// A callback for HTML parse errors (which are never fatal).
    pub on_parse_error: Option<Box<dyn FnMut(Cow<'static, str>)>>,
}
