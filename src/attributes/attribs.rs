use html5ever::LocalName;
#[cfg(feature = "namespaces")]
use html5ever::{Namespace, Prefix};
use indexmap::{map::Entry, IndexMap};

use super::{Attribute, ExpandedName};

/// Convenience wrapper around a indexmap that adds method for attributes in the null namespace.
#[derive(Debug, PartialEq, Clone)]
pub struct Attributes {
    /// A map of attributes whose name can have namespaces.
    pub map: IndexMap<ExpandedName, Attribute>,
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
    pub fn entry<A: Into<LocalName>>(
        &mut self,
        local_name: A,
    ) -> Entry<'_, ExpandedName, Attribute> {
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
    /// **Note:** This method requires the `namespaces` feature to be enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate html5ever;
    /// #[cfg(feature = "namespaces")]
    /// {
    /// # use brik::parse_html;
    /// # use brik::traits::*;
    /// let doc = parse_html().one(r#"<svg xmlns="http://www.w3.org/2000/svg" width="100"/>"#);
    /// let svg = doc.select_first("svg").unwrap();
    /// let attrs = svg.attributes.borrow();
    ///
    /// // SVG width attribute is in the null namespace
    /// assert_eq!(attrs.get_ns(&ns!(), &local_name!("width")), Some("100"));
    /// }
    /// ```
    #[cfg(feature = "namespaces")]
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
    /// **Note:** This method requires the `namespaces` feature to be enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate html5ever;
    /// #[cfg(feature = "namespaces")]
    /// {
    /// # use brik::parse_html;
    /// # use brik::traits::*;
    /// let doc = parse_html().one(r#"<div class="test">Content</div>"#);
    /// let div = doc.select_first("div").unwrap();
    /// let attrs = div.attributes.borrow();
    ///
    /// // class attribute is in the null namespace
    /// assert!(attrs.has_ns(&ns!(), &local_name!("class")));
    /// assert!(!attrs.has_ns(&ns!(), &local_name!("id")));
    /// }
    /// ```
    #[cfg(feature = "namespaces")]
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
    /// **Note:** This method requires the `namespaces` feature to be enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// #[cfg(feature = "namespaces")]
    /// {
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
    /// }
    /// ```
    #[cfg(feature = "namespaces")]
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
    /// **Note:** This method requires the `namespaces` feature to be enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate html5ever;
    /// #[cfg(feature = "namespaces")]
    /// {
    /// # use brik::parse_html;
    /// # use brik::traits::*;
    /// let doc = parse_html().one(r#"<div class="test">Content</div>"#);
    /// let div = doc.select_first("div").unwrap();
    /// let mut attrs = div.attributes.borrow_mut();
    ///
    /// assert!(attrs.has_ns(&ns!(), &local_name!("class")));
    /// attrs.remove_ns(&ns!(), &local_name!("class"));
    /// assert!(!attrs.has_ns(&ns!(), &local_name!("class")));
    /// }
    /// ```
    #[cfg(feature = "namespaces")]
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
    /// **Note:** This method requires the `namespaces` feature to be enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate html5ever;
    /// #[cfg(feature = "namespaces")]
    /// {
    /// # use brik::parse_html;
    /// # use brik::traits::*;
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
    /// }
    /// ```
    #[cfg(feature = "namespaces")]
    pub fn attrs_in_ns<N>(&self, namespace: N) -> impl Iterator<Item = (&LocalName, &str)>
    where
        N: Into<Namespace>,
    {
        let ns = namespace.into();
        self.map.iter().filter_map(move |(name, attr)| {
            if name.ns == ns {
                Some((&name.local, attr.value.as_str()))
            } else {
                None
            }
        })
    }

