use clap::Parser;

mod chunk;
mod chunk_error;
mod chunk_type;
mod cli;
mod commands;
mod file_type;
mod png;

use crate::cli::{Cli, Command};
use crate::commands::Commands;

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
        } => Commands::encode(file_path, chunk_type, message, output_path)?,
        Command::Decode {
            file_path,
            chunk_type,
        } => Commands::decode(file_path, chunk_type)?,
        Command::Remove {
            file_path,
            chunk_type,
        } => Commands::remove(file_path, chunk_type)?,
        Command::Print { file_path } => Commands::print(file_path)?,
    }

    Ok(())
}
