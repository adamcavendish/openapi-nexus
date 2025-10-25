//! OpenAPI Code Generator CLI

use clap::{Parser, Subcommand};
use openapi_generator_core::CodeGenerator;
use tracing::{info, Level};
use tracing_subscriber;

#[derive(Parser)]
#[command(name = "openapi-generator")]
#[command(about = "Generate code from OpenAPI 3.1 specifications")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate code from an OpenAPI specification
    Generate {
        /// Path to the OpenAPI specification file
        #[arg(short, long)]
        input: String,
        
        /// Output directory for generated code
        #[arg(short, long, default_value = "generated")]
        output: String,
        
        /// Languages to generate code for
        #[arg(short, long, default_values = ["typescript"])]
        languages: Vec<String>,
        
        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Validate an OpenAPI specification
    Validate {
        /// Path to the OpenAPI specification file
        #[arg(short, long)]
        input: String,
        
        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Initialize logging
    let log_level = if cli.command.is_verbose() {
        Level::DEBUG
    } else {
        Level::INFO
    };
    
    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .init();

    match cli.command {
        Commands::Generate { input, output, languages, .. } => {
            info!("Starting code generation");
            info!("Input: {}", input);
            info!("Output: {}", output);
            info!("Languages: {:?}", languages);

            let generator = CodeGenerator::new();
            generator.generate_from_file(&input, &output, &languages)?;
            
            info!("Code generation completed successfully");
        }
        Commands::Validate { input, .. } => {
            info!("Validating OpenAPI specification: {}", input);
            
            // TODO: Implement validation command
            println!("Validation not yet implemented");
        }
    }

    Ok(())
}

trait Verbose {
    fn is_verbose(&self) -> bool;
}

impl Verbose for Commands {
    fn is_verbose(&self) -> bool {
        match self {
            Commands::Generate { verbose, .. } => *verbose,
            Commands::Validate { verbose, .. } => *verbose,
        }
    }
}