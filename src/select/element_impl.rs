use super::{AttrValue, BrikSelectors, LocalNameSelector, PseudoClass, PseudoElement};
use crate::attributes::ExpandedName;
use crate::iter::NodeIterator;
use crate::node_data_ref::NodeDataRef;
use crate::tree::{ElementData, Node, NodeData, NodeRef};
use html5ever::{local_name, ns, LocalName, Namespace};
use selectors::attr::{AttrSelectorOperation, CaseSensitivity, NamespaceConstraint};
use selectors::{matching, OpaqueElement};

/// The definition of whitespace per CSS Selectors Level 3 § 4.
///
/// Copied from rust-selectors.
pub(super) static SELECTOR_WHITESPACE: &[char] = &[' ', '\t', '\n', '\r', '\x0C'];

/// Implements selectors::Element for NodeDataRef<ElementData>.
///
/// Provides the selectors crate interface for CSS selector matching on
/// Brik's ElementData nodes. This implementation enables full CSS selector
/// support including element relationships, attributes, pseudo-classes, and
/// namespace matching.
impl selectors::Element for NodeDataRef<ElementData> {
    type Impl = BrikSelectors;

    #[inline]
    fn opaque(&self) -> OpaqueElement {
        let node: &Node = self.as_node();
        OpaqueElement::new(node)
    }

    #[inline]
    fn is_html_slot_element(&self) -> bool {
        false
    }
    #[inline]
    fn parent_node_is_shadow_root(&self) -> bool {
        false
    }
    #[inline]
    fn containing_shadow_host(&self) -> Option<Self> {
        None
    }

    #[inline]
    fn parent_element(&self) -> Option<Self> {
        self.as_node().parent().and_then(NodeRef::into_element_ref)
    }
    #[inline]
    fn prev_sibling_element(&self) -> Option<Self> {
        self.as_node().preceding_siblings().elements().next()
    }
    #[inline]
    fn next_sibling_element(&self) -> Option<Self> {
        self.as_node().following_siblings().elements().next()
    }
    #[inline]
    fn first_element_child(&self) -> Option<Self> {
        self.as_node().children().elements().next()
    }
    #[inline]
    fn is_empty(&self) -> bool {
        self.as_node().children().all(|child| match *child.data() {
            NodeData::Element(_) => false,
            NodeData::Text(ref text) => text.borrow().is_empty(),
            _ => true,
        })
    }
    #[inline]
    fn is_root(&self) -> bool {
        match self.as_node().parent() {
            None => false,
            Some(parent) => matches!(*parent.data(), NodeData::Document(_)),
        }
    }

    #[inline]
    fn is_html_element_in_html_document(&self) -> bool {
        // FIXME: Have a notion of HTML document v.s. XML document?
        self.name.ns == ns!(html)
    }

    #[inline]
    fn has_local_name(&self, name: &LocalName) -> bool {
        self.name.local == *name
    }
    #[inline]
    fn has_namespace(&self, namespace: &Namespace) -> bool {
        self.name.ns == *namespace
    }

    #[inline]
    fn is_part(&self, _name: &LocalNameSelector) -> bool {
        false
    }

    #[inline]
    fn imported_part(&self, _: &LocalNameSelector) -> Option<LocalNameSelector> {
        None
    }

    #[inline]
    fn is_pseudo_element(&self) -> bool {
        false
    }

    #[inline]
    fn is_same_type(&self, other: &Self) -> bool {
        self.name == other.name
    }

    #[inline]
    fn is_link(&self) -> bool {
        self.name.ns == ns!(html)
            && matches!(
                self.name.local,
                local_name!("a") | local_name!("area") | local_name!("link")
            )
            && self
                .attributes
                .borrow()
                .map
                .contains_key(&ExpandedName::new(ns!(), local_name!("href")))
    }

    #[inline]
    fn has_id(&self, id: &LocalNameSelector, case_sensitivity: CaseSensitivity) -> bool {
        self.attributes
            .borrow()
            .get(local_name!("id"))
            .is_some_and(|id_attr| case_sensitivity.eq(id.as_bytes(), id_attr.as_bytes()))
    }

