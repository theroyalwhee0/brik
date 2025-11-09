//! Apply xmlns namespace declarations to elements and attributes in a document.
//!
//! This module provides functions to post-process parsed HTML documents by applying
//! namespace declarations from the `<html>` element to all prefixed elements and
//! attributes throughout the document.

use crate::tree::NodeRef;
use crate::{Attribute, Attributes, ExpandedName};
use html5ever::{LocalName, Namespace, Prefix, QualName};
use std::collections::{HashMap, HashSet};

use super::{NsError, NsResult};

/// Applies xmlns namespace declarations to elements and attributes (lenient).
///
/// This function extracts xmlns declarations from the `<html>` element and applies
/// them to all prefixed elements and attributes in the document. Elements like
/// `c:my-element` are split into prefix (`c`), local name (`my-element`), and
/// namespace URI (from `xmlns:c` declaration).
///
/// **Lenient mode**: If a prefix is used but not defined in xmlns declarations,
/// it is still split but assigned a null namespace. This will succeed and return
/// the document even with undefined prefixes.
///
/// # Returns
///
/// Returns the rebuilt document with namespace corrections applied.
///
/// # Errors
///
/// Returns an error for unexpected processing failures (not for undefined prefixes).
/// In practice, this should not happen during normal operation.
///
/// # Examples
///
/// ```
/// use brik::parse_html;
/// use brik::traits::*;
///
/// let html = r#"<html xmlns:c="https://example.com/custom">
///     <body><c:widget>Content</c:widget></body>
/// </html>"#;
///
/// let doc = parse_html().one(html);
/// let corrected = doc.apply_xmlns().unwrap();
///
/// // The c:widget element now has proper namespace information
/// ```
pub fn apply_xmlns(root: &NodeRef) -> NsResult<NodeRef> {
    match apply_xmlns_strict(root) {
        Ok(doc) => Ok(doc),
        Err(NsError::UndefinedPrefix(doc, _)) => Ok(doc),
        Err(e) => Err(e),
    }
}

/// Applies xmlns namespace declarations to elements and attributes (strict).
///
/// This function works identically to [`apply_xmlns`], but returns an error if any
/// prefixed element or attribute references an undefined namespace prefix.
///
/// **Strict mode**: Returns an error if undefined prefixes are encountered, but
/// the error contains the rebuilt document with those prefixes assigned null namespaces.
///
/// # Errors
///
/// Returns `NsError::UndefinedPrefix` if any element or attribute uses a namespace
/// prefix that has no corresponding `xmlns:prefix` declaration. The error contains
/// the rebuilt document and a list of undefined prefixes.
///
/// # Examples
///
/// ```
/// use brik::parse_html;
/// use brik::traits::*;
/// use brik::ns::NsError;
///
/// let html = r#"<html>
///     <body><c:widget>Content</c:widget></body>
/// </html>"#;
///
/// let doc = parse_html().one(html);
/// match doc.apply_xmlns_strict() {
///     Ok(corrected) => println!("All namespaces defined"),
///     Err(NsError::UndefinedPrefix(doc, prefixes)) => {
///         println!("Undefined prefixes: {:?}", prefixes);
///         // Can still use the document with null namespaces
///     }
///     Err(e) => panic!("Error: {}", e),
/// }
/// ```
pub fn apply_xmlns_strict(root: &NodeRef) -> NsResult<NodeRef> {
    // Step 1: Extract xmlns declarations from <html> element
    let xmlns_map = extract_xmlns_declarations(root);

    // Step 2: Rebuild the document tree with corrected namespaces
    let mut undefined_prefixes = HashSet::new();
    let new_root = rebuild_tree(root, &xmlns_map, &mut undefined_prefixes);

    // Step 3: Return result based on whether we found undefined prefixes
    if undefined_prefixes.is_empty() {
        Ok(new_root)
    } else {
        let mut prefix_list: Vec<_> = undefined_prefixes.into_iter().collect();
        prefix_list.sort();
        Err(NsError::UndefinedPrefix(new_root, prefix_list))
    }
}

