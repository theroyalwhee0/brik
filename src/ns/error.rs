/// Errors that can occur during namespace parsing operations.
#[derive(Debug)]
pub enum NsError {
    /// Failed to parse HTML structure.
    ParseError(String),
    /// Invalid slice position or index.
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