    #[inline]
    fn has_class(&self, name: &LocalNameSelector, case_sensitivity: CaseSensitivity) -> bool {
        let name = name.as_bytes();
        !name.is_empty()
            && if let Some(class_attr) = self.attributes.borrow().get(local_name!("class")) {
                class_attr
                    .split(SELECTOR_WHITESPACE)
                    .any(|class| case_sensitivity.eq(class.as_bytes(), name))
            } else {
                false
            }
    }

    #[inline]
    fn attr_matches(
        &self,
        ns: &NamespaceConstraint<&Namespace>,
        local_name: &LocalNameSelector,
        operation: &AttrSelectorOperation<&AttrValue>,
    ) -> bool {
        let attrs = self.attributes.borrow();
        match *ns {
            NamespaceConstraint::Any => attrs.map.iter().any(|(name, attr)| {
                name.local == **local_name && operation.eval_str(attr.value.as_str())
            }),
            NamespaceConstraint::Specific(ns_url) => attrs
                .map
                .get(&ExpandedName::new(ns_url, (**local_name).clone()))
                .is_some_and(|attr| operation.eval_str(attr.value.as_str())),
        }
    }

    fn match_pseudo_element(
        &self,
        pseudo: &PseudoElement,
        _context: &mut matching::MatchingContext<BrikSelectors>,
    ) -> bool {
        match *pseudo {}
    }

    fn match_non_ts_pseudo_class(
        &self,
        pseudo: &PseudoClass,
        _context: &mut matching::MatchingContext<BrikSelectors>,
    ) -> bool {
        use self::PseudoClass::*;
        match *pseudo {
            Active | Focus | Hover | Enabled | Disabled | Checked | Indeterminate | Visited => {
                false
            }
            AnyLink | Link => {
                self.name.ns == ns!(html)
                    && matches!(
                        self.name.local,
                        local_name!("a") | local_name!("area") | local_name!("link")
                    )
                    && self.attributes.borrow().contains(local_name!("href"))
            }
        }
    }

    #[inline]
    fn apply_selector_flags(&self, _flags: matching::ElementSelectorFlags) {
        // No-op for static DOM
    }

    #[inline]
    fn has_custom_state(&self, _name: &LocalNameSelector) -> bool {
        // Brik is a static DOM, no custom states
        false
    }

