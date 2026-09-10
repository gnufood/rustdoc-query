#![allow(missing_docs)]

#[test]
fn internal_error_types_are_not_public() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/ui/private_error.rs");
    cases.compile_fail("tests/ui/private_error_module.rs");
}
