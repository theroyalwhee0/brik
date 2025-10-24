use crate::tree::{Doctype, DocumentData, ElementData, Node, NodeRef};
use std::cell::RefCell;
use std::fmt;
use std::ops::Deref;

#[cfg(feature = "safe")]
use std::marker::PhantomData;

/// Discriminant for the type of node data being referenced (safe mode only).
#[cfg(feature = "safe")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NodeDataKind {
    /// Element node.
    Element,
    /// Text node.
    Text,
    /// Comment node.
    Comment,
    /// Processing instruction node.
    ProcessingInstruction,
    /// Doctype node.
    Doctype,
    /// Document node.
    Document,
    /// Document fragment node.
    DocumentFragment,
}

impl NodeRef {
    /// If this node is an element, return a strong reference to element-specific data.
    #[inline]
    pub fn into_element_ref(self) -> Option<NodeDataRef<ElementData>> {
        NodeDataRef::new_opt(self, Node::as_element)
    }

    /// If this node is a text node, return a strong reference to its contents.
    #[inline]
    pub fn into_text_ref(self) -> Option<NodeDataRef<RefCell<String>>> {
        NodeDataRef::new_opt(self, Node::as_text)
    }

    /// If this node is a comment, return a strong reference to its contents.
    #[inline]
    pub fn into_comment_ref(self) -> Option<NodeDataRef<RefCell<String>>> {
        NodeDataRef::new_opt(self, Node::as_comment)
    }

    /// If this node is a doctype, return a strong reference to doctype-specific data.
    #[inline]
    pub fn into_doctype_ref(self) -> Option<NodeDataRef<Doctype>> {
        NodeDataRef::new_opt(self, Node::as_doctype)
    }

    /// If this node is a document, return a strong reference to document-specific data.
    #[inline]
    pub fn into_document_ref(self) -> Option<NodeDataRef<DocumentData>> {
        NodeDataRef::new_opt(self, Node::as_document)
    }

    /// If this node is a processing instruction, return a strong reference to its contents.
    #[inline]
    pub fn into_processing_instruction_ref(self) -> Option<NodeDataRef<RefCell<(String, String)>>> {
        NodeDataRef::new_opt(self, Node::as_processing_instruction)
    }

    /// If this node is a document fragment, return a strong reference to it.
    #[inline]
    pub fn into_document_fragment_ref(self) -> Option<NodeDataRef<()>> {
        NodeDataRef::new_opt(self, Node::as_document_fragment)
    }
}

/// Holds a strong reference to a node, but dereferences to some component inside of it.
#[derive(Eq)]
pub struct NodeDataRef<T> {
    /// Keeps the node alive while this reference exists.
    _keep_alive: NodeRef,
    /// Raw pointer to the data within the node (unsafe mode).
    #[cfg(not(feature = "safe"))]
    _reference: *const T,
    /// Node data kind discriminant (safe mode).
    #[cfg(feature = "safe")]
    _kind: NodeDataKind,
    /// Phantom data to maintain generic parameter (safe mode).
    #[cfg(feature = "safe")]
    _phantom: PhantomData<T>,
}

/// Core methods for NodeDataRef.
///
/// Provides construction and access methods for typed references to node data.
impl<T> NodeDataRef<T> {
    /// Create a `NodeDataRef` for a component in a given node.
    #[inline]
    pub fn new<F>(rc: NodeRef, f: F) -> NodeDataRef<T>
    where
        F: FnOnce(&Node) -> &T,
    {
        #[cfg(not(feature = "safe"))]
        {
            NodeDataRef {
                _reference: f(&rc),
                _keep_alive: rc,
            }
        }
        #[cfg(feature = "safe")]
        {
            // Determine the node kind. Since every node must be one of the 7 types,
            // this should always succeed. The unreachable!() documents a logic bug.
            let kind = match &rc {
                _ if rc.as_element().is_some() => NodeDataKind::Element,
                _ if rc.as_text().is_some() => NodeDataKind::Text,
                _ if rc.as_comment().is_some() => NodeDataKind::Comment,
                _ if rc.as_processing_instruction().is_some() => {
                    NodeDataKind::ProcessingInstruction
                }
                _ if rc.as_doctype().is_some() => NodeDataKind::Doctype,
                _ if rc.as_document().is_some() => NodeDataKind::Document,
                _ if rc.as_document_fragment().is_some() => NodeDataKind::DocumentFragment,
                _ => unreachable!("All node types are covered"),
            };

            // We don't call f() because we trust the caller's function signature.
            // The infallible signature F: FnOnce(&Node) -> &T means the caller
            // guarantees this node has the correct type.
            drop(f);

            NodeDataRef {
                _keep_alive: rc,
                _kind: kind,
                _phantom: PhantomData,
            }
        }
    }