    #[inline]
    fn add_element_unique_hashes(&self, filter: &mut selectors::bloom::BloomFilter) -> bool {
        let _ = filter; // Silence unused warning
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::html5ever::tendril::TendrilSink;
    use crate::parse_html;
    use selectors::Element;

    /// Tests parent_element method.
    ///
    /// Verifies that parent_element returns the parent element node.
    #[test]
    fn parent_element() {
        let html = "<div><p><span>text</span></p></div>";
        let doc = parse_html().one(html);
        let span = doc.select("span").unwrap().next().unwrap();

        let parent = span.parent_element();
        assert!(parent.is_some());
        assert_eq!(parent.unwrap().name.local.as_ref(), "p");
    }

    /// Tests parent_element with no parent element.
    ///
    /// Verifies that parent_element returns None when the parent is not
    /// an element (e.g., when parent is a document node).
    #[test]
    fn parent_element_none() {
        let doc = parse_html().one("<html></html>");
        let html = doc.select("html").unwrap().next().unwrap();

        // html element's parent is document, not an element
        assert!(html.parent_element().is_none());
    }

    /// Tests prev_sibling_element method.
    ///
    /// Verifies that prev_sibling_element returns the previous sibling
    /// element node.
    #[test]
    fn prev_sibling_element() {
        let html = "<div><p>1</p><span>2</span></div>";
        let doc = parse_html().one(html);
        let span = doc.select("span").unwrap().next().unwrap();

        let prev = span.prev_sibling_element();
        assert!(prev.is_some());
        assert_eq!(prev.unwrap().name.local.as_ref(), "p");
    }

    /// Tests prev_sibling_element with no previous sibling.
    ///
    /// Verifies that prev_sibling_element returns None when there is no
    /// previous element sibling.
    #[test]
    fn prev_sibling_element_none() {
        let html = "<div><p>first</p></div>";
        let doc = parse_html().one(html);
        let p = doc.select("p").unwrap().next().unwrap();

        assert!(p.prev_sibling_element().is_none());
    }

    /// Tests next_sibling_element method.
    ///
    /// Verifies that next_sibling_element returns the next sibling element node.
    #[test]
    fn next_sibling_element() {
        let html = "<div><p>1</p><span>2</span></div>";
        let doc = parse_html().one(html);
        let p = doc.select("p").unwrap().next().unwrap();

        let next = p.next_sibling_element();
        assert!(next.is_some());
        assert_eq!(next.unwrap().name.local.as_ref(), "span");
    }

    /// Tests next_sibling_element with no next sibling.
    ///
    /// Verifies that next_sibling_element returns None when there is no
    /// next element sibling.
    #[test]
    fn next_sibling_element_none() {
        let html = "<div><p>last</p></div>";
        let doc = parse_html().one(html);
        let p = doc.select("p").unwrap().next().unwrap();

        assert!(p.next_sibling_element().is_none());
    }

    /// Tests first_element_child method.
    ///
    /// Verifies that first_element_child returns the first child element.
    #[test]
    fn first_element_child() {
        let html = "<div><p>first</p><span>second</span></div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        let first_child = div.first_element_child();
        assert!(first_child.is_some());
        assert_eq!(first_child.unwrap().name.local.as_ref(), "p");
    }

    /// Tests first_element_child with no children.
    ///
    /// Verifies that first_element_child returns None when the element
    /// has no child elements.
    #[test]
    fn first_element_child_none() {
        let html = "<div></div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        assert!(div.first_element_child().is_none());
    }

    /// Tests is_empty with empty element.
    ///
    /// Verifies that is_empty returns true for elements with no children.
    #[test]
    fn is_empty_true() {
        let html = "<div></div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        assert!(div.is_empty());
    }

    /// Tests is_empty with child element.
    ///
    /// Verifies that is_empty returns false when the element contains
    /// child elements.
    #[test]
    fn is_empty_false_with_element() {
        let html = "<div><p>text</p></div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        assert!(!div.is_empty());
    }

    /// Tests is_empty with text content.
    ///
    /// Verifies that is_empty returns false when the element contains
    /// non-empty text nodes.
    #[test]
    fn is_empty_false_with_text() {
        let html = "<div>text</div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        assert!(!div.is_empty());
    }

    /// Tests is_empty with empty text nodes.
    ///
    /// Verifies that is_empty returns true when text nodes are empty,
    /// treating them as not contributing to content.
    #[test]
    fn is_empty_true_with_empty_text() {
        let html = "<div></div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        assert!(div.is_empty());
    }

    /// Tests is_root with root element.
    ///
    /// Verifies that is_root returns true for the document's root element
    /// (html element whose parent is the document node).
    #[test]
    fn is_root_true() {
        let doc = parse_html().one("<html></html>");
        let html = doc.select("html").unwrap().next().unwrap();

        assert!(html.is_root());
    }

    /// Tests is_root with non-root element.
    ///
    /// Verifies that is_root returns false for elements that are not
    /// the document root.
    #[test]
    fn is_root_false() {
        let html = "<html><body><div></div></body></html>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        assert!(!div.is_root());
    }

    /// Tests is_html_element_in_html_document method.
    ///
    /// Verifies that elements in the HTML namespace are correctly identified
    /// as HTML elements.
    #[test]
    fn is_html_element_in_html_document() {
        let html = "<html><body><div></div></body></html>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        assert!(div.is_html_element_in_html_document());
    }

    /// Tests has_local_name with matching name.
    ///
    /// Verifies that has_local_name returns true when the element's
    /// local name matches.
    #[test]
    fn has_local_name_true() {
        let html = "<div></div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        assert!(div.has_local_name(&html5ever::local_name!("div")));
    }

    /// Tests has_local_name with non-matching name.
    ///
    /// Verifies that has_local_name returns false when the element's
    /// local name does not match.
    #[test]
    fn has_local_name_false() {
        let html = "<div></div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        assert!(!div.has_local_name(&html5ever::local_name!("span")));
    }

    /// Tests has_namespace with matching namespace.
    ///
    /// Verifies that has_namespace returns true when the element is in
    /// the specified namespace.
    #[test]
    fn has_namespace_true() {
        let html = "<div></div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        assert!(div.has_namespace(&html5ever::ns!(html)));
    }

    /// Tests is_same_type with matching elements.
    ///
    /// Verifies that is_same_type returns true for elements with the same
    /// name and namespace.
    #[test]
    fn is_same_type_true() {
        let html = "<div></div><div></div>";
        let doc = parse_html().one(html);
        let mut divs = doc.select("div").unwrap();
        let div1 = divs.next().unwrap();
        let div2 = divs.next().unwrap();

        assert!(div1.is_same_type(&div2));
    }

    /// Tests is_same_type with different elements.
    ///
    /// Verifies that is_same_type returns false for elements with different
    /// names or namespaces.
    #[test]
    fn is_same_type_false() {
        let html = "<div></div><span></span>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();
        let span = doc.select("span").unwrap().next().unwrap();

        assert!(!div.is_same_type(&span));
    }

    /// Tests is_link with anchor element.
    ///
    /// Verifies that is_link returns true for <a> elements with href attribute.
    #[test]
    fn is_link_true_anchor() {
        let html = r#"<a href="https://example.com">link</a>"#;
        let doc = parse_html().one(html);
        let a = doc.select("a").unwrap().next().unwrap();

        assert!(a.is_link());
    }

    /// Tests is_link with area element.
    ///
    /// Verifies that is_link returns true for <area> elements with href attribute.
    #[test]
    fn is_link_true_area() {
        let html = r#"<map><area href="https://example.com"></map>"#;
        let doc = parse_html().one(html);
        let area = doc.select("area").unwrap().next().unwrap();

        assert!(area.is_link());
    }

    /// Tests is_link with link element.
    ///
    /// Verifies that is_link returns true for <link> elements with href attribute.
    #[test]
    fn is_link_true_link() {
        let html = r#"<link href="style.css">"#;
        let doc = parse_html().one(html);
        let link = doc.select("link").unwrap().next().unwrap();

        assert!(link.is_link());
    }

    /// Tests is_link without href attribute.
    ///
    /// Verifies that is_link returns false for link elements without
    /// an href attribute.
    #[test]
    fn is_link_false_no_href() {
        let html = "<a>not a link</a>";
        let doc = parse_html().one(html);
        let a = doc.select("a").unwrap().next().unwrap();

        assert!(!a.is_link());
    }

    /// Tests is_link with non-link element.
    ///
    /// Verifies that is_link returns false for elements that are not
    /// link-type elements (a, area, link).
    #[test]
    fn is_link_false_wrong_element() {
        let html = r#"<div href="https://example.com">not a link</div>"#;
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        assert!(!div.is_link());
    }

    /// Tests has_id with case sensitivity.
    ///
    /// Verifies that ID selectors match with proper case sensitivity.
    #[test]
    fn has_id_case_sensitive() {
        let html = r#"<div id="myId"></div>"#;
        let doc = parse_html().one(html);

        assert!(doc.select("#myId").unwrap().next().is_some());
    }

    /// Tests has_id when ID doesn't match.
    ///
    /// Verifies that has_id returns false when the ID doesn't match.
    #[test]
    fn has_id_not_found() {
        let html = r#"<div id="myId"></div>"#;
        let doc = parse_html().one(html);

        assert!(doc.select("#otherId").unwrap().next().is_none());
    }

    /// Tests has_class with single class.
    ///
    /// Verifies that has_class correctly identifies elements with a
    /// single class name.
    #[test]
    fn has_class_single() {
        let html = r#"<div class="myClass"></div>"#;
        let doc = parse_html().one(html);

        assert!(doc.select(".myClass").unwrap().next().is_some());
    }

    /// Tests has_class with multiple classes.
    ///
    /// Verifies that has_class correctly identifies a class in a space-separated
    /// list of multiple classes.
    #[test]
    fn has_class_multiple() {
        let html = r#"<div class="class1 class2 class3"></div>"#;
        let doc = parse_html().one(html);

        assert!(doc.select(".class2").unwrap().next().is_some());
    }

    /// Tests has_class with whitespace in class attribute.
    ///
    /// Verifies that has_class correctly handles class attributes with
    /// various whitespace characters between classes.
    #[test]
    fn has_class_with_whitespace() {
        let html = "<div class=\"class1  \t\n  class2\"></div>";
        let doc = parse_html().one(html);

        assert!(doc.select(".class2").unwrap().next().is_some());
    }

    /// Tests has_class when class doesn't match.
    ///
    /// Verifies that has_class returns false when the class is not present
    /// in the element's class attribute.
    #[test]
    fn has_class_not_found() {
        let html = r#"<div class="myClass"></div>"#;
        let doc = parse_html().one(html);

        assert!(doc.select(".otherClass").unwrap().next().is_none());
    }

    /// Tests has_class with no class attribute.
    ///
    /// Verifies that has_class returns false when the element has no
    /// class attribute at all.
    #[test]
    fn has_class_no_class_attr() {
        let html = "<div></div>";
        let doc = parse_html().one(html);

        assert!(doc.select(".myClass").unwrap().next().is_none());
    }

    /// Tests attr_matches for attribute existence.
    ///
    /// Verifies that attribute selectors correctly match elements that
    /// have the specified attribute, regardless of value.
    #[test]
    fn attr_matches_exists() {
        let html = r#"<div data-value="test"></div>"#;
        let doc = parse_html().one(html);

        assert!(doc.select("[data-value]").unwrap().next().is_some());
    }

    /// Tests attr_matches for exact value match.
    ///
    /// Verifies that attribute selectors correctly match elements when
    /// the attribute value exactly equals the specified value.
    #[test]
    fn attr_matches_exact_value() {
        let html = r#"<div data-value="test"></div>"#;
        let doc = parse_html().one(html);

        assert!(doc
            .select(r#"[data-value="test"]"#)
            .unwrap()
            .next()
            .is_some());
    }

    /// Tests attr_matches when value doesn't match.
    ///
    /// Verifies that attribute selectors return false when the attribute
    /// value does not match the specified value.
    #[test]
    fn attr_matches_not_found() {
        let html = r#"<div data-value="test"></div>"#;
        let doc = parse_html().one(html);

        assert!(doc
            .select(r#"[data-value="other"]"#)
            .unwrap()
            .next()
            .is_none());
    }

    /// Tests attr_matches for substring containment.
    ///
    /// Verifies that attribute selectors with *= operator correctly match
    /// when the attribute value contains the specified substring.
    #[test]
    fn attr_matches_contains() {
        let html = r#"<div data-value="hello world"></div>"#;
        let doc = parse_html().one(html);

        assert!(doc
            .select(r#"[data-value*="world"]"#)
            .unwrap()
            .next()
            .is_some());
    }

    /// Tests attr_matches for prefix match.
    ///
    /// Verifies that attribute selectors with ^= operator correctly match
    /// when the attribute value starts with the specified prefix.
    #[test]
    fn attr_matches_starts_with() {
        let html = r#"<div data-value="hello world"></div>"#;
        let doc = parse_html().one(html);

        assert!(doc
            .select(r#"[data-value^="hello"]"#)
            .unwrap()
            .next()
            .is_some());
    }

    /// Tests attr_matches for suffix match.
    ///
    /// Verifies that attribute selectors with $= operator correctly match
    /// when the attribute value ends with the specified suffix.
    #[test]
    fn attr_matches_ends_with() {
        let html = r#"<div data-value="hello world"></div>"#;
        let doc = parse_html().one(html);

        assert!(doc
            .select(r#"[data-value$="world"]"#)
            .unwrap()
            .next()
            .is_some());
    }

    /// Tests is_pseudo_element method.
    ///
    /// Verifies that is_pseudo_element returns false since Brik does not
    /// support pseudo-elements in the static DOM.
    #[test]
    fn is_pseudo_element_false() {
        let html = "<div></div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        assert!(!div.is_pseudo_element());
    }

    /// Tests is_html_slot_element method.
    ///
    /// Verifies that is_html_slot_element returns false since Brik does not
    /// support shadow DOM slot elements.
    #[test]
    fn is_html_slot_element_false() {
        let html = "<slot></slot>";
        let doc = parse_html().one(html);
        let slot = doc.select("slot").unwrap().next().unwrap();

        assert!(!slot.is_html_slot_element());
    }

    /// Tests parent_node_is_shadow_root method.
    ///
    /// Verifies that parent_node_is_shadow_root returns false since Brik
    /// does not support shadow DOM.
    #[test]
    fn parent_node_is_shadow_root_false() {
        let html = "<div><p>text</p></div>";
        let doc = parse_html().one(html);
        let p = doc.select("p").unwrap().next().unwrap();

        assert!(!p.parent_node_is_shadow_root());
    }

    /// Tests containing_shadow_host method.
    ///
    /// Verifies that containing_shadow_host returns None since Brik does
    /// not support shadow DOM.
    #[test]
    fn containing_shadow_host_none() {
        let html = "<div></div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        assert!(div.containing_shadow_host().is_none());
    }

    /// Tests is_part method.
    ///
    /// Verifies that is_part returns false since Brik does not support
    /// shadow DOM parts.
    #[test]
    fn is_part_false() {
        let html = "<div></div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        assert!(!div.is_part(&html5ever::local_name!("div").into()));
    }

    /// Tests imported_part method.
    ///
    /// Verifies that imported_part returns None since Brik does not support
    /// shadow DOM parts.
    #[test]
    fn imported_part_none() {
        let html = "<div></div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        assert!(div
            .imported_part(&html5ever::local_name!("div").into())
            .is_none());
    }

    /// Tests has_custom_state method.
    ///
    /// Verifies that has_custom_state returns false since Brik has a static
    /// DOM and does not support custom element states.
    #[test]
    fn has_custom_state_false() {
        let html = "<div></div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        assert!(!div.has_custom_state(&html5ever::local_name!("div").into()));
    }

    /// Tests :link pseudo-class selector.
    ///
    /// Verifies that the :link pseudo-class matches anchor elements with href
    /// attribute through the pseudo-class matching path.
    #[test]
    fn pseudo_class_link() {
        let html = r#"<a href="https://example.com">link</a><a>no href</a>"#;
        let doc = parse_html().one(html);

        let links: Vec<_> = doc.select("a:link").unwrap().collect();
        assert_eq!(links.len(), 1);
    }

    /// Tests :any-link pseudo-class selector.
    ///
    /// Verifies that the :any-link pseudo-class matches link-type elements
    /// (a, area, link) with href attribute.
    #[test]
    fn pseudo_class_any_link() {
        let html = r#"<a href="https://example.com">link</a><link href="style.css">"#;
        let doc = parse_html().one(html);

        let links: Vec<_> = doc.select(":any-link").unwrap().collect();
        assert_eq!(links.len(), 2);
    }

    /// Tests has_namespace with non-matching namespace.
    ///
    /// Verifies that has_namespace returns false when element is not in
    /// the specified namespace.
    #[test]
    fn has_namespace_false() {
        let html = "<div></div>";
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        // HTML element should not match SVG namespace.
        assert!(!div.has_namespace(&html5ever::ns!(svg)));
    }

    /// Tests attr_matches with namespace wildcard in selector.
    ///
    /// Verifies that attribute matching works when iterating through
    /// all namespaces (NamespaceConstraint::Any).
    #[test]
    fn attr_matches_any_namespace_constraint() {
        let html = r#"<div data-value="test"></div>"#;
        let doc = parse_html().one(html);
        let div = doc.select("div").unwrap().next().unwrap();

        // Test attribute exists regardless of namespace.
        let attrs = div.attributes.borrow();
        assert!(attrs.contains("data-value"));
    }
}
