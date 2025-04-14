use std::{fs::read_to_string, path::Path};

use diffy::{create_patch, PatchFormatter};
use tempfile::NamedTempFile;

use crate::Forge;

// Hacky but working implementation of a diff producer.
// For it to be properly implemented, we would need to implement it
// on a per-forge level. This is a nice little blanket impl
pub trait ForgeDiff: Forge {
    fn diff(&self, into: impl AsRef<Path>) -> Result<String, Self::Error> {
        let tempfile = NamedTempFile::new().unwrap();
        // get a reference to a tempfile and copy if necessary
        let original = if into.as_ref().exists() {
            std::fs::copy(into, tempfile.path()).unwrap();

            let content = read_to_string(tempfile.path()).unwrap();

            content
        } else {
            String::new()
        };

        self.forge(&tempfile)?;

        let modified = read_to_string(tempfile.path()).unwrap();

        let patch = create_patch(&original, &modified);

        let f = PatchFormatter::new().with_color();

        let res = f.fmt_patch(&patch);

        // Return the formatted patch as an impl Display
        Ok(res.to_string())
    }
}

// blanket impl
impl<T: Forge> ForgeDiff for T {}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Anvil;
    use std::io::Write;
    use std::path::Path;
    use tempfile::{tempdir, NamedTempFile};

    // Mock implementation of Forge for testing
    struct MockForge {
        content: String,
    }

    impl Forge for MockForge {
        type Error = std::io::Error;

        fn forge(&self, into: impl AsRef<Path>) -> Result<(), Self::Error> {
            let path = into.as_ref();
            std::fs::write(path, &self.content)?;
            Ok(())
        }
    }

    // Mock implementation of Anvil for testing
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
    fn test_diff_with_new_file() {
        // Create a MockForge to test with
        let forge = MockForge {
            content: "New content".to_string(),
        };

        // Create a temporary directory for our test
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("new_file.txt");

        // Call diff on a non-existent file
        let diff_result = forge
            .diff(&file_path)
            .expect("Expected diff to succeed for a new file");

        // Assert that the diff contains the new content (added lines are prefixed with +)
        assert!(
            diff_result.contains("+New content"),
            "Diff should show added content: {}",
            diff_result
        );
        // Since it's a new file, there should be no removed lines
        assert!(
            !diff_result.contains("\n-"),
            "New file diff should not have any removed lines: {}",
            diff_result
        );
    }

    #[test]
    fn test_diff_with_existing_file() {
        // Create a temporary directory and file
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("existing_file.txt");

        // Create the file with initial content
        let original_content = "Original content";
        std::fs::write(&file_path, original_content).unwrap();

        // Create a MockForge with new content
        let forge = MockForge {
            content: "Modified content".to_string(),
        };

        // Call diff on the existing file
        let diff_result = forge
            .diff(&file_path)
            .expect("Expected diff to succeed for an existing file");

        // Assert that the diff shows the content change correctly
        assert!(
            diff_result.contains("-Original content"),
            "Diff should show removed original content: {}",
            diff_result
        );
        assert!(
            diff_result.contains("+Modified content"),
            "Diff should show added modified content: {}",
            diff_result
        );

        // Verify the original file has not been modified
        let content_after_diff = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(
            content_after_diff, original_content,
            "Original file should not be modified by diff"
        );
    }

    #[test]
    fn test_diff_with_forge_error() {
        // Create a failing forge
        struct FailingForge;

        impl Forge for FailingForge {
            type Error = std::io::Error;

            fn forge(&self, _into: impl AsRef<Path>) -> Result<(), Self::Error> {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Forge error",
                ))
            }
        }

        // Create a temporary file path
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("error_test.txt");

        // Test that diff propagates the forge error
        let failing_forge = FailingForge;
        let result = failing_forge.diff(&file_path);

        assert!(result.is_err(), "Expected diff to fail when forge fails");
        let error = result.unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::Other);
        assert_eq!(error.to_string(), "Forge error");
    }

    #[test]
    fn test_diff_preserves_file_content() {
        // Create a temporary file with content
        let mut temp_file = NamedTempFile::new().unwrap();
        let original_content = "Original file content";
        write!(temp_file, "{}", original_content).unwrap();
        let file_path = temp_file.path().to_owned();

        // Create a MockForge that will change the content
        let forge = MockForge {
            content: "Modified content by forge".to_string(),
        };

        // Run diff
        let result = forge.diff(&file_path);
        assert!(result.is_ok(), "Expected diff to succeed");

        // Verify the original file still has its original content
        let content_after_diff = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(
            content_after_diff, original_content,
            "Original file should not be modified by diff operation"
        );
    }

    #[test]
    fn test_diff_shows_correct_changes() {
        // This test is primarily for manual verification, as we can't easily
        // capture and parse the stdout output in a test

        // Create a temporary file with content
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("diff_test.txt");
        let original_content = "Line 1\nLine 2\nLine 3\n";
        std::fs::write(&file_path, original_content).unwrap();

        // Create a MockForge that will modify just one line
        let forge = MockForge {
            content: "Line 1\nModified Line 2\nLine 3\n".to_string(),
        };

        // Run diff
        let diff_result = forge.diff(&file_path).expect("Expected diff to succeed");

        // Verify that only the changed line is marked in the diff
        assert!(
            diff_result.contains("-Line 2"),
            "Diff should show removed line: {}",
            diff_result
        );
        assert!(
            diff_result.contains("+Modified Line 2"),
            "Diff should show added line: {}",
            diff_result
        );

        // Verify unchanged lines aren't marked as changed
        assert!(
            !diff_result.contains("-Line 1"),
            "Unchanged Line 1 shouldn't be marked as removed: {}",
            diff_result
        );
        assert!(
            !diff_result.contains("+Line 1"),
            "Unchanged Line 1 shouldn't be marked as added: {}",
            diff_result
        );
        assert!(
            !diff_result.contains("-Line 3"),
            "Unchanged Line 3 shouldn't be marked as removed: {}",
            diff_result
        );
        assert!(
            !diff_result.contains("+Line 3"),
            "Unchanged Line 3 shouldn't be marked as added: {}",
            diff_result
        );
        let original_content = "Line 1\nLine 2\nLine 3\n";
        std::fs::write(&file_path, original_content).unwrap();

        // Create a MockForge that will modify just one line
        let forge = MockForge {
            content: "Line 1\nModified Line 2\nLine 3\n".to_string(),
        };

        // Run diff
        let result = forge.diff(&file_path);
        assert!(result.is_ok(), "Expected diff to succeed");
    }
}
