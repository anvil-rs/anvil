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

/// Appending content to a file.
pub mod append;

/// Rendering either of two templates. If the first fails, the second is rendered.
pub mod either;

/// Creating a file from a template.
pub mod generate;

/// Moving or renaming a file.
pub mod mover;

/// Transforming the content of a file.
pub mod transform;

use std::{error::Error, path::Path};

/// Anvil is the base trait for all rendering engines.
/// It provides the basic functionality to render a template into a string or write it to a file.
/// The error is the error that the rendering engine can produce.
pub trait Anvil {
    type Error: Error + 'static;
    fn anvil(&self, writer: &mut (impl std::io::Write + Sized)) -> Result<(), Self::Error>;
}

/// Forge is the base trait for all scaffolding operations.
/// It provides the basic functionality to create a scaffold from a template.
/// The error is the error that the operation can produce.
///
/// How we define our forge determines how we use our anvil.
pub trait Forge {
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
// // Append utility which is just an extension of add, but adding using append mode.
// pub struct Append {
//     path: PathBuf,
//     template: T
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

//     fn forge() -> esult<(), String> {
//         MY_TEMPLATES.render
//     }
// }

// Anvil has fn anvil
// Forge has fn forge
//
// A forge needs an anvil to do it's work.
//
// Generate, Append, Transform are all Forgeable types.
//
// They require an anvil (a template for us to forge our directory)
//
// An askama template is an anvil type because we are able to implement
//
//
// // "Any good anvil should be used to forge"
