#![doc(
    html_logo_url = "https://raw.githubusercontent.com/sjcobb2022/anvil/refs/heads/main/.github/assets/ANVIL.png"
)]
//! Anvil is a modular templating system for creating user-defined scaffolding systems
//!
//! # Ethos
//!
//! - Configuration is code.
//! - Compile time errors are better than runtime errors.
//! - The library provides the building blocks, not the solutions.
//!
//! # Inspiration and Credits
//!
//! - [Laravel Artisan](https://laravel.com/docs/11.x/artisan)
//! - [Rails Generators](https://guides.rubyonrails.org/generators.html)
//! - [Loco.rs](https://loco.rs/docs/getting-started/tour/#adding-a-crud-api)
//! - [Cargo Generate](https://github.com/cargo-generate/cargo-generate)
//! - [Cookiecutter actix simple clean architecture](https://github.com/microsoft/cookiecutter-rust-actix-clean-architecture)
//!
//! # Example
//! ```no_run
//! use anvil::{append::*, either::*, generate::*, Anvil};
//! use askama::Template;
//! use regex::Regex;
//!
//! #[derive(Template)]
//! #[template(source="controller.rs", ext="txt")]
//! struct ExampleTemplate;
//!
//! let controller = &ExampleTemplate;
//! either(append(controller), generate(controller))
//!     .forge("src/controllers/mod.rs")
//!     .unwrap();
//! ```

/// Appending content to a file.
pub mod append;

/// Rendering either of two templates. If the first fails, the second is rendered.
pub mod either;

/// Additional askama filters.
pub mod filters;

/// Creating a file from a template.
pub mod generate;

/// Injecting content into a file.
pub mod inject;

/// Replacing content in a file
pub mod replace;

use std::{error::Error, path::Path};

/// Anvil is the base trait for all rendering engines.
/// It provides the basic functionality to render a template into a string or write it to a file.
/// The error is the error that the rendering engine can produce.
pub trait Anvil {
    type Error: Error;
    fn forge(&self, into: impl AsRef<Path>) -> Result<(), Self::Error>;
}

// A scaffold is a collection of generation steps.
// for a large scaffold type, we generally want these to be defined by the user (or the template)
// and then we can just run them in order.
// We need to define a construct that runs something in order?
// I guess a function works to some extent.

// TODO: Implement other types like in the other branch. Make type with const reference to template
// for dynamic templating like tera or minijinja.
//
// Implementation for tera:
// Have a const reference to templates such that we can use templates
// between crates.
// Then reference the template statically inside the rendering function.
//
//
// A different approach:
//  - File operations are a trait. These are defined as append, replace, inject, generate etc.
//  - File operations are composed into scaffolds.
//  - Scaffolds are run in order.
//  - Scaffolds are defined in a file.
//  - Scaffolds are defined in a file and then run in order.
// pub trait Forge {
//     type Error: Error;
//     fn forge(&self, ) -> Result<(), Self::Error>;
//     fn describe(&self) -> String;
// }
//
// Forge is defined for the scaffolds themselves.
//
// Forge implemented for Append, Generate, Transform etc.
// Anvil implemented for askama, tera etc.
//
// We implement forge for any type that generates anvil.
//
// Anvil is defined for the file operations. And templates
//
//
// // Create a new file
// pub struct Add<T: Template> {
//     path: PathBuf,
//     template: T,
// }
//
// // Delete a file
// pub struct Remove {
//     path: PathBuf,
// }
//
// // Rename/move a file
// pub struct Move {
//     from: PathBuf,
//     to: PathBuf,
// }
//
// // General-purpose transformer for file content
// pub struct Transform {
//     path: PathBuf,
//     transformer: Box<dyn Fn(String) -> Result<String, Error>>,
// }
//
//
//
// // Util
// pub struct Either {}
//


struct Template {
    name: String,
}

// impl Forge for Template {
//     fn forge() -> Result<(), String> {
//         MY_TEMPLATES.render
//     }
// }
