use super::node_edge::NodeEdge;
use super::siblings::State;
use crate::tree::NodeRef;

/// An iterator of the start and end edges of the nodes in a given subtree.
#[derive(Debug, Clone)]
pub struct Traverse(pub(super) Option<State<NodeEdge<NodeRef>>>);

/// Macro to implement iterator methods for tree traversal with start/end edges.
///
/// This macro generates the next() or next_back() method for the Traverse iterator,
/// handling the complex state transitions for depth-first tree traversal. The traversal
/// yields both Start and End edges for each node, allowing consumers to track when
/// they enter and exit each node in the tree.
macro_rules! traverse_next {
    ($next: ident, $next_back: ident, $first_child: ident, $next_sibling: ident, $Start: ident, $End: ident) => {
        fn $next(&mut self) -> Option<NodeEdge<NodeRef>> {
            #![allow(non_shorthand_field_patterns)]
            self.0.take().map(
                |State {
                     $next: next,
                     $next_back: next_back,
                 }| {
                    if next != next_back {
                        self.0 = match next {
                            NodeEdge::$Start(ref node) => match node.$first_child() {
                                Some(child) => Some(State {
                                    $next: NodeEdge::$Start(child),
                                    $next_back: next_back,
                                }),
                                None => Some(State {
                                    $next: NodeEdge::$End(node.clone()),
                                    $next_back: next_back,
                                }),
                            },
                            NodeEdge::$End(ref node) => match node.$next_sibling() {
                                Some(sibling) => Some(State {
                                    $next: NodeEdge::$Start(sibling),
                                    $next_back: next_back,
                                }),
                                None => node.parent().map(|parent| State {
                                    $next: NodeEdge::$End(parent),
                                    $next_back: next_back,
                                }),
                            },
                        };
                    }
                    next
                },
            )
        }
    };
}

/// Implements Iterator for Traverse.
///
/// Yields NodeEdge items representing both the start and end of each node
/// in depth-first pre-order traversal, allowing full tree structure tracking.
impl Iterator for Traverse {
    type Item = NodeEdge<NodeRef>;
    traverse_next!(next, next_back, first_child, next_sibling, Start, End);
}

/// Implements DoubleEndedIterator for Traverse.
///
/// Allows iterating in reverse order from the back, yielding End edges before
/// Start edges in reverse depth-first traversal.
impl DoubleEndedIterator for Traverse {
    traverse_next!(next_back, next, last_child, previous_sibling, End, Start);
}
