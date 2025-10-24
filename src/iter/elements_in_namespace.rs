use crate::node_data_ref::NodeDataRef;
use crate::tree::ElementData;

/// An element iterator adaptor that yields elements in a specific namespace.
#[derive(Debug, Clone)]
pub struct ElementsInNamespace<I> {
    /// The underlying iterator.
    pub(super) iter: I,
    /// The namespace to filter by.
    pub(super) namespace: html5ever::Namespace,
}

impl<I> Iterator for ElementsInNamespace<I>
where
    I: Iterator<Item = NodeDataRef<ElementData>>,
{
    type Item = NodeDataRef<ElementData>;

    #[inline]
    fn next(&mut self) -> Option<NodeDataRef<ElementData>> {
        let namespace = &self.namespace;
        self.iter
            .by_ref()
            .find(|element| element.namespace_uri() == namespace)
    }
}

impl<I> DoubleEndedIterator for ElementsInNamespace<I>
where
    I: DoubleEndedIterator<Item = NodeDataRef<ElementData>>,
{
    #[inline]
    fn next_back(&mut self) -> Option<NodeDataRef<ElementData>> {
        let namespace = &self.namespace;
        self.iter
            .by_ref()
            .rev()
            .find(|element| element.namespace_uri() == namespace)
    }
}
