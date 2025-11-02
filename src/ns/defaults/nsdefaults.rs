use html5ever::tendril::StrTendril;
use html5ever::Namespace;
use std::collections::HashMap;

use super::parse::HtmlTagInfo;

/// Processed HTML with namespace declarations to be added.
///
/// This struct contains the original HTML string and information about
/// which namespace declarations need to be added. The actual string
/// concatenation is deferred until the HTML is consumed or converted.
pub struct NsDefaults {
    /// The original HTML string (unchanged).
    pub(super) html: String,
    /// Map of namespace prefixes to their URIs that were configured.
    #[allow(dead_code)] // Stored for potential future use.
    pub(super) namespaces: HashMap<String, Namespace>,
    /// Information about the parsed HTML tag.
    pub(super) tag_info: HtmlTagInfo,
    /// The namespace declarations to add (e.g., " xmlns:svg=\"...\"").
    /// Empty string if no additions needed.
    pub(super) added_xmlns: String,
}

/// Methods for NsDefaults.
///
/// Provides access to the processed HTML with namespace declarations.
impl NsDefaults {
    /// Builds and returns the processed HTML string with namespace declarations.
    ///
    /// This allocates a new String by combining the HTML slices with the
    /// added namespace declarations.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use brik::ns::NsDefaultsBuilder;
    /// use std::string::ToString;
    ///
    /// let ns_defaults = NsDefaultsBuilder::new()
    ///     .namespace("svg", "http://www.w3.org/2000/svg")
    ///     .from_string("<html><body>Hello</body></html>")?;
    ///
    /// let html = ns_defaults.to_string();
    /// ```
    fn build_html_string(&self) -> String {
        if self.added_xmlns.is_empty() {
            // No additions needed, return original HTML.
            self.html.clone()
        } else {
            // Add namespace declarations at tag_close_start position.
            let mut result = String::with_capacity(
                self.html.len() + self.added_xmlns.len(),
            );
            result.push_str(&self.html[..self.tag_info.tag_close_start]);
            result.push_str(&self.added_xmlns);
            result.push_str(&self.html[self.tag_info.tag_close_start..]);
            result
        }
    }

    /// Returns slices of the HTML for iteration.
    ///
    /// Returns a vector of string slices that can be used to build the
    /// final HTML without intermediate allocations during iteration.
    fn slices(&self) -> Vec<&str> {
        if self.added_xmlns.is_empty() {
            vec![&self.html]
        } else {
            vec![
                &self.html[..self.tag_info.tag_close_start],
                &self.added_xmlns,
                &self.html[self.tag_info.tag_close_start..],
            ]
        }
    }
}

/// Implements Display for NsDefaults.
///
/// Formats the namespace-processed HTML for display.
impl std::fmt::Display for NsDefaults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build_html_string())
    }
}

/// Implements Into<String> for NsDefaults.
///
/// Allows converting NsDefaults into a String by consuming the instance
/// and returning the processed HTML with namespace declarations added.
impl From<NsDefaults> for String {
    fn from(ns_defaults: NsDefaults) -> Self {
        ns_defaults.build_html_string()
    }
}

/// Implements From<NsDefaults> for StrTendril.
///
/// Allows NsDefaults to be consumed and converted into a StrTendril,
/// which can be used with html5ever's `.one()` method.
///
/// Note: This will copy the HTML string (with added namespaces) into the tendril.
impl From<NsDefaults> for StrTendril {
    fn from(ns_defaults: NsDefaults) -> Self {
        StrTendril::from(ns_defaults.build_html_string())
    }
}

/// Implements IntoIterator for NsDefaults.
///
/// Yields string slices as StrTendrils: the HTML before the addition point,
/// the added namespace declarations, and the HTML after the addition point.
/// This can be used with html5ever's `.from_iter()` method.
impl IntoIterator for NsDefaults {
    type Item = StrTendril;
    type IntoIter = std::vec::IntoIter<StrTendril>;