    /// Create a `NodeDataRef` for and a component that may or may not be in a given node.
    #[inline]
    pub fn new_opt<F>(rc: NodeRef, f: F) -> Option<NodeDataRef<T>>
    where
        F: FnOnce(&Node) -> Option<&T>,
    {
        #[cfg(not(feature = "safe"))]
        {
            f(&rc).map(|r| r as *const T).map(move |r| NodeDataRef {
                _reference: r,
                _keep_alive: rc,
            })
        }
        #[cfg(feature = "safe")]
        {
            // Determine the node kind by checking which variant matches.
            // This is safe because we're only storing the discriminant, not the pointer.
            let kind = match &rc {
                _ if rc.as_element().is_some() => NodeDataKind::Element,
                _ if rc.as_text().is_some() => NodeDataKind::Text,
                _ if rc.as_comment().is_some() => NodeDataKind::Comment,
                _ if rc.as_processing_instruction().is_some() => {
                    NodeDataKind::ProcessingInstruction
                }
                _ if rc.as_doctype().is_some() => NodeDataKind::Doctype,
                _ if rc.as_document().is_some() => NodeDataKind::Document,
                _ if rc.as_document_fragment().is_some() => NodeDataKind::DocumentFragment,
                _ => return None,
            };

            // Verify that f returns Some for this node.
            if f(&rc).is_some() {
                Some(NodeDataRef {
                    _keep_alive: rc,
                    _kind: kind,
                    _phantom: PhantomData,
                })
            } else {
                None
            }
        }
    }

    /// Access the corresponding node.
    #[inline]
    pub fn as_node(&self) -> &NodeRef {
        &self._keep_alive
    }
}

/// Implements Deref for NodeDataRef (unsafe mode).
///
/// Allows transparent access to the underlying node data using unsafe
/// pointer dereferencing for performance.
// Generic Deref implementation for unsafe mode.
#[cfg(not(feature = "safe"))]
#[allow(unsafe_code)]
impl<T> Deref for NodeDataRef<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        unsafe { &*self._reference }
    }
}

/// Implements Deref for NodeDataRef<ElementData> (safe mode).
///
/// Provides safe access to ElementData by using runtime type checking
/// instead of raw pointer dereferencing.
// Specialized Deref implementations for safe mode.
#[cfg(feature = "safe")]
impl Deref for NodeDataRef<ElementData> {
    type Target = ElementData;
    #[inline]
    fn deref(&self) -> &ElementData {
        self._keep_alive
            .as_element()
            .expect("NodeDataRef<ElementData> must contain Element")
    }
}

/// Implements Deref for NodeDataRef<RefCell<String>> (safe mode).
///
/// Provides safe access to text or comment node contents using runtime
/// type discrimination.
#[cfg(feature = "safe")]
impl Deref for NodeDataRef<RefCell<String>> {
    type Target = RefCell<String>;
    #[inline]
    fn deref(&self) -> &RefCell<String> {
        match self._kind {
            NodeDataKind::Text => self
                ._keep_alive
                .as_text()
                .expect("NodeDataRef with Text kind must contain text"),
            NodeDataKind::Comment => self
                ._keep_alive
                .as_comment()
                .expect("NodeDataRef with Comment kind must contain comment"),
            _ => unreachable!("NodeDataRef<RefCell<String>> must be Text or Comment"),
        }
    }
}