    /// Removes all xmlns namespace declarations for a given namespace URI.
    ///
    /// Scans the element's attributes for any `xmlns:prefix="uri"` declarations where
    /// the URI matches the provided namespace URI, and removes them.
    ///
    /// xmlns declarations are attributes in the `http://www.w3.org/2000/xmlns/` namespace.
    /// The attribute's local name is the prefix (e.g., `xmlns:tmpl` has local name `tmpl`),
    /// and the attribute value is the namespace URI.
    ///
    /// **Note:** This method requires the `namespaces` feature to be enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// #[cfg(feature = "namespaces")]
    /// {
    /// use brik::Attributes;
    /// use html5ever::{Namespace, LocalName, Prefix};
    ///
    /// let mut attrs = Attributes {
    ///     map: Default::default(),
    /// };
    ///
    /// // Manually add xmlns declarations (HTML parser doesn't preserve these in xmlns namespace)
    /// let xmlns_ns = Namespace::from("http://www.w3.org/2000/xmlns/");
    /// attrs.insert_ns(
    ///     &xmlns_ns,
    ///     "tmpl",
    ///     Some(Prefix::from("xmlns")),
    ///     "http://example.com/tmpl".to_string(),
    /// );
    /// attrs.insert_ns(
    ///     &xmlns_ns,
    ///     "custom",
    ///     Some(Prefix::from("xmlns")),
    ///     "http://example.com/custom".to_string(),
    /// );
    ///
    /// // Remove the template namespace declaration
    /// attrs.remove_xmlns_for("http://example.com/tmpl");
    ///
    /// // The custom namespace declaration should still be present
    /// assert!(attrs.has_ns(&xmlns_ns, "custom"));
    /// assert!(!attrs.has_ns(&xmlns_ns, "tmpl"));
    /// }
    /// ```
    #[cfg(feature = "namespaces")]
    pub fn remove_xmlns_for(&mut self, namespace_uri: &str) {
        let xmlns_ns = Namespace::from("http://www.w3.org/2000/xmlns/");

        // Find all xmlns attributes whose value matches the target URI
        let to_remove: Vec<_> = self
            .map
            .iter()
            .filter_map(|(name, attr)| {
                if name.ns == xmlns_ns && attr.value == namespace_uri {
                    Some(name.local.clone())
                } else {
                    None
                }
            })
            .collect();

        // Remove each matching attribute
        for local_name in to_remove {
            self.remove_ns(&xmlns_ns, local_name);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_html;
    use crate::traits::*;

    /// Tests that `get_ns()` retrieves attributes from the null namespace.
    ///
    /// Regular HTML attributes (class, id, etc.) are in the null namespace.
    /// Verifies that get_ns can retrieve them correctly.
    #[test]
    #[cfg(feature = "namespaces")]
    fn get_ns_null_namespace() {
        let doc = parse_html().one(r#"<div class="test" id="main">Content</div>"#);
        let div = doc.select_first("div").unwrap();
        let attrs = div.attributes.borrow();

        // Regular HTML attributes are in the null namespace
        assert_eq!(attrs.get_ns(ns!(), "class"), Some("test"));
        assert_eq!(attrs.get_ns(ns!(), "id"), Some("main"));
        assert_eq!(attrs.get_ns(ns!(), "missing"), None);
    }

    /// Tests that SVG element attributes are still in the null namespace.
    ///
    /// Even within SVG elements, attributes like width and height
    /// are in the null namespace, not the SVG namespace.
    #[test]
    #[cfg(feature = "namespaces")]
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

    /// Tests that `has_ns()` correctly checks attribute existence in a namespace.
    ///
    /// Verifies both positive cases (attribute exists) and negative cases
    /// (attribute doesn't exist, or exists in wrong namespace).
    #[test]
    #[cfg(feature = "namespaces")]
    fn has_ns_checks_existence() {
        let doc = parse_html().one(r#"<div class="test">Content</div>"#);
        let div = doc.select_first("div").unwrap();
        let attrs = div.attributes.borrow();

        assert!(attrs.has_ns(ns!(), "class"));
        assert!(!attrs.has_ns(ns!(), "id"));
        assert!(!attrs.has_ns(ns!(html), "class"));
    }

    /// Tests that `insert_ns()` adds a new attribute in a custom namespace.
    ///
    /// Verifies that attributes can be created in arbitrary namespaces
    /// and that the prefix is preserved.
    #[test]
    #[cfg(feature = "namespaces")]
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

    /// Tests that `insert_ns()` replaces existing attributes and returns old value.
    ///
    /// When inserting an attribute that already exists, the old value
    /// should be returned and the new value should replace it.
    #[test]
    #[cfg(feature = "namespaces")]
    fn insert_ns_replaces_existing() {
        let mut attrs = Attributes {
            map: Default::default(),
        };

        attrs.insert_ns(ns!(), "test", None, "first".to_string());

        let old = attrs.insert_ns(ns!(), "test", None, "second".to_string());

        assert_eq!(old.as_ref().map(|a| a.value.as_str()), Some("first"));
        assert_eq!(attrs.get_ns(ns!(), "test"), Some("second"));
    }

    /// Tests that `remove_ns()` removes an attribute and returns its value.
    ///
    /// Verifies that the attribute is removed from the collection and
    /// the old value is returned.
    #[test]
    #[cfg(feature = "namespaces")]
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

    /// Tests that `remove_ns()` returns None for nonexistent attributes.
    ///
    /// Attempting to remove an attribute that doesn't exist should
    /// return None without error.
    #[test]
    #[cfg(feature = "namespaces")]
    fn remove_ns_returns_none_when_missing() {
        let doc = parse_html().one(r#"<div>Content</div>"#);
        let div = doc.select_first("div").unwrap();
        let mut attrs = div.attributes.borrow_mut();

        let removed = attrs.remove_ns(ns!(), "nonexistent");
        assert_eq!(removed, None);
    }

    /// Tests that `attrs_in_ns()` iterates all attributes in the null namespace.
    ///
    /// Parses HTML with multiple attributes and verifies that all
    /// null-namespace attributes are yielded by the iterator.
    #[test]
    #[cfg(feature = "namespaces")]
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

    /// Tests that `attrs_in_ns()` returns empty iterator when no attributes match.
    ///
    /// When querying a namespace that contains no attributes,
    /// the iterator should yield no items.
    #[test]
    #[cfg(feature = "namespaces")]
    fn attrs_in_ns_empty_when_no_match() {
        let doc = parse_html().one(r#"<div class="test">Content</div>"#);
        let div = doc.select_first("div").unwrap();
        let attrs = div.attributes.borrow();

        // HTML namespace - no attributes should match
        let html_ns_attrs: Vec<_> = attrs.attrs_in_ns(ns!(html)).collect();
        assert_eq!(html_ns_attrs.len(), 0);
    }

    /// Tests that `attrs_in_ns()` correctly filters attributes by namespace.
    ///
    /// Creates attributes in multiple namespaces and verifies that
    /// the iterator only yields attributes from the requested namespace.
    #[test]
    #[cfg(feature = "namespaces")]
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

    /// Tests that `remove_xmlns_for()` removes xmlns declarations for a URI.
    ///
    /// When multiple xmlns declarations exist with different URIs,
    /// only the one matching the specified URI should be removed.
    #[test]
    #[cfg(feature = "namespaces")]
    fn remove_xmlns_for_removes_matching_declarations() {
        let mut attrs = Attributes {
            map: Default::default(),
        };

        let xmlns_ns = Namespace::from("http://www.w3.org/2000/xmlns/");
        attrs.insert_ns(
            &xmlns_ns,
            "tmpl",
            Some(Prefix::from("xmlns")),
            "http://example.com/tmpl".to_string(),
        );
        attrs.insert_ns(
            &xmlns_ns,
            "custom",
            Some(Prefix::from("xmlns")),
            "http://example.com/custom".to_string(),
        );

        attrs.remove_xmlns_for("http://example.com/tmpl");

        // tmpl should be gone
        assert!(!attrs.has_ns(&xmlns_ns, "tmpl"));

        // custom should still be there
        assert!(attrs.has_ns(&xmlns_ns, "custom"));
    }

    /// Tests that `remove_xmlns_for()` removes all declarations with the same URI.
    ///
    /// When multiple xmlns declarations exist with the same URI but different
    /// prefixes, all should be removed.
    #[test]
    #[cfg(feature = "namespaces")]
    fn remove_xmlns_for_removes_multiple_declarations() {
        let mut attrs = Attributes {
            map: Default::default(),
        };

        let xmlns_ns = Namespace::from("http://www.w3.org/2000/xmlns/");
        // Add multiple declarations with the same URI
        attrs.insert_ns(
            &xmlns_ns,
            "tmpl",
            Some(Prefix::from("xmlns")),
            "http://example.com/same".to_string(),
        );
        attrs.insert_ns(
            &xmlns_ns,
            "template",
            Some(Prefix::from("xmlns")),
            "http://example.com/same".to_string(),
        );
        attrs.insert_ns(
            &xmlns_ns,
            "other",
            Some(Prefix::from("xmlns")),
            "http://example.com/different".to_string(),
        );

        attrs.remove_xmlns_for("http://example.com/same");

        assert!(!attrs.has_ns(&xmlns_ns, "tmpl"));
        assert!(!attrs.has_ns(&xmlns_ns, "template"));
        assert!(attrs.has_ns(&xmlns_ns, "other"));
    }

    /// Tests that `remove_xmlns_for()` does nothing when URI doesn't match.
    ///
    /// When the specified URI doesn't match any xmlns declarations,
    /// all declarations should remain unchanged.
    #[test]
    #[cfg(feature = "namespaces")]
    fn remove_xmlns_for_no_match() {
        let mut attrs = Attributes {
            map: Default::default(),
        };

        let xmlns_ns = Namespace::from("http://www.w3.org/2000/xmlns/");
        attrs.insert_ns(
            &xmlns_ns,
            "custom",
            Some(Prefix::from("xmlns")),
            "http://example.com/custom".to_string(),
        );

        attrs.remove_xmlns_for("http://example.com/nonexistent");

        // Original declaration should still be there
        assert!(attrs.has_ns(&xmlns_ns, "custom"));
    }

    /// Tests that `remove_xmlns_for()` handles empty attribute collections.
    ///
    /// Edge case: calling remove_xmlns_for on an empty Attributes
    /// should not panic.
    #[test]
    #[cfg(feature = "namespaces")]
    fn remove_xmlns_for_empty_attributes() {
        let mut attrs = Attributes {
            map: Default::default(),
        };

        // Should not panic
        attrs.remove_xmlns_for("http://example.com/any");
    }

    /// Tests that `remove_xmlns_for()` only removes xmlns declarations.
    ///
    /// Regular attributes (not in the xmlns namespace) should not
    /// be affected, even if their value matches the URI.
    #[test]
    #[cfg(feature = "namespaces")]
    fn remove_xmlns_for_doesnt_remove_regular_attributes() {
        let mut attrs = Attributes {
            map: Default::default(),
        };

        // Add a regular attribute (not in xmlns namespace)
        attrs.insert("class", "test".to_string());

        let xmlns_ns = Namespace::from("http://www.w3.org/2000/xmlns/");
        attrs.insert_ns(
            &xmlns_ns,
            "tmpl",
            Some(Prefix::from("xmlns")),
            "http://example.com/tmpl".to_string(),
        );

        attrs.remove_xmlns_for("http://example.com/tmpl");

        // Regular attribute should still be there
        assert_eq!(attrs.get("class"), Some("test"));
    }

    /// Tests that `get_mut()` allows in-place modification of attribute values.
    ///
    /// Retrieves a mutable reference to an attribute value and modifies it,
    /// then verifies the modification persisted.
    #[test]
    fn get_mut_modifies_attribute() {
        let doc = parse_html().one(r#"<div class="old">Content</div>"#);
        let div = doc.select_first("div").unwrap();
        let mut attrs = div.attributes.borrow_mut();

        if let Some(value) = attrs.get_mut("class") {
            *value = "new".to_string();
        }

        assert_eq!(attrs.get("class"), Some("new"));
    }

    /// Tests that `get_mut()` returns None for nonexistent attributes.
    ///
    /// Attempting to get a mutable reference to an attribute that
    /// doesn't exist should return None.
    #[test]
    fn get_mut_returns_none_for_missing() {
        let doc = parse_html().one(r#"<div>Content</div>"#);
        let div = doc.select_first("div").unwrap();
        let mut attrs = div.attributes.borrow_mut();

        assert!(attrs.get_mut("nonexistent").is_none());
    }

    /// Tests that `entry().or_insert()` adds a new attribute.
    ///
    /// Uses the entry API to insert an attribute only if it doesn't exist.
    /// Verifies that the attribute is added successfully.
    #[test]
    fn entry_insert_new_attribute() {
        let doc = parse_html().one(r#"<div>Content</div>"#);
        let div = doc.select_first("div").unwrap();
        let mut attrs = div.attributes.borrow_mut();

        attrs.entry("class").or_insert(Attribute {
            prefix: None,
            value: "test".to_string(),
        });

        assert_eq!(attrs.get("class"), Some("test"));
    }

    /// Tests that `entry().or_insert()` preserves existing attributes.
    ///
    /// Uses the entry API to attempt insertion when an attribute already exists.
    /// Verifies that the existing value is kept.
    #[test]
    fn entry_existing_attribute() {
        let doc = parse_html().one(r#"<div class="existing">Content</div>"#);
        let div = doc.select_first("div").unwrap();
        let mut attrs = div.attributes.borrow_mut();

        attrs.entry("class").or_insert(Attribute {
            prefix: None,
            value: "new".to_string(),
        });

        // Should keep existing value
        assert_eq!(attrs.get("class"), Some("existing"));
    }
}
