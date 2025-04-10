use anvil::{append::Append, either::either, generate::Generate, Forge};
use anvil_askama::prelude::*;
use askama::Template;
use clap::{Args, Parser, Subcommand};
use heck::{ToLowerCamelCase, ToSnakeCase, ToUpperCamelCase};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about = "Axum MVC Generator", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate MVC components
    Generate(GenerateArgs),
}

#[derive(Args)]
struct GenerateArgs {
    #[command(subcommand)]
    component: Component,
}

#[derive(Subcommand)]
enum Component {
    /// Generate a controller
    Controller(ControllerArgs),
    /// Generate a model
    Model(ModelArgs),
    /// Generate a complete CRUD resource
    Resource(ResourceArgs),
}

#[derive(Args, Clone)]
struct ControllerArgs {
    /// Name of the controller
    name: String,
    /// Base path for the controller routes
    #[arg(short, long)]
    path: Option<String>,
}

#[derive(Args, Clone)]
struct ModelArgs {
    /// Name of the model
    name: String,
    /// Model fields in format name:type
    #[arg(short, long, value_delimiter = ',')]
    fields: Vec<String>,
}

#[derive(Args, Clone)]
struct ResourceArgs {
    /// Name of the resource
    name: String,
    /// Model fields in format name:type
    #[arg(short, long, value_delimiter = ',')]
    fields: Vec<String>,
    /// Base path for the resource routes
    #[arg(short, long)]
    path: Option<String>,
}

// Templates

#[derive(Template)]
#[template(path = "axum_generator/model.rs", escape = "none")]
struct ModelTemplate {
    name: String,
    snake_name: String,
    fields: Vec<ModelField>,
}

#[derive(Template)]
#[template(path = "axum_generator/controller.rs", escape = "none")]
struct ControllerTemplate {
    name: String,
    snake_name: String,
    path: String,
    resource_name: String,
}

#[derive(Template)]
#[template(path = "axum_generator/controller_mod.rs", escape = "none")]
struct ControllerModTemplate {
    name: String,
    snake_name: String,
}

#[derive(Template)]
#[template(path = "axum_generator/model_mod.rs", escape = "none")]
struct ModelModTemplate {
    name: String,
    snake_name: String,
}

struct ModelField {
    name: String,
    rust_type: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate(args) => match &args.component {
            Component::Controller(controller_args) => {
                generate_controller(controller_args)?;
            }
            Component::Model(model_args) => {
                generate_model(model_args)?;
            }
            Component::Resource(resource_args) => {
                generate_resource(resource_args)?;
            }
        },
    }

    Ok(())
}

fn generate_controller(args: &ControllerArgs) -> Result<(), Box<dyn std::error::Error>> {
    let name = args.name.to_upper_camel_case();
    let snake_name = args.name.to_snake_case();
    let path = args
        .path
        .clone()
        .unwrap_or_else(|| format!("/{}", snake_name));

    let resource_name = name.clone();

    // Create the controller directory structure if it doesn't exist
    let controllers_dir = PathBuf::from("src/controllers");
    if !controllers_dir.exists() {
        std::fs::create_dir_all(&controllers_dir)?;
    }

    // Generate the controller
    let controller = ControllerTemplate {
        name: name.clone(),
        snake_name: snake_name.clone(),
        path,
        resource_name,
    };

    let controller_path = controllers_dir.join(format!("{}.rs", snake_name));
    Generate::askama(&controller).forge(&controller_path)?;
    println!("Generated controller at: {}", controller_path.display());

    // Add module declaration to mod.rs
    let controller_mod = ControllerModTemplate {
        name: name.clone(),
        snake_name: snake_name.clone(),
    };

    let mod_path = controllers_dir.join("mod.rs");
    either(
        Append::askama(&controller_mod),
        Generate::askama(&controller_mod),
    )
    .forge(&mod_path)?;

    println!("Updated controller module at: {}", mod_path.display());

    Ok(())
}

fn generate_model(args: &ModelArgs) -> Result<(), Box<dyn std::error::Error>> {
    let name = args.name.to_upper_camel_case();
    let snake_name = args.name.to_snake_case();

    // Parse fields
    let fields = parse_model_fields(&args.fields);

    // Create the models directory structure if it doesn't exist
    let models_dir = PathBuf::from("src/models");
    if !models_dir.exists() {
        std::fs::create_dir_all(&models_dir)?;
    }

    // Generate the model
    let model = ModelTemplate {
        name: name.clone(),
        snake_name: snake_name.clone(),
        fields,
    };

    let model_path = models_dir.join(format!("{}.rs", snake_name));
    Generate::askama(&model).forge(&model_path)?;
    println!("Generated model at: {}", model_path.display());

    // Add module declaration to mod.rs
    let model_mod = ModelModTemplate {
        name: name.clone(),
        snake_name: snake_name.clone(),
    };

    let mod_path = models_dir.join("mod.rs");
    either(Append::askama(&model_mod), Generate::askama(&model_mod)).forge(&mod_path)?;

    println!("Updated model module at: {}", mod_path.display());

    Ok(())
}

fn generate_resource(args: &ResourceArgs) -> Result<(), Box<dyn std::error::Error>> {
    // Generate both controller and model for a complete resource
    let controller_args = ControllerArgs {
        name: args.name.clone(),
        path: args.path.clone(),
    };

    let model_args = ModelArgs {
        name: args.name.clone(),
        fields: args.fields.clone(),
    };

    generate_model(&model_args)?;
    generate_controller(&controller_args)?;

    println!("Generated complete resource: {}", args.name);

    Ok(())
}

fn parse_model_fields(fields: &[String]) -> Vec<ModelField> {
    fields
        .iter()
        .filter_map(|field| {
            let parts: Vec<&str> = field.split(':').collect();
            if parts.len() == 2 {
                Some(ModelField {
                    name: parts[0].to_lower_camel_case(),
                    rust_type: map_to_rust_type(parts[1]),
                })
            } else {
                eprintln!(
                    "Warning: Invalid field format: {}. Expected name:type",
                    field
                );
                None
            }
        })
        .collect()
}

fn map_to_rust_type(input_type: &str) -> String {
    match input_type.to_lowercase().as_str() {
        "string" => "String".to_string(),
        "int" => "i32".to_string(),
        "integer" => "i32".to_string(),
        "float" => "f64".to_string(),
        "double" => "f64".to_string(),
        "bool" => "bool".to_string(),
        "boolean" => "bool".to_string(),
        "datetime" => "chrono::DateTime<chrono::Utc>".to_string(),
        "date" => "chrono::NaiveDate".to_string(),
        "uuid" => "uuid::Uuid".to_string(),
        _ => input_type.to_string(),
    }
}
