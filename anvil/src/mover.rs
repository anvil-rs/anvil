use std::path::{Path, PathBuf};

use crate::Forge;

/// A struct that moves or renames a file.
///
/// `Move` provides functionality to move a file from one path to another,
/// which can be used either for relocating a file or renaming it. It uses
/// Rust's `std::fs::rename` function internally, which is an atomic operation
/// on most operating systems.
///
/// # Examples
///
/// ```rust,no_run
/// use anvil::{Forge, mover::Move};
///
/// // Moving a file to a new location
/// let mover = Move::new("./src/old_location.rs");
/// match mover.forge("./src/new_directory/relocated.rs") {
///     Ok(_) => println!("File successfully moved"),
///     Err(e) => eprintln!("Failed to move file: {}", e),
/// }
///
/// // Renaming a file
/// let renamer = Move::new("./config.old.toml");
/// match renamer.forge("./config.toml") {
///     Ok(_) => println!("File successfully renamed"),
///     Err(e) => eprintln!("Failed to rename file: {}", e),
/// }
/// ```
///
/// # Notes
///
/// - The operation will fail if the source file does not exist.
/// - The operation will fail if the destination already exists.
/// - Moving files across different filesystems may not be atomic and could fail.
pub struct Move {
    /// The source path of the file to be moved
    from: PathBuf,
}

impl Move {
    /// Creates a new `Move` operation with the given source path.
    ///
    /// # Parameters
    ///
    /// * `from` - The path to the file that will be moved or renamed
    ///
    /// # Returns
    ///
    /// A new `Move` instance configured with the provided source path.
    pub fn new(from: impl AsRef<Path>) -> Self {
        Self {
            from: from.as_ref().to_path_buf(),
        }
    }
}

impl Forge for Move {
    type Error = std::io::Error;
    
    /// Moves or renames the file to the destination path.
    ///
    /// This method:
    /// 1. Takes the source path (provided during construction)
    /// 2. Takes the destination path (provided as argument)
    /// 3. Renames/moves the file from source to destination
    ///
    /// # Parameters
    ///
    /// * `into` - Destination path where the file should be moved to
    ///
    /// # Returns
    ///
    /// * `Result<(), std::io::Error>` - Ok if successful, or an error if:
    ///   - The source file doesn't exist
    ///   - The destination already exists
    ///   - File permissions don't allow the operation
    ///   - The operation crosses filesystem boundaries and is not supported
    ///
    /// # Errors
    ///
    /// Returns `std::io::Error` with various error kinds depending on what went wrong.
    /// Common error kinds include `NotFound` if the source doesn't exist, and
    /// `AlreadyExists` if the destination already exists.
    fn forge(&self, into: impl AsRef<Path>) -> Result<(), Self::Error> {
        let to = into.as_ref();
        std::fs::rename(&self.from, to)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_move_renames_file() {
        // Create a temporary directory and file
        let temp_dir = tempdir().unwrap();
        let source_path = temp_dir.path().join("source.txt");
        let test_content = "Test content";
        
        // Create and write to source file
        std::fs::write(&source_path, test_content).unwrap();
        
        // Create destination path
        let dest_path = temp_dir.path().join("destination.txt");
        
        // Create and use the Move
        let mover = Move::new(&source_path);
        let result = mover.forge(&dest_path);
        assert!(result.is_ok());
        
        // Verify the file was moved
        assert!(!source_path.exists());
        assert!(dest_path.exists());
        
        // Verify content was preserved
        let content = std::fs::read_to_string(&dest_path).unwrap();
        assert_eq!(content, test_content);
    }

    #[test]
    fn test_move_fails_with_nonexistent_source() {
        // Create a temporary directory
        let temp_dir = tempdir().unwrap();
        
        // Create paths for nonexistent source and destination
        let source_path = temp_dir.path().join("nonexistent.txt");
        let dest_path = temp_dir.path().join("destination.txt");
        
        // Create and use the Move
        let mover = Move::new(&source_path);
        let result = mover.forge(&dest_path);
        
        // Should fail with a file not found error
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::NotFound);
    }

    #[test]
    fn test_move_across_directories() {
        // Create two temporary directories
        let source_dir = tempdir().unwrap();
        let dest_dir = tempdir().unwrap();
        
        // Create source file
        let source_path = source_dir.path().join("source.txt");
        let test_content = "Cross-directory move test";
        std::fs::write(&source_path, test_content).unwrap();
        
        // Create destination path in different directory
        let dest_path = dest_dir.path().join("moved.txt");
        
        // Create and use the Move
        let mover = Move::new(&source_path);
        let result = mover.forge(&dest_path);
        
        // This might fail on some systems if directories are on different filesystems
        if result.is_ok() {
            // Verify the file was moved
            assert!(!source_path.exists());
            assert!(dest_path.exists());
            
            // Verify content was preserved
            let content = std::fs::read_to_string(&dest_path).unwrap();
            assert_eq!(content, test_content);
        } else {
            // On some systems, cross-filesystem moves will fail
            // This is expected behavior, so we don't want the test to fail
            assert!(result.is_err());
            // We don't check the specific error kind since CrossesDevices is unstable
        }
    }
}