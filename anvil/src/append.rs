use std::{io::BufWriter, path::Path};
use thiserror::Error;

use crate::{Anvil, Forge};

/// Errors that can occur during file append operations.
///
/// This enum represents the different types of errors that can occur when
/// appending content to a file using the [`Append`] struct.
#[derive(Error, Debug)]
pub enum AppendError {
    /// Error that occurred during file IO operations.
    #[error("failed to perform file I/O while appending content: {0}")]
    StdIo(#[from] std::io::Error),

    /// Error that occurred during template rendering.
    #[error("failed to render template during append operation: {0}")]
    Template(#[from] Box<dyn std::error::Error>),
}

/// A struct that appends template content to an existing file.
///
/// `Append` takes an [`Anvil`] implementation (template) and appends its rendered content
/// to an existing file. The file must already exist; if it doesn't, the operation will fail
/// with a "file not found" error.
///
/// # Examples
///
/// ```rust,no_run
/// use anvil::{Anvil, Forge, append::Append};
/// use std::io::Write;
///
/// // Define a simple template
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
/// // Create an append operation
/// let template = SimpleTemplate {
///     content: "\n\n// Appended content".to_string(),
/// };
///
/// let append = Append::new(template);
///
/// // Append to an existing file
/// match append.forge("./src/existing_file.rs") {
///     Ok(_) => println!("Content successfully appended"),
///     Err(e) => eprintln!("Failed to append: {}", e),
/// }
/// ```
pub struct Append<A: Anvil> {
    /// The template to render and append
    template: A,
}

impl<A: Anvil> Forge for Append<A> {
    type Error = AppendError;

    /// Appends the template content to an existing file.
    ///
    /// This method:
    /// 1. Opens the file in append mode
    /// 2. Renders the template
    /// 3. Appends the rendered content to the file
    ///
    /// # Parameters
    ///
    /// * `into` - Path to the file where content will be appended
    ///
    /// # Returns
    ///
    /// * `Result<(), AppendError>` - Ok if successful, or an error if:
    ///   - The file doesn't exist
    ///   - File permissions don't allow writing
    ///   - Template rendering fails
    ///
    /// # Errors
    ///
    /// Returns `AppendError::StdIo` if there's an IO error (like file not found),
    /// or `AppendError::Template` if there's an error during template rendering.
    fn forge(&self, into: impl AsRef<Path>) -> Result<(), Self::Error> {
        let path = into.as_ref();
        let file = std::fs::OpenOptions::new()
            .append(true)
            .open(path)
            .map_err(AppendError::StdIo)?;

        let mut writer = BufWriter::new(file);

        self.template
            .anvil(&mut writer)
            .map_err(|e| AppendError::Template(Box::new(e)))?;

        Ok(())
    }
}

impl<A: Anvil> Append<A> {
    /// Creates a new `Append` instance with the given template.
    ///
    /// # Parameters
    ///
    /// * `template` - An implementation of the [`Anvil`] trait that will be rendered
    ///   and appended to the target file.
    ///
    /// # Returns
    ///
    /// A new `Append` instance configured with the provided template.
    pub fn new(template: A) -> Self {
        Self { template }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::{tempdir, NamedTempFile};

    // Mock implementation for Anvil
    struct MockAnvil {
        content: String,
    }

    impl Anvil for MockAnvil {
        type Error = std::io::Error;

        fn anvil(&self, writer: &mut (impl std::io::Write + Sized)) -> Result<(), Self::Error> {
            writer.write_all(self.content.as_bytes())?;
            Ok(())
        }
    }

    #[test]
    fn test_append_adds_content_to_existing_file() {
        // Create a temporary file with some initial content
        let mut temp_file = NamedTempFile::new().unwrap();
        let initial_content = "Initial content\n";
        temp_file.write_all(initial_content.as_bytes()).unwrap();

        // Create the template content to append
        let template_content = "Appended content";
        let template = MockAnvil {
            content: template_content.to_string(),
        };

        // Create and use the Append
        let append = Append::new(template);
        let result = append.forge(temp_file.path());
        assert!(result.is_ok());

        // Read the file content and verify it was appended correctly
        let content = std::fs::read_to_string(temp_file.path()).unwrap();
        assert_eq!(content, format!("{}{}", initial_content, template_content));
    }

    #[test]
    fn test_append_fails_on_nonexistent_file() {
        // Create a path to a file that doesn't exist
        let nonexistent_path = tempdir().unwrap().path().join("nonexistent_file.txt");

        // Create a template
        let template = MockAnvil {
            content: "Some content".to_string(),
        };

        // Create and use the Append
        let append = Append::new(template);
        let result = append.forge(nonexistent_path);

        // Should fail with a file not found error
        assert!(result.is_err());

        match result {
            Err(AppendError::StdIo(err)) => assert_eq!(err.kind(), std::io::ErrorKind::NotFound),
            other => panic!("Expected AppendError::StdIo but got: {:?}", other),
        }
    }
}
