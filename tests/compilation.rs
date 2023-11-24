#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/enum_tuple.rs");
    t.compile_fail("tests/compile_fail/missing_attribute.rs");
    t.compile_fail("tests/compile_fail/newtype_struct.rs");
    t.compile_fail("tests/compile_fail/tuple_struct.rs");
    t.compile_fail("tests/compile_fail/unwrap_or_cant_be_used_with_into_and_from.rs");
    t.compile_fail("tests/compile_fail/unwrap_or_cant_be_used_with_with_fn.rs");
    t.compile_fail("tests/compile_fail/unwrap_or_cant_use_both.rs");
    t.compile_fail("tests/compile_fail/with_fn_cant_use_both.rs");
    t.compile_fail("tests/compile_fail/wrong_unwrap_or_cant_be_used.rs");
}
