# Brik

[![Documentation](https://docs.rs/brik/badge.svg)](https://docs.rs/brik)
[![Crates.io](https://img.shields.io/crates/v/brik.svg)](https://crates.io/crates/brik)
[![License](https://img.shields.io/crates/l/brik.svg)](https://github.com/theroyalwhee0/brik/blob/main/LICENSE)

A Rust library for parsing, manipulating, and querying HTML documents using CSS selectors.

## About

This is a fork of [Kuchiki (朽木)](https://github.com/kuchiki-rs/kuchiki) by way of Brave's [Kuchikiki fork](https://github.com/brave/kuchikiki). The original Kuchiki is now unmaintained.

**Brik** is a building block for HTML manipulation - simple, solid, and stackable.

This fork maintains compatibility with current servo crates (html5ever, selectors, etc.) and provides ongoing updates. See the [Changelog](CHANGELOG.md) for version history and updates.

## Features

- ✨ **Full HTML5 parsing** via [html5ever](https://github.com/servo/html5ever)
- 🎯 **CSS selector queries** via [selectors](https://github.com/servo/stylo/tree/main/selectors)
- 🌳 **Tree manipulation** - append, prepend, insert, detach nodes
- 🔍 **Node inspection** - traverse ancestors, siblings, descendants
- 📝 **Serialization** - convert trees back to HTML
- 🛡️ **Optional safe mode** - build without unsafe code

## Installation

> ⚠️ **Status: Alpha** - Passes all tests but not yet recommended for production use.

Add this to your `Cargo.toml`:

```toml
[dependencies]
brik = "0.8"
```

### Migrating from Kuchiki or Kuchikiki

This migration applies to both Kuchiki and Kuchikiki.

```toml
[dependencies]
brik = "0.8"  # Changed from "kuchiki" or "kuchikiki"
```

Update your code:

```rust
use brik::parse_html;  // Changed from kuchiki or kuchikiki
use brik::traits::*;
```

## Quick Start

```rust
use brik::parse_html;
use brik::traits::*;

// Parse HTML and query with CSS selectors
let document = parse_html().one("<p class='greeting'>Hello, world!</p>");
let greeting = document.select_first(".greeting").unwrap();
println!("{}", greeting.text_contents());
```

For more detailed examples, see the [examples](examples/) directory.

## Feature Flags

### Safe Mode

By default, brik uses unsafe code for performance. To build without any unsafe blocks:

```toml
[dependencies]
brik = { version = "0.8", features = ["safe"] }
```

Or via command line:

```bash
cargo build --features safe
cargo test --features safe
```

**Note:** This only affects brik's code, not its dependencies.

## Documentation

Full API documentation is available at [docs.rs/brik](https://docs.rs/brik).

## Examples

Run examples with:

```bash
cargo run --example quickstart
cargo run --example find_matches
```

See the [examples](examples/) directory for all available examples.

## Security

See the [Security Policy](SECURITY.md) for information on reporting vulnerabilities.

## Credits

This project builds on the work of:

- **Original Kuchiki library**: [kuchiki-rs/kuchiki](https://github.com/kuchiki-rs/kuchiki) by Simon Sapin
- **Brave fork**: [brave/kuchikiki](https://github.com/brave/kuchikiki) maintained by the Brave Authors and Ralph Giles

**Brik** is maintained by Adam Mill ([@theroyalwhee0](https://github.com/theroyalwhee0))

## Contributing

Contributions are welcome! Please see our [Contributing Guidelines](CONTRIBUTING.md) and [Code of Conduct](CODE_OF_CONDUCT.md) before submitting a Pull Request.

## License

Licensed under the MIT license. See [LICENSE](LICENSE) for details.
