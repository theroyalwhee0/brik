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

/// Implements Iterator for Descendants.
///
/// Yields nodes in tree order (depth-first pre-order traversal),
/// returning each node as it is first encountered.
impl Iterator for Descendants {
    type Item = NodeRef;
    descendants_next!(next);
}

/// Implements DoubleEndedIterator for Descendants.
///
/// Allows iterating in reverse tree order by calling `next_back()`.
impl DoubleEndedIterator for Descendants {
    descendants_next!(next_back);
}
