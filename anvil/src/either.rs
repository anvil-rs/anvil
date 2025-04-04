use crate::Forge;
use std::path::Path;

/// A struct that implements a fallback mechanism between two operations.
///
/// `Either` takes two operations that implement the [`Forge`] trait and tries
/// to execute the first one. If the first operation fails, it falls back to
/// the second operation. This provides a resilient way to handle operations
/// that might fail, giving an alternative approach.
///
/// # Type Parameters
///
/// * `L` - The primary (left) operation type that implements [`Forge`]
/// * `R` - The fallback (right) operation type that implements [`Forge`]
///
/// # Examples
///
/// ```
/// use anvil::{Forge, either::{either, Either}};
/// use std::path::Path;
///
/// // Assuming we have two forge operations:
/// struct PrimaryOperation;
/// struct FallbackOperation;
///
/// impl Forge for PrimaryOperation {
///     type Error = std::io::Error;
///     fn forge(&self, _into: impl AsRef<Path>) -> Result<(), Self::Error> {
///         // Try primary operation...
///         Err(std::io::Error::new(std::io::ErrorKind::Other, "Primary operation failed"))
///     }
/// }
///
/// impl Forge for FallbackOperation {
///     type Error = std::io::Error;
///     fn forge(&self, _into: impl AsRef<Path>) -> Result<(), Self::Error> {
///         // Fallback operation succeeds
///         Ok(())
///     }
/// }
///
/// // Create the Either operation with both options
/// let operation = either(PrimaryOperation, FallbackOperation);
///
/// // Execute - will use fallback when primary fails
/// let result = operation.forge("./output.txt");
/// assert!(result.is_ok());
/// ```
pub struct Either<L: Forge, R: Forge> {
    /// The primary (left) operation to try first
    left: L,
    /// The fallback (right) operation to try if the first fails
    right: R,
}

impl<L: Forge, R: Forge> Forge for Either<L, R> {
    /// The error type from the right operation
    type Error = R::Error;

    /// Attempts the left operation first, then falls back to the right operation if needed.
    ///
    /// This method:
    /// 1. Tries to execute the left operation
    /// 2. If the left operation fails, tries the right operation
    /// 3. Returns the result of the successful operation, or the error from the right operation
    ///
    /// # Parameters
    ///
    /// * `into` - Path where the operation will be performed
    ///
    /// # Returns
    ///
    /// * `Result<(), R::Error>` - Ok if either operation succeeds, or the error from the right operation
    ///   if both fail. Note that the error type is the one from the right operation.
    fn forge(&self, into: impl AsRef<Path>) -> Result<(), Self::Error> {
        self.left.forge(&into).or_else(|_| self.right.forge(&into))
    }
}

impl<L: Forge, R: Forge> Either<L, R> {
    /// Creates a new `Either` instance with the given primary and fallback operations.
    ///
    /// # Parameters
    ///
    /// * `left` - The primary operation to try first
    /// * `right` - The fallback operation to try if the first fails
    ///
    /// # Returns
    ///
    /// A new `Either` instance configured with the provided operations.
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

/// Convenience function to create an `Either` operation.
///
/// This is a shorthand for `Either::new()` that makes the code more readable.
///
/// # Parameters
///
/// * `left` - The primary operation to try first
/// * `right` - The fallback operation to try if the first fails
///
/// # Returns
///
/// An `Either` instance configured with the provided operations.
///
/// # Examples
///
/// ```
/// use anvil::{Forge, either::either};
/// use std::path::Path;
///
/// // Two hypothetical operations
/// struct Operation1;
/// struct Operation2;
///
/// // Implement Forge for the operations
/// impl Forge for Operation1 {
///     type Error = std::io::Error;
///     fn forge(&self, _into: impl AsRef<Path>) -> Result<(), Self::Error> {
///         Ok(())
///     }
/// }
///
/// impl Forge for Operation2 {
///     type Error = std::io::Error;
///     fn forge(&self, _into: impl AsRef<Path>) -> Result<(), Self::Error> {
///         Ok(())
///     }
/// }
///
/// // Using the convenience function
/// let operation = either(Operation1, Operation2);
/// ```
#[inline]
pub fn either<L: Forge, R: Forge>(left: L, right: R) -> Either<L, R> {
    Either::new(left, right)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::path::PathBuf;
    use tempfile::tempdir;

    // Mock Forge implementations for testing
    struct MockForge<F>
    where
        F: Fn() -> Result<(), io::Error>,
    {
        action: F,
    }

    impl<F> Forge for MockForge<F>
    where
        F: Fn() -> Result<(), io::Error>,
    {
        type Error = io::Error;

        fn forge(&self, _into: impl AsRef<Path>) -> Result<(), Self::Error> {
            (self.action)()
        }
    }

    #[test]
    fn test_either_uses_left_when_successful() {
        // Create temporary test directory
        let temp_dir = tempdir().unwrap();
        let test_path = temp_dir.path().join("test.txt");

        // Create successful left forge
        let left = MockForge { action: || Ok(()) };

        // Create successful right forge (should not be used)
        let right = MockForge {
            action: || {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Right should not be called",
                ))
            },
        };

        // Create and use Either
        let either_forge = either(left, right);
        let result = either_forge.forge(&test_path);

        // Should succeed using the left forge
        assert!(result.is_ok());
    }

    #[test]
    fn test_either_uses_right_when_left_fails() {
        // Create temporary test directory
        let temp_dir = tempdir().unwrap();
        let test_path = temp_dir.path().join("test.txt");

        // Create failing left forge
        let left = MockForge {
            action: || Err(io::Error::new(io::ErrorKind::Other, "Left fails")),
        };

        // Create successful right forge (should be used)
        let right = MockForge { action: || Ok(()) };

        // Create and use Either
        let either_forge = either(left, right);
        let result = either_forge.forge(&test_path);

        // Should succeed using the right forge
        assert!(result.is_ok());
    }

    #[test]
    fn test_either_fails_when_both_fail() {
        // Create temporary test directory
        let temp_dir = tempdir().unwrap();
        let test_path = temp_dir.path().join("test.txt");

        // Create failing left forge
        let left = MockForge {
            action: || Err(io::Error::new(io::ErrorKind::Other, "Left fails")),
        };

        // Create failing right forge with a different error
        let right_error = "Right fails too";
        let right = MockForge {
            action: move || Err(io::Error::new(io::ErrorKind::Other, right_error)),
        };

        // Create and use Either
        let either_forge = either(left, right);
        let result = either_forge.forge(&test_path);

        // Should fail with the right forge's error
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), right_error);
    }

    #[test]
    fn test_either_convenience_function() {
        // Create simple test forges
        let left = MockForge { action: || Ok(()) };

        let right = MockForge { action: || Ok(()) };

        // Create Either using the convenience function
        let either_forge = either(left, right);

        // Just a simple test to ensure the convenience function works
        let test_path = PathBuf::from("/tmp/test.txt"); // Path doesn't matter for this test
        let result = either_forge.forge(test_path);
        assert!(result.is_ok());
    }
}
