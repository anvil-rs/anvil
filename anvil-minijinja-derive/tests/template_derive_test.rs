
use anvil_minijinja::Shrine;
use anvil_minijinja_derive::Template;
use serde::Serialize;

// Create a test template struct with the Template derive macro
#[derive(Serialize, Template)]
#[template("test.txt")]
struct TestTemplate {
    name: String,
}

// Test the template rendering
#[test]
fn test_template_derive() {
    // Setup the test template
    let template = TestTemplate {
        name: String::from("John"),
    };
    
    // Render the template to a buffer
    let mut buffer = Vec::new();
    template.minijinja(&mut buffer).expect("Failed to render template");
    
    // Verify the output
    let output = String::from_utf8(buffer).expect("Output was not valid UTF-8");
    assert!(output.contains("Hello, John!"), "Template output doesn't match expected content");
}
