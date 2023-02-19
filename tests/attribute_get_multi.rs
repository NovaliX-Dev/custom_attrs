use custom_attrs::CustomAttrs;

#[derive(CustomAttrs)]
#[attr(
    pub a: usize = 8,
    b: usize
)]
enum Enum {
    #[attr(a = 5, b = 2)]
    Variant1,

    #[attr(a = 3)]
    #[attr(b = 3)]
    Variant2,

    #[attr(b = 4)]
    Variant3,
}

#[derive(CustomAttrs)]
#[attr(pub a: Option<usize>)]
#[attr(pub b: Option<usize>)]
enum Enum2 {
    #[attr(a = 5, b = 3)]
    Variant1,

    Variant2,

    #[attr(a = None)]
    #[attr(b = None)]
    Variant3,
}

#[test]
fn test_attribute_get() {
    assert_eq!(Enum::Variant1.get_a(), 5);
    assert_eq!(Enum::Variant2.get_a(), 3);

    assert_eq!(Enum::Variant1.get_b(), 2);
    assert_eq!(Enum::Variant2.get_b(), 3);
    assert_eq!(Enum::Variant3.get_b(), 4);
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

    assert_eq!(Enum2::Variant1.get_b(), Some(3));
    assert!(Enum2::Variant3.get_b().is_none());
}
