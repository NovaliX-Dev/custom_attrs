# Custom Attrs

[![Building](https://github.com/NovaliX-Dev/custom_attrs/actions/workflows/build.yml/badge.svg)](https://github.com/NovaliX-Dev/custom_attrs/actions/workflows/build.yml)
[![Crates.io](https://img.shields.io/crates/v/custom_attrs.svg)](https://crates.io/crates/custom_attrs)
[![License](https://img.shields.io/crates/l/custom_attrs.svg)](./LICENSE)
[![Documentation](https://docs.rs/custom_attrs/badge.svg)](https://docs.rs/custom_attrs)

A library that allows you to configure values specific to each variants of an enum.

## Features
- Optional attributes
- Default values

## Installation

Add this to your `Cargo.toml` file :
```toml
[dependencies]
custom_attrs = "1.2.2"
```

## Examples

```Rust
use custom_attrs::CustomAttrs;

// ...

#[derive(CustomAttrs)]

// set the attributes
#[attr(pub a: usize)]
#[attr(b: Option<usize>)] // attributes can be optional
#[attr(c: &'static str = "Hello world!")] // and can also have default values
enum Enum {
    #[attr(a = 5)]
    #[attr(b = 3)]
    Variant1,

    #[attr(a = 3)]
    #[attr(c = "Hello again !")]
    Variant2,
}

fn main() {
    let a = Enum::Variant1.get_a();
}
```

See the examples directory for more details.

# License

Licensed under the MIT license.
