#[test]
pub fn pass() {
    macrotest::expand("tests/*.rs");
}