/// Extracts xmlns namespace declarations from the document's <html> element.
///
/// Returns a map from prefix to namespace URI.
fn extract_xmlns_declarations(root: &NodeRef) -> HashMap<String, Namespace> {
    let mut xmlns_map = HashMap::new();

    // Find the <html> element
    for node in root.descendants() {
        if let Some(element) = node.as_element() {
            if element.name.local.as_ref() == "html" {
                // Extract xmlns:* attributes
                let attrs = element.attributes.borrow();
                for (expanded_name, attr) in &attrs.map {
                    // Check if this is an xmlns declaration
                    // xmlns:prefix="uri" has local name "prefix" and might be in xmlns namespace
                    // But HTML5 parser might put them in null namespace with name "xmlns:prefix"
                    let local_str = expanded_name.local.as_ref();
                    if let Some(prefix) = local_str.strip_prefix("xmlns:") {
                        xmlns_map.insert(prefix.to_string(), Namespace::from(attr.value.as_str()));
                    }
                }
                break;
            }
        }
    }

    xmlns_map
}

/// Rebuilds the entire document tree with corrected namespace information.
///
/// Creates new nodes with properly split and namespaced element/attribute names.
fn rebuild_tree(
    node: &NodeRef,
    xmlns_map: &HashMap<String, Namespace>,
    undefined_prefixes: &mut HashSet<String>,
) -> NodeRef {
    use crate::tree::NodeData;

    match node.data() {
        NodeData::Element(element) => {
            // Process element name
            let new_name = process_qualified_name(&element.name, xmlns_map, undefined_prefixes);

            // Process attributes
            let attrs = element.attributes.borrow();
            let new_attrs = process_attributes(&attrs, xmlns_map, undefined_prefixes);

            // Create new element with corrected name and attributes
            let new_node = NodeRef::new_element(new_name, new_attrs.map);

            // Handle template contents (if this is an HTML <template> element)
            if let Some(ref template_contents) = element.template_contents {
                // The new_element will have created its own template_contents
                // (a DocumentFragment) if it's an HTML template element.
                // We need to populate it with the rebuilt children from the original.
                if let Some(new_element) = new_node.as_element() {
                    if let Some(ref new_template_frag) = new_element.template_contents {
                        // Rebuild each child of the original template contents
                        // and append to the new template's fragment
                        for child in template_contents.children() {
                            let new_child = rebuild_tree(&child, xmlns_map, undefined_prefixes);
                            new_template_frag.append(new_child);
                        }
                    }
                }
            }

            // Recursively rebuild children
            for child in node.children() {
                let new_child = rebuild_tree(&child, xmlns_map, undefined_prefixes);
                new_node.append(new_child);
            }

            new_node
        }
        NodeData::Text(text) => NodeRef::new_text(text.borrow().clone()),
        NodeData::Comment(comment) => NodeRef::new_comment(comment.borrow().clone()),
        NodeData::ProcessingInstruction(pi) => {
            let pi_data = pi.borrow();
            NodeRef::new_processing_instruction(pi_data.0.clone(), pi_data.1.clone())
        }
        NodeData::Doctype(doctype) => NodeRef::new_doctype(
            doctype.name.clone(),
            doctype.public_id.clone(),
            doctype.system_id.clone(),
        ),
        NodeData::Document(_) => {
            let new_doc = NodeRef::new_document();
            for child in node.children() {
                let new_child = rebuild_tree(&child, xmlns_map, undefined_prefixes);
                new_doc.append(new_child);
            }
            new_doc
        }
        NodeData::DocumentFragment => {
            let new_frag = NodeRef::new(NodeData::DocumentFragment);
            for child in node.children() {
                let new_child = rebuild_tree(&child, xmlns_map, undefined_prefixes);
                new_frag.append(new_child);
            }
            new_frag
        }
    }
}

/// Processes a QualName, splitting prefixed names and applying namespaces.
fn process_qualified_name(
    name: &QualName,
    xmlns_map: &HashMap<String, Namespace>,
    undefined_prefixes: &mut HashSet<String>,
) -> QualName {
    let local_str = name.local.as_ref();

    // Check if the local name contains a colon (prefixed name)
    if let Some(colon_pos) = local_str.find(':') {
        let prefix_str = &local_str[..colon_pos];
        let local_part = &local_str[colon_pos + 1..];

        // Look up the namespace for this prefix
        if let Some(namespace) = xmlns_map.get(prefix_str) {
            // Found namespace - create corrected QualName
            QualName::new(
                Some(Prefix::from(prefix_str)),
                namespace.clone(),
                LocalName::from(local_part),
            )
        } else {
            // Undefined prefix - record it and use null namespace
            undefined_prefixes.insert(prefix_str.to_string());
            QualName::new(
                Some(Prefix::from(prefix_str)),
                ns!(),
                LocalName::from(local_part),
            )
        }
    } else {
        // No prefix - keep original name
        name.clone()
    }
}

