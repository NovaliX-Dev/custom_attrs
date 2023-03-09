use custom_attrs::CustomAttrs;

#[derive(CustomAttrs)]
#[attr(pub a: usize)]
enum Enum {
    #[attr(a = 5)]
    #[attr(a = 5)]
    Variant1,

    #[attr(
        a = 3,
        a = 3
    )]
    Variant2
}

#[derive(CustomAttrs)]
#[attr(pub a: usize)]
enum Enum2 {
    #[attr(a = #self.0)]
    #[attr(a = #self.0)]
    Variant1(usize),

    #[attr(
        a = #self.field1,
        a = #self.field1
    )]
    Variant2 {
        field1: usize
    }
}


fn main() {
    // let _a = Enum::Variant1.get_a();
}