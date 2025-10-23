/*!

Brik is an HTML tree manipulation library for Rust.

A building block for HTML parsing and manipulation - simple, solid, and stackable.

# Quick Start

```rust
use brik::parse_html;
use brik::traits::*;

let document = parse_html().one("<p class='greeting'>Hello, world!</p>");
let greeting = document.select_first(".greeting").unwrap();
assert_eq!(greeting.text_contents(), "Hello, world!");
```

*/

#[macro_use]
extern crate html5ever;

/// Attribute handling and storage.
mod attributes;
/// Specialized Cell methods for performance-critical operations.
mod cell_extras;
/// Node iteration and traversal.
pub mod iter;
/// Type-safe node data references.
mod node_data_ref;
/// HTML parsing into the tree structure.
mod parser;
/// CSS selector matching implementation.
mod select;
/// HTML serialization from the tree structure.
mod serializer;
/// Test suite.
#[cfg(test)]
mod tests;
/// DOM tree structure and manipulation.
mod tree;

pub use attributes::{Attribute, Attributes, ExpandedName};
pub use node_data_ref::NodeDataRef;
pub use parser::{parse_fragment, parse_html, parse_html_with_options, ParseOpts, Sink};
pub use select::{Selector, Selectors, Specificity};
pub use tree::{Doctype, DocumentData, ElementData, Node, NodeData, NodeRef};

// Re-export namespace-related types from html5ever for convenience
pub use html5ever::{LocalName, Namespace, Prefix};

/// This module re-exports a number of traits that are useful when using Brik.
/// It can be used with:
///
/// ```rust
/// use brik::traits::*;
/// ```
pub mod traits {
    pub use crate::iter::{ElementIterator, NodeIterator};
    pub use html5ever::tendril::TendrilSink;
}
