use custom_attrs::CustomAttrs;

#[derive(CustomAttrs)]
#[attr(pub a: usize)]
enum Enum {
    #[attr(a = #self.5)]
    Variant1(usize, u32),

    #[attr(a = #self.unknown)]
    Variant2 {
        #[allow(unused)]
        field1: u32,

        #[allow(unused)]
        field2: usize,
    },
}

fn main() {}
