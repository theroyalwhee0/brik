# Namespace Support

Brik provides comprehensive namespace support for working with XML-namespaced content in HTML documents, such as SVG, MathML, and custom XML vocabularies.

## Overview

Namespace support is controlled by the `namespaces` feature flag and provides:

- Namespace-aware attribute operations
- Namespace prefix handling
- Element namespace queries
- Selector matching with namespace prefixes

## Enabling Namespace Support

Add the `namespaces` feature to your `Cargo.toml`:

```toml
[dependencies]
brik = { version = "0.9.2", features = ["namespaces"] }
```

## Core Concepts

### Namespace URIs

Elements and attributes can belong to specific namespaces identified by URIs:

- **HTML/XHTML**: `http://www.w3.org/1999/xhtml`
- **SVG**: `http://www.w3.org/2000/svg`
- **MathML**: `http://www.w3.org/1998/Math/MathML`
- **xmlns**: `http://www.w3.org/2000/xmlns/`

### Namespace Prefixes

Namespace prefixes provide short aliases for namespace URIs in XML syntax:

```xml
<svg:rect xmlns:svg="http://www.w3.org/2000/svg" width="100"/>
```

In HTML5 parsing, prefixes are typically absent even when elements are in specific namespaces.

## API Additions

### Element Namespace Methods

#### `namespace_uri()` - Get Element Namespace

Returns the namespace URI of an element.

```rust
use brik::parse_html;
use brik::traits::*;

let doc = parse_html().one(r#"
    <svg xmlns="http://www.w3.org/2000/svg">
        <rect width="100"/>
    </svg>
"#);

let rect = doc.select_first("rect").unwrap();
assert_eq!(
    rect.namespace_uri().as_ref(),
    "http://www.w3.org/2000/svg"
);
```

#### `prefix()` - Get Namespace Prefix

Returns the namespace prefix of an element, if any.

```rust
let div = doc.select_first("div").unwrap();
// HTML elements typically have no prefix
assert_eq!(div.prefix(), None);
```

**Note**: In HTML5, elements rarely have prefixes even when in specific namespaces.

### Attribute Namespace Methods

The `Attributes` type gains several namespace-aware methods:

#### `get_ns()` - Get Attribute by Namespace

Retrieve an attribute value from a specific namespace:

```rust
use html5ever::ns;

let attrs = element.attributes.borrow();
let value = attrs.get_ns(ns!(), "class");
```

#### `has_ns()` - Check Attribute Existence

Check if an attribute exists in a specific namespace:

```rust
if attrs.has_ns(ns!(svg), "viewBox") {
    // SVG viewBox attribute exists
}
```

#### `insert_ns()` - Insert Namespaced Attribute

Add or replace an attribute in a specific namespace:

```rust
use html5ever::{Namespace, Prefix};

let mut attrs = element.attributes.borrow_mut();
attrs.insert_ns(
    Namespace::from("http://example.com/ns"),
    "custom",
    Some(Prefix::from("ex")),
    "value".to_string(),
);
```

#### `remove_ns()` - Remove Namespaced Attribute

Remove an attribute from a specific namespace:

```rust
let old = attrs.remove_ns(ns!(), "class");
```

#### `attrs_in_ns()` - Iterate Namespace Attributes

Get an iterator over all attributes in a specific namespace:

```rust
for (name, value) in attrs.attrs_in_ns(ns!()) {
    println!("{}: {}", name.as_ref(), value);
}
```

#### `remove_xmlns_for()` - Remove xmlns Declarations

Remove all xmlns namespace declarations for a given URI:

```rust
// Remove all xmlns declarations for a namespace
attrs.remove_xmlns_for("http://example.com/tmpl");
```

This is useful for cleaning up namespace declarations after transformations.

### Selector Namespace Support

Use namespace prefixes in CSS selectors with `SelectorContext`:

```rust
use brik::{Selectors, SelectorContext};
use html5ever::ns;

let mut context = SelectorContext::new();
context.add_namespace("svg".to_string(), ns!(svg));

let selectors = Selectors::compile_with_context("svg|rect", &context).unwrap();
```

The selector `svg|rect` will match `<rect>` elements in the SVG namespace.

## Common Patterns

### Working with SVG in HTML

SVG elements embedded in HTML5 documents are automatically placed in the SVG namespace:

```rust
let doc = parse_html().one(r#"
    <!DOCTYPE html>
    <html>
    <body>
        <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
            <circle cx="50" cy="50" r="40"/>
        </svg>
    </body>
    </html>
"#);

let circle = doc.select_first("circle").unwrap();
assert_eq!(
    circle.namespace_uri().as_ref(),
    "http://www.w3.org/2000/svg"
);
```

