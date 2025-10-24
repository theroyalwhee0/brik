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
}
