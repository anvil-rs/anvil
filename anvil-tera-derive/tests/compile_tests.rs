// This file runs the compile-fail tests using trybuild
// It verifies that our derive macro properly handles error cases

#[cfg(feature = "compile-tests")]
mod tests {
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
        t.compile_fail(dir.join("missing_path.rs"));
        t.compile_fail(dir.join("missing_tera.rs"));
        t.compile_fail(dir.join("incorrect_format.rs"));
    }
}

// When compile-tests feature is not enabled, provide a dummy test
#[cfg(not(feature = "compile-tests"))]
#[test]
fn dummy_test() {
    // This test always passes
    // The real compile failure tests require the compile-tests feature to be enabled
    println!("Skipping compile failure tests (enable with --features compile-tests)");
}
