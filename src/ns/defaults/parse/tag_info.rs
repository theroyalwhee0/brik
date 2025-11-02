use crate::ns::{NsError, NsResult};

/// Information about the parsed HTML tag.
///
/// This struct contains the position information and existing namespace
/// declarations extracted from parsing the `<html>` tag. It is used to
/// determine where to inject missing namespace declarations.
///
/// The `existing_xmlns` field stores byte positions (start, end) for both
/// the prefix and URI, allowing zero-copy extraction from the original HTML string.
#[derive(Debug)]
pub struct HtmlTagInfo {
    /// Position where the html tag starts (at the `<` character).
    pub tag_start: usize,
    /// Position where the tag close (`>` or `/>`) starts.
    pub tag_close_start: usize,
    /// Position after the tag close (after `>` or `/>`).
    pub tag_end: usize,
    /// Map of existing xmlns attributes with their positions in the source HTML.
    /// Each entry maps a prefix (as byte positions: start, end) to a URI (as byte positions: start, end).
    pub existing_xmlns: Vec<((usize, usize), (usize, usize))>,
}

/// Methods for HtmlTagInfo.
///
/// Provides helper methods to extract slices from the original HTML string
/// using the stored position information.
impl HtmlTagInfo {
    /// Returns the namespace prefix at the given index.
    ///
    /// Extracts a slice from the HTML string corresponding to the namespace prefix
    /// at the specified index in the `existing_xmlns` vector.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the namespace entry in the `existing_xmlns` vector
    /// * `html` - The original HTML string that was parsed
    ///
    /// # Returns
    ///
    /// Returns a string slice containing the namespace prefix.
    ///
    /// # Errors
    ///
    /// Returns `NsError::InvalidSlice` if the index is out of bounds or the
    /// stored positions are invalid for the given HTML string.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use brik::ns::defaults::parse::parse_preamble;
    ///
    /// let html = r#"<html xmlns:svg="http://www.w3.org/2000/svg">"#;
    /// let info = parse_preamble(html).unwrap();
    /// let prefix = info.get_prefix(0, html).unwrap();
    /// assert_eq!(prefix, "svg");
    /// ```
    pub fn get_prefix<'a>(&self, index: usize, html: &'a str) -> NsResult<&'a str> {
        let ((start, end), _) = self
            .existing_xmlns
            .get(index)
            .ok_or_else(|| NsError::InvalidSlice("Index out of bounds".to_string()))?;

        html.get(*start..*end)
            .ok_or_else(|| NsError::InvalidSlice("Invalid prefix position".to_string()))
    }

    /// Returns the namespace URI at the given index.
    ///
    /// Extracts a slice from the HTML string corresponding to the namespace URI
    /// at the specified index in the `existing_xmlns` vector.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the namespace entry in the `existing_xmlns` vector
    /// * `html` - The original HTML string that was parsed
    ///
    /// # Returns
    ///
    /// Returns a string slice containing the namespace URI.
    ///
    /// # Errors
    ///
    /// Returns `NsError::InvalidSlice` if the index is out of bounds or the
    /// stored positions are invalid for the given HTML string.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use brik::ns::defaults::parse::parse_preamble;
    ///
    /// let html = r#"<html xmlns:svg="http://www.w3.org/2000/svg">"#;
    /// let info = parse_preamble(html).unwrap();
    /// let uri = info.get_uri(0, html).unwrap();
    /// assert_eq!(uri, "http://www.w3.org/2000/svg");
    /// ```
    pub fn get_uri<'a>(&self, index: usize, html: &'a str) -> NsResult<&'a str> {
        let (_, (start, end)) = self
            .existing_xmlns
            .get(index)
            .ok_or_else(|| NsError::InvalidSlice("Index out of bounds".to_string()))?;

        html.get(*start..*end)
            .ok_or_else(|| NsError::InvalidSlice("Invalid URI position".to_string()))
    }

    /// Returns both the namespace prefix and URI at the given index.
    ///
    /// Extracts slices from the HTML string for both the prefix and URI
    /// at the specified index in the `existing_xmlns` vector.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the namespace entry in the `existing_xmlns` vector
    /// * `html` - The original HTML string that was parsed
    ///
    /// # Returns
    ///
    /// Returns a tuple containing string slices for the prefix and URI.
    ///
    /// # Errors
    ///
    /// Returns `NsError::InvalidSlice` if the index is out of bounds or the
    /// stored positions are invalid for the given HTML string.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use brik::ns::defaults::parse::parse_preamble;
    ///
    /// let html = r#"<html xmlns:svg="http://www.w3.org/2000/svg">"#;
    /// let info = parse_preamble(html).unwrap();
    /// let (prefix, uri) = info.get_namespace(0, html).unwrap();
    /// assert_eq!(prefix, "svg");
    /// assert_eq!(uri, "http://www.w3.org/2000/svg");
    /// ```
    pub fn get_namespace<'a>(&self, index: usize, html: &'a str) -> NsResult<(&'a str, &'a str)> {
        let prefix = self.get_prefix(index, html)?;
        let uri = self.get_uri(index, html)?;
        Ok((prefix, uri))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests extracting namespace prefix and URI using helper methods.
    ///
    /// Verifies that the helper methods correctly slice the HTML string
    /// to extract namespace prefixes and URIs.
    #[test]
    fn test_get_namespace_methods() {
        let html =
            r#"<html xmlns:svg="http://www.w3.org/2000/svg" xmlns:custom="http://example.com">"#;

        let info = HtmlTagInfo {
            tag_start: 0,
            tag_close_start: html.len() - 1,
            tag_end: html.len(),
            existing_xmlns: vec![
                ((12, 15), (17, 43)), // svg -> http://www.w3.org/2000/svg
                ((51, 57), (59, 77)), // custom -> http://example.com
            ],
        };

        // Test get_prefix.
        assert_eq!(info.get_prefix(0, html).unwrap(), "svg");
        assert_eq!(info.get_prefix(1, html).unwrap(), "custom");

        // Test get_uri.
        assert_eq!(info.get_uri(0, html).unwrap(), "http://www.w3.org/2000/svg");
        assert_eq!(info.get_uri(1, html).unwrap(), "http://example.com");

        // Test get_namespace.
        let (prefix, uri) = info.get_namespace(0, html).unwrap();
        assert_eq!(prefix, "svg");
        assert_eq!(uri, "http://www.w3.org/2000/svg");

        let (prefix, uri) = info.get_namespace(1, html).unwrap();
        assert_eq!(prefix, "custom");
        assert_eq!(uri, "http://example.com");

        // Test out of bounds.
        assert!(info.get_prefix(2, html).is_err());
        assert!(info.get_uri(2, html).is_err());
        assert!(info.get_namespace(2, html).is_err());
    }
}
