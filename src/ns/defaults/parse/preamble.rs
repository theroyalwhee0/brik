use pest_derive::Parser;

/// Minimal HTML parser for locating and parsing the `<html>` tag.
///
/// This parser is designed specifically for namespace injection. It parses only
/// the beginning of an HTML document up to and including the `<html>` tag,
/// allowing efficient extraction and modification of namespace declarations
/// without processing the entire document.
///
/// The parser handles common HTML preamble elements (processing instructions,
/// DOCTYPEs, comments) and correctly identifies the `<html>` tag even when
/// these elements contain `<html>` in their content.
///
/// # Grammar
///
/// The parser is generated from `ns/defaults/parse/preamble.pest` and recognizes:
/// - Processing instructions: `<?xml version="1.0"?>`
/// - DOCTYPEs with optional internal subsets
/// - HTML comments (including those containing `<html>`)
/// - Case-insensitive `<html>` tag matching
/// - All attribute formats (boolean, empty, quoted, unquoted)
/// - Self-closing tags (`/>`)
///
/// # Robustness
///
/// The parser is forgiving with malformed preambles:
/// - Malformed or unclosed comments/PIs/DOCTYPEs in the preamble are skipped
/// - The parser focuses on finding the `<html>` tag
/// - Content that doesn't match known constructs is consumed until `<html>` is found
///
/// However, the `<html>` tag itself must be well-formed:
/// - The `<html>` tag must be present in the document
/// - Attributes on the `<html>` tag must be properly formatted
///
/// This design allows the parser to handle real-world HTML that may have
/// quirks in the preamble, while still reliably extracting namespace information
/// from the `<html>` tag.
///
/// # Examples
///
/// ```ignore
/// use brik::ns::defaults::parse::preamble::{HtmlPreamble, Rule};
/// use pest::Parser;
///
/// let html = r#"<!DOCTYPE html><html lang="en"><body>Hello</body></html>"#;
/// let pairs = HtmlPreamble::parse(Rule::document, html).unwrap();
/// ```
#[derive(Parser)]
#[grammar = "ns/defaults/parse/preamble.pest"]
// pest_derive's Parser macro contains allows that conflict with our forbid.
#[allow(clippy::all)]
pub struct HtmlPreamble;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    /// Tests parsing a basic HTML document.
    ///
    /// Verifies the parser correctly handles a standard HTML5 document with
    /// DOCTYPE, html tag with attributes, and body content.
    #[test]
    fn parse_simple_html() {
        let html = r#"<!DOCTYPE html>
<html lang="en">
<body>Hello</body>
</html>"#;

        let result = HtmlPreamble::parse(Rule::document, html);
        assert!(result.is_ok());
    }

    /// Tests parsing HTML with a processing instruction.
    ///
    /// Verifies the parser correctly skips XML processing instructions in the
    /// preamble and does not falsely match `<html>` text that may appear within them.
    #[test]
    fn parse_with_pi() {
        let html = r#"<?xml version="1.0"?>
<!DOCTYPE html>
<html>
<body>Hello</body>
</html>"#;

        let result = HtmlPreamble::parse(Rule::document, html);
        assert!(result.is_ok());
    }

    /// Tests parsing HTML with comments containing the html tag text.
    ///
    /// Verifies the parser correctly skips comments in the preamble and does not
    /// falsely match `<html>` text appearing within comment content.
    #[test]
    fn parse_with_comment() {
        let html = r#"<!-- This is a comment with <html> inside -->
<!DOCTYPE html>
<html>
<body>Hello</body>
</html>"#;

        let result = HtmlPreamble::parse(Rule::document, html);
        assert!(result.is_ok());
    }

    /// Tests case-insensitive html tag matching.
    ///
    /// Verifies the parser matches `<HTML>` in uppercase, consistent with
    /// HTML's case-insensitive tag names.
    #[test]
    fn parse_case_insensitive() {
        let html = r#"<!DOCTYPE html>
<HTML lang="en">
<body>Hello</body>
</HTML>"#;

        let result = HtmlPreamble::parse(Rule::document, html);
        assert!(result.is_ok());
    }

    /// Tests parsing self-closing html tags.
    ///
    /// Verifies the parser correctly handles the `/>` self-closing syntax,
    /// though this is non-standard for HTML elements.
    #[test]
    fn parse_self_closing() {
        let html = r#"<!DOCTYPE html>
<html lang="en"/>"#;

        let result = HtmlPreamble::parse(Rule::document, html);
        assert!(result.is_ok());
    }

    /// Tests parsing html tags with existing xmlns namespace declarations.
    ///
    /// Verifies the parser correctly handles namespace attributes with colons
    /// in their names (xmlns:prefix format).
    #[test]
    fn parse_with_xmlns() {
        let html = r#"<!DOCTYPE html>
<html xmlns:custom="http://example.com/ns" lang="en">
<body>Hello</body>
</html>"#;

        let result = HtmlPreamble::parse(Rule::document, html);
        assert!(result.is_ok());
    }

    /// Tests parsing boolean attributes without values.
    ///
    /// Verifies the parser correctly handles attributes that have no value
    /// assigned (e.g., `data-custom` without an equals sign).
    #[test]
    fn parse_boolean_attribute() {
        let html = r#"<!DOCTYPE html>
<html lang="en" data-custom>
<body>Hello</body>
</html>"#;

        let result = HtmlPreamble::parse(Rule::document, html);
        assert!(result.is_ok());
    }

    /// Tests parsing attributes with empty values.
    ///
    /// Verifies the parser correctly handles attributes with an equals sign
    /// but no value following it (e.g., `class=`).
    #[test]
    fn parse_empty_attribute_value() {
        let html = r#"<!DOCTYPE html>
<html class=>
<body>Hello</body>
</html>"#;

        let result = HtmlPreamble::parse(Rule::document, html);
        assert!(result.is_ok());
    }

    /// Tests parsing attributes with unquoted values.
    ///
    /// Verifies the parser correctly handles attribute values that are not
    /// enclosed in quotes (e.g., `lang=en`).
    #[test]
    fn parse_unquoted_attribute() {
        let html = r#"<!DOCTYPE html>
<html lang=en>
<body>Hello</body>
</html>"#;

        let result = HtmlPreamble::parse(Rule::document, html);
        assert!(result.is_ok());
    }
}
