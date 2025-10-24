use html5ever::serialize::TraversalScope::*;
use html5ever::serialize::{serialize, Serialize, SerializeOpts, Serializer, TraversalScope};
use html5ever::QualName;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

use crate::tree::{NodeData, NodeRef};

/// Implements Serialize for NodeRef.
///
/// Enables HTML serialization of DOM nodes using html5ever's serialization
/// infrastructure. Handles all node types including elements, text, comments,
/// doctypes, processing instructions, documents, and document fragments.
impl Serialize for NodeRef {
    fn serialize<S: Serializer>(
        &self,
        serializer: &mut S,
        traversal_scope: TraversalScope,
    ) -> io::Result<()> {
        match (traversal_scope, self.data()) {
            (ref scope, NodeData::Element(element)) => {
                if *scope == IncludeNode {
                    let attrs = element.attributes.borrow();

                    // Unfortunately we need to allocate something to hold these &'a QualName
                    let attrs = attrs
                        .map
                        .iter()
                        .map(|(name, attr)| {
                            (
                                QualName::new(
                                    attr.prefix.clone(),
                                    name.ns.clone(),
                                    name.local.clone(),
                                ),
                                &attr.value,
                            )
                        })
                        .collect::<Vec<_>>();

                    serializer.start_elem(
                        element.name.clone(),
                        attrs.iter().map(|&(ref name, value)| (name, &**value)),
                    )?
                }

                let children = match element.template_contents.as_ref() {
                    Some(template_root) => template_root.children(),
                    None => self.children(),
                };

                for child in children {
                    Serialize::serialize(&child, serializer, IncludeNode)?
                }

                if *scope == IncludeNode {
                    serializer.end_elem(element.name.clone())?
                }
                Ok(())
            }

            (_, &NodeData::DocumentFragment) | (_, &NodeData::Document(_)) => {
                for child in self.children() {
                    Serialize::serialize(&child, serializer, IncludeNode)?
                }
                Ok(())
            }

            (ChildrenOnly(_), _) => Ok(()),

            (IncludeNode, NodeData::Doctype(doctype)) => serializer.write_doctype(&doctype.name),
            (IncludeNode, NodeData::Text(text)) => serializer.write_text(&text.borrow()),
            (IncludeNode, NodeData::Comment(text)) => serializer.write_comment(&text.borrow()),
            (IncludeNode, NodeData::ProcessingInstruction(contents)) => {
                let contents = contents.borrow();
                serializer.write_processing_instruction(&contents.0, &contents.1)
            }
        }
    }
}

/// Implements Display for NodeRef.
///
/// Formats the node and its descendants as an HTML string. Uses the
/// Serialize implementation to generate the HTML output.
impl fmt::Display for NodeRef {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Call the html serializer for the node (sub)tree.
        let mut bytes = Vec::new();
        self.serialize(&mut bytes).or(Err(fmt::Error))?;
        let html = String::from_utf8(bytes).or(Err(fmt::Error))?;
        f.write_str(&html)
    }
}

/// Methods for HTML serialization.
///
/// Provides convenient methods for serializing DOM nodes to HTML strings,
/// byte streams, and files.
impl NodeRef {
    /// Serialize this node and its descendants in HTML syntax to the given stream.
    ///
    /// # Errors
    ///
    /// Returns an `io::Error` if writing to the stream fails.
    #[inline]
    pub fn serialize<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        serialize(
            writer,
            self,
            SerializeOpts {
                traversal_scope: IncludeNode,
                ..Default::default()
            },
        )
    }

    /// Serialize this node and its descendants in HTML syntax to a new file at the given path.
    ///
    /// # Errors
    ///
    /// Returns an `io::Error` if the file cannot be created or if writing fails.
    #[inline]
    pub fn serialize_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let mut file = File::create(&path)?;
        self.serialize(&mut file)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_html;
    use crate::traits::*;
    use tempfile::TempDir;

    /// Tests serializing to a file and reading it back.
    ///
    /// Verifies that serialize_to_file() correctly writes HTML to disk
    /// and that the resulting file can be parsed to produce an equivalent
    /// DOM structure.
    #[test]
    fn serialize_and_read_file() {
        let tempdir = TempDir::new().unwrap();
        let mut path = tempdir.path().to_path_buf();
        path.push("temp.html");

        let html =
            r"<!DOCTYPE html><html><head><title>Title</title></head><body>Body</body></html>";
        let document = parse_html().one(html);
        let _ = document.serialize_to_file(path.clone());

        let document2 = parse_html().from_utf8().from_file(&path).unwrap();
        assert_eq!(document.to_string(), document2.to_string());
    }

    /// Tests Display trait for NodeRef.
    ///
    /// Verifies that to_string() produces correct HTML output for a
    /// subtree, properly serializing element tags and attributes.
    #[test]
    fn to_string() {
        let html = r"<!DOCTYPE html>
<html>
    <head>
        <title>Test case</title>
    </head>
    <body>
        <p class=foo>Foo
    </body>
</html>";

        let document = parse_html().one(html);
        assert_eq!(
            document
                .inclusive_descendants()
                .nth(11)
                .unwrap()
                .to_string(),
            "<p class=\"foo\">Foo\n    \n</p>"
        );
    }
}
