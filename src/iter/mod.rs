//! Node iterators

// Addressing this lint is a semver-breaking change.
// Remove this once the issue has been addressed.
#![allow(clippy::result_unit_err)]

/// Ancestor node iterator.
mod ancestors;
/// Descendant node iterator.
mod descendants;
/// Element iterator trait.
mod element_iterator;
/// Element-related iterator.
#[cfg(feature = "namespaces")]
mod elements_in_namespace;
/// Filter-map iterators for elements, comments, and text nodes.
mod filter_iterators;
/// Node edge marker for tree traversal.
mod node_edge;
/// Node iterator trait.
mod node_iterator;
/// NodeRef iterator methods.
mod node_ref_impls;
/// Selector-matching iterator.
mod select;
/// Sibling node iterator.
mod siblings;
/// Tree traversal iterator.
mod traverse;

pub use ancestors::Ancestors;
pub use descendants::Descendants;
pub use element_iterator::ElementIterator;
#[cfg(feature = "namespaces")]
pub use elements_in_namespace::ElementsInNamespace;
pub use filter_iterators::{Comments, Elements, TextNodes};
pub use node_edge::NodeEdge;
pub use node_iterator::NodeIterator;
pub use select::Select;
pub use siblings::Siblings;
pub use traverse::Traverse;

#[cfg(test)]
mod tests {
    use crate::parser::parse_html;
    use crate::traits::*;

    /// Tests filtering elements by namespace.
    ///
    /// Verifies that elements_in_ns() correctly filters an element iterator
    /// to include only elements in the specified namespace (SVG in this case).
    #[test]
    #[cfg(feature = "namespaces")]
    fn elements_in_ns_filters_by_namespace() {
        let html = r#"<!DOCTYPE html>
<html>
<body>
  <div>HTML element 1</div>
  <svg xmlns="http://www.w3.org/2000/svg">
    <circle r="10"/>
    <rect width="20" height="20"/>
  </svg>
  <p>HTML element 2</p>
</body>
</html>"#;

        let doc = parse_html().one(html);

        // Find all SVG elements
        let svg_elements: Vec<_> = doc
            .descendants()
            .elements()
            .elements_in_ns(ns!(svg))
            .collect();

        assert_eq!(svg_elements.len(), 3); // svg, circle, rect
        assert!(svg_elements.iter().all(|e| e.namespace_uri() == &ns!(svg)));
    }

    /// Tests elements_in_ns with no matching elements.
    ///
    /// Verifies that elements_in_ns() returns an empty iterator when
    /// no elements match the specified namespace.
    #[test]
    #[cfg(feature = "namespaces")]
    fn elements_in_ns_empty_when_no_match() {
        let html = r#"<div><p>Only HTML elements</p></div>"#;
        let doc = parse_html().one(html);

        let svg_elements: Vec<_> = doc
            .descendants()
            .elements()
            .elements_in_ns(ns!(svg))
            .collect();

        assert_eq!(svg_elements.len(), 0);
    }

    /// Tests elements_in_ns with nested elements.
    ///
    /// Verifies that elements_in_ns() correctly includes nested elements
    /// within the same namespace.
    #[test]
    #[cfg(feature = "namespaces")]
    fn elements_in_ns_works_with_nested_elements() {
        let html = r#"<!DOCTYPE html>
<html>
<body>
  <svg xmlns="http://www.w3.org/2000/svg">
    <g>
      <circle r="10"/>
      <circle r="20"/>
    </g>
  </svg>
</body>
</html>"#;

        let doc = parse_html().one(html);

        let svg_elements: Vec<_> = doc
            .descendants()
            .elements()
            .elements_in_ns(ns!(svg))
            .collect();

        // svg, g, circle, circle
        assert_eq!(svg_elements.len(), 4);
    }

    /// Tests double-ended iteration with elements_in_ns.
    ///
    /// Verifies that elements_in_ns iterator supports both forward
    /// and reverse iteration via next() and next_back().
    #[test]
    #[cfg(feature = "namespaces")]
    fn elements_in_ns_double_ended_iteration() {
        let html = r#"<!DOCTYPE html>
<html>
<body>
  <svg xmlns="http://www.w3.org/2000/svg">
    <circle r="10"/>
    <rect width="20" height="20"/>
    <line x1="0" y1="0" x2="10" y2="10"/>
  </svg>
</body>
</html>"#;

        let doc = parse_html().one(html);

        let mut svg_elements = doc.descendants().elements().elements_in_ns(ns!(svg));

        // Test forward iteration
        let first = svg_elements.next().unwrap();
        assert_eq!(first.local_name().as_ref(), "svg");

        // Test reverse iteration
        let last = svg_elements.next_back().unwrap();
        assert_eq!(last.local_name().as_ref(), "line");
    }

