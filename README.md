# Custom Attrs

[![Build](https://github.com/NovaliX-Dev/custom_attrs/actions/workflows/build.yml/badge.svg)](https://github.com/NovaliX-Dev/custom_attrs/actions/workflows/build.yml)
[![Tests](https://github.com/NovaliX-Dev/custom_attrs/actions/workflows/tests.yml/badge.svg)](https://github.com/NovaliX-Dev/custom_attrs/actions/workflows/tests.yml)
[![Crates.io](https://img.shields.io/crates/v/custom_attrs.svg)](https://crates.io/crates/custom_attrs)
[![License](https://img.shields.io/crates/l/custom_attrs.svg)](./LICENSE)
[![Documentation](https://docs.rs/custom_attrs/badge.svg)](https://docs.rs/custom_attrs)

A library that allows you to configure values specific to each variants of an enum.

## Installation and Usage

Add this to your `Cargo.toml` file :
```toml
[dependencies]
custom_attrs = "1.4"
```

Then you can use the `derive` attribute to use the library.

```rust
use custom_attrs::CustomAttrs;

#[derive(CustomAttrs)]

// all attributes declarations will be here.

enum Enum {
    // ...
}
```

### Attribute declaration.

By default, an attribute declaration is composed of two parts : an attribute's name and it's type.

```rust
#[attr(name: u32)]
enum Enum {}
```

You can declare many attribute declarations in a single `attr`.

```rust
#[attr(
    name: u32,
    name2: usize
)]
enum Enum {}
```

Optionally, you can add more components.

### Optional components

You can set the visibility of the attribute. This will change the visibility of the getter function.

```rust
#[attr(pub name: u32)]
enum Enum {}
```

By default, each attribute declared require a value to be set for  each variant.
If this requirement is not set, the library will produce an error.

You can disable this behavior by making it optional, by writing type into an `Option`, or by adding a default value behind the attribute declaration. See the example below.

```rust
#[attr(name: Option<u32>)]
enum Enum {}
```

```rust
#[attr(name: u32 = 3)]
enum Enum {}
```

You can add documentation avoid declared attributes. This documentation will override the one of the getter function.

```rust
#[attr(
    /// Attribute documentation
    attribute: u32
)]
enum Enum {}
```

### Setting a value

To set a value for a variant, just add the name of the attribute followed by the value you want to set.


```rust
enum Enum {
    #[attr(name = 4)]
    VariantA
}
```

Like declarations, you can set many values at once.

```rust
enum Enum {
    #[attr(
        name = 4,
        name2 = 1
    )]
    VariantA
}
```

If the attribute is optional, you don't have to wrap it in a `Some`. `custom_attrs` will do this for you. If you want the value to be `None`, just put `None` behind the it.

```rust
enum Enum {
    #[attr(name = 4)]
    VariantA,

    #[attr(name = None)]
    VariantB,

    // you can still wrap the value into an option
    #[attr(name = Some(5))]
    VariantC,
}
```

### Getting a value attribute

To get the value from a variant, simple call `get_<attribute name>`.

```rust
Element::VariantA.get_a();
```

If you've set a documentation on the attribute, it will be shown on this function.

## Examples

```rust
use custom_attrs::CustomAttrs;

// ...

#[derive(CustomAttrs)]

#[attr(pub a: usize)]
#[attr(b: Option<usize>)]
#[attr(c: &'static str = "Hello world!")]
enum Enum {
    #[attr(a = 5)]
    #[attr(b = 3)]
    Variant1,

    #[attr(a = 3)]
    #[attr(c = "Hello again !")]
    Variant2,

    #[attr(
        a = 1,
        b = 5,
        c = "Hello for the last time !"
    )]
    Variant3
}
```

See the examples directory for more details.

# Features

- `help_span` : Merge main error and help notes, while these have they own span, and do not produce a separated error.
This features is nightly only.

# License

Licensed under the MIT license.