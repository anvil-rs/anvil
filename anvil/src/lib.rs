#![doc(
    html_logo_url = "https://raw.githubusercontent.com/sjcobb2022/anvil/refs/heads/main/.github/assets/ANVIL.png"
)]
//! # Anvil
//!
//! Anvil is a modular templating system for creating user-defined scaffolding systems.
//! It provides a composable API for file operations like generating, appending, transforming,
//! and moving files.
//!
//! ## Core Concepts
//!
//! Anvil is built around two primary traits:
//!
//! * [`Anvil`] - The base trait for template rendering engines
//! * [`Forge`] - The base trait for file operations using rendered templates
//!
//! Think of `Anvil` as the template you render, and `Forge` as what you do with that rendered content
//! (create a file, append to a file, transform a file, etc.).
//!
//! ## Design Philosophy
//!
//! - **Configuration is code**: Your scaffolding logic is defined directly in code, enabling
//!   compile-time checking and integration with your application.
//! - **Compile time errors are better than runtime errors**: Detect issues at compile time whenever possible.
//! - **The library provides the building blocks, not the solutions**: Anvil gives you composable
//!   components to build your own custom scaffolding systems.
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use anvil::{Anvil, Forge, generate::Generate};
//! use std::io::Write;
//!
//! // Simple implementation of the Anvil trait
//! struct SimpleTemplate {
//!     content: String,
//! }
//!
//! impl Anvil for SimpleTemplate {
//!     type Error = std::io::Error;
//!
//!     fn anvil(&self, writer: &mut (impl Write + Sized)) -> Result<(), Self::Error> {
//!         writer.write_all(self.content.as_bytes())?;
//!         Ok(())
//!     }
//! }
//!
//! // Using Generate for file creation
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let template = SimpleTemplate {
//!         content: "Hello, Anvil!".to_string(),
//!     };
//!     
//!     // Create a file generator using our template
//!     let generator = Generate::new(template);
//!     
//!     // Generate the file
//!     generator.forge("./output.txt")?;
//!     
//!     println!("File generated successfully!");
//!     Ok(())
//! }
//! ```
//!
//! ## Available Operations
//!
//! Anvil provides these main operations:
//!
//! - [`generate`] - Create new files from templates
//! - [`append`] - Add content to existing files
//! - [`transform`] - Transform the content of existing files
//! - [`mover`] - Move/rename files
//! - [`either`] - Fallback mechanism for operations
//!
//! These operations can be composed to create complex scaffolding workflows.
//!
//! ## Inspiration and Credits
//!
//! - [Laravel Artisan](https://laravel.com/docs/11.x/artisan)
//! - [Rails Generators](https://guides.rubyonrails.org/generators.html)
//! - [Loco.rs](https://loco.rs/docs/getting-started/tour/#adding-a-crud-api)
//! - [Cargo Generate](https://github.com/cargo-generate/cargo-generate)
//! - [Cookiecutter actix simple clean architecture](https://github.com/microsoft/cookiecutter-rust-actix-clean-architecture)
//!

/// Module for appending content to existing files.
///
/// The operations in this module allow you to add content to the end of existing files
/// using the [`Append`](`append::Append`) struct.
///
/// # Example
///
/// ```rust,no_run
/// use anvil::{Anvil, Forge, append::Append};
/// use std::io::Write;
///
/// // Simple template that renders fixed content
/// struct SimpleTemplate {
///     content: String,
/// }
///
/// impl Anvil for SimpleTemplate {
///     type Error = std::io::Error;
///
///     fn anvil(&self, writer: &mut (impl Write + Sized)) -> Result<(), Self::Error> {
///         writer.write_all(self.content.as_bytes())?;
///         Ok(())
///     }
/// }
///
/// // Create a template and append it to a file
/// fn append_to_file() -> Result<(), Box<dyn std::error::Error>> {
///     let template = SimpleTemplate {
///         content: "\nAppended content".to_string(),
///     };
///     
///     let append_op = Append::new(template);
///     append_op.forge("./existing_file.txt")?;
///     
///     Ok(())
/// }
/// ```
pub mod append;

/// Module for fallback mechanisms between two operations.
///
/// This module provides the [`Either`](`either::Either`) struct for creating fallback operations - if the first
/// operation fails, the second one will be attempted.
///
/// # Example
///
/// ```
/// use anvil::{Anvil, Forge, generate::Generate, either::{either, Either}};
/// use std::io::Write;
///
/// // Helper trait implementation (defined elsewhere)
/// struct TemplateA;
/// struct TemplateB;
///
/// impl Anvil for TemplateA {
///     type Error = std::io::Error;
///     fn anvil(&self, writer: &mut (impl Write + Sized)) -> Result<(), Self::Error> {
///         writer.write_all(b"Content from TemplateA")?;
///         Ok(())
///     }
/// }
///
/// impl Anvil for TemplateB {
///     type Error = std::io::Error;
///     fn anvil(&self, writer: &mut (impl Write + Sized)) -> Result<(), Self::Error> {
///         writer.write_all(b"Content from TemplateB")?;
///         Ok(())
///     }
/// }
///
/// // Try to use TemplateA, but fall back to TemplateB if it fails
/// fn create_with_fallback() -> Result<(), Box<dyn std::error::Error>> {
///     let primary_generator = Generate::new(TemplateA);
///     let fallback_generator = Generate::new(TemplateB);
///     
///     // Create an Either operation with both generators
///     let operation = either(primary_generator, fallback_generator);
///     
///     // Execute the operation with fallback
///     // Using a fake path for doc testing
///     let result = operation.forge("fake_path.txt");
///     // Just show we can handle the result
///     if result.is_ok() {
///         println!("Operation succeeded!");
///     }
///     
///     Ok(())
/// }
/// ```
pub mod either;

/// Module for creating files from templates.
///
/// This module provides the [`Generate`](`generate::Generate`) struct for generating new files from templates.
/// The file and parent directories will be created if they don't exist.
///
/// # Example
///
/// ```rust,no_run
/// use anvil::{Anvil, Forge, generate::Generate};
/// use std::io::Write;
///
/// // Simple template implementation
/// struct SimpleTemplate {
///     content: String,
/// }
///
/// impl Anvil for SimpleTemplate {
///     type Error = std::io::Error;
///
///     fn anvil(&self, writer: &mut (impl Write + Sized)) -> Result<(), Self::Error> {
///         writer.write_all(self.content.as_bytes())?;
///         Ok(())
///     }
/// }
///
/// // Generate a new file
/// fn generate_file() -> Result<(), Box<dyn std::error::Error>> {
///     let template = SimpleTemplate {
///         content: "# New File\n\nThis is a generated file.".to_string(),
///     };
///     
///     let generator = Generate::new(template);
///     generator.forge("./path/to/new_file.md")?;
///     
///     Ok(())
/// }
/// ```
pub mod generate;

/// Module for moving or renaming files.
///
/// This module provides the [`Move`](`mover::Move`) struct for moving or renaming files.
///
/// # Example
///
/// ```rust,no_run
/// use anvil::{Forge, mover::Move};
///
/// // Rename a file from old name to new name
/// fn rename_file() -> Result<(), Box<dyn std::error::Error>> {
///     let mover = Move::new("./old_name.txt");
///     mover.forge("./new_name.txt")?;
///     
///     Ok(())
/// }
/// ```
pub mod mover;

/// Module for transforming the content of existing files.
///
/// This module provides the [`Transform`](`transform::Transform`) struct for reading, modifying,
/// and writing back file content.
///
/// # Example
///
/// ```rust,no_run
/// use anvil::{Forge, transform::Transform};
///
/// // Transform a file by adding a line to the beginning
/// fn add_header_comment() -> Result<(), Box<dyn std::error::Error>> {
///     let transform = Transform::new(|content| {
///         Ok(format!("// Generated file - do not edit directly\n{}", content))
///     });
///     
///     transform.forge("./src/generated.rs")?;
///     
///     Ok(())
/// }
/// ```
pub mod transform;

use std::{error::Error, path::Path};

/// The core trait for template rendering engines.
///
/// `Anvil` provides the foundational functionality for rendering templates into strings
/// or writing them directly to files. Any type that implements this trait can be used
/// with the file operation types (Generate, Append, etc).
///
/// # Type Parameters
///
/// * `Error` - The error type that the rendering engine can produce.
///
/// # Implementation
///
/// When implementing this trait, you should provide the rendering logic in the `anvil` method,
/// which writes the rendered template to the provided writer.
///
/// # Examples
///
/// ```
/// use anvil::Anvil;
/// use std::io::Write;
///
/// struct SimpleTemplate {
///     content: String,
/// }
///
/// impl Anvil for SimpleTemplate {
///     type Error = std::io::Error;
///
///     fn anvil(&self, writer: &mut (impl Write + Sized)) -> Result<(), Self::Error> {
///         writer.write_all(self.content.as_bytes())?;
///         Ok(())
///     }
/// }
/// ```
pub trait Anvil {
    /// The error type that this anvil implementation can produce.
    type Error: Error + 'static;

    /// Renders the template to the provided writer.
    ///
    /// # Parameters
    ///
    /// * `writer` - A mutable reference to any type that implements `std::io::Write`.
    ///   The rendered content will be written to this writer.
    ///
    /// # Returns
    ///
    /// * `Result<(), Self::Error>` - Ok if rendering was successful, Err otherwise.
    fn anvil(&self, writer: &mut (impl std::io::Write + Sized)) -> Result<(), Self::Error>;
}

/// The core trait for file operations.
///
/// `Forge` represents operations that can create, modify, or transform files.
/// It defines a uniform interface for all file manipulation actions in the Anvil system.
///
/// # Type Parameters
///
/// * `Error` - The error type that this operation can produce.
///
/// # Implementation
///
/// When implementing this trait, you should provide the file operation logic in the `forge` method,
/// which typically creates or modifies a file at the specified path.
///
/// # Examples
///
/// ```
/// use anvil::Forge;
/// use std::path::Path;
///
/// // A simple file creator
/// struct EmptyFileCreator;
///
/// impl Forge for EmptyFileCreator {
///     type Error = std::io::Error;
///
///     fn forge(&self, into: impl AsRef<Path>) -> Result<(), Self::Error> {
///         let path = into.as_ref();
///         std::fs::File::create(path)?;
///         Ok(())
///     }
/// }
/// ```
pub trait Forge {
    /// The error type that this forge implementation can produce.
    type Error: Error;

    /// Performs a file operation using the provided path.
    ///
    /// # Parameters
    ///
    /// * `into` - A reference to a path where the operation should be performed.
    ///   This could be a target file to create/modify, or a destination for moving files.
    ///
    /// # Returns
    ///
    /// * `Result<(), Self::Error>` - Ok if the operation was successful, Err otherwise.
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

#[cfg(test)]
mod tests {
    use super::*;

    // Simple implementation of the Anvil trait for testing
    struct SimpleAnvil {
        content: String,
    }

    impl Anvil for SimpleAnvil {
        type Error = std::io::Error;

        fn anvil(&self, writer: &mut (impl std::io::Write + Sized)) -> Result<(), Self::Error> {
            writer.write_all(self.content.as_bytes())?;
            Ok(())
        }
    }

    // Simple implementation of the Forge trait for testing
    struct SimpleForge {
        error: Option<std::io::Error>,
    }

    impl Forge for SimpleForge {
        type Error = std::io::Error;

        fn forge(&self, _into: impl AsRef<Path>) -> Result<(), Self::Error> {
            if let Some(ref err) = self.error {
                Err(std::io::Error::new(err.kind(), err.to_string()))
            } else {
                Ok(())
            }
        }
    }

    #[test]
    fn test_anvil_trait_implementation() {
        let anvil = SimpleAnvil {
            content: "Hello, world!".to_string(),
        };

        let mut buffer = Vec::new();
        let result = anvil.anvil(&mut buffer);

        assert!(result.is_ok());
        assert_eq!(String::from_utf8(buffer).unwrap(), "Hello, world!");
    }

    #[test]
    fn test_forge_trait_successful_implementation() {
        let forge = SimpleForge { error: None };
        let result = forge.forge("/tmp/test.txt");
        assert!(result.is_ok());
    }

    #[test]
    fn test_forge_trait_error_implementation() {
        let error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let forge = SimpleForge { error: Some(error) };
        let result = forge.forge("/tmp/nonexistent.txt");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::NotFound);
    }

    #[test]
    fn test_integration_of_anvil_and_forge() {
        // Test that our forge can use an anvil
        // We'll test this with a real implementation from another module
        use crate::generate::Generate;
        use tempfile::tempdir;

        // Create a temporary directory for testing
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("output.txt");

        // Create our anvil
        let anvil = SimpleAnvil {
            content: "Generated by combined anvil and forge".to_string(),
        };

        // Create the generate forge which uses our anvil
        let generate = Generate::new(anvil);

        // Use the forge
        let result = generate.forge(&file_path);
        assert!(result.is_ok());

        // Verify the file was created with the correct content
        assert!(file_path.exists());
        let content = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "Generated by combined anvil and forge");
    }
}
