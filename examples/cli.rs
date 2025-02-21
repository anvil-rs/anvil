use anvil::{append::*, either::*, filters, generate::*, Anvil};

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

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate(con) => match con {
            Gen::Controller(controller) => {
                // these two are equivelant.

                either(append(controller), generate(controller))
                    .forge("src/controllers/mod.rs")
                    .unwrap();

                Either::new(Append::new(controller), Generate::new(controller))
                    .forge("src/controllers/mod.rs")
                    .unwrap();
            }
        },
    }
}