/// Processes attributes, splitting prefixed names and applying namespaces.
fn process_attributes(
    attrs: &Attributes,
    xmlns_map: &HashMap<String, Namespace>,
    undefined_prefixes: &mut HashSet<String>,
) -> Attributes {
    let mut new_map = indexmap::IndexMap::new();

    for (expanded_name, attr) in &attrs.map {
        let local_str = expanded_name.local.as_ref();

        // Check if this is an xmlns declaration - skip it in the new attributes
        if local_str.starts_with("xmlns:") || local_str == "xmlns" {
            continue;
        }

        // Check if the local name contains a colon (prefixed attribute)
        if let Some(colon_pos) = local_str.find(':') {
            let prefix_str = &local_str[..colon_pos];
            let local_part = &local_str[colon_pos + 1..];

            // Look up the namespace for this prefix
            let (namespace, prefix) = if let Some(ns) = xmlns_map.get(prefix_str) {
                (ns.clone(), Some(Prefix::from(prefix_str)))
            } else {
                // Undefined prefix - record it and use null namespace
                undefined_prefixes.insert(prefix_str.to_string());
                (ns!(), Some(Prefix::from(prefix_str)))
            };

            let new_expanded = ExpandedName::new(namespace, LocalName::from(local_part));
            new_map.insert(
                new_expanded,
                Attribute {
                    prefix,
                    value: attr.value.clone(),
                },
            );
        } else {
            // No prefix - keep original
            new_map.insert(expanded_name.clone(), attr.clone());
        }
    }

    Attributes { map: new_map }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_html;
    use crate::traits::*;

    /// Tests applying xmlns to a document with defined namespaces.
    ///
    /// Verifies that elements with prefixes get properly namespaced when
    /// the prefix is defined in the html element.
    #[test]
    #[cfg(feature = "namespaces")]
    fn apply_xmlns_with_defined_prefix() {
        let html = r#"<html xmlns:c="https://example.com/custom">
            <body><c:widget id="test">Content</c:widget></body>
        </html>"#;

        let doc = parse_html().one(html);
        let result = apply_xmlns(&doc).unwrap();

        // Find the widget element
        let widget = result.select_first("widget").unwrap();
        assert_eq!(widget.local_name().as_ref(), "widget");
        assert_eq!(widget.prefix().unwrap().as_ref(), "c");
        assert_eq!(
            widget.namespace_uri().as_ref(),
            "https://example.com/custom"
        );
    }

    /// Tests applying xmlns to a document with undefined namespaces (lenient).
    ///
    /// Verifies that the lenient version processes elements even when
    /// prefixes are not defined, assigning null namespace.
    #[test]
    #[cfg(feature = "namespaces")]
    fn apply_xmlns_lenient_undefined_prefix() {
        let html = r#"<html>
            <body><c:widget>Content</c:widget></body>
        </html>"#;

        let doc = parse_html().one(html);
        let result = apply_xmlns(&doc).unwrap();

        // Find the widget element
        let widget = result.select_first("widget").unwrap();
        assert_eq!(widget.local_name().as_ref(), "widget");
        assert_eq!(widget.prefix().unwrap().as_ref(), "c");
        assert_eq!(widget.namespace_uri().as_ref(), ""); // Null namespace
    }

    /// Tests strict mode with undefined prefixes.
    ///
    /// Verifies that strict mode returns an error but includes the
    /// processed document in the error.
    #[test]
    #[cfg(feature = "namespaces")]
    fn apply_xmlns_strict_undefined_prefix() {
        let html = r#"<html>
            <body><c:widget foo:bar="test">Content</c:widget></body>
        </html>"#;

        let doc = parse_html().one(html);
        let err = apply_xmlns_strict(&doc).expect_err("Should return error for undefined prefixes");

        match err {
            NsError::UndefinedPrefix(new_doc, prefixes) => {
                assert_eq!(prefixes.len(), 2);
                assert!(prefixes.contains(&"c".to_string()));
                assert!(prefixes.contains(&"foo".to_string()));

                // Document should still be usable
                let widget = new_doc.select_first("widget").unwrap();
                assert_eq!(widget.local_name().as_ref(), "widget");
            }
            _ => unreachable!("Only UndefinedPrefix errors are possible from apply_xmlns_strict"),
        }
    }

    /// Tests that HTML template elements are properly handled.
    ///
    /// Verifies that template contents are rebuilt and namespace-corrected
    /// when the template element is processed.
    #[test]
    #[cfg(feature = "namespaces")]
    fn apply_xmlns_handles_template_contents() {
        let html = r#"<html xmlns:c="https://example.com/custom">
            <body>
                <template>
                    <c:widget>Template content</c:widget>
                </template>
            </body>
        </html>"#;

        let doc = parse_html().one(html);
        let result = apply_xmlns(&doc).unwrap();

        // Find the template element
        if let Ok(template) = result.select_first("template") {
            // Get the element data
            if let Some(elem_data) = template.as_node().as_element() {
                // Check that template_contents exists
                assert!(
                    elem_data.template_contents.is_some(),
                    "Template should have contents"
                );

                if let Some(ref contents) = elem_data.template_contents {
                    // Find the widget inside template contents
                    let mut found_widget = false;
                    for child in contents.descendants() {
                        if let Some(element) = child.as_element() {
                            if element.name.local.as_ref() == "widget" {
                                found_widget = true;
                                assert_eq!(element.name.prefix.as_ref().unwrap().as_ref(), "c");
                                assert_eq!(element.name.ns.as_ref(), "https://example.com/custom");
                            }
                        }
                    }
                    assert!(found_widget, "Should find widget in template contents");
                }
            }
        }
    }

    /// Tests that comments are preserved during namespace processing.
    ///
    /// Verifies that comment nodes are correctly cloned to the new tree.
    #[test]
    fn apply_xmlns_preserves_comments() {
        let html = r#"<html>
            <!-- This is a comment -->
            <body>Content</body>
        </html>"#;

        let doc = parse_html().one(html);
        let result = apply_xmlns(&doc).unwrap();

        // Find the comment
        let mut found_comment = false;
        for node in result.descendants() {
            if let Some(comment) = node.as_comment() {
                assert_eq!(comment.borrow().trim(), "This is a comment");
                found_comment = true;
            }
        }
        assert!(found_comment, "Should preserve comments");
    }

    /// Tests that doctype nodes are preserved during namespace processing.
    ///
    /// Verifies that DOCTYPE declarations are correctly cloned to the new tree.
    #[test]
    fn apply_xmlns_preserves_doctype() {
        let html = r#"<!DOCTYPE html>
        <html>
            <body>Content</body>
        </html>"#;

        let doc = parse_html().one(html);
        let result = apply_xmlns(&doc).unwrap();

        // Find the doctype
        let mut found_doctype = false;
        for node in result.children() {
            if let Some(doctype) = node.as_doctype() {
                assert_eq!(doctype.name.as_str(), "html");
                found_doctype = true;
            }
        }
        assert!(found_doctype, "Should preserve DOCTYPE");
    }

    /// Tests namespace processing on attributes with prefixes.
    ///
    /// Verifies that attributes like foo:bar="value" are properly split
    /// and assigned namespaces.
    #[test]
    #[cfg(feature = "namespaces")]
    fn apply_xmlns_processes_prefixed_attributes() {
        let html = r#"<html xmlns:data="https://example.com/data">
            <body>
                <div data:id="123" data:type="widget">Content</div>
            </body>
        </html>"#;

        let doc = parse_html().one(html);
        let result = apply_xmlns(&doc).unwrap();

        if let Ok(div) = result.select_first("div") {
            let attrs = div.attributes.borrow();

            // Check for the namespaced attributes
            let mut found_id = false;
            let mut found_type = false;

            for (name, attr) in &attrs.map {
                if name.local.as_ref() == "id" && name.ns.as_ref() == "https://example.com/data" {
                    assert_eq!(attr.value, "123");
                    assert_eq!(attr.prefix.as_ref().unwrap().as_ref(), "data");
                    found_id = true;
                }
                if name.local.as_ref() == "type" && name.ns.as_ref() == "https://example.com/data" {
                    assert_eq!(attr.value, "widget");
                    assert_eq!(attr.prefix.as_ref().unwrap().as_ref(), "data");
                    found_type = true;
                }
            }

            assert!(found_id, "Should find namespaced id attribute");
            assert!(found_type, "Should find namespaced type attribute");
        }
    }

    /// Tests behavior when no html element exists.
    ///
    /// Verifies that processing works even without an <html> element
    /// (no xmlns declarations to extract).
    #[test]
    fn apply_xmlns_without_html_element() {
        let html = r#"<body><div>Content</div></body>"#;

        let doc = parse_html().one(html);
        let result = apply_xmlns(&doc).unwrap();

        // Should succeed even without <html> element
        assert!(result.select_first("div").is_ok());
    }

    /// Tests that processing instructions are handled (if html5ever creates them).
    ///
    /// Note: HTML5 parser typically doesn't create PI nodes, but if it did,
    /// this verifies they would be preserved.
    #[test]
    fn html5ever_pi_handling() {
        // HTML5 spec says PIs should be parsed as comments or bogus comments
        let html = r#"<?xml version="1.0"?><html><body>Test</body></html>"#;
        let doc = parse_html().one(html);

        // Check if any PI nodes exist
        let mut found_pi = false;
        for node in doc.descendants() {
            if node.as_processing_instruction().is_some() {
                found_pi = true;
                break;
            }
        }

        // HTML5 parser doesn't create PI nodes - they become comments or are dropped
        assert!(
            !found_pi,
            "HTML5 parser should not create ProcessingInstruction nodes"
        );
    }

    /// Tests that manually inserted processing instructions are preserved.
    ///
    /// Verifies that apply_xmlns correctly handles ProcessingInstruction nodes
    /// even though html5ever doesn't create them during parsing.
    #[test]
    #[cfg(feature = "namespaces")]
    fn apply_xmlns_preserves_processing_instructions() {
        let html = r#"<html xmlns:c="https://example.com/custom">
            <body><c:widget>Content</c:widget></body>
        </html>"#;

        let doc = parse_html().one(html);

        // Manually insert a PI node into the document
        let pi = NodeRef::new_processing_instruction(
            "xml-stylesheet".to_string(),
            "href=\"style.css\"".to_string(),
        );

        // Insert it before the html element
        if let Some(html_elem) = doc.children().next() {
            html_elem.insert_before(pi.clone());
        }

        // Apply xmlns
        let result = apply_xmlns(&doc).unwrap();

        // Verify the PI was preserved
        let mut found_pi = false;
        for node in result.descendants() {
            if let Some(pi_data) = node.as_processing_instruction() {
                let (target, data) = &*pi_data.borrow();
                assert_eq!(target, "xml-stylesheet");
                assert_eq!(data, "href=\"style.css\"");
                found_pi = true;
                break;
            }
        }

        assert!(
            found_pi,
            "ProcessingInstruction should be preserved during apply_xmlns"
        );

        // Also verify the namespaced element was processed
        let widget = result.select_first("widget").unwrap();
        assert_eq!(
            widget.namespace_uri().as_ref(),
            "https://example.com/custom"
        );
    }

    /// Tests that standalone DocumentFragment nodes are preserved.
    ///
    /// Verifies that apply_xmlns correctly handles DocumentFragment nodes
    /// when they appear in the tree (though rare in practice).
    #[test]
    fn apply_xmlns_preserves_document_fragments() {
        use crate::tree::NodeData;

        let html = r#"<html>
            <body>Content</body>
        </html>"#;

        let doc = parse_html().one(html);

        // Manually create and insert a DocumentFragment with some text
        let frag = NodeRef::new(NodeData::DocumentFragment);
        let text_node = NodeRef::new_text("Fragment content".to_string());
        frag.append(text_node);

        // Insert the fragment into the body
        if let Ok(body) = doc.select_first("body") {
            body.as_node().append(frag.clone());
        }

        // Apply xmlns (even though no namespaces are defined)
        let result = apply_xmlns(&doc).unwrap();

        // Verify fragment was preserved
        let mut found_frag = false;
        let mut found_text = false;
        for node in result.descendants() {
            if node.as_document_fragment().is_some() {
                found_frag = true;
                // Check that children were preserved
                for child in node.children() {
                    if let Some(text) = child.as_text() {
                        assert_eq!(text.borrow().as_str(), "Fragment content");
                        found_text = true;
                    }
                }
            }
        }

        assert!(
            found_frag,
            "DocumentFragment should be preserved during apply_xmlns"
        );
        assert!(found_text, "DocumentFragment children should be preserved");
    }

    /// Tests that xmlns declarations are not copied to new attributes.
    ///
    /// Verifies that xmlns:* attributes are filtered out during processing.
    #[test]
    fn apply_xmlns_removes_xmlns_attributes() {
        let html = r#"<html xmlns:c="https://example.com/custom">
            <body><c:widget>Content</c:widget></body>
        </html>"#;

        let doc = parse_html().one(html);
        let result = apply_xmlns(&doc).unwrap();

        // Find the html element
        if let Ok(html_elem) = result.select_first("html") {
            let attrs = html_elem.attributes.borrow();
            // xmlns:c should not be in the rebuilt document's attributes
            assert!(!attrs
                .map
                .iter()
                .any(|(name, _)| { name.local.as_ref().starts_with("xmlns:") }));
        }
    }
}
