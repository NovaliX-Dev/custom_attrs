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

#[derive(CustomAttrs)]
#[attr(pub a: usize)]
#[attr(b: u32)]
enum Enum2 {
    #[attr(
        a = 5,
        b = *#self.0
    )]
    Variant1(u32),

    #[attr(a = *#self.field1, b = 3)]
    Variant2 {
        field1: usize
    }
}

fn main() {
    let _a = Enum::Variant1.get_a();
    let _a = Enum::Variant1.get_b();
}