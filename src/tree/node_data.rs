use std::cell::RefCell;

use super::{Doctype, DocumentData, ElementData};

/// Node data specific to the node type.
#[derive(Debug, PartialEq, Clone)]
pub enum NodeData {
    /// Element node
    Element(ElementData),

    /// Text node
    Text(RefCell<String>),

    /// Comment node
    Comment(RefCell<String>),

    /// Processing instruction node
    ProcessingInstruction(RefCell<(String, String)>),

    /// Doctype node
    Doctype(Doctype),

    /// Document node
    Document(DocumentData),

    /// Document fragment node
    DocumentFragment,
}
