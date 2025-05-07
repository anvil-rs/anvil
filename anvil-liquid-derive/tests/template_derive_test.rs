use anvil_liquid::Water;
use anvil_liquid_derive::Template;
use liquid::ParserBuilder;
use serde::Serialize;
use std::sync::LazyLock;

static PARSER: LazyLock<liquid::Parser> =
    LazyLock::new(|| ParserBuilder::with_stdlib().build().unwrap());

// Test with simple string literal format
#[derive(Serialize, Template)]
#[template("tests/templates/test.txt")]
struct SimpleTemplate {
    name: String,
}

// Test with key-value format without parser
#[derive(Serialize, Template)]
#[template(path = "tests/templates/test.txt")]
struct KeyValueTemplate {
    name: String,
}

// Test with key-value format with custom parser
#[derive(Serialize, Template)]
#[template(path = "tests/templates/test.txt", parser = PARSER)]
struct CustomParserTemplate {
    name: String,
}

#[test]
fn test_simple_template() {
    let template = SimpleTemplate {
        name: "World".to_string(),
    };

    let mut buf = Vec::new();
    template.liquid(&mut buf).unwrap();
    let result = String::from_utf8(buf).unwrap();
    assert_eq!(result, "Hello, World!");
}

#[test]
fn test_key_value_template() {
    let template = KeyValueTemplate {
        name: "World".to_string(),
    };

    let mut buf = Vec::new();
    template.liquid(&mut buf).unwrap();
    let result = String::from_utf8(buf).unwrap();
    assert_eq!(result, "Hello, World!");
}

#[test]
fn test_custom_parser_template() {
    let template = CustomParserTemplate {
        name: "World".to_string(),
    };

    let mut buf = Vec::new();
    template.liquid(&mut buf).unwrap();
    let result = String::from_utf8(buf).unwrap();
    assert_eq!(result, "Hello, World!");
}
