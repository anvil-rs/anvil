// This file tests the error when the path attribute is specified but no value is given
// Expected error: "Expected path to be a string literal"

use anvil_liquid_derive::Template;
use serde::Serialize;

#[derive(Serialize, Template)]
#[template(path)]  // Missing path value
struct MissingPathValueTemplate {
    value: String,
}

fn main() {
    // This will never run, we're only testing the compile-time error
}
