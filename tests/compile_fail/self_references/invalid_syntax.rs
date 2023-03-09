use custom_attrs::CustomAttrs;

#[derive(CustomAttrs)]
#[attr(pub a: usize)]
enum Enum {
    #[attr(a = #)]
    Variant1(usize, u32),

    #[attr(a = #self)]
    Variant3(usize, u32),

    #[attr(a = #self.)]
    Variant4(usize, u32),

    // valid
    #[attr(a = #self.0)]
    Variant5(usize, u32),

    // valid
    #[attr(a = #self.field1)]
    Variant6 {
        field1: u32
    },
}

fn main() {}
