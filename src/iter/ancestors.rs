use crate::tree::NodeRef;

/// An iterator on ancestor nodes.
#[derive(Debug, Clone)]
pub struct Ancestors(pub(super) Option<NodeRef>);

impl Iterator for Ancestors {
    type Item = NodeRef;

    #[inline]
    fn next(&mut self) -> Option<NodeRef> {
        self.0.take().inspect(|node| {
            self.0 = node.parent();
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::html5ever::tendril::TendrilSink;
    use crate::parse_html;

    #[test]
    fn ancestors_iteration() {
        let html = r#"
            <html>
                <body>
                    <div>
                        <p>
                            <span id="target">text</span>
                        </p>
                    </div>
                </body>
            </html>
        "#;
        let doc = parse_html().one(html);
        let span = doc.select("#target").unwrap().next().unwrap();

        let ancestors: Vec<_> = span.as_node().ancestors().collect();

        // Should have: p, div, body, html, document
        assert_eq!(ancestors.len(), 5);

        // Check the chain: p -> div -> body -> html -> document
        assert_eq!(ancestors[0].as_element().unwrap().name.local.as_ref(), "p");
        assert_eq!(
            ancestors[1].as_element().unwrap().name.local.as_ref(),
            "div"
        );
        assert_eq!(
            ancestors[2].as_element().unwrap().name.local.as_ref(),
            "body"
        );
        assert_eq!(
            ancestors[3].as_element().unwrap().name.local.as_ref(),
            "html"
        );
        assert!(ancestors[4].as_document().is_some());
    }

    #[test]
    fn ancestors_root_node() {
        let doc = parse_html().one("<html></html>");

        // Document node has no ancestors
        let ancestors: Vec<_> = doc.ancestors().collect();
        assert_eq!(ancestors.len(), 0);
    }

    #[test]
    fn ancestors_clone() {
        let html = "<div><p><span>text</span></p></div>";
        let doc = parse_html().one(html);
        let span = doc.select("span").unwrap().next().unwrap();

        let mut iter1 = span.as_node().ancestors();
        let mut iter2 = iter1.clone();

        // Both iterators should produce the same results
        assert_eq!(
            iter1
                .next()
                .unwrap()
                .as_element()
                .unwrap()
                .name
                .local
                .as_ref(),
            "p"
        );
        assert_eq!(
            iter2
                .next()
                .unwrap()
                .as_element()
                .unwrap()
                .name
                .local
                .as_ref(),
            "p"
        );
    }
}
