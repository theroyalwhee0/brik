use html5ever::{LocalName, Namespace};

/// <https://www.w3.org/TR/REC-xml-names/#dt-expname>
#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub struct ExpandedName {
    /// Namespace URL
    pub ns: Namespace,
    /// "Local" part of the name
    pub local: LocalName,
}

impl ExpandedName {
    /// Trivial constructor
    pub fn new<N: Into<Namespace>, L: Into<LocalName>>(ns: N, local: L) -> Self {
        ExpandedName {
            ns: ns.into(),
            local: local.into(),
        }
    }
}
