use crate::node_data_ref::NodeDataRef;
use crate::tree::{ElementData, NodeRef};
use std::cell::RefCell;

/// Macro to create filter-map-like iterator wrappers.
macro_rules! filter_map_like_iterator {
    (#[$doc: meta] $name: ident: $f: expr, $from: ty => $to: ty) => {
        #[$doc]
        #[derive(Debug, Clone)]
        pub struct $name<I>(pub I);

        impl<I> Iterator for $name<I>
        where
            I: Iterator<Item = $from>,
        {
            type Item = $to;

            #[inline]
            fn next(&mut self) -> Option<$to> {
                for x in self.0.by_ref() {
                    if let Some(y) = ($f)(x) {
                        return Some(y);
                    }
                }
                None
            }
        }

        impl<I> DoubleEndedIterator for $name<I>
        where
            I: DoubleEndedIterator<Item = $from>,
        {
            #[inline]
            fn next_back(&mut self) -> Option<$to> {
                for x in self.0.by_ref().rev() {
                    if let Some(y) = ($f)(x) {
                        return Some(y);
                    }
                }
                None
            }
        }
    };
}

filter_map_like_iterator! {
    /// A node iterator adaptor that yields element nodes.
    Elements: NodeRef::into_element_ref, NodeRef => NodeDataRef<ElementData>
}

filter_map_like_iterator! {
    /// A node iterator adaptor that yields comment nodes.
    Comments: NodeRef::into_comment_ref, NodeRef => NodeDataRef<RefCell<String>>
}

filter_map_like_iterator! {
    /// A node iterator adaptor that yields text nodes.
    TextNodes: NodeRef::into_text_ref, NodeRef => NodeDataRef<RefCell<String>>
}
