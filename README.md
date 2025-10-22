# Kuchikiki (口利き)

[![Documentation](https://docs.rs/kuchikiki/badge.svg)](https://docs.rs/kuchikiki)
[![Crates.io](https://img.shields.io/crates/v/kuchikiki.svg)](https://crates.io/crates/kuchikiki)
[![License](https://img.shields.io/crates/l/kuchikiki.svg)](https://github.com/theroyalwhee0/kuchikiki/blob/main/LICENSE)

A Rust library for parsing, manipulating, and querying HTML documents using CSS selectors.

## About

This is a fork of the [Kuchiki (朽木)](https://github.com/kuchiki-rs/kuchiki) library, which is now unmaintained. The Brave project continues to maintain this fork as an active, supported alternative.

**Kuchikiki** means "intermediary" or "mediation" in Japanese (口利き), reflecting its role in bridging HTML parsing and manipulation.

## Features

- ✨ **Full HTML5 parsing** via [html5ever](https://github.com/servo/html5ever)
- 🎯 **CSS selector queries** for finding elements
- 🌳 **Tree manipulation** - append, prepend, insert, detach nodes
- 🔍 **Node inspection** - traverse ancestors, siblings, descendants
- 📝 **Serialization** - convert trees back to HTML
- 🛡️ **Optional safe mode** - build without unsafe code
- ⚡ **Performance optimizations** - optional bloom filter for selectors

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
kuchikiki = "0.8"
```

### Migrating from Kuchiki

If you're migrating from the original `kuchiki` crate, update your `Cargo.toml` (add an extra `ki`!):

```toml
[dependencies]
kuchikiki = "0.8"  # Changed from "kuchiki"
```

Then remap code references:

```rust
use kuchikiki as kuchiki;
```

## Quick Start

```rust
use kuchikiki::parse_html;
use kuchikiki::traits::*;

// Parse HTML and query with CSS selectors
let document = parse_html().one("<p class='greeting'>Hello, world!</p>");
let greeting = document.select_first(".greeting").unwrap();
println!("{}", greeting.text_contents());
```

For more detailed examples, see the [examples](examples/) directory.

## Feature Flags

### Safe Mode

By default, kuchikiki uses unsafe code for performance. To build without any unsafe blocks:

```toml
[dependencies]
kuchikiki = { version = "0.8", features = ["safe"] }
```

Or via command line:

```bash
cargo build --features safe
cargo test --features safe
```

**Note:** This only affects kuchikiki's code, not its dependencies.

### Bloom Filter Optimization

Enable bloom filter optimization for faster CSS selector matching:

```toml
[dependencies]
kuchikiki = { version = "0.8", features = ["bloom-filter"] }
```

## Documentation

Full API documentation is available at [docs.rs/kuchikiki](https://docs.rs/kuchikiki).

## Examples

Run examples with:

```bash
cargo run --example quickstart
cargo run --example find_matches
```

See the [examples](examples/) directory for all available examples.

## Security

See the [Security Policy](SECURITY.md) for information on reporting vulnerabilities.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Licensed under the MIT license. See [LICENSE](LICENSE) for details.

## Credits

- Original Kuchiki library by Simon Sapin
- Maintained by the Brave Authors
- Current maintainer: Adam Mill (@theroyalwhee0)
