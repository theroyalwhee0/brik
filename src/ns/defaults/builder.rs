use html5ever::Namespace;
use std::collections::HashMap;

use crate::ns::{defaults::parse::parse_preamble, NsResult};

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
    /// * `html` - The HTML string to process (takes ownership)
    ///
    /// # Errors
    ///
    /// Returns `NsError::ParseError` if the HTML cannot be parsed or the
    /// `<html>` tag is not found in the document.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use brik::ns::NsDefaultsBuilder;
    ///
    /// let ns_defaults = NsDefaultsBuilder::new()
    ///     .namespace("custom", "http://example.com/ns")
    ///     .from_string("<html><body>Hello</body></html>".to_string());
    /// ```
    pub fn from_string(self, html: impl Into<String>) -> NsResult<NsDefaults> {
        let html = html.into();
        let tag_info = parse_preamble(&html)?;

        // Build the xmlns declarations to add.
        let added_xmlns = build_xmlns_declarations(&self.namespaces, &tag_info, &html);

        Ok(NsDefaults {
            html,
            namespaces: self.namespaces,
            tag_info,
            added_xmlns,
        })
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

/// Builds the xmlns declarations string for namespaces that need to be added.
///
/// Compares the configured namespaces against the existing xmlns attributes
/// in the HTML and returns a string containing the missing declarations.
fn build_xmlns_declarations(
    namespaces: &HashMap<String, Namespace>,
    tag_info: &super::parse::HtmlTagInfo,
    html: &str,
) -> String {
    if namespaces.is_empty() {
        return String::new();
    }

    // Collect existing xmlns prefixes from the HTML.
    let mut existing_prefixes = std::collections::HashSet::new();
    for i in 0..tag_info.existing_xmlns.len() {
        if let Ok(prefix) = tag_info.get_prefix(i, html) {
            existing_prefixes.insert(prefix.to_string());
        }
    }

    // Build xmlns declarations for missing namespaces.
    let mut declarations = String::new();
    for (prefix, uri) in namespaces {
        if !existing_prefixes.contains(prefix) {
            declarations.push_str(&format!(" xmlns:{prefix}=\"{uri}\""));
        }
    }

    declarations
}
