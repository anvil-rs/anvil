// This file tests the error when a template is missing the required template attribute
// Expected error: "Missing #[template(\"template_name.ext\")] or #[template(path = \"...\")] attribute"

use anvil_liquid_derive::Template;
use serde::Serialize;

#[derive(Serialize, Template)]  // Missing #[template("...")] attribute
struct MissingAttributeTemplate {
    value: String,
}

fn main() {
    // This will never run, we're only testing the compile-time error
}
