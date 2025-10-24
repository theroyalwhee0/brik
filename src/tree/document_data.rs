use html5ever::tree_builder::QuirksMode;
use std::cell::Cell;

/// Data specific to document nodes.
#[derive(Debug, PartialEq, Clone)]
pub struct DocumentData {
    #[doc(hidden)]
    pub _quirks_mode: Cell<QuirksMode>,
}

impl DocumentData {
    /// The quirks mode of the document, as determined by the HTML parser.
    #[inline]
    pub fn quirks_mode(&self) -> QuirksMode {
        self._quirks_mode.get()
    }
}
