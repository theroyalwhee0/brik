# NsDefaults: Namespace Declaration Injection

NsDefaults provides automatic injection of missing namespace declarations into HTML documents, enabling the use of namespace-prefixed elements (like `svg:rect`) without manually adding `xmlns:` attributes.

## Overview

When working with namespaced content in HTML (particularly SVG or custom XML vocabularies), elements must reference declared namespace prefixes. NsDefaults automatically injects missing namespace declarations into the `<html>` tag, ensuring that your prefixed elements are properly recognized.

## The Problem

Consider this HTML with a prefixed SVG element:

```html
<html>
<body>
  <svg:rect width="100" height="50"/>
</body>
</html>
```

Without a namespace declaration, the `svg:` prefix is meaningless, and parsers won't recognize `<svg:rect>` as an SVG element. You would need to manually add:

```html
<html xmlns:svg="http://www.w3.org/2000/svg">
  ...
</html>
```

This becomes tedious when:

- Generating HTML programmatically
- Working with template systems
- Processing documents that use multiple namespaces
- Converting between different document formats

## The Solution

NsDefaults solves this by:

1. Parsing the HTML to locate the `<html>` tag
2. Detecting which namespace declarations are already present
3. Injecting only the missing declarations
4. Providing efficient integration with html5ever's parser

## Basic Usage

```rust
use brik::ns::NsDefaultsBuilder;
use brik::parse_html;

// HTML with prefixed elements but no xmlns declarations
let html = r#"<html><body><svg:rect width="100"/></body></html>"#;

// Configure which namespaces should be present
let ns_defaults = NsDefaultsBuilder::new()
    .namespace("svg", "http://www.w3.org/2000/svg")
    .from_string(html)?;

// Parse with the injected namespaces
let document = parse_html().from_iter(ns_defaults);
```

The resulting document will have `xmlns:svg="http://www.w3.org/2000/svg"` automatically added to the `<html>` tag.

## API

### NsDefaultsBuilder

The builder pattern separates namespace configuration from HTML processing:

```rust
use brik::ns::NsDefaultsBuilder;

let builder = NsDefaultsBuilder::new()
    .namespace("svg", "http://www.w3.org/2000/svg")
    .namespace("custom", "http://example.com/ns");
```

#### `new()` → NsDefaultsBuilder

Creates an empty builder with no registered namespaces.

#### `namespace(prefix, uri)` → NsDefaultsBuilder

Registers a namespace prefix mapping. Parameters:

- `prefix`: The namespace prefix (e.g., "svg", "custom")
- `uri`: The namespace URI or anything convertible to `Namespace`

Returns `self` for method chaining. If the same prefix is registered multiple times, the last registration wins.

#### `from_string(html)` → Result\<NsDefaults\>

Processes the HTML string and returns an `NsDefaults` instance. Errors if the HTML cannot be parsed or lacks an `<html>` tag.

### NsDefaults

The result of processing HTML with a builder. Provides three consumption paths:

#### Into\<String\>

Allocates and returns the complete modified HTML:

```rust
let html_string: String = ns_defaults.into();
```

#### From\<NsDefaults\> for StrTendril

Converts to html5ever's string type for use with `.one()`:

```rust
use html5ever::tendril::StrTendril;

let tendril: StrTendril = ns_defaults.into();
let document = parse_html().one(tendril);
```

#### IntoIterator (Recommended)

Yields string slices for efficient parsing with `.from_iter()`:

```rust
// Most efficient: no intermediate string allocation in NsDefaults
let document = parse_html().from_iter(ns_defaults);
```

## Advanced Usage

### Multiple Namespaces

Register as many namespaces as needed:

```rust
let ns_defaults = NsDefaultsBuilder::new()
    .namespace("svg", "http://www.w3.org/2000/svg")
    .namespace("math", "http://www.w3.org/1998/Math/MathML")
    .namespace("custom", "http://example.com/app")
    .from_string(html)?;
```

Declarations are added in alphabetical order by prefix for deterministic output.

### Existing Declarations

NsDefaults only adds missing declarations. If your HTML already has some namespace declarations, they won't be duplicated:

```rust
let html = r#"<html xmlns:svg="http://www.w3.org/2000/svg">
  <body><custom:widget/></body>
</html>"#;

// Only custom: will be added; svg: is already present
let ns_defaults = NsDefaultsBuilder::new()
    .namespace("svg", "http://www.w3.org/2000/svg")  // Already present
    .namespace("custom", "http://example.com/ns")     // Will be added
    .from_string(html)?;
```

Result:

```html
<html xmlns:svg="http://www.w3.org/2000/svg" xmlns:custom="http://example.com/ns">
```

### Overwriting Registrations

If you register the same prefix multiple times, the last value wins:

```rust
let ns_defaults = NsDefaultsBuilder::new()
    .namespace("svg", "http://example.com/wrong")
    .namespace("svg", "http://www.w3.org/2000/svg")  // This one is used
    .from_string(html)?;
```

### Using with html5ever Macros

Combine with html5ever's namespace macros:

```rust
use html5ever::ns;

let ns_defaults = NsDefaultsBuilder::new()
    .namespace("svg", ns!(svg))    // Uses html5ever's built-in SVG URI
    .namespace("html", ns!(html))
    .namespace("mathml", ns!(mathml))
    .from_string(html)?;
```

## Architecture

NsDefaults uses a **slice-based design** for efficiency:

1. **Parse Phase**: The HTML is parsed to locate the `<html>` tag and find the insertion point (just before the closing `>`).

