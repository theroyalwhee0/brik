//! TreeSink implementation for building DOM trees during HTML parsing.

use crate::attributes;
use crate::tree::NodeRef;
use html5ever::tendril::StrTendril;
use html5ever::tree_builder::{ElementFlags, NodeOrText, QuirksMode, TreeSink};
use html5ever::{Attribute, ExpandedName, QualName};
use std::borrow::Cow;
use std::cell::RefCell;

/// Type alias for the parse error callback handler.
type ParseErrorHandler = RefCell<Option<Box<dyn FnMut(Cow<'static, str>)>>>;

/// Receives new tree nodes during parsing.
pub struct Sink {
    /// The root document node being constructed.
    pub(super) document_node: NodeRef,
    /// Optional callback for handling parse errors.
    pub(super) on_parse_error: ParseErrorHandler,
}

/// Implements TreeSink for Sink.
///
/// Provides the html5ever TreeSink interface for building a DOM tree during
/// HTML parsing. Handles node creation, tree manipulation, and parse error
/// callbacks as the parser processes HTML content.
impl TreeSink for Sink {
    type Output = NodeRef;

    fn finish(self) -> NodeRef {
        self.document_node
    }

    type Handle = NodeRef;

    type ElemName<'a>
        = ExpandedName<'a>
    where
        Self: 'a;

    #[inline]
    fn parse_error(&self, message: Cow<'static, str>) {
        if let Some(ref mut handler) = *self.on_parse_error.borrow_mut() {
            handler(message)
        }
    }

    #[inline]
    fn get_document(&self) -> NodeRef {
        self.document_node.clone()
    }

    #[inline]
    fn set_quirks_mode(&self, mode: QuirksMode) {
        self.document_node
            .as_document()
            .unwrap()
            ._quirks_mode
            .set(mode)
    }

    #[inline]
    fn same_node(&self, x: &NodeRef, y: &NodeRef) -> bool {
        x == y
    }

    #[inline]
    fn elem_name<'a>(&self, target: &'a NodeRef) -> ExpandedName<'a> {
        target.as_element().unwrap().name.expanded()
    }

    #[inline]
    fn create_element(
        &self,
        name: QualName,
        attrs: Vec<Attribute>,
        _flags: ElementFlags,
    ) -> NodeRef {
        NodeRef::new_element(
            name,
            attrs.into_iter().map(|attr| {
                let Attribute {
                    name: QualName { prefix, ns, local },
                    value,
                } = attr;
                let value = String::from(value);
                (
                    attributes::ExpandedName { ns, local },
                    attributes::Attribute { prefix, value },
                )
            }),
        )
    }

    #[inline]
    fn create_comment(&self, text: StrTendril) -> NodeRef {
        NodeRef::new_comment(text)
    }

    #[inline]
    fn create_pi(&self, target: StrTendril, data: StrTendril) -> NodeRef {
        NodeRef::new_processing_instruction(target, data)
    }

    #[inline]
    fn append(&self, parent: &NodeRef, child: NodeOrText<NodeRef>) {
        match child {
            NodeOrText::AppendNode(node) => parent.append(node),
            NodeOrText::AppendText(text) => {
                if let Some(last_child) = parent.last_child() {
                    if let Some(existing) = last_child.as_text() {
                        existing.borrow_mut().push_str(&text);
                        return;
                    }
                }
                parent.append(NodeRef::new_text(text))
            }
        }
    }

    #[inline]
    fn append_before_sibling(&self, sibling: &NodeRef, child: NodeOrText<NodeRef>) {
        match child {
            NodeOrText::AppendNode(node) => sibling.insert_before(node),
            NodeOrText::AppendText(text) => {
                if let Some(previous_sibling) = sibling.previous_sibling() {
                    if let Some(existing) = previous_sibling.as_text() {
                        existing.borrow_mut().push_str(&text);
                        return;
                    }
                }
                sibling.insert_before(NodeRef::new_text(text))
            }
        }
    }

    #[inline]
    fn append_doctype_to_document(
        &self,
        name: StrTendril,
        public_id: StrTendril,
        system_id: StrTendril,
    ) {
        self.document_node
            .append(NodeRef::new_doctype(name, public_id, system_id))
    }

    #[inline]
    fn add_attrs_if_missing(&self, target: &NodeRef, attrs: Vec<Attribute>) {
        let element = target.as_element().unwrap();
        let mut attributes = element.attributes.borrow_mut();

        for Attribute {
            name: QualName { prefix, ns, local },
            value,
        } in attrs
        {
            attributes
                .map
                .entry(attributes::ExpandedName { ns, local })
                .or_insert_with(|| {
                    let value = String::from(value);
                    attributes::Attribute { prefix, value }
                });
        }
    }

    #[inline]
    fn remove_from_parent(&self, target: &NodeRef) {
        target.detach()
    }

    #[inline]
    fn reparent_children(&self, node: &NodeRef, new_parent: &NodeRef) {
        // FIXME: Can this be done more effciently in rctree,
        // by moving the whole linked list of children at once?
        for child in node.children() {
            new_parent.append(child)
        }
    }

    #[inline]
    fn mark_script_already_started(&self, _node: &NodeRef) {
        // FIXME: Is this useful outside of a browser?
    }

    #[inline]
    fn get_template_contents(&self, target: &NodeRef) -> NodeRef {
        target
            .as_element()
            .unwrap()
            .template_contents
            .clone()
            .unwrap()
    }

    fn append_based_on_parent_node(
        &self,
        element: &NodeRef,
        prev_element: &NodeRef,
        child: NodeOrText<NodeRef>,
    ) {
        if element.parent().is_some() {
            self.append_before_sibling(element, child)
        } else {
            self.append(prev_element, child)
        }
    }
}
