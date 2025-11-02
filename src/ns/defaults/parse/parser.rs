use super::preamble::{HtmlPreamble, Rule};
use super::tag_info::{HtmlTagInfo, XmlnsPositions};
use crate::ns::{NsError, NsResult};
use pest::iterators::Pair;
use pest::Parser;

/// Parses the top of an HTML document to locate and analyze the `<html>` tag.
///
/// This function uses the Pest parser to efficiently parse only the beginning
/// of the HTML document (typically ~1KB) to find the `<html>` tag and extract
/// information about its attributes, particularly existing `xmlns:*` declarations.
///
/// The function walks the parse tree to:
/// - Locate the `<html>` tag position
/// - Extract all existing `xmlns:*` attributes
/// - Record the injection point (before the tag close)
///
/// # Arguments
///
/// * `html` - The HTML string to parse
///
/// # Returns
///
/// Returns `Ok(HtmlTagInfo)` with information about the html tag location and
/// existing xmlns attributes, or `Err(NsError)` if parsing fails.
///
/// # Errors
///
/// Returns `NsError::ParseError` if:
/// - The `<html>` tag is not found in the document
/// - The `<html>` tag attributes are malformed
/// - The HTML structure cannot be parsed by the Pest grammar
///
/// Note: The parser is forgiving with malformed preambles (comments, DOCTYPEs, etc.)
/// and will skip over them to find the `<html>` tag. However, the `<html>` tag
/// itself must be well-formed
///
/// # Examples
///
/// ```ignore
/// use brik::ns::defaults::parse::parse_preamble;
///
/// let html = r#"<!DOCTYPE html><html lang="en"><body>Hello</body></html>"#;
/// let info = parse_preamble(html).unwrap();
/// assert!(info.existing_xmlns.is_empty());
/// ```
pub fn parse_preamble(html: impl AsRef<str>) -> NsResult<HtmlTagInfo> {
    let html = html.as_ref();

    // Parse the document using Pest.
    let pairs = HtmlPreamble::parse(Rule::document, html)
        .map_err(|e| NsError::ParseError(format!("Failed to parse HTML: {e}")))?;

    // Walk the parse tree to find the html_tag and extract information.
    for pair in pairs {
        if pair.as_rule() == Rule::document {
            for inner in pair.into_inner() {
                if inner.as_rule() == Rule::html_tag {
                    return extract_html_tag_info(inner);
                }
            }
        }
    }

    Err(NsError::ParseError(
        "No <html> tag found in document".to_string(),
    ))
}

/// Extracts tag information from an html_tag parse node.
///
/// Processes the html_tag's children to extract tag positions and xmlns attributes.
#[inline]
fn extract_html_tag_info(html_tag: Pair<Rule>) -> NsResult<HtmlTagInfo> {
    let tag_start = html_tag.as_span().start();
    let tag_end = html_tag.as_span().end();
    let mut tag_close_start = 0;
    let mut existing_xmlns = Vec::new();

    // Process the html_tag's children.
    for tag_part in html_tag.into_inner() {
        match tag_part.as_rule() {
            Rule::attributes => {
                extract_xmlns_attributes(tag_part, &mut existing_xmlns);
            }
            Rule::tag_close => {
                // Mark where the tag close starts (position of `>` or `/>`).
                tag_close_start = tag_part.as_span().start();
            }
            _ => {}
        }
    }

    Ok(HtmlTagInfo {
        tag_start,
        tag_close_start,
        tag_end,
        existing_xmlns,
    })
}

/// Extracts xmlns:* attributes from an attributes parse node.
///
/// Processes all attributes and stores the positions of xmlns:* declarations.
#[inline]
fn extract_xmlns_attributes(attributes: Pair<Rule>, existing_xmlns: &mut Vec<XmlnsPositions>) {
    for attr in attributes.into_inner() {
        if attr.as_rule() == Rule::attribute {
            if let Some(xmlns_positions) = extract_xmlns_from_attribute(attr) {
                existing_xmlns.push(xmlns_positions);
            }
        }
    }
}

