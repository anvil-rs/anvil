use anvil::{
    append::append, either::either, filters, generate::generate, render, Anvil, Append, Either,
    Generate,
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

#[derive(Args, Template, Clone)]
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
                // these two are equivelant.

                render(
                    either(append(controller), generate(controller)),
                    "src/controllers/mod.rs",
                );

                Either::new(Append::new(controller), Generate::new(controller))
                    .forge("src/controllers/mod.rs")
                    .unwrap();

                // would it be worth adding a chainable API to this?
                // so you could do:
                //
            }
        },
    }
}
