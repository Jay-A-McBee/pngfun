use clap::{Args, Parser, Subcommand};
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Hide a message in a PNG file
    Encode {
        /// relative path of png file
        file_path: String,
        /// 4 ascii character string chunk type ex. RuST
        chunk_type: String,
        /// message to hide in png
        message: String,
        /// optional write path for final png file
        output_path: Option<String>,
    },
    /// Find a message in a PNG file
    Decode {
        /// relative path of png file
        file_path: String,
        /// 4 ascii character string chunk type ex. RuST
        chunk_type: String,
    },

    /// Remove a hidden message from a PNG file
    Remove {
        /// relative path of png file
        file_path: String,
        /// 4 ascii character string chunk type ex. RuST
        chunk_type: String,
    },
    /// print the contents of a PNG file
    Print {
        /// relative path of png file
        file_path: String,
    },
}
