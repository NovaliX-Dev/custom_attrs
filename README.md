Custom Attrs

A library that allows you to configure values specific to each variants of an enum.

## Features
- Optional attributes
- Default values

## Examples

```
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