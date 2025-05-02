// This file tests the error when a template is missing the required path attribute
// Expected error: "Missing 'path' attribute within #[template(...)]"

use anvil_tera_derive::Template;
use serde::Serialize;
use std::sync::LazyLock;
use tera::Tera;

static TEMPLATES: LazyLock<Tera> = LazyLock::new(|| Tera::default());

#[derive(Serialize, Template)]
#[template(tera = TEMPLATES)]  // Missing path attribute
struct MissingPathTemplate {
    value: String,
}

fn main() {
    // This will never run, we're only testing the compile-time error
}
