mod ast;
mod codegen;
mod compiler;
mod hir;
mod index_vec;
mod mir;
mod types;

use std::env::current_dir;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;

use clap::{Parser, Subcommand};

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
            let root = match target_dir {
                Some(root) => root.clone(),
                None => match current_dir() {
                    Ok(root) => root,
                    Err(e) => panic!("failed to get target directory {:?}", e),
                },
            };

            let mut ctx = Context::new();
            let mut builder = ctx.create_builder();
            let mut output_file = fs::File::create("exec")?;
            builder.compile(root.as_path(), &mut output_file);
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
