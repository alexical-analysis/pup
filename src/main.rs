mod ast;
mod compiler;
mod types;

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

use crate::ast::parser::Module;
use crate::compiler::context::Context;

// the CLI for the Pup programming language
#[derive(Parser, Debug)]
#[command(name = "pup")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Compile the project and produce an executable")]
    Build {
        #[arg(short, long, value_name = "TARGET_DIR")]
        target_dir: Option<PathBuf>,
    },
    #[command(about = "Init a new Pup project in the current directory")]
    Init {
        #[arg(value_name = "MOD_NAME")]
        mod_name: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build { target_dir } => {
            // TODO: need to actually have this live in the compiler module
            let mut ctx = Context::new();
            // TODO: we should actually be getting to the parser/ ast module through the builder
            let mut ast_module = Module::new(&mut ctx);
            let mut parser = ast_module.create_parser();
            parser.parse(
                r#"mod main
                fn main() {
                    print("Hello Pup!")
                }
            "#,
            );

            // TODO: The goal here is that the ast_module is now populated...
        }
        Commands::Init { mod_name } => {
            if Path::new("pup.mod").exists() {
                println!("pup.mod file already exists");
                return Ok(());
            }

            fs::write("pup.mod", mod_name)?;
        }
    }

    Ok(())
}
