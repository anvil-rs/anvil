use anvil_tera::Earth;
use anvil_tera_derive::Template;
use serde::Serialize;
use std::sync::LazyLock;
use tera::{Tera, Value, to_value, Result as TeraResult};

// Custom filter function for uppercase
fn uppercase(value: &Value, _: &std::collections::HashMap<String, Value>) -> TeraResult<Value> {
    let s = value.as_str().unwrap_or_default();
    Ok(to_value(s.to_uppercase()).unwrap())
}

// Set up a static Tera instance for testing
static TEMPLATES: LazyLock<Tera> = LazyLock::new(|| {
    let mut tera = Tera::default();
    // Get the template file path
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let template_path = manifest_dir.join("tests/templates");
    
    // Add the test templates
    tera.add_template_file(template_path.join("test.txt"), Some("test.txt")).unwrap();
    tera.add_template_file(template_path.join("conditional.txt"), Some("conditional.txt")).unwrap();
    tera.add_template_file(template_path.join("override.txt"), Some("override.txt")).unwrap();
    tera.add_template_file(template_path.join("filter.txt"), Some("filter.txt")).unwrap();
    
    // Register the uppercase filter
    tera.register_filter("uppercase", uppercase);
    
    tera
});

// Create a test template struct with the Template derive macro
#[derive(Serialize, Template)]
#[template(path = "test.txt", tera = TEMPLATES)]
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
        .tera(&mut buffer)
        .expect("Failed to render template");

    // Verify the output
    let output = String::from_utf8(buffer).expect("Output was not valid UTF-8");
    assert!(
        output.contains("Hello, John!"),
        "Template output doesn't match expected content"
    );
}

// The tests below verify error handling in the derive macro

// Test compile error for missing path attribute
// This will cause a compile error: "Missing 'path' attribute within #[template(...)]"
#[test]
fn test_missing_path_attribute() {
    // This test will only verify that the code compiles correctly
    // The actual compile error will be tested during build time via a separate file

    // Intentionally commented out to prevent actual compile errors:
    // #[derive(Serialize, Template)]
    // #[template(tera = TEMPLATES)]  // Missing path attribute
    // struct MissingPathTemplate {
    //     value: String,
    // }
}

// Test compile error for missing tera attribute
// This will cause a compile error: "Missing 'tera' attribute within #[template(...)]"
#[test]
fn test_missing_tera_attribute() {
    // This test will only verify that the code compiles correctly
    // The actual compile error will be tested during build time via a separate file

    // Intentionally commented out to prevent actual compile errors:
    // #[derive(Serialize, Template)]
    // #[template(path = "test.txt")]  // Missing tera attribute
    // struct MissingTeraTemplate {
    //     value: String,
    // }
}

// Test compile error for incorrect template attribute format
// This will cause a compile error: "Expected #[template(...)] attribute list format"
#[test]
fn test_incorrect_template_format() {
    // This test will only verify that the code compiles correctly
    // The actual compile error will be tested during build time via a separate file

    // Intentionally commented out to prevent actual compile errors:
    // #[derive(Serialize, Template)]
    // #[template = "test.txt"]  // Using incorrect format
    // struct IncorrectFormatTemplate {
    //     value: String,
    // }
}

// Test template with conditional statements
#[derive(Serialize, Template)]
#[template(path = "conditional.txt", tera = TEMPLATES)]
struct ConditionalTemplate {
    name: String,
    show_greeting: bool,
}

#[test]
fn test_conditional_template() {
    // Test with greeting shown
    let template_with_greeting = ConditionalTemplate {
        name: String::from("Jane"),
        show_greeting: true,
    };

    let mut buffer = Vec::new();
    template_with_greeting
        .tera(&mut buffer)
        .expect("Failed to render template with greeting");

    let output = String::from_utf8(buffer).expect("Output was not valid UTF-8");
    assert!(
        output.contains("Hello, Jane!"),
        "Template output doesn't match expected content with greeting"
    );

    // Test with greeting hidden
    let template_without_greeting = ConditionalTemplate {
        name: String::from("Jane"),
        show_greeting: false,
    };

    let mut buffer = Vec::new();
    template_without_greeting
        .tera(&mut buffer)
        .expect("Failed to render template without greeting");

    let output = String::from_utf8(buffer).expect("Output was not valid UTF-8");
    assert!(
        output.contains("Welcome!"),
        "Template output doesn't match expected content without greeting"
    );
}

// Test template with context overrides
#[derive(Serialize)]
struct ManualOverrideTemplate {
    name: String,
    #[serde(skip)]
    additional_context: Option<tera::Context>,
}

impl ManualOverrideTemplate {
    fn with_override(name: &str, greeting: &str) -> Self {
        let mut context = tera::Context::new();
        context.insert("override_greeting", &true);
        context.insert("custom_greeting", greeting);
        
        Self {
            name: name.to_string(),
            additional_context: Some(context),
        }
    }
    
    fn without_override(name: &str) -> Self {
        Self {
            name: name.to_string(),
            additional_context: None,
        }
    }
}

// Implement Earth trait with context overrides
impl Earth for ManualOverrideTemplate {
    fn tera(&self, writer: &mut (impl std::io::Write + ?Sized)) -> tera::Result<()> {
        let mut context = tera::Context::from_serialize(self)
            .map_err(|e| tera::Error::msg(format!("Failed to serialize context: {}", e)))?;
            
        // Add additional context if provided
        if let Some(ref additional) = self.additional_context {
            for (key, value) in additional.clone().into_json().as_object().unwrap() {
                context.insert(key, value);
            }
        }
        
        TEMPLATES.render_to("override.txt", &context, writer)
    }
}

#[test]
fn test_context_override() {
    // Test with custom greeting
    let template_with_override = ManualOverrideTemplate::with_override("Alice", "Greetings");

    let mut buffer = Vec::new();
    template_with_override
        .tera(&mut buffer)
        .expect("Failed to render template with override");

    let output = String::from_utf8(buffer).expect("Output was not valid UTF-8");
    assert!(
        output.contains("Greetings, Alice!"),
        "Template output doesn't match expected content with override"
    );

    // Test without custom greeting
    let template_without_override = ManualOverrideTemplate::without_override("Alice");

    let mut buffer = Vec::new();
    template_without_override
        .tera(&mut buffer)
        .expect("Failed to render template without override");

    let output = String::from_utf8(buffer).expect("Output was not valid UTF-8");
    assert!(
        output.contains("Hello, Alice!"),
        "Template output doesn't match expected content without override"
    );
}

// Test template with filter
#[derive(Serialize, Template)]
#[template(path = "filter.txt", tera = TEMPLATES)]
struct FilterTemplate {
    name: String,
}

#[test]
fn test_template_with_filter() {
    // Setup the test template
    let template = FilterTemplate {
        name: String::from("John"),
    };

    // Render the template to a buffer
    let mut buffer = Vec::new();
    template
        .tera(&mut buffer)
        .expect("Failed to render template with filter");

    // Verify the output
    let output = String::from_utf8(buffer).expect("Output was not valid UTF-8");
    assert!(
        output.contains("JOHN"),
        "Template output doesn't match expected content with uppercase filter"
    );
}
