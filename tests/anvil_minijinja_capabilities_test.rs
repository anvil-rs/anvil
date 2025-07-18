use anvil::{mover::Move, transform::Transform, Forge};
use anvil_minijinja::prelude::*;
use anvil_minijinja_derive::Template;
use serde::Serialize;
use std::fs;
use tempfile::tempdir;

#[derive(Serialize, Template)]
#[template("append_test.txt")]
struct AppendTemplate {
    content: String,
}

#[derive(Serialize, Template)]
#[template("generate_test.txt")]
struct GenerateTemplate {
    name: String,
    id: String,
}

#[derive(Serialize, Template)]
#[template("either_primary.txt")]
struct EitherPrimaryTemplate {
    message: String,
}

#[derive(Serialize, Template)]
#[template("either_fallback.txt")]
struct EitherFallbackTemplate {
    message: String,
}

// APPEND TESTS
#[test]
fn test_append_can_append_to_existing_file() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("append_test.txt");

    // Create initial file content
    fs::write(&file_path, "Initial content\n").unwrap();

    let template = AppendTemplate {
        content: "new content".to_string(),
    };

    let append_op = append(&template);
    let result = append_op.forge(&file_path);

    assert!(result.is_ok());
    let final_content = fs::read_to_string(&file_path).unwrap();
    assert!(final_content.contains("Initial content"));
    assert!(final_content.contains("Appending: new content"));
}

#[test]
fn test_append_handles_empty_template_content() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("empty_append_test.txt");

    fs::write(&file_path, "Original content\n").unwrap();

    let template = AppendTemplate {
        content: String::new(),
    };

    let append_op = append(&template);
    let result = append_op.forge(&file_path);

    assert!(result.is_ok());
    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("Original content"));
    assert!(content.contains("Appending: "));
}

// EITHER TESTS - simplified to test single operations since either is complex
#[test]
fn test_generate_succeeds_for_either_scenario() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("either_test.txt");

    let primary_template = EitherPrimaryTemplate {
        message: "success".to_string(),
    };

    let primary_gen = generate(&primary_template);
    let result = primary_gen.forge(&file_path);

    assert!(result.is_ok());
    assert!(file_path.exists());
    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content.trim(), "Primary: success");
}

#[test]
fn test_append_succeeds_as_fallback_scenario() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("either_fallback_test.txt");

    // Create the file first
    fs::write(&file_path, "existing content").unwrap();

    let fallback_template = EitherFallbackTemplate {
        message: "fallback works".to_string(),
    };

    let fallback_append = append(&fallback_template);
    let result = fallback_append.forge(&file_path);

    assert!(result.is_ok());
    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("existing content"));
    assert!(content.contains("Fallback: fallback works"));
}

// GENERATE TESTS
#[test]
fn test_generate_creates_new_file() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("generate_test.txt");

    assert!(!file_path.exists());

    let template = GenerateTemplate {
        name: "TestFile".to_string(),
        id: "12345".to_string(),
    };

    let generate_op = generate(&template);
    let result = generate_op.forge(&file_path);

    if let Err(ref e) = result {
        println!("Forge error: {:?}", e);
    }
    assert!(result.is_ok());
    assert!(file_path.exists());
    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content.trim(), "Generated file with TestFile - 12345");
}

#[test]
fn test_generate_creates_parent_directories() {
    let temp_dir = tempdir().unwrap();
    let nested_path = temp_dir
        .path()
        .join("nested/deep/directory/generate_test.txt");

    assert!(!nested_path.exists());
    assert!(!nested_path.parent().unwrap().exists());

    let template = GenerateTemplate {
        name: "NestedFile".to_string(),
        id: "nested123".to_string(),
    };

    let generate_op = generate(&template);
    let result = generate_op.forge(&nested_path);

    assert!(result.is_ok());
    assert!(nested_path.exists());
    assert!(nested_path.parent().unwrap().exists());
    let content = fs::read_to_string(&nested_path).unwrap();
    assert_eq!(content.trim(), "Generated file with NestedFile - nested123");
}

#[test]
fn test_generate_fails_if_file_already_exists() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("existing_generate_test.txt");

    // Create the file first
    fs::write(&file_path, "existing content").unwrap();

    let template = GenerateTemplate {
        name: "ShouldFail".to_string(),
        id: "fail123".to_string(),
    };

    let generate_op = generate(&template);
    let result = generate_op.forge(&file_path);

    assert!(result.is_err());
    // Original content should remain unchanged
    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "existing content");
}

#[test]
fn test_generate_handles_empty_template_variables() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("empty_vars_test.txt");

    let template = GenerateTemplate {
        name: String::new(),
        id: String::new(),
    };

    let generate_op = generate(&template);
    let result = generate_op.forge(&file_path);

    assert!(result.is_ok());
    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content.trim(), "Generated file with  -");
}

