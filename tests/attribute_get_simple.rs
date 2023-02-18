use custom_attrs::CustomAttrs;

#[derive(CustomAttrs)]
#[attr(pub a: usize = 8)]
enum Enum {
    #[attr(a = 5)]
    Variant1,

    #[attr(a = 3)]
    Variant2,

    Variant3,
}

#[derive(CustomAttrs)]
#[attr(pub a: Option<usize>)]
enum Enum2 {
    #[attr(a = 5)]
    Variant1,

    Variant2,

    #[attr(a = None)]
    Variant3,
}

#[test]
fn test_attribute_get() {
    assert_eq!(Enum::Variant1.get_a(), 5);
    assert_eq!(Enum::Variant2.get_a(), 3);
}

#[test]
fn test_attribute_default() {
    assert_eq!(Enum::Variant3.get_a(), 8)
}

#[test]
fn test_attribute_options() {
    assert_eq!(Enum2::Variant1.get_a(), Some(5));
    assert!(Enum2::Variant2.get_a().is_none());
    assert!(Enum2::Variant3.get_a().is_none());
}
