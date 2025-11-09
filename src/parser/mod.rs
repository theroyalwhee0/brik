//! HTML parsing functionality.
//!
//! This module provides HTML parsing using html5ever, with support for both
//! full document and fragment parsing modes.

pub mod parse_fragment;
pub mod parse_html;
pub mod parse_opts;
pub mod sink;

pub use parse_fragment::{parse_fragment, parse_fragment_with_options};
pub use parse_html::{parse_html, parse_html_with_options};
pub use parse_opts::ParseOpts;
pub use sink::Sink;
