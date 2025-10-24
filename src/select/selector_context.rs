use html5ever::Namespace;

/// Context for compiling CSS selectors.
///
/// This struct holds configuration that affects how selectors are parsed and matched.
/// Currently, it provides namespace prefix mappings for namespace-aware selector matching.
///
/// **Note:** While `SelectorContext` is always available for API consistency, namespace-related
/// features (prefix mappings and default namespace) only have an effect when the `namespaces`
/// feature is enabled. Without the feature, namespace prefixes in selectors will fail to match.
///
/// # Examples
///
/// Basic usage (requires `namespaces` feature):
///
/// ```
/// #[cfg(feature = "namespaces")]
/// {
/// use brik::SelectorContext;
/// use html5ever::ns;
///
/// let mut context = SelectorContext::new();
/// context.add_namespace("svg".to_string(), ns!(svg));
/// context.set_default_namespace(ns!(html));
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct SelectorContext {
    /// Map from namespace prefixes to namespace URIs.
    pub(super) namespaces: std::collections::HashMap<String, Namespace>,
    /// Optional default namespace for unprefixed element selectors.
    pub(super) default_namespace: Option<Namespace>,
}

impl SelectorContext {
    /// Create a new empty selector context.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a namespace prefix mapping.
    ///
    /// This allows selectors to use the prefix in type selectors (e.g., `svg|rect`)
    /// and attribute selectors (e.g., `[tmpl|if]`).
    ///
    /// **Note:** This method requires the `namespaces` feature to have an effect.
    /// Without the feature, namespace prefixes in selectors will not match elements.
    ///
    /// # Examples
    ///
    /// ```
    /// #[cfg(feature = "namespaces")]
    /// {
    /// use brik::SelectorContext;
    /// use html5ever::ns;
    ///
    /// let mut context = SelectorContext::new();
    /// context.add_namespace("svg".to_string(), ns!(svg));
    /// }
    /// ```
    pub fn add_namespace(&mut self, prefix: String, url: Namespace) -> &mut Self {
        self.namespaces.insert(prefix, url);
        self
    }

    /// Set the default namespace for unprefixed element selectors.
    ///
    /// **Note:** This method requires the `namespaces` feature to have an effect.
    /// Without the feature, the default namespace setting will be ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// #[cfg(feature = "namespaces")]
    /// {
    /// use brik::SelectorContext;
    /// use html5ever::ns;
    ///
    /// let mut context = SelectorContext::new();
    /// context.set_default_namespace(ns!(html));
    /// }
    /// ```
    pub fn set_default_namespace(&mut self, url: Namespace) -> &mut Self {
        self.default_namespace = Some(url);
        self
    }
}
