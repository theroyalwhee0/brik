use crate::node_data_ref::NodeDataRef;
use crate::select::Selectors;
use crate::tree::ElementData;
use std::borrow::Borrow;

/// An element iterator adaptor that yields elements maching given selectors.
pub struct Select<I, S = Selectors>
where
    I: Iterator<Item = NodeDataRef<ElementData>>,
    S: Borrow<Selectors>,
{
    /// The underlying iterator.
    pub iter: I,

    /// The selectors to be matched.
    pub selectors: S,
}

impl<I, S> Iterator for Select<I, S>
where
    I: Iterator<Item = NodeDataRef<ElementData>>,
    S: Borrow<Selectors>,
{
    type Item = NodeDataRef<ElementData>;

    #[inline]
    fn next(&mut self) -> Option<NodeDataRef<ElementData>> {
        let selectors = self.selectors.borrow();
        self.iter
            .by_ref()
            .find(|element| selectors.matches(element))
    }
}

impl<I, S> DoubleEndedIterator for Select<I, S>
where
    I: DoubleEndedIterator<Item = NodeDataRef<ElementData>>,
    S: Borrow<Selectors>,
{
    #[inline]
    fn next_back(&mut self) -> Option<NodeDataRef<ElementData>> {
        let selectors = self.selectors.borrow();
        self.iter
            .by_ref()
            .rev()
            .find(|element| selectors.matches(element))
    }
}
