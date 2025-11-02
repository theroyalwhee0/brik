use html5ever::Namespace;
use std::collections::HashMap;

use super::nsdefaults::NsDefaults;

/// Builder for configuring namespace defaults.
///
/// This builder allows registering namespace prefix mappings that should be
/// injected into HTML documents when they are missing from the `<html>` tag.
pub struct NsDefaultsBuilder {
    /// Map of namespace prefixes to their URIs.
    namespaces: HashMap<String, Namespace>,
}

/// Methods for NsDefaultsBuilder.
///
/// Provides configuration methods for building namespace defaults.
impl NsDefaultsBuilder {
    /// Creates a new empty namespace builder.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use brik::ns::NsDefaultsBuilder;
    ///
    /// let builder = NsDefaultsBuilder::new();
    /// ```
    pub fn new() -> Self {
        NsDefaultsBuilder {
            namespaces: HashMap::default(),
        }
    }

    /// Registers a namespace prefix mapping.
    ///
    /// Adds a namespace prefix and its corresponding URI to the builder.
    /// When processing HTML, this namespace will be injected into the `<html>`
    /// tag if it is not already present.
    ///
    /// # Arguments
    ///
    /// * `prefix` - The namespace prefix (e.g., "svg", "custom")
    /// * `ns` - The namespace URI or anything that can be converted to a Namespace
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use brik::ns::NsDefaultsBuilder;
    /// use html5ever::ns;
    ///
    /// let builder = NsDefaultsBuilder::new()
    ///     .namespace("svg", ns!(svg))
    ///     .namespace("custom", "http://example.com/ns");
    /// ```
    pub fn namespace(mut self, prefix: impl AsRef<str>, ns: impl Into<Namespace>) -> Self {
        let prefix = prefix.as_ref().to_string();
        let ns = ns.into();
        self.namespaces.insert(prefix, ns);
        self
    }

    /// Processes an HTML string to inject missing namespace declarations.
    ///
    /// This method analyzes the provided HTML to determine which namespace
    /// declarations are missing and builds an NsDefaults instance with
    /// the processed HTML.
    ///
    /// # Arguments
    ///
    /// * `html` - The HTML string to process
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use brik::ns::NsDefaultsBuilder;
    ///
    /// let ns_defaults = NsDefaultsBuilder::new()
    ///     .namespace("custom", "http://example.com/ns")
    ///     .from_str("<html><body>Hello</body></html>");
    /// ```
    pub fn from_str(self, html: impl AsRef<str>) -> NsDefaults {
        // TODO: Implement namespace injection logic.
        NsDefaults {
            html: html.as_ref().to_string(),
            namespaces: self.namespaces,
        }
    }
}

/// Implements Default for NsDefaultsBuilder.
///
/// Creates an empty namespace builder with no registered namespaces.
impl Default for NsDefaultsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
