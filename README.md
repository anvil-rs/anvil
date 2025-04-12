# Anvil

Anvil is a modular templating system for creating user-defined scaffolding systems. It provides a composable API for file operations like generating, appending, transforming, and moving files.

## Core Concepts

Anvil is built around two primary traits:

* `Anvil` - The base trait for template rendering engines
* `Forge` - The base trait for file operations using rendered templates

Think of `Anvil` as the template you render, and `Forge` as what you do with that rendered content (create a file, append to a file, transform a file, etc.).

## Design Philosophy

- **Configuration is code**: Your scaffolding logic is defined directly in code, enabling compile-time checking and integration with your application.
- **Compile time errors are better than runtime errors**: Detect issues at compile time whenever possible.
- **The library provides the building blocks, not the solutions**: Anvil gives you composable components to build your own custom scaffolding systems.

## Example Usage

```rust
use anvil::{Anvil, Forge, generate::Generate};
use std::io::Write;

// Simple implementation of the Anvil trait
struct SimpleTemplate {
    content: String,
}

impl Anvil for SimpleTemplate {
    type Error = std::io::Error;

    fn anvil(&self, writer: &mut (impl Write + Sized)) -> Result<(), Self::Error> {
        writer.write_all(self.content.as_bytes())?;
        Ok(())
    }
}

// Using Generate for file creation
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let template = SimpleTemplate {
        content: "Hello, Anvil!".to_string(),
    };
    
    // Create a file generator using our template
    let generator = Generate::new(template);
    
    // Generate the file
    generator.forge("./output.txt")?;
    
    println!("File generated successfully!");
    Ok(())
}
```

## Inspiration and Credits

- [Laravel Artisan](https://laravel.com/docs/11.x/artisan)
- [Rails Generators](https://guides.rubyonrails.org/generators.html)
- [Loco.rs](https://loco.rs/docs/getting-started/tour/#adding-a-crud-api)
- [Cargo Generate](https://github.com/cargo-generate/cargo-generate)
- [Cookiecutter actix simple clean architecture](https://github.com/microsoft/cookiecutter-rust-actix-clean-architecture)
