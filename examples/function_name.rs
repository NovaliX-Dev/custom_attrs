#![allow(unused)]
use custom_attrs::CustomAttrs;

#[derive(CustomAttrs)]
#[attr(
    #[function = "function_name"]
    pub a: usize
)]
enum Enum {
    #[attr(a = 5)]
    Variant1,

    #[attr(a = 3)]
    Variant2,
}

fn main() {
    let a = Enum::Variant1.function_name();
}