### Distinguishing HTML and SVG Elements

Elements with the same local name but different namespaces are distinct:

```rust
// HTML <a> element
let html_link = doc.select_first("a").unwrap();
assert_eq!(html_link.namespace_uri().as_ref(), "http://www.w3.org/1999/xhtml");

// SVG <a> element (if present)
// Would have namespace "http://www.w3.org/2000/svg"
```

### Custom Namespace Attributes

Add attributes in custom namespaces for application-specific metadata:

```rust
use html5ever::{Namespace, Prefix};

let mut attrs = element.attributes.borrow_mut();

// Add template directive in custom namespace
attrs.insert_ns(
    Namespace::from("http://example.com/template"),
    "if",
    Some(Prefix::from("tmpl")),
    "condition".to_string(),
);

// Later, query the attribute
let condition = attrs.get_ns("http://example.com/template", "if");
```

### Filtering Attributes by Namespace

Separate application attributes from standard HTML attributes:

```rust
let attrs = element.attributes.borrow();

// Get only custom namespace attributes
let custom_ns = "http://example.com/app";
let app_attrs: Vec<_> = attrs.attrs_in_ns(custom_ns).collect();

for (name, value) in app_attrs {
    println!("Custom attribute {}: {}", name.as_ref(), value);
}
```

## Backward Compatibility

Namespace methods require the `namespaces` feature. Code that doesn't use namespaces continues to work without the feature:

```rust
// Works with or without namespaces feature
let attrs = element.attributes.borrow();
let class = attrs.get("class");  // Gets from null namespace
```

Namespace-aware methods are only available when the feature is enabled:

```rust
// Only available with "namespaces" feature
#[cfg(feature = "namespaces")]
{
    let uri = element.namespace_uri();
}
```

## Migration Guide

### From Non-Namespace-Aware Code

If you're upgrading from code that didn't use namespaces:

1. **Existing code continues to work**: Methods like `get()`, `insert()`, and `remove()` operate on the null namespace by default.

2. **Enable the feature for namespace operations**: Add `features = ["namespaces"]` to use namespace-aware methods.

3. **Use `_ns` methods for explicit namespace control**: When you need to distinguish between attributes in different namespaces, use the `_ns` variants.

### Example Migration

**Before** (works with or without feature):

```rust
let attrs = element.attributes.borrow();
let class = attrs.get("class");
```

**After** (explicit namespace, requires feature):

```rust
#[cfg(feature = "namespaces")]
{
    use html5ever::ns;
    let attrs = element.attributes.borrow();
    let class = attrs.get_ns(ns!(), "class");
}
```

## Best Practices

### Use Null Namespace for HTML Attributes

Regular HTML attributes are in the null namespace:

```rust
use html5ever::ns;

// Both of these access the same attribute
let value1 = attrs.get("id");
let value2 = attrs.get_ns(ns!(), "id");
```

### Cache Namespace Objects

Create namespace objects once and reuse them:

```rust
use html5ever::Namespace;

let custom_ns = Namespace::from("http://example.com/app");

// Reuse custom_ns throughout your code
attrs.get_ns(&custom_ns, "version");
attrs.has_ns(&custom_ns, "enabled");
```

### Clean Up Namespace Declarations

When transforming documents, remove obsolete namespace declarations:

```rust
// After removing all elements from a namespace
attrs.remove_xmlns_for("http://example.com/template");
```

### Use Namespace Prefixes in Selectors

When querying namespaced elements, use namespace-aware selectors:

```rust
let mut context = SelectorContext::new();
context.add_namespace("custom".to_string(), custom_ns);

let selectors = Selectors::compile_with_context("custom|widget", &context)?;
let widgets = doc.select_all(&selectors);
```

## Limitations

### HTML5 Parser Namespace Handling

The HTML5 parser automatically assigns namespaces based on context:

- Elements inside `<svg>` get the SVG namespace
- Elements inside `<math>` get the MathML namespace
- Regular elements get the HTML namespace
- Namespace prefixes are typically not preserved in HTML5 mode

### Attribute Namespaces

Most HTML attributes are in the null namespace. Namespaced attributes are rare in HTML5:

```rust
// Most attributes are in the null namespace
attrs.get_ns(ns!(), "class");  // HTML class attribute
attrs.get_ns(ns!(), "width");  // Works for both HTML and SVG

// XLink attributes (rarely used in HTML5) would be in the XLink namespace
```

## See Also

- [html5ever documentation](https://docs.rs/html5ever/) - Parser behavior and namespace handling
- [CSS Selectors Level 4](https://www.w3.org/TR/selectors-4/) - Namespace selector syntax
- [Namespaces in XML](https://www.w3.org/TR/xml-names/) - XML namespace specification