/// Implements Deref for NodeDataRef<Doctype> (safe mode).
///
/// Provides safe access to Doctype node data using runtime type checking.
#[cfg(feature = "safe")]
impl Deref for NodeDataRef<Doctype> {
    type Target = Doctype;
    #[inline]
    fn deref(&self) -> &Doctype {
        self._keep_alive
            .as_doctype()
            .expect("NodeDataRef<Doctype> must contain Doctype")
    }
}

/// Implements Deref for NodeDataRef<DocumentData> (safe mode).
///
/// Provides safe access to Document node data using runtime type checking.
#[cfg(feature = "safe")]
impl Deref for NodeDataRef<DocumentData> {
    type Target = DocumentData;
    #[inline]
    fn deref(&self) -> &DocumentData {
        self._keep_alive
            .as_document()
            .expect("NodeDataRef<DocumentData> must contain Document")
    }
}

/// Implements Deref for NodeDataRef<RefCell<(String, String)>> (safe mode).
///
/// Provides safe access to ProcessingInstruction node data using runtime
/// type checking.
#[cfg(feature = "safe")]
impl Deref for NodeDataRef<RefCell<(String, String)>> {
    type Target = RefCell<(String, String)>;
    #[inline]
    fn deref(&self) -> &RefCell<(String, String)> {
        self._keep_alive
            .as_processing_instruction()
            .expect("NodeDataRef<RefCell<(String, String)>> must contain ProcessingInstruction")
    }
}

/// Implements Deref for NodeDataRef<()> (safe mode).
///
/// Provides safe access to DocumentFragment nodes using runtime type checking.
#[cfg(feature = "safe")]
impl Deref for NodeDataRef<()> {
    type Target = ();
    #[inline]
    fn deref(&self) -> &() {
        self._keep_alive
            .as_document_fragment()
            .expect("NodeDataRef<()> must contain DocumentFragment")
    }
}

/// Implements PartialEq for NodeDataRef.
///
/// Compares NodeDataRef instances by comparing their underlying NodeRef,
/// not the type parameter T. This avoids requiring T: PartialEq.
// #[derive(PartialEq)] would compare both fields
impl<T> PartialEq for NodeDataRef<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self._keep_alive == other._keep_alive
    }
}

/// Implements Clone for NodeDataRef.
///
/// Clones the NodeDataRef by cloning the underlying NodeRef and copying
/// the type information. Avoids requiring T: Clone.
// #[derive(Clone)] would have an unnecessary `T: Clone` bound
impl<T> Clone for NodeDataRef<T> {
    #[inline]
    fn clone(&self) -> Self {
        #[cfg(not(feature = "safe"))]
        {
            NodeDataRef {
                _keep_alive: self._keep_alive.clone(),
                _reference: self._reference,
            }
        }
        #[cfg(feature = "safe")]
        {
            NodeDataRef {
                _keep_alive: self._keep_alive.clone(),
                _kind: self._kind,
                _phantom: PhantomData,
            }
        }
    }
}

/// Implements Debug for NodeDataRef (unsafe mode).
///
/// Formats the referenced data for debugging using the data's Debug impl.
// Generic Debug implementation for unsafe mode.
#[cfg(not(feature = "safe"))]
impl<T: fmt::Debug> fmt::Debug for NodeDataRef<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(&**self, f)
    }
}

/// Implements Debug for NodeDataRef<ElementData> (safe mode).
///
/// Formats ElementData for debugging by delegating to ElementData's Debug impl.
// Specialized Debug implementations for safe mode.
#[cfg(feature = "safe")]
impl fmt::Debug for NodeDataRef<ElementData> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(&**self, f)
    }
}

/// Implements Debug for NodeDataRef<RefCell<String>> (safe mode).
///
/// Formats text or comment node contents for debugging.
#[cfg(feature = "safe")]
impl fmt::Debug for NodeDataRef<RefCell<String>> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(&**self, f)
    }
}

/// Implements Debug for NodeDataRef<Doctype> (safe mode).
///
/// Formats Doctype node data for debugging.
#[cfg(feature = "safe")]
impl fmt::Debug for NodeDataRef<Doctype> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(&**self, f)
    }
}

