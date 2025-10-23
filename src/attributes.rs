use html5ever::{LocalName, Namespace, Prefix};
use indexmap::{map::Entry, IndexMap};

/// Convenience wrapper around a indexmap that adds method for attributes in the null namespace.
#[derive(Debug, PartialEq, Clone)]
pub struct Attributes {
    /// A map of attributes whose name can have namespaces.
    pub map: IndexMap<ExpandedName, Attribute>,
}

/// <https://www.w3.org/TR/REC-xml-names/#dt-expname>
#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub struct ExpandedName {
    /// Namespace URL
    pub ns: Namespace,
    /// "Local" part of the name
    pub local: LocalName,
}

impl ExpandedName {
    /// Trivial constructor
    pub fn new<N: Into<Namespace>, L: Into<LocalName>>(ns: N, local: L) -> Self {
        ExpandedName {
            ns: ns.into(),
            local: local.into(),
        }
    }
}

/// The non-identifying parts of an attribute
#[derive(Debug, PartialEq, Clone)]
pub struct Attribute {
    /// The namespace prefix, if any
    pub prefix: Option<Prefix>,
    /// The attribute value
    pub value: String,
}

impl Attributes {
    /// Like IndexMap::contains
    pub fn contains<A: Into<LocalName>>(&self, local_name: A) -> bool {
        self.map.contains_key(&ExpandedName::new(ns!(), local_name))
    }

    /// Like IndexMap::get
    pub fn get<A: Into<LocalName>>(&self, local_name: A) -> Option<&str> {
        self.map
            .get(&ExpandedName::new(ns!(), local_name))
            .map(|attr| &*attr.value)
    }

    /// Like IndexMap::get_mut
    pub fn get_mut<A: Into<LocalName>>(&mut self, local_name: A) -> Option<&mut String> {
        self.map
            .get_mut(&ExpandedName::new(ns!(), local_name))
            .map(|attr| &mut attr.value)
    }

