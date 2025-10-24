/// The non-identifying parts of an attribute.
pub mod attrib;

/// Convenience wrapper around an IndexMap for HTML/XML attributes.
pub mod attribs;

/// Expanded name with namespace URL and local name.
pub mod expanded_name;

pub use attrib::Attribute;
pub use attribs::Attributes;
pub use expanded_name::ExpandedName;
