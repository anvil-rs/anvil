// This file tests the error when a template has an incorrectly formatted template attribute
// Expected error: "Expected #[template(...)] attribute list format, e.g., #[template(path = \"...\")]"

use anvil_tera_derive::Template;
use serde::Serialize;

#[derive(Serialize, Template)]
#[template = "test.txt"]  // Using incorrect format
struct IncorrectFormatTemplate {
    value: String,
}

fn main() {
    // This will never run, we're only testing the compile-time error
}