/// Implements Debug for NodeDataRef<DocumentData> (safe mode).
///
/// Formats Document node data for debugging.
#[cfg(feature = "safe")]
impl fmt::Debug for NodeDataRef<DocumentData> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(&**self, f)
    }
}

/// Implements Debug for NodeDataRef<RefCell<(String, String)>> (safe mode).
///
/// Formats ProcessingInstruction node data for debugging.
#[cfg(feature = "safe")]
impl fmt::Debug for NodeDataRef<RefCell<(String, String)>> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(&**self, f)
    }
}

/// Implements Debug for NodeDataRef<()> (safe mode).
///
/// Formats DocumentFragment nodes for debugging.
#[cfg(feature = "safe")]
impl fmt::Debug for NodeDataRef<()> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(&**self, f)
    }
}

/// Element-specific methods for NodeDataRef<ElementData>.
///
/// Provides convenience methods for working with element nodes.
impl NodeDataRef<ElementData> {
    /// Return the concatenation of all text nodes in this subtree.
    pub fn text_contents(&self) -> String {
        self.as_node().text_contents()
    }

    /// Returns the namespace URI of the element.
    ///
    /// **Note:** This method requires the `namespaces` feature to be enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// #[cfg(feature = "namespaces")]
    /// {
    /// use brik::parse_html;
    /// use brik::traits::*;
    ///
    /// let doc = parse_html().one("<div>Hello</div>");
    /// let div = doc.select_first("div").unwrap();
    /// // HTML elements use the XHTML namespace
    /// assert_eq!(div.namespace_uri().as_ref(), "http://www.w3.org/1999/xhtml");
    /// }
    /// ```
    #[inline]
    #[cfg(feature = "namespaces")]
    pub fn namespace_uri(&self) -> &html5ever::Namespace {
        (**self).namespace_uri()
    }

    /// Returns the local name of the element without any namespace prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// use brik::parse_html;
    /// use brik::traits::*;
    ///
    /// let doc = parse_html().one("<div>Hello</div>");
    /// let div = doc.select_first("div").unwrap();
    /// assert_eq!(div.local_name().as_ref(), "div");
    /// ```
    #[inline]
    pub fn local_name(&self) -> &html5ever::LocalName {
        (**self).local_name()
    }

