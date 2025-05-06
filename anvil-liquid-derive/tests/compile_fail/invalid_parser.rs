// This file tests the error when a template has an invalid parser expression
// Expected error: "Parser expression must be a valid identifier"

use anvil_liquid_derive::Template;
use serde::Serialize;

#[derive(Serialize, Template)]
#[template(path = "test.txt", parser = 123)]  // Using a number instead of an identifier
struct InvalidParserTemplate {
    value: String,
}

fn main() {
    // This will never run, we're only testing the compile-time error
}