    fn into_iter(self) -> Self::IntoIter {
        let slices = self.slices();
        slices.into_iter().map(StrTendril::from).collect::<Vec<_>>().into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the Display implementation without additions.
    ///
    /// Verifies that Display returns the original HTML when no additions are needed.
    #[test]
    fn test_display_no_additions() {
        let html = "<html><body>Test</body></html>";
        let ns_defaults = NsDefaults {
            html: html.to_string(),
            namespaces: HashMap::new(),
            tag_info: HtmlTagInfo {
                tag_start: 0,
                tag_close_start: 5,
                tag_end: 6,
                existing_xmlns: vec![],
            },
            added_xmlns: String::new(),
        };

        assert_eq!(ns_defaults.to_string(), html);
    }

    /// Tests the Display implementation with additions.
    ///
    /// Verifies that Display correctly adds namespace declarations.
    #[test]
    fn test_display_with_additions() {
        let html = "<html><body>Test</body></html>";
        let ns_defaults = NsDefaults {
            html: html.to_string(),
            namespaces: HashMap::new(),
            tag_info: HtmlTagInfo {
                tag_start: 0,
                tag_close_start: 5,
                tag_end: 6,
                existing_xmlns: vec![],
            },
            added_xmlns: " xmlns:svg=\"http://www.w3.org/2000/svg\"".to_string(),
        };

        let expected = "<html xmlns:svg=\"http://www.w3.org/2000/svg\"><body>Test</body></html>";
        assert_eq!(ns_defaults.to_string(), expected);
    }

    /// Tests the Into<String> implementation without additions.
    ///
    /// Verifies that NsDefaults can be converted into a String.
    #[test]
    fn test_into_string() {
        let html = "<html><body>Test</body></html>";
        let ns_defaults = NsDefaults {
            html: html.to_string(),
            namespaces: HashMap::new(),
            tag_info: HtmlTagInfo {
                tag_start: 0,
                tag_close_start: 5,
                tag_end: 6,
                existing_xmlns: vec![],
            },
            added_xmlns: String::new(),
        };

        let html_string: String = ns_defaults.into();
        assert_eq!(html_string, html);
    }

    /// Tests the Into<StrTendril> implementation.
    ///
    /// Verifies that NsDefaults can be converted into a StrTendril
    /// for use with html5ever's .one() method.
    #[test]
    fn test_into_str_tendril() {
        let html = "<html><body>Test</body></html>";
        let ns_defaults = NsDefaults {
            html: html.to_string(),
            namespaces: HashMap::new(),
            tag_info: HtmlTagInfo {
                tag_start: 0,
                tag_close_start: 5,
                tag_end: 6,
                existing_xmlns: vec![],
            },
            added_xmlns: String::new(),
        };

        let tendril: StrTendril = ns_defaults.into();
        assert_eq!(tendril.as_ref(), html);
    }

    /// Tests the IntoIterator implementation without additions.
    ///
    /// Verifies that NsDefaults yields a single slice when no additions are needed.
    #[test]
    fn test_into_iterator_no_additions() {
        let html = "<html><body>Test</body></html>";
        let ns_defaults = NsDefaults {
            html: html.to_string(),
            namespaces: HashMap::new(),
            tag_info: HtmlTagInfo {
                tag_start: 0,
                tag_close_start: 5,
                tag_end: 6,
                existing_xmlns: vec![],
            },
            added_xmlns: String::new(),
        };

        let tendrils: Vec<StrTendril> = ns_defaults.into_iter().collect();
        assert_eq!(tendrils.len(), 1);
        assert_eq!(tendrils[0].as_ref(), html);
    }

    /// Tests the IntoIterator implementation with additions.
    ///
    /// Verifies that NsDefaults yields three slices when additions are needed.
    #[test]
    fn test_into_iterator_with_additions() {
        let html = "<html><body>Test</body></html>";
        let ns_defaults = NsDefaults {
            html: html.to_string(),
            namespaces: HashMap::new(),
            tag_info: HtmlTagInfo {
                tag_start: 0,
                tag_close_start: 5,
                tag_end: 6,
                existing_xmlns: vec![],
            },
            added_xmlns: " xmlns:svg=\"http://www.w3.org/2000/svg\"".to_string(),
        };

        let tendrils: Vec<StrTendril> = ns_defaults.into_iter().collect();
        assert_eq!(tendrils.len(), 3);
        assert_eq!(tendrils[0].as_ref(), "<html");
        assert_eq!(tendrils[1].as_ref(), " xmlns:svg=\"http://www.w3.org/2000/svg\"");
        assert_eq!(tendrils[2].as_ref(), "><body>Test</body></html>");
    }
}
