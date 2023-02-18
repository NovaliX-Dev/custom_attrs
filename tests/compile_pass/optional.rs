#![allow(unused)]
use custom_attrs::CustomAttrs;

#[derive(CustomAttrs)]
#[attr(pub a: Option<usize> = 3)]
#[attr(pub b: Option<usize> = Some(3))]
#[attr(pub c: Option<usize> = std::option::Option::Some(4))]
#[attr(pub d: Option<usize> = core::option::Option::Some(4))]
enum Enum {
    Variant1,

    #[attr(a = 3)]
    Variant2,

    #[attr(a = None)]
    Variant3,

    #[attr(a = std::option::Option::None)]
    Variant4,

    #[attr(a = core::option::Option::None)]
    Variant5,

    #[attr(a = Some(4))]
    Variant6,

    #[attr(a = std::option::Option::Some(4))]
    Variant7,

    #[attr(a = core::option::Option::Some(4))]
    Variant8,
}

fn main() {
    let a = Enum::Variant1.get_a();

    assert_eq!(a.unwrap(), 3);
    assert_eq!(Enum::Variant3.get_a(), None);
    assert_eq!(Enum::Variant8.get_a(), Some(4));
}

