use custom_attrs::CustomAttrs;

#[derive(CustomAttrs)]
#[attr(pub a: usize)]
#[attr(pub b: Option<u32>)]
enum Enum {
    #[attr(a = *#self.0)]
    #[attr(b = *#self.1)]
    Variant1(usize, u32),

    #[attr(a = *#self.field2)]
    #[attr(b = *#self.field1)]
    Variant2 {
        #[allow(unused)]
        field1: u32,

        #[allow(unused)]
        field2: usize,
    },
}

const VARIANT1: Enum = Enum::Variant1(0, 1);
const VARIANT2: Enum = Enum::Variant2 {
    field1: 2,
    field2: 4,
};

fn main() {
    let _a = Enum::Variant1(0, 1).get_a();
    let _b = Enum::Variant2 {
        field1: 2,
        field2: 4,
    }
    .get_a();
}

#[test]
fn test_attribute_get() {
    assert_eq!(VARIANT1.get_a(), 0);
    assert_eq!(VARIANT2.get_a(), 4);
}

#[test]
fn test_attribute_option_get() {
    assert_eq!(VARIANT1.get_b(), Some(1));
    assert_eq!(VARIANT2.get_b(), Some(2));
}
