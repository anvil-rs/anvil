use clap::{Args, Parser, Subcommand};
use heck::ToSnakeCase;
use std::fs::File;
use std::io::BufWriter;
use askama::Template;

mod filters;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate
    #[command(subcommand)]
    Generate(Generate),
}

#[derive(Subcommand)]
enum Generate {
    /// Controller
    Controller(Controller),
}

#[derive(Args, Template)]
#[template(path = "controller.rs", escape = "none")] // using the template in this path, relative
struct Controller {
    /// Name
    name: String,
}


#[derive(Args, Template)]
#[template(path = "controller_mod.rs", escape = "none")] // using the template in this path, relative
struct ControllerFrontMatter {
    name: String,
}


fn generate<T, S>(template: &T, path: S) -> std::io::Result<()>
where
    T: Template,
    S: AsRef<std::path::Path>,
{
    let path = path.as_ref();
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();

    // Check if file exists
    if path.exists() {
        return Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "File already exists"));
    }


    let file = File::create(path).unwrap();
    let mut buffer = BufWriter::new(file);

    template.write_into(&mut buffer).unwrap();

    Ok(())
}

fn append<T, S>(template: &T, path: S) -> std::io::Result<()>
where
    T: Template,
    S: AsRef<std::path::Path>,
{
    let path = path.as_ref();
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();

    let file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .unwrap();
    
    let mut buffer = BufWriter::new(file);

    template.write_into(&mut buffer).unwrap();

    Ok(())
}


// Generating things is a one-time operation.
// 
// Therefore, we can also use this to "add" things to a system
// We could, in theory, have a module that inits a totally new project.

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate(con) => match con {
            Generate::Controller(controller) => {

                match generate(controller, format!("src/controllers/{}.rs", controller.name.to_snake_case())) {
                    Ok(_) => println!("File generated successfully"),
                    Err(e) => println!("Error: {}", e),
                }

                let frontmatter = ControllerFrontMatter {
                    name: controller.name.clone(),
                };

                match append(&frontmatter, "src/controllers/mod.rs") {
                    Ok(_) => println!("File appended successfully"),
                    Err(e) => println!("Error: {}", e),
                }
            }
        },
    }
}
