use custom_attrs::CustomAttrs;

#[derive(CustomAttrs)]
#[attr(pub a: usize)]
#[attr(pub a: usize)]
enum Enum {
    #[attr(a = 3)]
    Variant1,

    #[attr(a = 3)]
    Variant2
}

#[derive(CustomAttrs)]
#[attr(
    pub b: Option<usize>,
    pub b: Option<usize>,
)]
enum Enum2 {
    Variant1,

    #[attr(a = 3)]
    Variant2
}

fn main() {
    // let _a = Enum::Variant1.get_a();
}