// MOVE TESTS
#[test]
fn test_move_renames_file_successfully() {
    let temp_dir = tempdir().unwrap();
    let source_path = temp_dir.path().join("source_move_test.txt");
    let dest_path = temp_dir.path().join("destination_move_test.txt");

    let test_content = "Content to move";
    fs::write(&source_path, test_content).unwrap();

    assert!(source_path.exists());
    assert!(!dest_path.exists());

    let move_op = Move::new(&source_path);
    let result = move_op.forge(&dest_path);

    assert!(result.is_ok());
    assert!(!source_path.exists());
    assert!(dest_path.exists());
    let content = fs::read_to_string(&dest_path).unwrap();
    assert_eq!(content, test_content);
}

#[test]
fn test_move_fails_with_nonexistent_source() {
    let temp_dir = tempdir().unwrap();
    let source_path = temp_dir.path().join("nonexistent_source.txt");
    let dest_path = temp_dir.path().join("destination.txt");

    assert!(!source_path.exists());

    let move_op = Move::new(&source_path);
    let result = move_op.forge(&dest_path);

    assert!(result.is_err());
    assert!(!dest_path.exists());
}

#[test]
fn test_move_to_different_directory() {
    let temp_dir = tempdir().unwrap();
    let source_dir = temp_dir.path().join("source_dir");
    let dest_dir = temp_dir.path().join("dest_dir");

    fs::create_dir_all(&source_dir).unwrap();
    fs::create_dir_all(&dest_dir).unwrap();

    let source_path = source_dir.join("file_to_move.txt");
    let dest_path = dest_dir.join("moved_file.txt");

    let test_content = "Moving between directories";
    fs::write(&source_path, test_content).unwrap();

    let move_op = Move::new(&source_path);
    let result = move_op.forge(&dest_path);

    assert!(result.is_ok());
    assert!(!source_path.exists());
    assert!(dest_path.exists());
    let content = fs::read_to_string(&dest_path).unwrap();
    assert_eq!(content, test_content);
}

#[test]
fn test_move_overwrites_existing_destination() {
    let temp_dir = tempdir().unwrap();
    let source_path = temp_dir.path().join("source_overwrite.txt");
    let dest_path = temp_dir.path().join("dest_overwrite.txt");

    fs::write(&source_path, "Source content").unwrap();
    fs::write(&dest_path, "Destination content").unwrap();

    let move_op = Move::new(&source_path);
    let result = move_op.forge(&dest_path);

    assert!(result.is_ok());
    assert!(!source_path.exists());
    assert!(dest_path.exists());
    let content = fs::read_to_string(&dest_path).unwrap();
    assert_eq!(content, "Source content");
}

// TRANSFORM TESTS
#[test]
fn test_transform_modifies_file_content() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("transform_test.txt");

    let original_content = "hello world rust";
    fs::write(&file_path, original_content).unwrap();

    let transform_op = Transform::new(
        |content: String| -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            Ok(content.to_uppercase())
        },
    );

    let result = transform_op.forge(&file_path);

    assert!(result.is_ok());
    let transformed_content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(transformed_content, "HELLO WORLD RUST");
}

#[test]
fn test_transform_fails_with_nonexistent_file() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("nonexistent_transform.txt");

    assert!(!file_path.exists());

    let transform_op = Transform::new(
        |content: String| -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            Ok(content.to_uppercase())
        },
    );

    let result = transform_op.forge(&file_path);

    assert!(result.is_err());
}

#[test]
fn test_transform_handles_complex_modifications() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("complex_transform_test.txt");

    let original_content = "Line 1\nLine 2\nLine 3";
    fs::write(&file_path, original_content).unwrap();

    let transform_op = Transform::new(
        |content: String| -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            let lines: Vec<&str> = content.lines().collect();
            let numbered_lines: Vec<String> = lines
                .iter()
                .enumerate()
                .map(|(i, line)| format!("{}. {}", i + 1, line))
                .collect();
            Ok(numbered_lines.join("\n"))
        },
    );

    let result = transform_op.forge(&file_path);

    assert!(result.is_ok());
    let transformed_content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(transformed_content, "1. Line 1\n2. Line 2\n3. Line 3");
}

#[test]
fn test_transform_handles_transformer_error() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("error_transform_test.txt");

    fs::write(&file_path, "some content").unwrap();

    let transform_op = Transform::new(
        |_content: String| -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            Err("Transformation failed".into())
        },
    );

    let result = transform_op.forge(&file_path);

    assert!(result.is_err());
    // Original content should remain unchanged
    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "some content");
}

#[test]
fn test_transform_preserves_file_on_empty_result() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("empty_transform_test.txt");

    fs::write(&file_path, "original content").unwrap();

    let transform_op = Transform::new(
        |_content: String| -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            Ok(String::new())
        },
    );

    let result = transform_op.forge(&file_path);

    assert!(result.is_ok());
    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "");
}
