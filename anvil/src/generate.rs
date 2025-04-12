use std::path::Path;
use std::{fs::File, io::BufWriter};

use thiserror::Error;

use crate::Anvil;
use crate::Forge;

/// A struct that generates a new file from a template.
///
/// `Generate` takes an [`Anvil`] implementation (template) and creates a new file with
/// the rendered content. It will create any necessary parent directories automatically,
/// ensuring that the file can be created even in a nested directory structure.
///
/// # Examples
///
/// ```rust,no_run
/// use anvil::{Anvil, Forge, generate::Generate};
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
/// // Create a file generator
/// let template = SimpleTemplate {
///     content: "# New File\n\nThis is generated content.".to_string(),
/// };
///
/// let generator = Generate::new(template);
///
/// // Generate a new file (will create directories if needed)
/// match generator.forge("./src/path/to/new_file.md") {
///     Ok(_) => println!("File successfully generated"),
///     Err(e) => eprintln!("Failed to generate file: {}", e),
/// }
/// ```
pub struct Generate<A: Anvil> {
    /// The template to render in the generated file
    template: A,
}

/// Errors that can occur during file generation operations.
///
/// This enum represents the different types of errors that can occur when
/// generating a file using the [`Generate`] struct.
#[derive(Error, Debug)]
pub enum GenerateError {
    /// Error that occurred during file IO operations.
    #[error("failed to perform file I/O while generating file: {0}")]
    StdIo(#[from] std::io::Error),

    /// Error that occurred during template rendering.
    #[error("failed to render template during file generation: {0}")]
    Template(#[from] Box<dyn std::error::Error>),
}

impl<A: Anvil> Forge for Generate<A> {
    type Error = GenerateError;

    /// Generates a new file with content from the template.
    ///
    /// This method:
    /// 1. Creates all necessary parent directories
    /// 2. Creates a new file (fails if it already exists)
    /// 3. Renders the template into the file
    ///
    /// # Parameters
    ///
    /// * `into` - Path where the new file will be created
    ///
    /// # Returns
    ///
    /// * `Result<(), GenerateError>` - Ok if successful, or an error if:
    ///   - Parent directories couldn't be created
    ///   - File already exists
    ///   - File permissions don't allow writing
    ///   - Template rendering fails
    ///
    /// # Errors
    ///
    /// Returns `GenerateError::StdIo` if there's an IO error (like file already exists),
    /// or `GenerateError::Template` if there's an error during template rendering.
    fn forge(&self, into: impl AsRef<Path>) -> Result<(), Self::Error> {
        let path = into.as_ref();

        let prefix = path.parent().expect("no parent directory");
        std::fs::create_dir_all(prefix).map_err(GenerateError::StdIo)?;

        let file = File::create_new(path).map_err(GenerateError::StdIo)?;

        let mut writer = BufWriter::new(file);

        self.template
            .anvil(&mut writer)
            .map_err(|e| GenerateError::Template(Box::new(e)))?;

        Ok(())
    }
}

impl<A: Anvil> Generate<A> {
    /// Creates a new `Generate` instance with the given template.
    ///
    /// # Parameters
    ///
    /// * `template` - An implementation of the [`Anvil`] trait that will be rendered
    ///   into the newly created file.
    ///
    /// # Returns
    ///
    /// A new `Generate` instance configured with the provided template.
    pub fn new(template: A) -> Self {
        Self { template }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use tempfile::tempdir;

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
    fn test_generate_creates_new_file() {
        // Create a temporary directory
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("generated_file.txt");

        // Create a template with content
        let expected_content = "Generated content";
        let template = MockAnvil {
            content: expected_content.to_string(),
        };

        // Create and use the Generate
        let generate = Generate::new(template);
        let result = generate.forge(&file_path);
        assert!(result.is_ok());

        // Read the file content and verify it was created correctly
        assert!(file_path.exists());
        let mut file = File::open(&file_path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        assert_eq!(content, expected_content);
    }

    #[test]
    fn test_generate_creates_parent_directories() {
        // Create a temporary directory
        let temp_dir = tempdir().unwrap();
        let nested_path = temp_dir.path().join("nested/directories/file.txt");

        // Create a template with content
        let expected_content = "Content in nested directories";
        let template = MockAnvil {
            content: expected_content.to_string(),
        };

        // Create and use the Generate
        let generate = Generate::new(template);
        let result = generate.forge(&nested_path);
        assert!(result.is_ok());

        // Verify the directories were created
        assert!(nested_path.exists());
        assert!(nested_path.parent().unwrap().exists());

        // Read the file content and verify it was created correctly
        let mut file = File::open(&nested_path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        assert_eq!(content, expected_content);
    }

    #[test]
    fn test_generate_handles_template_error() {
        // Create a dummy template that always fails
        struct FailingAnvil;

        impl Anvil for FailingAnvil {
            type Error = std::io::Error;

            fn anvil(
                &self,
                _writer: &mut (impl std::io::Write + Sized),
            ) -> Result<(), Self::Error> {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Template error",
                ))
            }
        }

        // Create a temporary directory
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("should_not_be_created.txt");

        // Create and use the Generate
        let generate = Generate::new(FailingAnvil);
        let result = generate.forge(&file_path);

        // Should return a template error
        assert!(result.is_err());
        match result {
            Err(GenerateError::Template(err)) => assert_eq!(err.to_string(), "Template error"),
            other => unreachable!("Expected Template error but got: {:?}", other),
        }
    }
}