    /// Tests detach_all removing all matched elements.
    ///
    /// Verifies that detach_all() successfully removes all elements
    /// from the iterator, leaving the parent empty.
    #[test]
    fn detach_all_removes_elements() {
        let html = r#"<div><p>One</p><p>Two</p><p>Three</p></div>"#;
        let doc = parse_html().one(html);
        let div = doc.select_first("div").unwrap();

        let initial_count = div.as_node().children().elements().count();
        assert_eq!(initial_count, 3);

        // Detach all paragraph elements
        let paragraphs: Vec<_> = div
            .as_node()
            .descendants()
            .elements()
            .filter(|e| e.local_name().as_ref() == "p")
            .collect();

        paragraphs
            .into_iter()
            .map(|p| p.as_node().clone())
            .detach_all();

        assert_eq!(div.as_node().children().elements().count(), 0);
    }

    /// Tests detach_all with an empty iterator.
    ///
    /// Verifies that calling detach_all() on an empty iterator does not
    /// panic and handles the edge case gracefully.
    #[test]
    fn detach_all_with_empty_iterator() {
        let html = r#"<div><p>Test</p></div>"#;
        let doc = parse_html().one(html);

        // Try to detach elements that don't exist - should not panic
        doc.descendants()
            .elements()
            .filter(|e| e.local_name().as_ref() == "nonexistent")
            .map(|e| e.as_node().clone())
            .detach_all();
    }

    /// Tests detach_all with mixed namespace elements.
    ///
    /// Verifies that detach_all() can selectively remove elements from
    /// one namespace while preserving elements in other namespaces.
    #[test]
    #[cfg(feature = "namespaces")]
    fn detach_all_with_mixed_namespaces() {
        let html = r#"<!DOCTYPE html>
<html>
<body>
  <div>HTML</div>
  <svg xmlns="http://www.w3.org/2000/svg">
    <circle r="10"/>
  </svg>
  <p>More HTML</p>
</body>
</html>"#;

        let doc = parse_html().one(html);
        let body = doc.select_first("body").unwrap();

        // Detach only SVG elements
        let svg_elements: Vec<_> = body
            .as_node()
            .descendants()
            .elements()
            .elements_in_ns(ns!(svg))
            .collect();

        svg_elements
            .into_iter()
            .map(|e| e.as_node().clone())
            .detach_all();

        // HTML elements should still be present
        let remaining_html: Vec<_> = body
            .as_node()
            .descendants()
            .elements()
            .elements_in_ns(ns!(html))
            .collect();

        assert_eq!(remaining_html.len(), 2); // div, p

        // SVG elements should be gone
        let remaining_svg: Vec<_> = body
            .as_node()
            .descendants()
            .elements()
            .elements_in_ns(ns!(svg))
            .collect();

        assert_eq!(remaining_svg.len(), 0);
    }

    /// Tests iterating and modifying text nodes.
    ///
    /// Verifies that text_nodes() correctly collects all text nodes in
    /// a subtree and that the text content can be modified through the
    /// returned references.
    #[test]
    fn text_nodes() {
        let html = r"
<!doctype html>
<title>Test case</title>
<p>Content contains <b>Important</b> data</p>";
        let document = parse_html().one(html);
        let paragraph = document.select("p").unwrap().collect::<Vec<_>>();
        assert_eq!(paragraph.len(), 1);
        assert_eq!(
            paragraph[0].text_contents(),
            "Content contains Important data"
        );
        let texts = paragraph[0]
            .as_node()
            .descendants()
            .text_nodes()
            .collect::<Vec<_>>();
        assert_eq!(texts.len(), 3);
        assert_eq!(&*texts[0].borrow(), "Content contains ");
        assert_eq!(&*texts[1].borrow(), "Important");
        assert_eq!(&*texts[2].borrow(), " data");
        {
            let mut x = texts[0].borrow_mut();
            x.truncate(0);
            x.push_str("Content doesn't contain ");
        }
        assert_eq!(&*texts[0].borrow(), "Content doesn't contain ");
    }

    /// Tests double-ended iteration for Elements iterator.
    ///
    /// Verifies that Elements supports both next() and next_back()
    /// for bidirectional iteration over element nodes.
    #[test]
    fn elements_double_ended() {
        let html = "<div><p>1</p><span>2</span><b>3</b><i>4</i></div>";
        let doc = parse_html().one(html);
        let div = doc.select_first("div").unwrap();

        let mut elements = div.as_node().descendants().elements();

        // Forward from start.
        let first = elements.next().unwrap();
        assert_eq!(first.name.local.as_ref(), "p");

        // Backward from end.
        let last = elements.next_back().unwrap();
        assert_eq!(last.name.local.as_ref(), "i");

        // Continue from both ends.
        let second = elements.next().unwrap();
        assert_eq!(second.name.local.as_ref(), "span");

        let second_last = elements.next_back().unwrap();
        assert_eq!(second_last.name.local.as_ref(), "b");

        // Should be exhausted.
        assert!(elements.next().is_none());
    }

