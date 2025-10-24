use super::{ElementsInNamespace, Select};
use crate::node_data_ref::NodeDataRef;
use crate::select::Selectors;
use crate::tree::ElementData;

/// Convenience methods for element iterators.
pub trait ElementIterator: Sized + Iterator<Item = NodeDataRef<ElementData>> {
    /// Filter this element iterator to elements maching the given selectors.
    ///
    /// # Errors
    ///
    /// Returns `Err(())` if the selector string fails to parse.
    #[inline]
    fn select(self, selectors: &str) -> Result<Select<Self>, ()> {
        Selectors::compile(selectors).map(|s| Select {
            iter: self,
            selectors: s,
        })
    }

    /// Filter this element iterator to elements in the given namespace.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate html5ever;
    /// use brik::parse_html;
    /// use brik::traits::*;
    ///
    /// let html = r#"<!DOCTYPE html>
    /// <html>
    /// <body>
    ///   <div>HTML element</div>
    ///   <svg xmlns="http://www.w3.org/2000/svg">
    ///     <circle r="10"/>
    ///     <rect width="20" height="20"/>
    ///   </svg>
    /// </body>
    /// </html>
    /// "#;
    ///
    /// let doc = parse_html().one(html);
    ///
    /// // Find all SVG elements
    /// let svg_elements: Vec<_> = doc
    ///     .descendants()
    ///     .elements()
    ///     .elements_in_ns(ns!(svg))
    ///     .collect();
    ///
    /// assert_eq!(svg_elements.len(), 3); // svg, circle, rect
    /// ```
    #[inline]
    fn elements_in_ns(self, namespace: html5ever::Namespace) -> ElementsInNamespace<Self> {
        ElementsInNamespace {
            iter: self,
            namespace,
        }
    }
}

impl<I> ElementIterator for I where I: Iterator<Item = NodeDataRef<ElementData>> {}
