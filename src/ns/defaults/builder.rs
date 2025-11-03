use html5ever::Namespace;
use std::collections::BTreeMap;

use crate::ns::{defaults::parse::parse_preamble, NsResult};

use super::nsdefaults::NsDefaults;

/// Estimated bytes per namespace declaration for capacity pre-allocation.
///
/// Format: ' xmlns:prefix="uri"' typically ranges from 30-80 bytes.
/// This estimate helps avoid reallocations when building declaration strings.
const ESTIMATED_BYTES_PER_NAMESPACE: usize = 50;

/// Builder for configuring namespace defaults.
///
/// This builder allows registering namespace prefix mappings that should be
/// injected into HTML documents when they are missing from the `<html>` tag.
pub struct NsDefaultsBuilder {
    /// Map of namespace prefixes to their URIs.
    /// BTreeMap ensures deterministic, alphabetically-sorted output.
    namespaces: BTreeMap<String, Namespace>,
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
            namespaces: BTreeMap::new(),
        }
    }

    /// Registers a namespace prefix mapping.
    ///
    /// Adds a namespace prefix and its corresponding URI to the builder.
    /// When processing HTML, this namespace will be injected into the `<html>`
    /// tag if it is not already present.
    ///
    /// If the same prefix is registered multiple times, the last registration
    /// overwrites previous ones. This allows updating namespace URIs if needed.
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
    ///
    /// // Duplicate registrations overwrite previous values:
    /// let builder = NsDefaultsBuilder::new()
    ///     .namespace("svg", "http://example.com/fake")  // Overwritten
    ///     .namespace("svg", ns!(svg));                   // This value is used
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
        let added_xmlns = build_xmlns_decl(&self.namespaces, &tag_info, &html);

        Ok(NsDefaults {
            html,
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
/// Declarations are added in alphabetical order by prefix.
fn build_xmlns_decl(
    namespaces: &BTreeMap<String, Namespace>,
    tag_info: &super::parse::HtmlTagInfo,
    html: &str,
) -> String {
    if namespaces.is_empty() {
        return String::new();
    }

    // Collect existing xmlns prefixes from the HTML.
    let mut existing_prefixes = std::collections::HashSet::new();
    for i in 0..tag_info.xmlns_count() {
        if let Ok(prefix) = tag_info.get_prefix(i, html) {
            existing_prefixes.insert(prefix.to_string());
        }
    }

    // Build xmlns declarations for missing namespaces.
    // Pre-allocate capacity to avoid reallocations.
    let estimated_capacity = namespaces.len() * ESTIMATED_BYTES_PER_NAMESPACE;
    let mut declarations = String::with_capacity(estimated_capacity);

    for (prefix, uri) in namespaces {
        if !existing_prefixes.contains(prefix) {
            declarations.push_str(&format!(" xmlns:{prefix}=\"{uri}\""));
        }
    }

    declarations
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that duplicate namespace registrations overwrite previous values.
    ///
    /// Verifies that when the same prefix is registered multiple times,
    /// the last registration is used in the final output.
    #[test]
    fn test_duplicate_namespace_overwrites() {
        let html = r#"<html><body>Test</body></html>"#;

        let ns_defaults = NsDefaultsBuilder::new()
            .namespace("svg", "http://example.com/fake-svg")
            .namespace("svg", "http://www.w3.org/2000/svg")
            .from_string(html)
            .expect("Failed to parse HTML");

        let result = ns_defaults.to_string();

        // Should use the last registered value.
        assert!(result.contains("xmlns:svg=\"http://www.w3.org/2000/svg\""));
        assert!(!result.contains("http://example.com/fake-svg"));
    }

    /// Tests that multiple different namespaces are all added.
    ///
    /// Verifies that registering multiple distinct namespaces results in
    /// all of them being added to the HTML output.
    #[test]
    fn test_multiple_namespaces() {
        let html = r#"<html><body>Test</body></html>"#;

        let ns_defaults = NsDefaultsBuilder::new()
            .namespace("svg", "http://www.w3.org/2000/svg")
            .namespace("custom", "http://example.com/ns")
            .namespace("other", "http://other.com/ns")
            .from_string(html)
            .expect("Failed to parse HTML");

        let result = ns_defaults.to_string();

        // All namespaces should be present.
        assert!(result.contains("xmlns:svg=\"http://www.w3.org/2000/svg\""));
        assert!(result.contains("xmlns:custom=\"http://example.com/ns\""));
        assert!(result.contains("xmlns:other=\"http://other.com/ns\""));
    }

    /// Tests that existing namespaces are not duplicated.
    ///
    /// Verifies that if a namespace is already present in the HTML,
    /// it is not added again even if registered in the builder.
    #[test]
    fn test_existing_namespace_not_duplicated() {
        let html = r#"<html xmlns:svg="http://www.w3.org/2000/svg"><body>Test</body></html>"#;

        let ns_defaults = NsDefaultsBuilder::new()
            .namespace("svg", "http://www.w3.org/2000/svg")
            .namespace("custom", "http://example.com/ns")
            .from_string(html)
            .expect("Failed to parse HTML");

        let result = ns_defaults.to_string();

        // Custom should be added, but svg should not be duplicated.
        assert!(result.contains("xmlns:custom=\"http://example.com/ns\""));

        // Count occurrences of xmlns:svg - should only appear once.
        let svg_count = result.matches("xmlns:svg").count();
        assert_eq!(svg_count, 1);
    }

    /// Tests that Default trait creates an empty builder.
    ///
    /// Verifies that NsDefaultsBuilder::default() produces the same result
    /// as NsDefaultsBuilder::new().
    #[test]
    fn test_default_implementation() {
        let default_builder = NsDefaultsBuilder::default();
        let new_builder = NsDefaultsBuilder::new();

        // Both should produce identical results.
        let html = "<html><body>Test</body></html>";

        let ns1 = default_builder.from_string(html).expect("Failed to parse");
        let ns2 = new_builder.from_string(html).expect("Failed to parse");

        assert_eq!(ns1.to_string(), ns2.to_string());
    }

    /// Tests that empty builder produces no modifications.
    ///
    /// Verifies that a builder with no registered namespaces returns
    /// the original HTML unchanged.
    #[test]
    fn test_empty_builder_no_modifications() {
        let html = r#"<html lang="en"><body>Test</body></html>"#;

        let ns_defaults = NsDefaultsBuilder::new()
            .from_string(html)
            .expect("Failed to parse HTML");

        let result = ns_defaults.to_string();
        assert_eq!(result, html);
    }
}