    /// Tests double-ended iteration for Comments iterator.
    ///
    /// Verifies that Comments supports both next() and next_back()
    /// for bidirectional iteration over comment nodes.
    #[test]
    fn comments_double_ended() {
        let html = "<div><!-- first --><p>text</p><!-- second --><!-- third --></div>";
        let doc = parse_html().one(html);
        let div = doc.select_first("div").unwrap();

        let mut comments = div.as_node().descendants().comments();

        // Forward from start.
        let first = comments.next().unwrap();
        assert_eq!(&*first.borrow(), " first ");

        // Backward from end.
        let last = comments.next_back().unwrap();
        assert_eq!(&*last.borrow(), " third ");

        // Middle comment.
        let middle = comments.next().unwrap();
        assert_eq!(&*middle.borrow(), " second ");

        // Should be exhausted.
        assert!(comments.next().is_none());
    }

    /// Tests double-ended iteration for Descendants iterator.
    ///
    /// Verifies that descendants can be iterated both forward and backward,
    /// respecting depth-first traversal order in both directions.
    #[test]
    fn descendants_double_ended() {
        let html = "<div><p>1</p><span>2</span><b>3</b></div>";
        let doc = parse_html().one(html);
        let div = doc.select_first("div").unwrap();

        let mut descendants = div.as_node().descendants();

        // Forward - first descendant.
        let first = descendants.next().unwrap();
        assert_eq!(first.as_element().unwrap().name.local.as_ref(), "p");

        // Backward - last descendant (text node "3").
        let last = descendants.next_back().unwrap();
        assert!(last.as_text().is_some());

        // Forward - second element.
        let second = descendants.next().unwrap();
        assert!(second.as_text().is_some()); // text "1"
    }

    /// Tests double-ended iteration for Siblings iterator.
    ///
    /// Verifies that siblings can be iterated both forward and backward
    /// within the same parent's children.
    #[test]
    fn siblings_double_ended() {
        let html = "<div><p>1</p><span>2</span><b>3</b><i>4</i></div>";
        let doc = parse_html().one(html);
        let div = doc.select_first("div").unwrap();

        let mut siblings = div.as_node().children();

        // Forward from start.
        let first = siblings.next().unwrap();
        assert_eq!(first.as_element().unwrap().name.local.as_ref(), "p");

        // Backward from end.
        let last = siblings.next_back().unwrap();
        assert_eq!(last.as_element().unwrap().name.local.as_ref(), "i");

        // Continue from both ends.
        let second = siblings.next().unwrap();
        assert_eq!(second.as_element().unwrap().name.local.as_ref(), "span");

        let second_last = siblings.next_back().unwrap();
        assert_eq!(second_last.as_element().unwrap().name.local.as_ref(), "b");

        // Should be exhausted.
        assert!(siblings.next().is_none());
    }

    /// Tests double-ended iteration for Traverse iterator.
    ///
    /// Verifies that tree traversal edges can be iterated both forward
    /// and backward, yielding Start and End edges appropriately.
    #[test]
    fn traverse_double_ended() {
        let html = "<div><p>text</p></div>";
        let doc = parse_html().one(html);
        let div = doc.select_first("div").unwrap();

        let mut traverse = div.as_node().traverse();

        // Forward - first edge should be Start(p).
        let first = traverse.next().unwrap();
        if let crate::iter::NodeEdge::Start(node) = first {
            assert_eq!(node.as_element().unwrap().name.local.as_ref(), "p");
        } else {
            panic!("Expected Start edge");
        }

        // Backward - last edge should be End(text).
        let last = traverse.next_back().unwrap();
        assert!(matches!(last, crate::iter::NodeEdge::End(_)));
    }

    /// Tests NodeEdge enum variants and traits.
    ///
    /// Verifies that NodeEdge correctly represents Start and End edges,
    /// and that Debug, Clone, PartialEq implementations work as expected.
    #[test]
    fn node_edge_basics() {
        use crate::iter::NodeEdge;

        let html = "<div></div>";
        let doc = parse_html().one(html);
        let div = doc.select_first("div").unwrap();

        let start = NodeEdge::Start(div.as_node().clone());
        let end = NodeEdge::End(div.as_node().clone());

        // Test Clone.
        let start_clone = start.clone();
        assert_eq!(start, start_clone);

        // Test PartialEq - same variants with same node should be equal.
        assert_eq!(start, start_clone);

        // Test PartialEq - different variants should not be equal.
        assert_ne!(start, end);

        // Test Debug (just verify it doesn't panic).
        let debug_str = format!("{start:?}");
        assert!(debug_str.contains("Start"));
    }
}
