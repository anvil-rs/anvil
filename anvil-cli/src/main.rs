use askama::Template;
use clap::{Args, Parser, Subcommand};
use heck::ToSnakeCase;

use anvil::{append, either, filters, generate, render};
use anvil::{either::Either, Anvil, Append, Generate};

// Meta framewokr idea: ablke to pull in templates from dependencies if we want to change the way
// things are generated.

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
    Generate(Gen),
}

#[derive(Subcommand)]
enum Gen {
    /// Controller
    Controller(Controller),
}

#[derive(Args, Template)]
#[template(path = "controller.rs", escape = "none")] // using the template in this path, relative
struct Controller {
    name: String,
}

// Generating things is a one-time operation.
//
// Therefore, we can also use this to "add" things to a system
// We could, in theory, have a module that inits a totally new project.

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate(con) => match con {
            Gen::Controller(controller) => {
                // these two may be equivelant:
                render!(
                    either!(append!(controller), generate!(controller)),
                    "src/controllers/mod.rs"
                );

                Either::new(Append::new(controller), Generate::new(controller))
                    .render("src/controllers/mod.rs")
                    .unwrap();

                Generate::new(controller)
                    .render(format!(
                        "src/controller/{}.rs",
                        controller.name.to_snake_case()
                    ))
                    .unwrap();
            }
        },
    }
}
