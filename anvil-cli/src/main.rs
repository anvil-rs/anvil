use anvil_cli::Anvil;
use askama::Template;
use clap::{Args, Parser, Subcommand};

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
#[template(path = "test.rs", escape = "none")] // using the template in this path, relative
struct Controller {
    /// Name
    name: String,
}

impl Anvil for Controller {
    fn path(&self) -> String {
        format!("test/{}", self.name)
    }
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Generate(con) => match con {
            Generate::Controller(controller) => {
                println!("Path: {}", controller.path());
                println!("{}", controller.name);
                println!("{}", controller.render().unwrap());
            }
        },
    }
}
