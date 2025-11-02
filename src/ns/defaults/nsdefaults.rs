use html5ever::Namespace;
use std::collections::HashMap;

/// Processed HTML with namespace declarations injected.
///
/// This struct contains the HTML string with missing namespace declarations
/// added to the `<html>` tag based on the configured namespace defaults.
pub struct NsDefaults {
    /// The processed HTML string with namespace declarations injected.
    pub(super) html: String,
    /// Map of namespace prefixes to their URIs that were configured.
    #[allow(dead_code)] // Will be used when implementing the Into conversion.
    pub(super) namespaces: HashMap<String, Namespace>,
}

/// Methods for NsDefaults.
///
/// Provides access to the processed HTML.
impl NsDefaults {
    /// Returns the processed HTML string.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use brik::ns::NsDefaultsBuilder;
    ///
    /// let ns_defaults = NsDefaultsBuilder::new()
    ///     .namespace("svg", "http://www.w3.org/2000/svg")
    ///     .from_str("<html><body>Hello</body></html>");
    ///
    /// let html = ns_defaults.as_str();
    /// ```
    pub fn as_str(&self) -> &str {
        &self.html
    }
}

/// Implements Into<String> for NsDefaults.
///
/// Allows converting NsDefaults into a String by consuming the instance
/// and returning the processed HTML.
impl From<NsDefaults> for String {
    fn from(ns_defaults: NsDefaults) -> Self {
        ns_defaults.html
    }
}

/// Implements AsRef<str> for NsDefaults.
///
/// Allows NsDefaults to be used anywhere a &str is expected.
impl AsRef<str> for NsDefaults {
    fn as_ref(&self) -> &str {
        &self.html
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the as_str method.
    ///
    /// Verifies that as_str returns a reference to the HTML string.
    #[test]
    fn test_as_str() {
        let ns_defaults = NsDefaults {
            html: "<html><body>Test</body></html>".to_string(),
            namespaces: HashMap::new(),
        };

        assert_eq!(ns_defaults.as_str(), "<html><body>Test</body></html>");
    }

    /// Tests the AsRef<str> implementation.
    ///
    /// Verifies that NsDefaults can be used via AsRef<str>.
    #[test]
    fn test_as_ref() {
        let ns_defaults = NsDefaults {
            html: "<html><body>Test</body></html>".to_string(),
            namespaces: HashMap::new(),
        };

        fn takes_str_ref(s: &str) -> usize {
            s.len()
        }

        assert_eq!(takes_str_ref(ns_defaults.as_ref()), 30);
    }

    /// Tests the Into<String> implementation.
    ///
    /// Verifies that NsDefaults can be converted into a String.
    #[test]
    fn test_into_string() {
        let ns_defaults = NsDefaults {
            html: "<html><body>Test</body></html>".to_string(),
            namespaces: HashMap::new(),
        };

        let html_string: String = ns_defaults.into();
        assert_eq!(html_string, "<html><body>Test</body></html>");
    }
}
