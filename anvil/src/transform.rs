use std::{error::Error, path::Path};

use thiserror::Error;

use crate::Forge;

/// A type alias for a boxed error that can be sent across threads.
///
/// This alias simplifies the error handling in transformer functions 
/// by allowing any error type that implements `Error + Send + Sync` to be returned.
pub type BoxedError = Box<dyn Error + Send + Sync>;

/// A struct that transforms the content of an existing file.
///
/// `Transform` provides a way to read a file, apply a transformation function to its content,
/// and write the transformed content back to the same file. This is useful for making
/// programmatic changes to file content.
///
/// # Examples
///
/// ```rust,no_run
/// use anvil::{Forge, transform::{Transform, BoxedError}};
///
/// // Transform a file by adding a header comment
/// let add_header = Transform::new(|content| -> Result<String, BoxedError> {
///     Ok(format!("// AUTO-GENERATED - DO NOT EDIT\n\n{}", content))
/// });
///
/// match add_header.forge("./src/generated.rs") {
///     Ok(_) => println!("File successfully transformed"),
///     Err(e) => eprintln!("Failed to transform file: {}", e),
/// }
///
/// // Transform another file example
/// let transform_content = Transform::new(|content| -> Result<String, BoxedError> {
///     // Process the content without external dependencies
///     let modified = content.replace("old", "new");
///     Ok(modified)
/// });
///
/// match transform_content.forge("./config.txt") {
///     Ok(_) => println!("File transformed"),
///     Err(e) => eprintln!("Failed to transform file: {}", e),
/// }
/// ```
pub struct Transform {
    /// The transformation function to apply to file content
    transformer: Box<dyn Fn(String) -> Result<String, BoxedError>>,
}

impl Transform {
    /// Creates a new `Transform` with the given transformation function.
    ///
    /// # Parameters
    ///
    /// * `transformer` - A function that takes a string and returns a transformed string or an error.
    ///   This function will be applied to the content of the file.
    ///
    /// # Returns
    ///
    /// A new `Transform` instance configured with the provided transformer function.
    ///
    /// # Examples
    ///
    /// ```
    /// use anvil::transform::{Transform, BoxedError};
    ///
    /// // Simple transformer that adds line numbers
    /// let add_line_numbers = Transform::new(|content| -> Result<String, BoxedError> {
    ///     let numbered = content.lines()
    ///         .enumerate()
    ///         .map(|(i, line)| format!("{}: {}", i+1, line))
    ///         .collect::<Vec<_>>()
    ///         .join("\n");
    ///     Ok(numbered)
    /// });
    /// ```
    pub fn new<F>(transformer: F) -> Self
    where
        F: Fn(String) -> Result<String, BoxedError> + 'static,
    {
        Self {
            transformer: Box::new(transformer),
        }
    }

    /// Applies the transformation function to the given input string.
    ///
    /// This method is useful for testing the transformer or for applying
    /// the same transformation to content from a different source.
    ///
    /// # Parameters
    ///
    /// * `input` - The string to transform
    ///
    /// # Returns
    ///
    /// * `Result<String, BoxedError>` - The transformed string if successful,
    ///   or an error if the transformation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use anvil::transform::{Transform, BoxedError};
    ///
    /// let uppercase = Transform::new(|content| -> Result<String, BoxedError> {
    ///     Ok(content.to_uppercase())
    /// });
    /// assert_eq!(uppercase.apply("hello").unwrap(), "HELLO");
    /// ```
    pub fn apply(&self, input: &str) -> Result<String, BoxedError> {
        (self.transformer)(input.to_string())
    }
}

