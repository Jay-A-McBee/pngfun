use clap::Parser;

mod args;
mod chunk;
mod chunk_error;
mod chunk_type;
mod commands;
mod png;

use crate::args::{Cli, Command};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::Encode {
            file_path,
            chunk_type,
            message,
            output_path,
        } => {
            todo!()
        }
        Command::Decode {
            file_path,
            chunk_type,
        } => {
            todo!()
        }
        Command::Remove {
            file_path,
            chunk_type,
        } => {
            todo!()
        }
        Command::Print { file_path } => {
            todo!()
        }
    }

    Ok(())
}