/// Extracts xmlns namespace positions from a single attribute.
///
/// Returns positions for the prefix and URI if this is an xmlns:* attribute.
#[inline]
fn extract_xmlns_from_attribute(attr: Pair<Rule>) -> Option<XmlnsPositions> {
    let mut attr_name_span = None;
    let mut attr_value_span = None;

    for attr_part in attr.into_inner() {
        match attr_part.as_rule() {
            Rule::attr_name => {
                attr_name_span = Some(attr_part.as_span());
            }
            Rule::attr_value => {
                attr_value_span = Some(extract_value_positions(attr_part));
            }
            _ => {}
        }
    }

    // If this is an xmlns:* attribute, return its positions.
    if let (Some(name_span), Some(value_span)) = (attr_name_span, attr_value_span) {
        let name = name_span.as_str();
        if name.starts_with("xmlns:") {
            let prefix_offset = "xmlns:".len();
            let prefix_start = name_span.start() + prefix_offset;
            let prefix_end = name_span.end();

            return Some(((prefix_start, prefix_end), value_span));
        }
    }

    None
}

/// Extracts value positions from an attribute value, excluding quotes.
///
/// Calculates the start and end positions of the attribute value,
/// removing surrounding quotes if present.
#[inline]
fn extract_value_positions(value_pair: Pair<Rule>) -> (usize, usize) {
    let span = value_pair.as_span();
    let value = span.as_str();

    // Calculate positions excluding quotes if present.
    let starts_with_quote = value.starts_with('"') || value.starts_with('\'');
    let ends_with_quote = value.ends_with('"') || value.ends_with('\'');
    let start_offset = if starts_with_quote { 1 } else { 0 };
    let end_offset = if ends_with_quote { 1 } else { 0 };

    (span.start() + start_offset, span.end() - end_offset)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests parsing a simple HTML document without xmlns attributes.
    ///
    /// Verifies that the parser correctly identifies the html tag positions
    /// and returns an empty xmlns map when no namespace declarations exist.
    #[test]
    fn parse_simple_html() {
        let html = r#"<!DOCTYPE html>
<html lang="en">
<body>Hello</body>
</html>"#;

        let result = parse_preamble(html);
        assert!(result.is_ok());

        let info = result.unwrap();
        assert!(info.existing_xmlns.is_empty());
        assert!(info.tag_start < info.tag_close_start);
        assert!(info.tag_close_start < info.tag_end);
    }

    /// Tests parsing HTML with existing xmlns declarations.
    ///
    /// Verifies that the parser correctly extracts xmlns:* attributes
    /// and stores them with their positions in the existing_xmlns vector.
    #[test]
    fn parse_with_xmlns() {
        let html = r#"<!DOCTYPE html>
<html xmlns:custom="http://example.com/ns" xmlns:other="http://other.com" lang="en">
<body>Hello</body>
</html>"#;

        let result = parse_preamble(html);
        assert!(result.is_ok());

        let info = result.unwrap();
        assert_eq!(info.existing_xmlns.len(), 2);

        // Verify the first xmlns attribute (custom).
        let ((prefix_start, prefix_end), (uri_start, uri_end)) = info.existing_xmlns[0];
        assert_eq!(&html[prefix_start..prefix_end], "custom");
        assert_eq!(&html[uri_start..uri_end], "http://example.com/ns");

        // Verify the second xmlns attribute (other).
        let ((prefix_start, prefix_end), (uri_start, uri_end)) = info.existing_xmlns[1];
        assert_eq!(&html[prefix_start..prefix_end], "other");
        assert_eq!(&html[uri_start..uri_end], "http://other.com");
    }

    /// Tests parsing HTML with comments in the preamble.
    ///
    /// Verifies that the parser correctly skips comments and doesn't
    /// falsely match html tags within comment content.
    #[test]
    fn parse_with_comment() {
        let html = r#"<!-- This has <html> in it -->
<!DOCTYPE html>
<html>
<body>Hello</body>
</html>"#;

        let result = parse_preamble(html);
        assert!(result.is_ok());
    }

    /// Tests that missing html tag returns an error.
    ///
    /// Verifies that documents without an html tag return an appropriate error.
    #[test]
    fn parse_missing_html_tag() {
        let html = r#"<!DOCTYPE html>
<body>Hello</body>"#;

        let result = parse_preamble(html);
        assert!(result.is_err());
    }

    /// Tests parsing HTML with processing instructions.
    ///
    /// Verifies that the parser correctly skips XML processing instructions
    /// in the preamble.
    #[test]
    fn parse_with_pi() {
        let html = r#"<?xml version="1.0"?>
<!DOCTYPE html>
<html>
<body>Hello</body>
</html>"#;

        let result = parse_preamble(html);
        assert!(result.is_ok());
    }
}
