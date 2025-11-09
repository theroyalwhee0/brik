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

#[cfg(test)]
mod tests {
    use super::*;
    use html5ever::tree_builder::NodeOrText;

    /// Tests that create_pi creates a processing instruction node.
    ///
    /// Verifies the TreeSink implementation can create PI nodes even though
    /// the HTML5 parser doesn't normally generate them.
    #[test]
    fn create_pi() {
        let sink = Sink {
            document_node: NodeRef::new_document(),
            on_parse_error: RefCell::new(None),
        };

        let pi = sink.create_pi(
            StrTendril::from("xml-stylesheet"),
            StrTendril::from("href=\"style.css\""),
        );

        let pi_data = pi.as_processing_instruction().expect("Should be a PI node");
        let (target, data) = &*pi_data.borrow();
        assert_eq!(target, "xml-stylesheet");
        assert_eq!(data, "href=\"style.css\"");
    }

    /// Tests append_before_sibling with a node.
    ///
    /// Verifies that nodes can be inserted before a sibling in the tree.
    #[test]
    fn append_before_sibling_with_node() {
        let sink = Sink {
            document_node: NodeRef::new_document(),
            on_parse_error: RefCell::new(None),
        };

        let parent = NodeRef::new_element(
            QualName::new(None, ns!(html), local_name!("div")),
            std::iter::empty(),
        );
        let sibling = NodeRef::new_element(
            QualName::new(None, ns!(html), local_name!("span")),
            std::iter::empty(),
        );
        let new_node = NodeRef::new_element(
            QualName::new(None, ns!(html), local_name!("p")),
            std::iter::empty(),
        );

        parent.append(sibling.clone());
        sink.append_before_sibling(&sibling, NodeOrText::AppendNode(new_node.clone()));

        // Verify the new node is before the sibling
        assert_eq!(parent.children().count(), 2);
        let first = parent.first_child().unwrap();
        assert_eq!(first.as_element().unwrap().name.local.as_ref(), "p");
        let second = first.next_sibling().unwrap();
        assert_eq!(second.as_element().unwrap().name.local.as_ref(), "span");
    }

    /// Tests append_before_sibling with text that gets coalesced.
    ///
    /// Verifies that text nodes are merged with previous text siblings
    /// when inserting before an element.
    #[test]
    fn append_before_sibling_with_text_coalesce() {
        let sink = Sink {
            document_node: NodeRef::new_document(),
            on_parse_error: RefCell::new(None),
        };

        let parent = NodeRef::new_element(
            QualName::new(None, ns!(html), local_name!("div")),
            std::iter::empty(),
        );
        let text1 = NodeRef::new_text("Hello ");
        let sibling = NodeRef::new_element(
            QualName::new(None, ns!(html), local_name!("span")),
            std::iter::empty(),
        );

        parent.append(text1.clone());
        parent.append(sibling.clone());

        // Append text before the span - should coalesce with previous text
        sink.append_before_sibling(&sibling, NodeOrText::AppendText(StrTendril::from("World")));

        // Should have coalesced into the first text node
        assert_eq!(parent.children().count(), 2);
        let first = parent.first_child().unwrap();
        let text_content: &str = &first.as_text().unwrap().borrow();
        assert_eq!(text_content, "Hello World");
    }

    /// Tests append_before_sibling with text creating a new node.
    ///
    /// Verifies that a new text node is created when there's no previous
    /// text sibling to coalesce with.
    #[test]
    fn append_before_sibling_with_text_new_node() {
        let sink = Sink {
            document_node: NodeRef::new_document(),
            on_parse_error: RefCell::new(None),
        };

        let parent = NodeRef::new_element(
            QualName::new(None, ns!(html), local_name!("div")),
            std::iter::empty(),
        );
        let element = NodeRef::new_element(
            QualName::new(None, ns!(html), local_name!("p")),
            std::iter::empty(),
        );
        let sibling = NodeRef::new_element(
            QualName::new(None, ns!(html), local_name!("span")),
            std::iter::empty(),
        );

        parent.append(element);
        parent.append(sibling.clone());

        // Append text before the span - previous sibling is element, not text
        sink.append_before_sibling(&sibling, NodeOrText::AppendText(StrTendril::from("Hello")));

        // Should have created a new text node
        assert_eq!(parent.children().count(), 3);
        let children: Vec<_> = parent.children().collect();
        assert_eq!(children[0].as_element().unwrap().name.local.as_ref(), "p");
        let text_content: &str = &children[1].as_text().unwrap().borrow();
        assert_eq!(text_content, "Hello");
        assert_eq!(
            children[2].as_element().unwrap().name.local.as_ref(),
            "span"
        );
    }

    /// Tests add_attrs_if_missing adds new attributes.
    ///
    /// Verifies that attributes not already present are added to an element.
    #[test]
    fn add_attrs_if_missing_adds_new() {
        let sink = Sink {
            document_node: NodeRef::new_document(),
            on_parse_error: RefCell::new(None),
        };

        let element = NodeRef::new_element(
            QualName::new(None, ns!(html), local_name!("div")),
            vec![(
                attributes::ExpandedName {
                    ns: ns!(),
                    local: local_name!("id"),
                },
                attributes::Attribute {
                    prefix: None,
                    value: "test".to_string(),
                },
            )],
        );

        let new_attrs = vec![Attribute {
            name: QualName::new(None, ns!(), local_name!("class")),
            value: StrTendril::from("container"),
        }];

        sink.add_attrs_if_missing(&element, new_attrs);

        let attrs = element.as_element().unwrap().attributes.borrow();
        assert_eq!(attrs.get("id"), Some("test"));
        assert_eq!(attrs.get("class"), Some("container"));
    }

