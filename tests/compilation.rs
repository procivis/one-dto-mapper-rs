#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/missing_attribute.rs");
    t.compile_fail("tests/compile_fail/newtype_struct.rs");
    t.compile_fail("tests/compile_fail/tuple_struct.rs");
    t.compile_fail("tests/compile_fail/enum_tuple.rs");
    t.compile_fail("tests/compile_fail/with_fn_cant_use_both.rs");
}
