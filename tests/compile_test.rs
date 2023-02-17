#[test]
fn compile_test() {
    let test = trybuild::TestCases::new();
    test.pass("tests/compile_pass/*.rs");
    test.compile_fail("tests/compile_fail/*.rs");
}
