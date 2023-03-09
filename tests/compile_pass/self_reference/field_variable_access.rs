use custom_attrs::CustomAttrs;

struct A {
    b: usize
}

#[derive(CustomAttrs)]
#[attr(pub a: usize)]
enum Enum {
    #[attr(a = #self.0.b)]
    Variant1(A, u32),

    #[attr(a = #self.field1.b)]
    Variant2 {
        #[allow(unused)]
        field1: A,

        #[allow(unused)]
        field2: usize,
    },
}

fn main() {}
