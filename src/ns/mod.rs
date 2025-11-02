//! Namespace handling for HTML documents.
//!
//! This module provides tools for injecting missing namespace declarations into HTML
//! documents, particularly useful when working with prefixed elements (like `svg:rect`)
//! that require namespace declarations on the root `<html>` element.
//!
//! # Architecture
//!
//! The module uses a **slice-based design** for efficient integration with html5ever:
//!
//! 1. **Parse**: HTML is parsed to locate the `<html>` tag insertion point
//! 2. **Store**: Original HTML and insertion position are stored (no copying)
//! 3. **Output**: When consumed, yields string slices that can be:
//!    - Concatenated into a single String (`Into<String>`)
//!    - Converted to a StrTendril (`From<NsDefaults>`)
//!    - Iterated as slices (`IntoIterator`) for zero-copy html5ever parsing
//!
//! This design avoids unnecessary string allocation until the result is actually needed.
//!
//! # Example
//!
//! ```ignore
//! use brik::ns::NsDefaultsBuilder;
//! use brik::parse_html;
//!
//! let html = r#"<html><body><svg:rect /></body></html>"#;
//!
//! // Inject missing namespace declaration
//! let ns_defaults = NsDefaultsBuilder::new()
//!     .namespace("svg", "http://www.w3.org/2000/svg")
//!     .from_string(html)?;
//!
//! // Use with html5ever (zero-copy via IntoIterator)
//! let doc = parse_html().from_iter(ns_defaults);
//! ```

/// Default namespace configuration and injection.
pub mod defaults;
/// Error types for namespace operations.
mod error;

pub use defaults::{NsDefaults, NsDefaultsBuilder};
pub use error::{NsError, NsResult};
