use anvil::{
    append::Append, either::either, generate::Generate, mover::Move, transform::Transform, Anvil,
    Forge,
};
use std::io::Write;
use std::path::Path;

// A simple template implementation for demonstration
struct SimpleTemplate {
    content: String,
}

impl Anvil for SimpleTemplate {
    type Error = std::io::Error;

    fn anvil(&self, writer: &mut (impl Write + ?Sized)) -> Result<(), Self::Error> {
        writer.write_all(self.content.as_bytes())?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a demo directory to hold our files
    let demo_dir = Path::new("demo");
    if !demo_dir.exists() {
        std::fs::create_dir(demo_dir)?;
    }

    println!("Starting demonstration of Anvil file operations...\n");

    // 1. GENERATE: Create a new file
    println!("1. Generating a new file...");
    let template = SimpleTemplate {
        content: "# Hello from Anvil\n\nThis file was created using the Generate operation.\n"
            .to_string(),
    };

    let readme_path = demo_dir.join("README.md");
    Generate::new(template).forge(&readme_path)?;
    println!("   ✓ Generated file: {}\n", readme_path.display());

    // 2. APPEND: Add content to an existing file
    println!("2. Appending content to the file...");
    let appendix = SimpleTemplate {
        content:
            "\n## Additional Content\n\nThis content was appended using the Append operation.\n"
                .to_string(),
    };

    Append::new(appendix).forge(&readme_path)?;
    println!("   ✓ Appended content to: {}\n", readme_path.display());

    // 3. TRANSFORM: Modify the content of an existing file
    println!("3. Transforming file content...");
    let transform_op = Transform::new(
        |content| -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            let modified = content.replace("Hello from Anvil", "Hello from Anvil (Transformed!)");
            Ok(modified)
        },
    );

    transform_op.forge(&readme_path)?;
    println!("   ✓ Transformed content in: {}\n", readme_path.display());

    // 4. MOVE: Rename or move a file
    println!("4. Moving/renaming file...");
    let new_path = demo_dir.join("DOCS.md");
    Move::new(&readme_path).forge(&new_path)?;
    println!(
        "   ✓ Moved file from {} to {}\n",
        readme_path.display(),
        new_path.display()
    );

    // 5. EITHER: Demonstration of fallback mechanisms
    println!("5. Demonstrating Either operation (fallback mechanism)...");

    // Create a file for our Either demo
    let either_path = demo_dir.join("either_demo.txt");
    let initial_template = SimpleTemplate {
        content: "This file will be used to demonstrate the Either operation.\n".to_string(),
    };
    Generate::new(initial_template).forge(&either_path)?;

    // Create operations for our Either demo
    // The first operation will fail (trying to generate a file that already exists)
    let will_fail = Generate::new(SimpleTemplate {
        content: "This operation will fail because the file already exists.\n".to_string(),
    });

    // The second operation will succeed (append to the file)
    let will_succeed = Append::new(SimpleTemplate {
        content: "The first operation failed, so this content was appended instead.\n".to_string(),
    });

    // Try the operations with Either
    either(will_fail, will_succeed).forge(&either_path)?;
    println!(
        "   ✓ Successfully demonstrated Either operation on: {}\n",
        either_path.display()
    );

    // 6. Bonus: Combining operations
    println!("6. Bonus: Combining multiple operations together...");
    let combo_path = demo_dir.join("combined_ops.txt");

    // First generate a new file
    let generate_op = Generate::new(SimpleTemplate {
        content: "Step 1: File created with Generate\n".to_string(),
    });
    generate_op.forge(&combo_path)?;

    // Then append to it
    let append_op = Append::new(SimpleTemplate {
        content: "Step 2: Content appended with Append\n".to_string(),
    });
    append_op.forge(&combo_path)?;

    // Finally transform it
    let transform_op = Transform::new(
        |content| -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            Ok(format!(
                "{}\nStep 3: Content transformed with Transform\n",
                content
            ))
        },
    );
    transform_op.forge(&combo_path)?;

    println!(
        "   ✓ Successfully combined operations on: {}\n",
        combo_path.display()
    );

    // Show final result
    println!("Demonstration completed successfully!");
    println!("You can examine the results in the 'demo' directory.");

    Ok(())
}
