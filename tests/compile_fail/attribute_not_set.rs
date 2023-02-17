use custom_attrs::CustomAttrs;

#[derive(CustomAttrs)]
#[attr(pub a: usize)]
enum Enum {
    Variant1,

    #[attr(a = 3)]
    Variant2
}

fn main() {
    // let _a = Enum::Variant1.get_a();
}