use custom_attrs::CustomAttrs;

#[derive(CustomAttrs)]
#[attr(
    pub a: u32,
    pub b: Option<u32>
)]
enum Enum {
    #[attr(a = #self.list[*#self.index])]
    Variant {
        #[allow(unused)]
        list: [u32; 4],

        #[allow(unused)]
        index: usize,
    },
}

fn main() {
    let _b = Enum::Variant {
        list: [0, 1, 2, 3],
        index: 2,
    }
    .get_a();
}
