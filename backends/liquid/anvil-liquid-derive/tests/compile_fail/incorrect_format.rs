// This file tests the error when a template has an incorrectly formatted template attribute
// Expected error: "Expected template attribute to be in the form #[template(\"template_name.ext\")] or #[template(path = \"...\", ...)]"

use anvil_liquid_derive::Template;
use serde::Serialize;

#[derive(Serialize, Template)]
#[template(123)]  // Using a number instead of a string literal
struct IncorrectFormatTemplate {
    value: String,
}

fn main() {
    // This will never run, we're only testing the compile-time error
}
