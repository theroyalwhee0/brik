/// Errors that can occur during namespace parsing operations.
///
/// This enum distinguishes between three types of errors:
/// - Parsing errors occur when the HTML structure cannot be parsed
/// - Slice errors occur when extracting positions from parsed HTML fails
/// - Undefined prefix errors occur when applying namespaces to elements/attributes
#[derive(Debug)]
pub enum NsError {
    /// Failed to parse HTML structure.
    ///
    /// This error occurs when the Pest parser cannot parse the HTML preamble,
    /// typically due to malformed HTML or when required elements (like `<html>`)
    /// cannot be found.
    ///
    /// # Examples
    ///
    /// ```text
    /// NS Parse error: Failed to parse HTML: unexpected token
    /// NS Parse error: No <html> tag found in preamble
    /// ```
    ParseError(String),

    /// Invalid slice position or index.
    ///
    /// This error occurs when extracting slice positions from parsed HTML fails,
    /// typically due to index out of bounds or invalid position calculations.
    /// This indicates an internal inconsistency between the parser and slice
    /// extraction logic.
    ///
    /// # Examples
    ///
    /// ```text
    /// NS Invalid slice: Index out of bounds
    /// NS Invalid slice: Invalid prefix position
    /// ```
    InvalidSlice(String),

    /// Undefined namespace prefix.
    ///
    /// This error occurs when applying namespaces in strict mode and an element
    /// or attribute uses a namespace prefix that has no corresponding xmlns
    /// declaration in the document.
    ///
    /// Contains the rebuilt document (with undefined prefixes using null namespace)
    /// and a list of undefined prefix strings found during processing.
    ///
    /// # Examples
    ///
    /// ```text
    /// NS Undefined prefix: Found 2 undefined prefixes: 'c', 'foo'
    /// ```
    UndefinedPrefix(crate::NodeRef, Vec<String>),
}

/// Result type for namespace parsing operations.
///
/// This is a convenience type alias that uses `NsError` as the error type.
/// Functions in the namespace module return this type to indicate
/// success or failure of parsing and injection operations.
pub type NsResult<T> = Result<T, NsError>;

/// Implements Display for NsError.
///
/// Provides human-readable error messages for namespace parsing errors.
impl std::fmt::Display for NsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NsError::ParseError(msg) => write!(f, "NS Parse error: {msg}"),
            NsError::InvalidSlice(msg) => write!(f, "NS Invalid slice: {msg}"),
            NsError::UndefinedPrefix(_, prefixes) => {
                write!(
                    f,
                    "NS Undefined prefix: Found {} undefined prefix{}: {}",
                    prefixes.len(),
                    if prefixes.len() == 1 { "" } else { "es" },
                    prefixes
                        .iter()
                        .map(|p| format!("'{p}'"))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
    }
}

/// Implements Error for NsError.
///
/// Allows NsError to be used with Rust's standard error handling mechanisms.
impl std::error::Error for NsError {}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests Display formatting for ParseError variant.
    ///
    /// Verifies that ParseError produces correctly formatted error messages.
    #[test]
    fn test_display_parse_error() {
        let error = NsError::ParseError("Failed to parse HTML".to_string());
        let display = format!("{error}");
        assert_eq!(display, "NS Parse error: Failed to parse HTML");
    }

    /// Tests Display formatting for InvalidSlice variant.
    ///
    /// Verifies that InvalidSlice produces correctly formatted error messages.
    #[test]
    fn test_display_invalid_slice() {
        let error = NsError::InvalidSlice("Index out of bounds".to_string());
        let display = format!("{error}");
        assert_eq!(display, "NS Invalid slice: Index out of bounds");
    }

    /// Tests Display formatting for UndefinedPrefix variant.
    ///
    /// Verifies that UndefinedPrefix produces correctly formatted error messages.
    #[test]
    fn test_display_undefined_prefix() {
        use crate::NodeRef;

        let doc = NodeRef::new_document();
        let error = NsError::UndefinedPrefix(doc, vec!["c".to_string(), "foo".to_string()]);
        let display = format!("{error}");
        assert_eq!(
            display,
            "NS Undefined prefix: Found 2 undefined prefixes: 'c', 'foo'"
        );
    }

    /// Tests Display formatting for UndefinedPrefix with single prefix.
    ///
    /// Verifies correct singular/plural handling in error message.
    #[test]
    fn test_display_undefined_prefix_single() {
        use crate::NodeRef;

        let doc = NodeRef::new_document();
        let error = NsError::UndefinedPrefix(doc, vec!["c".to_string()]);
        let display = format!("{error}");
        assert_eq!(
            display,
            "NS Undefined prefix: Found 1 undefined prefix: 'c'"
        );
    }

    /// Tests that NsError implements std::error::Error trait.
    ///
    /// Verifies that NsError can be used with error handling mechanisms.
    #[test]
    fn test_error_trait() {
        use std::error::Error;

        let error = NsError::ParseError("test".to_string());
        // Calling source() verifies Error trait is implemented.
        assert!(error.source().is_none());

        let error = NsError::InvalidSlice("test".to_string());
        assert!(error.source().is_none());
    }

    /// Tests Debug formatting for NsError variants.
    ///
    /// Verifies that Debug is properly derived and formats correctly.
    #[test]
    fn test_debug_formatting() {
        let parse_error = NsError::ParseError("test parse".to_string());
        let debug = format!("{parse_error:?}");
        assert!(debug.contains("ParseError"));
        assert!(debug.contains("test parse"));

        let slice_error = NsError::InvalidSlice("test slice".to_string());
        let debug = format!("{slice_error:?}");
        assert!(debug.contains("InvalidSlice"));
        assert!(debug.contains("test slice"));
    }
}
