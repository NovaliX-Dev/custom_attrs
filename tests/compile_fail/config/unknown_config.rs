#![allow(unused)]
use custom_attrs::CustomAttrs;

#[derive(CustomAttrs)]
#[attr(
    /// Should pass

    #[unknown = "", another_unknown = ""]
    #[unknown2]
    
    pub a: usize
)]
enum Enum {
    #[attr(a = 5)]
    Variant1,

    #[attr(a = 3)]
    Variant2,
}

fn main() {
    // let a = Enum::Variant1.get_a();
}
