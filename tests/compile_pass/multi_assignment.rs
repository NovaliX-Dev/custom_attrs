use custom_attrs::CustomAttrs;

#[derive(CustomAttrs)]
#[attr(pub a: usize)]
#[attr(b: u32)]
enum Enum {
    #[attr(
        a = 5,
        b = 2
    )]
    Variant1,

    #[attr(a = 3)]
    #[attr(b = 3)]
    Variant2
}

fn main() {
    let _a = Enum::Variant1.get_a();
    let _a = Enum::Variant1.get_b();
}