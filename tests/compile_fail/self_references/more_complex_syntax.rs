use custom_attrs::CustomAttrs;

#[derive(CustomAttrs)]
#[attr(pub a: usize)]
enum Enum {
    #[attr(a = #self.5)]
    Variant1(usize, u32),

    #[attr(a = #self.unknown)]
    Variant2 {
        #[allow(unused)]
        list: &[u32],

        #[allow(unused)]
        field2: usize,
    },
}

fn main() {
    let _a = Enum::Variant1(0, 1).get_a();
    let _b = Enum::Variant2 {
        field1: 2,
        field2: 4,
    }
    .get_a();
}
