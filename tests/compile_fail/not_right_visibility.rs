mod module {
    use custom_attrs::CustomAttrs;

    #[derive(CustomAttrs)]
    #[attr(a: usize)]
    #[attr(pub b: usize)]
    pub enum Enum {
        #[attr(a = 3)]
        #[attr(b = 3)]
        Variant1,

        #[attr(a = 3)]
        #[attr(b = 5)]
        Variant2
    }
}

fn main() {
    let _a = module::Enum::Variant1.get_a();
    let _b = module::Enum::Variant1.get_b();
}