    /// Tests add_attrs_if_missing doesn't overwrite existing attributes.
    ///
    /// Verifies that existing attributes are preserved when adding new ones.
    #[test]
    fn add_attrs_if_missing_preserves_existing() {
        let sink = Sink {
            document_node: NodeRef::new_document(),
            on_parse_error: RefCell::new(None),
        };

        let element = NodeRef::new_element(
            QualName::new(None, ns!(html), local_name!("div")),
            vec![(
                attributes::ExpandedName {
                    ns: ns!(),
                    local: local_name!("id"),
                },
                attributes::Attribute {
                    prefix: None,
                    value: "original".to_string(),
                },
            )],
        );

        let new_attrs = vec![
            Attribute {
                name: QualName::new(None, ns!(), local_name!("id")),
                value: StrTendril::from("should-not-replace"),
            },
            Attribute {
                name: QualName::new(None, ns!(), local_name!("class")),
                value: StrTendril::from("container"),
            },
        ];

        sink.add_attrs_if_missing(&element, new_attrs);

        let attrs = element.as_element().unwrap().attributes.borrow();
        // Original id should be preserved
        assert_eq!(attrs.get("id"), Some("original"));
        // New class should be added
        assert_eq!(attrs.get("class"), Some("container"));
    }

    /// Tests parse_error callback when handler is set.
    ///
    /// Verifies that parse error callbacks are invoked when provided.
    #[test]
    fn parse_error_with_callback() {
        use std::sync::{Arc, Mutex};

        let error_messages = Arc::new(Mutex::new(Vec::new()));
        let error_messages_clone = Arc::clone(&error_messages);

        let sink = Sink {
            document_node: NodeRef::new_document(),
            on_parse_error: RefCell::new(Some(Box::new(move |msg: Cow<'static, str>| {
                error_messages_clone.lock().unwrap().push(msg.into_owned());
            }))),
        };

        sink.parse_error(Cow::Borrowed("Test error 1"));
        sink.parse_error(Cow::Borrowed("Test error 2"));

        let messages = error_messages.lock().unwrap();
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0], "Test error 1");
        assert_eq!(messages[1], "Test error 2");
    }

    /// Tests parse_error without callback doesn't panic.
    ///
    /// Verifies that parse errors are handled gracefully when no callback
    /// is provided.
    #[test]
    fn parse_error_without_callback() {
        let sink = Sink {
            document_node: NodeRef::new_document(),
            on_parse_error: RefCell::new(None),
        };

        // Should not panic
        sink.parse_error(Cow::Borrowed("This error is ignored"));
    }

    /// Tests append_based_on_parent_node when element has parent.
    ///
    /// Verifies that the method delegates to append_before_sibling when
    /// the element has a parent.
    #[test]
    fn append_based_on_parent_node_with_parent() {
        let sink = Sink {
            document_node: NodeRef::new_document(),
            on_parse_error: RefCell::new(None),
        };

        let parent = NodeRef::new_element(
            QualName::new(None, ns!(html), local_name!("div")),
            std::iter::empty(),
        );
        let element = NodeRef::new_element(
            QualName::new(None, ns!(html), local_name!("span")),
            std::iter::empty(),
        );
        let prev_element = NodeRef::new_element(
            QualName::new(None, ns!(html), local_name!("p")),
            std::iter::empty(),
        );

        parent.append(element.clone());

        let new_node = NodeRef::new_element(
            QualName::new(None, ns!(html), local_name!("b")),
            std::iter::empty(),
        );

        // Element has a parent, so should use append_before_sibling
        sink.append_based_on_parent_node(
            &element,
            &prev_element,
            NodeOrText::AppendNode(new_node.clone()),
        );

        // New node should be inserted before element in parent
        let children: Vec<_> = parent.children().collect();
        assert_eq!(children.len(), 2);
        assert_eq!(children[0].as_element().unwrap().name.local.as_ref(), "b");
        assert_eq!(
            children[1].as_element().unwrap().name.local.as_ref(),
            "span"
        );
    }

    /// Tests append_based_on_parent_node when element has no parent.
    ///
    /// Verifies that the method delegates to append when the element
    /// has no parent.
    #[test]
    fn append_based_on_parent_node_without_parent() {
        let sink = Sink {
            document_node: NodeRef::new_document(),
            on_parse_error: RefCell::new(None),
        };

        let element = NodeRef::new_element(
            QualName::new(None, ns!(html), local_name!("span")),
            std::iter::empty(),
        );
        let prev_element = NodeRef::new_element(
            QualName::new(None, ns!(html), local_name!("p")),
            std::iter::empty(),
        );

        let new_node = NodeRef::new_element(
            QualName::new(None, ns!(html), local_name!("b")),
            std::iter::empty(),
        );

        // Element has no parent, so should use append to prev_element
        sink.append_based_on_parent_node(
            &element,
            &prev_element,
            NodeOrText::AppendNode(new_node.clone()),
        );

        // New node should be appended to prev_element
        let children: Vec<_> = prev_element.children().collect();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].as_element().unwrap().name.local.as_ref(), "b");
    }
}