    /// Returns the namespace prefix of the element, if any.
    ///
    /// **Note:** This method requires the `namespaces` feature to be enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// #[cfg(feature = "namespaces")]
    /// {
    /// use brik::parse_html;
    /// use brik::traits::*;
    ///
    /// let doc = parse_html().one("<div>Hello</div>");
    /// let div = doc.select_first("div").unwrap();
    /// // HTML elements typically have no prefix
    /// assert_eq!(div.prefix(), None);
    /// }
    /// ```
    #[inline]
    #[cfg(feature = "namespaces")]
    pub fn prefix(&self) -> Option<&html5ever::Prefix> {
        (**self).prefix()
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_html;
    use crate::traits::*;

    /// Tests namespace_uri convenience method.
    ///
    /// Verifies that namespace_uri() can be called directly on NodeDataRef
    /// without needing to dereference.
    #[test]
    #[cfg(feature = "namespaces")]
    fn node_data_ref_namespace_uri() {
        let doc = parse_html().one(r#"<div>Test</div>"#);
        let div = doc.select_first("div").unwrap();

        // Should work without .as_element().unwrap()
        assert_eq!(div.namespace_uri().as_ref(), "http://www.w3.org/1999/xhtml");
    }

    /// Tests local_name convenience method.
    ///
    /// Verifies that local_name() can be called directly on NodeDataRef
    /// without needing to dereference.
    #[test]
    fn node_data_ref_local_name() {
        let doc = parse_html().one(r#"<span>Content</span>"#);
        let span = doc.select_first("span").unwrap();

        // Should work without .as_element().unwrap()
        assert_eq!(span.local_name().as_ref(), "span");
    }

    /// Tests prefix convenience method.
    ///
    /// Verifies that prefix() can be called directly on NodeDataRef
    /// without needing to dereference.
    #[test]
    #[cfg(feature = "namespaces")]
    fn node_data_ref_prefix() {
        let doc = parse_html().one(r#"<p>Paragraph</p>"#);
        let p = doc.select_first("p").unwrap();

        // Should work without .as_element().unwrap()
        assert_eq!(p.prefix(), None);
    }

    /// Tests namespace handling with SVG elements.
    ///
    /// Verifies that SVG namespace, local name, and prefix are correctly
    /// accessible via NodeDataRef methods.
    #[test]
    #[cfg(feature = "namespaces")]
    fn node_data_ref_svg_namespace() {
        let svg_html = r#"<!DOCTYPE html>
<html>
<body>
<svg xmlns="http://www.w3.org/2000/svg">
  <circle r="50"/>
</svg>
</body>
</html>"#;
        let doc = parse_html().one(svg_html);
        let circle = doc.select_first("circle").unwrap();

        assert_eq!(
            circle.namespace_uri().as_ref(),
            "http://www.w3.org/2000/svg"
        );
        assert_eq!(circle.local_name().as_ref(), "circle");
        assert_eq!(circle.prefix(), None);
    }

    /// Tests into_element_ref with element node.
    ///
    /// Verifies that into_element_ref returns Some when called on an element node.
    #[test]
    fn into_element_ref_some() {
        let doc = parse_html().one(r#"<div>Content</div>"#);
        let div_node = doc.select("div").unwrap().next().unwrap().as_node().clone();

        let element_ref = div_node.into_element_ref();
        assert!(element_ref.is_some());
        assert_eq!(element_ref.unwrap().name.local.as_ref(), "div");
    }

    /// Tests into_element_ref with non-element node.
    ///
    /// Verifies that into_element_ref returns None when called on a non-element node.
    #[test]
    fn into_element_ref_none() {
        let doc = parse_html().one(r#"<div>text</div>"#);
        let div = doc.select("div").unwrap().next().unwrap();
        let text_node = div.as_node().first_child().unwrap();

        let element_ref = text_node.into_element_ref();
        assert!(element_ref.is_none());
    }

    /// Tests into_text_ref with text node.
    ///
    /// Verifies that into_text_ref returns Some with the text contents when
    /// called on a text node.
    #[test]
    fn into_text_ref_some() {
        let doc = parse_html().one(r#"<div>text content</div>"#);
        let div = doc.select("div").unwrap().next().unwrap();
        let text_node = div.as_node().first_child().unwrap();

        let text_ref = text_node.into_text_ref();
        assert!(text_ref.is_some());
        assert_eq!(&*text_ref.unwrap().borrow(), "text content");
    }

    /// Tests into_text_ref with non-text node.
    ///
    /// Verifies that into_text_ref returns None when called on a non-text node.
    #[test]
    fn into_text_ref_none() {
        let doc = parse_html().one(r#"<div></div>"#);
        let div = doc.select("div").unwrap().next().unwrap();

        let text_ref = div.as_node().clone().into_text_ref();
        assert!(text_ref.is_none());
    }

    /// Tests into_comment_ref with comment node.
    ///
    /// Verifies that into_comment_ref returns Some with the comment contents
    /// when called on a comment node.
    #[test]
    fn into_comment_ref_some() {
        let doc = parse_html().one(r#"<!-- comment --><div></div>"#);
        let comment_node = doc.first_child().unwrap();

        let comment_ref = comment_node.into_comment_ref();
        assert!(comment_ref.is_some());
        assert_eq!(&*comment_ref.unwrap().borrow(), " comment ");
    }

    /// Tests into_comment_ref with non-comment node.
    ///
    /// Verifies that into_comment_ref returns None when called on a non-comment node.
    #[test]
    fn into_comment_ref_none() {
        let doc = parse_html().one(r#"<div></div>"#);
        let div = doc.select("div").unwrap().next().unwrap();

        let comment_ref = div.as_node().clone().into_comment_ref();
        assert!(comment_ref.is_none());
    }

    /// Tests into_doctype_ref with doctype node.
    ///
    /// Verifies that into_doctype_ref returns Some with the doctype data when
    /// called on a doctype node.
    #[test]
    fn into_doctype_ref_some() {
        let doc = parse_html().one(r#"<!DOCTYPE html><html></html>"#);
        let doctype_node = doc.first_child().unwrap();

        let doctype_ref = doctype_node.into_doctype_ref();
        assert!(doctype_ref.is_some());
        assert_eq!(&*doctype_ref.unwrap().name, "html");
    }

    /// Tests into_doctype_ref with non-doctype node.
    ///
    /// Verifies that into_doctype_ref returns None when called on a non-doctype node.
    #[test]
    fn into_doctype_ref_none() {
        let doc = parse_html().one(r#"<div></div>"#);
        let div = doc.select("div").unwrap().next().unwrap();

        let doctype_ref = div.as_node().clone().into_doctype_ref();
        assert!(doctype_ref.is_none());
    }

    /// Tests into_document_ref with document node.
    ///
    /// Verifies that into_document_ref returns Some when called on a document node.
    #[test]
    fn into_document_ref_some() {
        let doc = parse_html().one(r#"<html></html>"#);

        let document_ref = doc.into_document_ref();
        assert!(document_ref.is_some());
    }

    /// Tests into_document_ref with non-document node.
    ///
    /// Verifies that into_document_ref returns None when called on a non-document node.
    #[test]
    fn into_document_ref_none() {
        let doc = parse_html().one(r#"<div></div>"#);
        let div = doc.select("div").unwrap().next().unwrap();

        let document_ref = div.as_node().clone().into_document_ref();
        assert!(document_ref.is_none());
    }

    /// Tests into_processing_instruction_ref with non-PI node.
    ///
    /// Verifies that into_processing_instruction_ref returns None when called
    /// on a non-processing-instruction node.
    #[test]
    fn into_processing_instruction_ref_none() {
        let doc = parse_html().one(r#"<div></div>"#);
        let div = doc.select("div").unwrap().next().unwrap();

        let pi_ref = div.as_node().clone().into_processing_instruction_ref();
        assert!(pi_ref.is_none());
    }

    /// Tests into_document_fragment_ref with non-fragment node.
    ///
    /// Verifies that into_document_fragment_ref returns None when called on
    /// a non-document-fragment node.
    #[test]
    fn into_document_fragment_ref_none() {
        let doc = parse_html().one(r#"<div></div>"#);
        let div = doc.select("div").unwrap().next().unwrap();

        let frag_ref = div.as_node().clone().into_document_fragment_ref();
        assert!(frag_ref.is_none());
    }

    /// Tests text_contents method.
    ///
    /// Verifies that text_contents collects all text from nested elements.
    #[test]
    fn text_contents() {
        let doc = parse_html().one(r#"<div>Hello <b>World</b>!</div>"#);
        let div = doc.select("div").unwrap().next().unwrap();

        assert_eq!(div.text_contents(), "Hello World!");
    }

    /// Tests text_contents with deeply nested elements.
    ///
    /// Verifies that text_contents traverses all nesting levels to collect text.
    #[test]
    fn text_contents_nested() {
        let doc = parse_html().one(r#"<div><p>A</p><span>B<i>C</i></span>D</div>"#);
        let div = doc.select("div").unwrap().next().unwrap();

        assert_eq!(div.text_contents(), "ABCD");
    }

    /// Tests text_contents with empty element.
    ///
    /// Verifies that text_contents returns an empty string for elements
    /// with no text content.
    #[test]
    fn text_contents_empty() {
        let doc = parse_html().one(r#"<div></div>"#);
        let div = doc.select("div").unwrap().next().unwrap();

        assert_eq!(div.text_contents(), "");
    }

    /// Tests as_node method.
    ///
    /// Verifies that as_node returns a reference to the underlying NodeRef.
    #[test]
    fn as_node() {
        let doc = parse_html().one(r#"<div></div>"#);
        let div = doc.select("div").unwrap().next().unwrap();

        let node = div.as_node();
        assert!(node.as_element().is_some());
    }
}
