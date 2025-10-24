use crate::tree::NodeRef;

/// Internal state for double-ended iterators.
#[derive(Debug, Clone)]
pub(super) struct State<T> {
    /// The next item to be returned from the front of the iterator.
    pub(super) next: T,
    /// The next item to be returned from the back of the iterator.
    pub(super) next_back: T,
}

/// A double-ended iterator of sibling nodes.
#[derive(Debug, Clone)]
pub struct Siblings(pub(super) Option<State<NodeRef>>);

/// Macro to implement iterator methods for sibling traversal.
macro_rules! siblings_next {
    ($next: ident, $next_back: ident, $next_sibling: ident) => {
        fn $next(&mut self) -> Option<NodeRef> {
            #![allow(non_shorthand_field_patterns)]
            self.0.take().map(
                |State {
                     $next: next,
                     $next_back: next_back,
                 }| {
                    if let Some(sibling) = next.$next_sibling() {
                        if next != next_back {
                            self.0 = Some(State {
                                $next: sibling,
                                $next_back: next_back,
                            })
                        }
                    }
                    next
                },
            )
        }
    };
}

impl Iterator for Siblings {
    type Item = NodeRef;
    siblings_next!(next, next_back, next_sibling);
}

impl DoubleEndedIterator for Siblings {
    siblings_next!(next_back, next, previous_sibling);
}
