// This file runs the compile-fail tests using trybuild
// It verifies that our derive macro properly handles error cases

use std::path::PathBuf;

#[test]
#[cfg_attr(miri, ignore)]
fn compile_tests() {
    let t = trybuild::TestCases::new();

    // Path to our compile-fail tests
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("compile_fail");

    // Add all error case tests in the compile_fail directory
    t.compile_fail(dir.join("missing_attribute.rs"));
    t.compile_fail(dir.join("incorrect_format.rs"));
    t.compile_fail(dir.join("missing_path_value.rs"));
}
