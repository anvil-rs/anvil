use anvil::{either::either, forge, Append, Either, Forge, Generate};

use anvil_askama::{
    append::{append, AskamaAppendExt},
    filters,
    generate::{generate, AskamaGenerateExt},
};

use askama::Template;
use clap::{Args, Parser, Subcommand};

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
                forge(
                    either(append(controller), generate(controller)),
                    "src/controllers/mod.rs",
                );

                Either::new(Append::askama(controller), Generate::askama(controller))
                    .forge("src/controllers/mod.rs")
                    .unwrap();
            }
        },
    }
}
