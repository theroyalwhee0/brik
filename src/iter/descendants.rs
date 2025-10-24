use super::node_edge::NodeEdge;
use super::traverse::Traverse;
use crate::tree::NodeRef;

/// An iterator of references to a given node and its descendants, in tree order.
#[derive(Debug, Clone)]
pub struct Descendants(pub(super) Traverse);

/// Macro to implement iterator methods for descendant traversal.
macro_rules! descendants_next {
    ($next: ident) => {
        #[inline]
        fn $next(&mut self) -> Option<NodeRef> {
            loop {
                match (self.0).$next() {
                    Some(NodeEdge::Start(node)) => return Some(node),
                    Some(NodeEdge::End(_)) => {}
                    None => return None,
                }
            }
        }
    };
}

impl Iterator for Descendants {
    type Item = NodeRef;
    descendants_next!(next);
}

impl DoubleEndedIterator for Descendants {
    descendants_next!(next_back);
}
