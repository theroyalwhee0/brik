use crate::tree::NodeRef;

/// An iterator on ancestor nodes.
#[derive(Debug, Clone)]
pub struct Ancestors(pub(super) Option<NodeRef>);

impl Iterator for Ancestors {
    type Item = NodeRef;

    #[inline]
    fn next(&mut self) -> Option<NodeRef> {
        self.0.take().inspect(|node| {
            self.0 = node.parent();
        })
    }
}
