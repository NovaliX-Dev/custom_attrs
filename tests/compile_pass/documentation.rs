use custom_attrs::CustomAttrs;

#[derive(CustomAttrs)]
#[attr(
    /// Get attribute a which represent a value
    pub a: usize,

    /// Get attribute b which represent another value
    #[doc = r"represent another value"]
    pub b: u32 = 3
)]
enum Enum {
    #[attr(a = 5)]
    Variant1,

    #[attr(a = 3)]
    Variant2,
}

fn main() {
    let _a = Enum::Variant1.get_a();
    let _b = Enum::Variant1.get_b();
}