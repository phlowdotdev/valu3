#[test]
pub fn pass() {
    macrotest::expand("tests/derive_test.rs");
}
