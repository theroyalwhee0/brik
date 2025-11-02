/// Errors that can occur during namespace parsing operations.
///
/// This enum distinguishes between two types of errors:
/// - Parsing errors occur when the HTML structure cannot be parsed
/// - Slice errors occur when extracting positions from parsed HTML fails
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
        }
    }
}

/// Implements Error for NsError.
///
/// Allows NsError to be used with Rust's standard error handling mechanisms.
impl std::error::Error for NsError {}
