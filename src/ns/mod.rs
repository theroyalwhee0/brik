//! Namespace handling for HTML documents.
//!
//! This module provides tools for processing namespace declarations in HTML documents,
//! particularly useful when working with prefixed elements (like `svg:rect`, `c:widget`)
//! that use namespace prefixes.
//!
//! # Overview
//!
//! Since HTML5 parsers don't process namespace prefixes, elements like `<svg:rect>` are
//! parsed as literal tag names. This module provides post-processing functions to split
//! prefixed names and apply namespace URIs based on `xmlns:*` declarations.
//!
//! # Example
//!
//! ```
//! #[cfg(feature = "namespaces")]
//! {
//! use brik::ns::{NsOptions, NsError};
//! use brik::parse_html;
//! use brik::traits::*;
//! use html5ever::ns;
//! use std::collections::HashMap;
//!
//! let html = r#"<html xmlns:c="https://example.com/custom">
//!     <body><svg:rect /><c:widget>Content</c:widget></body>
//! </html>"#;
//!
//! let doc = parse_html().one(html);
//!
//! // Provide additional namespaces via options
//! let mut namespaces = HashMap::new();
//! namespaces.insert("svg".to_string(), ns!(svg));
//!
//! let options = NsOptions {
//!     namespaces,
//!     strict: false,
//! };
//!
//! // Apply namespace processing
//! let corrected = doc.apply_xmlns_opts(&options).unwrap();
//!
//! // Now prefixes are properly split and namespaced
//! let widget = corrected.select_first("widget").unwrap();
//! assert_eq!(widget.prefix().unwrap().as_ref(), "c");
//! assert_eq!(widget.namespace_uri().as_ref(), "https://example.com/custom");
//! }
//! ```

/// Apply xmlns declarations to document elements and attributes.
mod apply_xmlns;
/// Default namespace configuration and injection.
///
/// **DEPRECATED**: This module is deprecated. Use [`apply_xmlns_opts`] with [`NsOptions`] instead.
#[deprecated(
    since = "0.9.2",
    note = "Use `apply_xmlns_opts` with `NsOptions` instead of NsDefaultsBuilder"
)]
pub mod defaults;
/// Error types for namespace operations.
mod error;

#[allow(deprecated)]
pub use apply_xmlns::{apply_xmlns, apply_xmlns_opts, apply_xmlns_strict, NsOptions};
#[allow(deprecated)]
pub use defaults::{NsDefaults, NsDefaultsBuilder};
pub use error::{NsError, NsResult};