2. **Storage Phase**: The original HTML is stored unchanged along with:
   - Tag position information (`HtmlTagInfo`)
   - The namespace declarations to add (as a single string)

3. **Output Phase**: When consumed, the HTML is provided in slices:
   - HTML before insertion point
   - Namespace declarations to add
   - HTML after insertion point

This design avoids unnecessary string allocations until the result is actually needed.

### Performance

The three consumption paths have different performance characteristics:

| Method | Allocation | Use Case |
|--------|-----------|----------|
| `IntoIterator` | None | Parsing with html5ever (recommended) |
| `Into<StrTendril>` | Single allocation | Parsing with `.one()` |
| `Into<String>` | Single allocation | String output needed |

For the best performance when feeding to html5ever, use the `IntoIterator` path:

```rust
let document = parse_html().from_iter(ns_defaults);  // Zero-copy from NsDefaults
```

**Note:** While `IntoIterator` avoids intermediate allocations in NsDefaults, html5ever's parser still copies the underlying string data into its internal representation. The performance benefit is that NsDefaults doesn't create an unnecessary intermediate concatenated string before feeding to the parser.

## Integration Examples

### Template Processing

Inject namespaces into template-generated HTML:

```rust
use brik::ns::NsDefaultsBuilder;
use brik::parse_html;
use brik::traits::*;

fn process_template(template_html: &str) -> Result<String, Box<dyn Error>> {
    // Inject template namespace
    let ns_defaults = NsDefaultsBuilder::new()
        .namespace("tmpl", "http://example.com/template")
        .from_string(template_html)?;

    // Parse and process
    let doc = parse_html().from_iter(ns_defaults);

    // Process template directives...

    Ok(doc.to_string())
}
```

### SVG with Custom Attributes

Combine standard SVG namespace with custom application namespace:

```rust
let html = r#"<html>
<body>
  <svg:svg width="200" height="100">
    <svg:rect app:id="widget-1" width="180" height="80"/>
  </svg:svg>
</body>
</html>"#;

let ns_defaults = NsDefaultsBuilder::new()
    .namespace("svg", "http://www.w3.org/2000/svg")
    .namespace("app", "http://example.com/app")
    .from_string(html)?;

let doc = parse_html().from_iter(ns_defaults);
```

### Document Format Conversion

When converting from other formats to HTML:

```rust
fn xml_to_html(xml: &str, namespaces: &[(String, String)])
    -> Result<Document, Box<dyn Error>>
{
    let mut builder = NsDefaultsBuilder::new();

    // Register all namespaces from the source format
    for (prefix, uri) in namespaces {
        builder = builder.namespace(prefix, uri.as_str());
    }

    let ns_defaults = builder.from_string(xml)?;
    Ok(parse_html().from_iter(ns_defaults))
}
```

## Common Patterns

### Reusable Builder

Create a configured builder for repeated use:

```rust
fn create_svg_builder() -> NsDefaultsBuilder {
    NsDefaultsBuilder::new()
        .namespace("svg", "http://www.w3.org/2000/svg")
        .namespace("xlink", "http://www.w3.org/1999/xlink")
}

// Use with multiple documents
let doc1 = parse_html().from_iter(
    create_svg_builder().from_string(html1)?
);
let doc2 = parse_html().from_iter(
    create_svg_builder().from_string(html2)?
);
```

### Display for Debugging

Use the `Display` implementation to see the processed HTML:

```rust
let ns_defaults = NsDefaultsBuilder::new()
    .namespace("svg", "http://www.w3.org/2000/svg")
    .from_string(html)?;

println!("Processed HTML:\n{}", ns_defaults);
```

### No-Op Processing

An empty builder passes HTML through unchanged:

```rust
let ns_defaults = NsDefaultsBuilder::new()
    .from_string(html)?;

// No namespaces added; HTML remains unchanged
assert_eq!(html, ns_defaults.to_string());
```

## Error Handling

`from_string()` returns `Result<NsDefaults, NsError>` and can fail if:

- The HTML cannot be parsed
- No `<html>` tag is found in the document

```rust
use brik::ns::{NsDefaultsBuilder, NsError};

let result = NsDefaultsBuilder::new()
    .namespace("svg", "http://www.w3.org/2000/svg")
    .from_string(invalid_html);

match result {
    Ok(ns_defaults) => {
        // Process successfully
    }
    Err(NsError::ParseError(msg)) => {
        eprintln!("Failed to parse HTML: {}", msg);
    }
}
```

## Limitations

### HTML Tag Required

NsDefaults requires an `<html>` tag in the document. Fragments without an `<html>` tag will fail:

```rust
// This will error - no <html> tag
let result = NsDefaultsBuilder::new()
    .from_string("<div>Hello</div>");

assert!(result.is_err());
```

### Attribute Preservation

All existing attributes on the `<html>` tag are preserved:

```html
<html lang="en" class="no-js">
```

After processing:

```html
<html lang="en" class="no-js" xmlns:svg="http://www.w3.org/2000/svg">
```

### Declaration Location

Namespace declarations are always added to the `<html>` tag, never to descendant elements. This follows XML best practices of declaring namespaces at the root element.

### Prefix-Only Checking

NsDefaults only checks if a prefix is already declared, not whether the URI matches. If `xmlns:svg` exists with a different URI, NsDefaults won't add it again or warn about the mismatch.

## See Also

- [namespaces.md](namespaces.md) - Comprehensive guide to namespace support in brik
- [examples/ns_defaults.rs](../examples/ns_defaults.rs) - Working example with integration test
- [html5ever documentation](https://docs.rs/html5ever/) - Parser integration details
