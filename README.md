# Custom Attrs

[![Build](https://github.com/NovaliX-Dev/custom_attrs/actions/workflows/build.yml/badge.svg)](https://github.com/NovaliX-Dev/custom_attrs/actions/workflows/build.yml)
[![Tests](https://github.com/NovaliX-Dev/custom_attrs/actions/workflows/tests.yml/badge.svg)](https://github.com/NovaliX-Dev/custom_attrs/actions/workflows/tests.yml)
[![Crates.io](https://img.shields.io/crates/v/custom_attrs.svg)](https://crates.io/crates/custom_attrs)
[![License](https://img.shields.io/crates/l/custom_attrs.svg)](./LICENSE)
[![Documentation](https://docs.rs/custom_attrs/badge.svg)](https://docs.rs/custom_attrs)

A library that allows you to configure attributes with values specific to each variants of an enum.

## Installation and Usage

Add this to your `Cargo.toml` file :
```toml
[dependencies]
custom_attrs = "1.5"
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
#[attr(pub attribute: u32)]
enum Enum {}
```

By default, each attribute declared require a value to be set for  each variant.
If this requirement is not set, the library will produce an error.

You can disable this behavior by making it optional, by writing type into an `Option`, or by adding a default value behind the attribute declaration. See the example below.

```rust
#[attr(attribute: Option<u32>)]
enum Enum {}
```

```rust
#[attr(attribute: u32 = 3)]
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
    #[attr(attribute = 4)]
    VariantA
}
```

Like declarations, you can set many values at once.

```rust
#[attr(
    attr1: usize,
    attr2: usize
)]
enum Enum {
    #[attr(
        attr1 = 4,
        attr2 = 1
    )]
    VariantA
}
```

If the attribute is optional, you don't have to wrap it in a `Some`. `custom_attrs` will do this for you. If you want the value to be `None`, just put `None` behind the it.

```rust
#[attr(optional: Option<usize>)]
enum Enum {
    #[attr(optional = 4)]
    VariantA,

    #[attr(optional = None)]
    VariantB,

    // you can still wrap the value into an option
    #[attr(optional = Some(5))]
    VariantC,
}
```

#### Self References

In attribute values you set, you can add a reference to a field of the variant.

The syntax is the following :

```rust
#[attr(name: usize)]
enum Enum {
    /// Use the name of the field if it's named
    #[attr(name = #self.field)]
    Variant {
        field: usize
    },

    /// Otherwise use it's position
    #[attr(name = #self.0)]
    Variant2(usize)
}
```

Self references are processed before the value is parsed as expression, so you can use them anywhere you need :

```rust
enum Enum {
    #[attr(a = #self.list[*#self.index])]
    Variant3 {
        list: [usize; 4],
        index: usize,
    },
}
```

Please note that the value returned a **reference** ! To deref it, just add a `*` before the syntax, like so :

```rust
#[attr(name = *#self.<field>)]
```

### Attribute properties

You can add properties to attributes to defines their characteristics. Even the documentation, in itself, is a property.

The syntax of a property is the following :
```rust, ignore
#[attr(
    #[<config_name> = <value>]
    <attribute>: <type>
)]
```

Configs can also be flags: 
```rust, ignore
#[attr(
    #[<config_name>]
    <attribute>: <type>
)]
```

Like attributes, you can define many properties in one bloc:
```rust, ignore
#[attr(
    #[<config_name>, <config_name2> = <value>]
    <attribute>: <type>
])
```

Here is a list of all the properties :
- `function` : defines the name of the function to get the attribute

### Getting a value attribute

To get the value from a variant, simple call `get_<attribute name>` or the name
/// you've set in the properties of the attributes.

```rust
Element::VariantA.get_a();
```

If you've set a documentation on the attribute, it will be shown on this function.

## Examples

```rust
use custom_attrs::CustomAttrs;

// ...

#[derive(CustomAttrs)]

#[attr(
    #[function = "a_getter"]
    pub a: usize
)]
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
    Variant3,

    /// You can access fields of the variant
    #[attr(a = *#self.field)]
    Variant4 {
        field: usize
    }
}

fn main() {
    Enum::Variant1.a_getter(); // custom getter name
    Enum::Variant2.get_b(); // default getter name
}
```

See the examples directory for more details.

# Features

- `help_span` : Merge main error and help notes, while these have they own span, and do not produce a separated error.
This features is nightly only.

# License

Licensed under the MIT license.
