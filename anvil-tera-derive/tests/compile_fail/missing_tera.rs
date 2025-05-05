// This file tests the error when a template is missing the required tera attribute
// Expected error: "Missing 'tera' attribute within #[template(...)]"

use anvil_tera_derive::Template;
use serde::Serialize;

#[derive(Serialize, Template)]
#[template(path = "test.txt")]  // Missing tera attribute
struct MissingTeraTemplate {
    value: String,
}

fn main() {
    // This will never run, we're only testing the compile-time error
}
