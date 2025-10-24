use html5ever::Prefix;

/// The non-identifying parts of an attribute
#[derive(Debug, PartialEq, Clone)]
pub struct Attribute {
    /// The namespace prefix, if any
    pub prefix: Option<Prefix>,
    /// The attribute value
    pub value: String,
}
