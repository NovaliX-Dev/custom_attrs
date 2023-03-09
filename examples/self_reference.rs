use custom_attrs::CustomAttrs;

#[derive(CustomAttrs)]
#[attr(pub a: usize)]
enum Enum {
    #[attr(a = *#self.0)]
    Variant1(usize, u32),

    #[attr(a = *#self.field2)]
    Variant2 {
        #[allow(unused)]
        field1: u32,

        field2: usize,
    },

    #[allow(unused)]
    #[attr(a = #self.list[*#self.index])]
    Variant3 { list: [usize; 4], index: usize },
}

fn main() {
    let _a = Enum::Variant1(0, 1).get_a();
    let _b = Enum::Variant2 {
        field1: 2,
        field2: 4,
    }
    .get_a();
}
