use anvil_minijinja::Shrine;
use anvil_minijinja_derive::Template;
use serde::Serialize;

// Create a test template struct with the Template derive macro
#[derive(Serialize, Template)]
#[template("test.txt")]
struct TestTemplate {
    name: String,
}

// Write a basic test to verify the derive macro works correctly
#[test]
fn test_template_derive() {
    // Setup the test template
    let template = TestTemplate {
        name: String::from("John"),
    };

    // Render the template to a buffer
    let mut buffer = Vec::new();
    template
        .minijinja(&mut buffer)
        .expect("Failed to render template");

    // Verify the output
    let output = String::from_utf8(buffer).expect("Output was not valid UTF-8");
    assert!(
        output.contains("Hello, John!"),
        "Template output doesn't match expected content"
    );
}

// The tests below verify error handling in the derive macro

// Test compile error for missing template attribute
// This will cause a compile error: "Missing #[template(\"template_name.ext\")] attribute"
#[test]
fn test_missing_template_attribute() {
    // This test will only verify that the code compiles correctly
    // The actual compile error will be tested during build time via a separate file

    // Intentionally commented out to prevent actual compile errors:
    // #[derive(Serialize, Template)]
    // struct MissingAttributeTemplate {
    //     value: String,
    // }
}

// Test compile error for incorrect template attribute format
// This will cause a compile error: "Expected template attribute to be in the form #[template(\"template_name.ext\")]"
#[test]
fn test_incorrect_template_format() {
    // This test will only verify that the code compiles correctly
    // The actual compile error will be tested during build time via a separate file

    // Intentionally commented out to prevent actual compile errors:
    // #[derive(Serialize, Template)]
    // #[template(123)]  // Using a number instead of a string literal
    // struct IncorrectFormatTemplate {
    //     value: String,
    // }
}