/// Errors that can occur during file transformation operations.
///
/// This enum represents the different types of errors that can occur when
/// transforming a file using the [`Transform`] struct.
#[derive(Error, Debug)]
pub enum TransformError {
    /// Error that occurred during file IO operations (reading or writing).
    #[error("failed to perform file I/O while transforming file: {0}")]
    StdIo(#[from] std::io::Error),
    
    /// Error that occurred during the transformation function.
    #[error("failed to apply transformation to file content: {0}")]
    Transform(#[from] BoxedError),
}

impl Forge for Transform {
    type Error = TransformError;
    
    /// Transforms the content of the file at the specified path.
    ///
    /// This method:
    /// 1. Reads the file content
    /// 2. Applies the transformation function
    /// 3. Writes the transformed content back to the file
    ///
    /// # Parameters
    ///
    /// * `into` - Path to the file to transform
    ///
    /// # Returns
    ///
    /// * `Result<(), TransformError>` - Ok if successful, or an error if:
    ///   - The file doesn't exist
    ///   - File permissions don't allow reading or writing
    ///   - The transformation function fails
    ///
    /// # Errors
    ///
    /// Returns `TransformError::StdIo` if there's an IO error (like file not found),
    /// or `TransformError::Transform` if there's an error during the transformation.
    fn forge(&self, into: impl AsRef<Path>) -> Result<(), Self::Error> {
        let path = into.as_ref();
        let content = std::fs::read_to_string(path).map_err(TransformError::StdIo)?;
        let transformed = self.apply(&content).map_err(TransformError::Transform)?;
        std::fs::write(path, transformed).map_err(TransformError::StdIo)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::Write, sync::Arc};
    use tempfile::{tempdir, NamedTempFile};

    #[test]
    fn test_transform_applies_function() {
        // Create a transform that converts text to uppercase
        let transformer = |input: String| -> Result<String, BoxedError> {
            Ok(input.to_uppercase())
        };
        let transform = Transform::new(transformer);
        
        let input = "hello world";
        let result = transform.apply(input).unwrap();
        assert_eq!(result, "HELLO WORLD");
    }

    #[test]
    fn test_transform_file_content() {
        // Create a temporary file with content
        let mut temp_file = NamedTempFile::new().unwrap();
        let original_content = "hello world";
        temp_file.write_all(original_content.as_bytes()).unwrap();

        // Create a transform that converts text to uppercase
        let transformer = |input: String| -> Result<String, BoxedError> {
            Ok(input.to_uppercase())
        };
        let transform = Transform::new(transformer);
        
        // Apply the transform
        let result = transform.forge(temp_file.path());
        assert!(result.is_ok());

        // Read and verify the transformed content
        let content = std::fs::read_to_string(temp_file.path()).unwrap();
        assert_eq!(content, "HELLO WORLD");
    }

    #[test]
    fn test_transform_handles_error() {
        // Create a transformer that always returns an error
        let transformer = |_: String| -> Result<String, BoxedError> {
            Err("transform failed".into())
        };
        let transform = Transform::new(transformer);
        
        // Create a temporary file
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"content").unwrap();
        
        // Apply the transform
        let result = transform.forge(temp_file.path());
        assert!(result.is_err());
        
        match result {
            Err(TransformError::Transform(err)) => assert_eq!(err.to_string(), "transform failed"),
            other => panic!("Expected Transform error but got: {:?}", other),
        }
    }

    #[test]
    fn test_transform_handles_file_not_found() {
        // Create a path to a file that doesn't exist
        let nonexistent_path = tempdir().unwrap().path().join("nonexistent_file.txt");
        
        // Create a simple transform
        let transform = Transform::new(|s| Ok(s));
        
        // Apply the transform
        let result = transform.forge(nonexistent_path);
        
        // Should fail with a file not found error
        assert!(result.is_err());
        match result {
            Err(TransformError::StdIo(err)) => assert_eq!(err.kind(), std::io::ErrorKind::NotFound),
            other => panic!("Expected StdIo error but got: {:?}", other),
        }
    }

    #[test]
    fn test_transform_can_use_captured_data() {
        // Create a counter to track transform calls
        let counter = Arc::new(std::sync::Mutex::new(0));
        let counter_clone = counter.clone();
        
        // Create a transform that increments counter
        let transformer = move |input: String| -> Result<String, BoxedError> {
            let mut count = counter_clone.lock().unwrap();
            *count += 1;
            Ok(format!("{} (transformed {} times)", input, *count))
        };
        
        let transform = Transform::new(transformer);
        
        // Apply the transform multiple times
        let input = "hello";
        let result1 = transform.apply(input).unwrap();
        let result2 = transform.apply(input).unwrap();
        
        assert_eq!(result1, "hello (transformed 1 times)");
        assert_eq!(result2, "hello (transformed 2 times)");
        assert_eq!(*counter.lock().unwrap(), 2);
    }
}
