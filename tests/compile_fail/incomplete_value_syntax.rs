#![allow(unused)]
use custom_attrs::CustomAttrs;

#[derive(CustomAttrs)]
#[attr(pub a: usize)]
#[attr(pub b: usize)]
enum Enum {
    #[attr(a = b =)]
    #[attr(b)]
    Variant1,

    // valid
    #[attr(a = 3)]
    #[attr(b = 3)]
    Variant2,
    
    #[attr(a = )]
    #[attr(b = 3)]
    Variant3,
}

fn main() {
    // let a = Enum::Variant1.get_a();
}
