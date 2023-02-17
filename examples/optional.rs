use custom_attrs::CustomAttrs;

#[derive(CustomAttrs)]
#[attr(pub a: Option<usize> = Some(3))]
enum Enum {
    Variant1,

    #[attr(a = 3)]
    Variant2,
}

fn main() {
    let a = Enum::Variant1.get_a();

    assert!(a.is_none());
}
