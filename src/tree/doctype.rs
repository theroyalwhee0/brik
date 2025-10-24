/// Data specific to doctype nodes.
#[derive(Debug, PartialEq, Clone)]
pub struct Doctype {
    /// The name of the doctype
    pub name: String,

    /// The public ID of the doctype
    pub public_id: String,

    /// The system ID of the doctype
    pub system_id: String,
}
