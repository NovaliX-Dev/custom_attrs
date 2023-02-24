#![allow(unused)]
use custom_attrs::CustomAttrs;

#[derive(CustomAttrs)]
#[attr(pub a: usize)]
enum Enum {
    #[attr(a = 5)]
    Variant1(usize, u32),

    #[attr(a = 3)]
    Variant2 {
        field1: u32,
        field2: usize
    },
}

fn main() {
    let a = Enum::Variant1(0, 1).get_a();
}