    /// Like IndexMap::entry
    pub fn entry<A: Into<LocalName>>(&mut self, local_name: A) -> Entry<'_, ExpandedName, Attribute> {
        self.map.entry(ExpandedName::new(ns!(), local_name))
    }

    /// Like IndexMap::insert
    pub fn insert<A: Into<LocalName>>(
        &mut self,
        local_name: A,
        value: String,
    ) -> Option<Attribute> {
        self.map.insert(
            ExpandedName::new(ns!(), local_name),
            Attribute {
                prefix: None,
                value,
            },
        )
    }

    /// Like IndexMap::remove
    pub fn remove<A: Into<LocalName>>(&mut self, local_name: A) -> Option<Attribute> {
        self.map.swap_remove(&ExpandedName::new(ns!(), local_name))
    }

    /// Returns the value of an attribute in a specific namespace.
    ///
    /// Similar to DOM's `getAttributeNS()`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate html5ever;
    /// use brik::parse_html;
    /// use brik::traits::*;
    ///
    /// let doc = parse_html().one(r#"<svg xmlns="http://www.w3.org/2000/svg" width="100"/>"#);
    /// let svg = doc.select_first("svg").unwrap();
    /// let attrs = svg.attributes.borrow();
    ///
    /// // SVG width attribute is in the null namespace
    /// assert_eq!(attrs.get_ns(&ns!(), &local_name!("width")), Some("100"));
    /// ```
    pub fn get_ns<N, L>(&self, namespace: N, local_name: L) -> Option<&str>
    where
        N: Into<Namespace>,
        L: Into<LocalName>,
    {
        self.map
            .get(&ExpandedName::new(namespace, local_name))
            .map(|attr| &*attr.value)
    }

    /// Checks if an attribute exists in a specific namespace.
    ///
    /// Similar to DOM's `hasAttributeNS()`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate html5ever;
    /// use brik::parse_html;
    /// use brik::traits::*;
    ///
    /// let doc = parse_html().one(r#"<div class="test">Content</div>"#);
    /// let div = doc.select_first("div").unwrap();
    /// let attrs = div.attributes.borrow();
    ///
    /// // class attribute is in the null namespace
    /// assert!(attrs.has_ns(&ns!(), &local_name!("class")));
    /// assert!(!attrs.has_ns(&ns!(), &local_name!("id")));
    /// ```
    pub fn has_ns<N, L>(&self, namespace: N, local_name: L) -> bool
    where
        N: Into<Namespace>,
        L: Into<LocalName>,
    {
        self.map
            .contains_key(&ExpandedName::new(namespace, local_name))
    }

    /// Inserts an attribute with a specific namespace.
    ///
    /// Similar to DOM's `setAttributeNS()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use brik::{Attributes, Attribute};
    /// use html5ever::{LocalName, Namespace};
    ///
    /// let mut attrs = Attributes {
    ///     map: Default::default(),
    /// };
    ///
    /// attrs.insert_ns(
    ///     "http://example.com/ns",
    ///     "custom",
    ///     Some("ex".into()),
    ///     "value".to_string(),
    /// );
    ///
    /// assert_eq!(
    ///     attrs.get_ns("http://example.com/ns", "custom"),
    ///     Some("value")
    /// );
    /// ```
    pub fn insert_ns<N, L>(
        &mut self,
        namespace: N,
        local_name: L,
        prefix: Option<Prefix>,
        value: String,
    ) -> Option<Attribute>
    where
        N: Into<Namespace>,
        L: Into<LocalName>,
    {
        self.map.insert(
            ExpandedName::new(namespace, local_name),
            Attribute { prefix, value },
        )
    }

    /// Removes an attribute from a specific namespace.
    ///
    /// Similar to DOM's `removeAttributeNS()`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate html5ever;
    /// use brik::parse_html;
    /// use brik::traits::*;
    ///
    /// let doc = parse_html().one(r#"<div class="test">Content</div>"#);
    /// let div = doc.select_first("div").unwrap();
    /// let mut attrs = div.attributes.borrow_mut();
    ///
    /// assert!(attrs.has_ns(&ns!(), &local_name!("class")));
    /// attrs.remove_ns(&ns!(), &local_name!("class"));
    /// assert!(!attrs.has_ns(&ns!(), &local_name!("class")));
    /// ```
    pub fn remove_ns<N, L>(&mut self, namespace: N, local_name: L) -> Option<Attribute>
    where
        N: Into<Namespace>,
        L: Into<LocalName>,
    {
        self.map
            .swap_remove(&ExpandedName::new(namespace, local_name))
    }

    /// Returns an iterator over all attributes in a specific namespace.
    ///
    /// Yields (local_name, value) pairs for each attribute in the given namespace.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate html5ever;
    /// use brik::parse_html;
    /// use brik::traits::*;
    ///
    /// let doc = parse_html().one(r#"<div class="test" id="main" data-value="foo">Content</div>"#);
    /// let div = doc.select_first("div").unwrap();
    /// let attrs = div.attributes.borrow();
    ///
    /// // Collect all attributes in the null namespace
    /// let mut null_ns_attrs: Vec<_> = attrs.attrs_in_ns(ns!()).collect();
    /// null_ns_attrs.sort_by(|(a, _), (b, _)| a.as_ref().cmp(b.as_ref()));
    ///
    /// assert_eq!(null_ns_attrs.len(), 3);
    /// assert_eq!(null_ns_attrs[0].0.as_ref(), "class");
    /// assert_eq!(null_ns_attrs[0].1, "test");
    /// ```
    pub fn attrs_in_ns<N>(&self, namespace: N) -> impl Iterator<Item = (&LocalName, &str)>
    where
        N: Into<Namespace>,
    {
        let ns = namespace.into();
        self.map
            .iter()
            .filter_map(move |(name, attr)| {
                if name.ns == ns {
                    Some((&name.local, attr.value.as_str()))
                } else {
                    None
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_html;
    use crate::traits::*;

    #[test]
    fn get_ns_null_namespace() {
        let doc = parse_html().one(r#"<div class="test" id="main">Content</div>"#);
        let div = doc.select_first("div").unwrap();
        let attrs = div.attributes.borrow();

        // Regular HTML attributes are in the null namespace
        assert_eq!(attrs.get_ns(ns!(), "class"), Some("test"));
        assert_eq!(attrs.get_ns(ns!(), "id"), Some("main"));
        assert_eq!(attrs.get_ns(ns!(), "missing"), None);
    }

    #[test]
    fn get_ns_svg_namespace() {
        let svg_html = r#"<!DOCTYPE html>
<html>
<body>
<svg xmlns="http://www.w3.org/2000/svg" width="100" height="50">
  <rect width="100" height="50"/>
</svg>
</body>
</html>"#;
        let doc = parse_html().one(svg_html);
        let rect = doc.select_first("rect").unwrap();
        let attrs = rect.attributes.borrow();

        // SVG attributes are still in the null namespace
        assert_eq!(attrs.get_ns(ns!(), "width"), Some("100"));
        assert_eq!(attrs.get_ns(ns!(), "height"), Some("50"));
    }

    #[test]
    fn has_ns_checks_existence() {
        let doc = parse_html().one(r#"<div class="test">Content</div>"#);
        let div = doc.select_first("div").unwrap();
        let attrs = div.attributes.borrow();

        assert!(attrs.has_ns(ns!(), "class"));
        assert!(!attrs.has_ns(ns!(), "id"));
        assert!(!attrs.has_ns(ns!(html), "class"));
    }

    #[test]
    fn insert_ns_adds_attribute() {
        let mut attrs = Attributes {
            map: Default::default(),
        };

        // Insert attribute in custom namespace
        let custom_ns = "http://example.com/ns";
        attrs.insert_ns(
            custom_ns,
            "custom",
            Some(Prefix::from("ex")),
            "value".to_string(),
        );

        assert!(attrs.has_ns(custom_ns, "custom"));
        assert_eq!(attrs.get_ns(custom_ns, "custom"), Some("value"));
    }

    #[test]
    fn insert_ns_replaces_existing() {
        let mut attrs = Attributes {
            map: Default::default(),
        };

        attrs.insert_ns(
            ns!(),
            "test",
            None,
            "first".to_string(),
        );

        let old = attrs.insert_ns(
            ns!(),
            "test",
            None,
            "second".to_string(),
        );

        assert_eq!(old.as_ref().map(|a| a.value.as_str()), Some("first"));
        assert_eq!(attrs.get_ns(ns!(), "test"), Some("second"));
    }

    #[test]
    fn remove_ns_removes_attribute() {
        let doc = parse_html().one(r#"<div class="test" id="main">Content</div>"#);
        let div = doc.select_first("div").unwrap();
        let mut attrs = div.attributes.borrow_mut();

        assert!(attrs.has_ns(ns!(), "class"));

        let removed = attrs.remove_ns(ns!(), "class");
        assert_eq!(removed.as_ref().map(|a| a.value.as_str()), Some("test"));

        assert!(!attrs.has_ns(ns!(), "class"));
        assert_eq!(attrs.get_ns(ns!(), "class"), None);
    }

    #[test]
    fn remove_ns_returns_none_when_missing() {
        let doc = parse_html().one(r#"<div>Content</div>"#);
        let div = doc.select_first("div").unwrap();
        let mut attrs = div.attributes.borrow_mut();

        let removed = attrs.remove_ns(ns!(), "nonexistent");
        assert_eq!(removed, None);
    }

    #[test]
    fn attrs_in_ns_iterates_null_namespace() {
        let doc = parse_html().one(r#"<div class="test" id="main" data-value="foo">Content</div>"#);
        let div = doc.select_first("div").unwrap();
        let attrs = div.attributes.borrow();

        let mut null_ns_attrs: Vec<_> = attrs.attrs_in_ns(ns!()).collect();
        null_ns_attrs.sort_by(|(a, _), (b, _)| a.as_ref().cmp(b.as_ref()));

        assert_eq!(null_ns_attrs.len(), 3);
        assert_eq!(null_ns_attrs[0].0.as_ref(), "class");
        assert_eq!(null_ns_attrs[0].1, "test");
        assert_eq!(null_ns_attrs[1].0.as_ref(), "data-value");
        assert_eq!(null_ns_attrs[1].1, "foo");
        assert_eq!(null_ns_attrs[2].0.as_ref(), "id");
        assert_eq!(null_ns_attrs[2].1, "main");
    }

    #[test]
    fn attrs_in_ns_empty_when_no_match() {
        let doc = parse_html().one(r#"<div class="test">Content</div>"#);
        let div = doc.select_first("div").unwrap();
        let attrs = div.attributes.borrow();

        // HTML namespace - no attributes should match
        let html_ns_attrs: Vec<_> = attrs.attrs_in_ns(ns!(html)).collect();
        assert_eq!(html_ns_attrs.len(), 0);
    }

    #[test]
    fn attrs_in_ns_custom_namespace() {
        let mut attrs = Attributes {
            map: Default::default(),
        };

        let custom_ns = "http://example.com/ns";
        attrs.insert_ns(custom_ns, "attr1", None, "value1".to_string());
        attrs.insert_ns(custom_ns, "attr2", None, "value2".to_string());
        attrs.insert_ns(ns!(), "regular", None, "value3".to_string());

        let mut custom_attrs: Vec<_> = attrs.attrs_in_ns(custom_ns).collect();
        custom_attrs.sort_by(|(a, _), (b, _)| a.as_ref().cmp(b.as_ref()));

        assert_eq!(custom_attrs.len(), 2);
        assert_eq!(custom_attrs[0].0.as_ref(), "attr1");
        assert_eq!(custom_attrs[0].1, "value1");
        assert_eq!(custom_attrs[1].0.as_ref(), "attr2");
        assert_eq!(custom_attrs[1].1, "value2");
    }
}
