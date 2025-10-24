use super::{AttrValue, LocalNameSelector, PseudoClass, PseudoElement};
use html5ever::{LocalName, Namespace};
use selectors::parser::SelectorImpl;

/// Selector implementation for Brik's DOM.
#[derive(Debug, Clone)]
pub struct BrikSelectors;

/// Implements SelectorImpl for BrikSelectors.
///
/// Provides the type mappings required by the selectors crate for CSS
/// selector parsing and matching. Associates Brik's selector types
/// (LocalNameSelector, AttrValue, PseudoClass, PseudoElement) with
/// the generic selector implementation.
impl SelectorImpl for BrikSelectors {
    type AttrValue = AttrValue;
    type Identifier = LocalNameSelector;
    type LocalName = LocalNameSelector;
    type NamespacePrefix = LocalNameSelector;
    type NamespaceUrl = Namespace;
    type BorrowedNamespaceUrl = Namespace;
    type BorrowedLocalName = LocalName;

    type NonTSPseudoClass = PseudoClass;
    type PseudoElement = PseudoElement;

    type ExtraMatchingData<'a> = ();
}
