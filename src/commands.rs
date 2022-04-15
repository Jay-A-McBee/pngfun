use std::path::PathBuf;
use std::{error, fmt, fs, io};

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::Result;
use reqwest::blocking::Client;

pub enum FileType {
    Local(String),
    Url(String),
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum CommandErrors {
    Encode(&'static str),
    Decode(&'static str),
}

impl error::Error for CommandErrors {}

impl fmt::Display for CommandErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = match self {
            CommandErrors::Encode(msg) => msg,
            CommandErrors::Decode(msg) => msg,
        };

        write!(f, "{}", error)
    }
}

#[derive(Debug)]
pub struct Commands {}

impl Commands {
    /// Writes the altered png contents to disk.
    ///
    /// Location is output_path if defined or file_path
    /// of original png file.
    fn write_file(file_path: String, contents: Vec<u8>) -> io::Result<()> {
        let path_buf = PathBuf::from(file_path);
        fs::write(path_buf, contents)
    }

    fn convert_to_4_byte_array(val: &String) -> [u8; 4] {
        let bytes = val.as_bytes();
        [bytes[0], bytes[1], bytes[2], bytes[3]]
    }

    /// Converts an http/https url to a Png struct.
    ///
    ///
    fn convert_url_to_png(url: &String) -> Result<Png> {
        println!("\nMaking a request to {}", url);
        let client = Client::new();
        let resp = client.get(url).send()?;
        let bytes = resp.bytes()?.to_vec();
        Png::try_from(bytes.as_slice())
    }

    /// Converts a local png file to a Png struct.
    fn convert_file_to_png(file_path: &String) -> Result<Png> {
        let path_buf = PathBuf::from(file_path);
        Png::try_from(path_buf)
    }

    /// Convert passed file_path arg to a Png.
    ///
    /// Accepts local and absolute system file paths
    /// as well as http/https urls.
    ///
    /// Not Async -> TODO: Make this async
    fn convert_to_png(file_path: &String) -> Result<Png> {
        match Self::convert_to_file_type(file_path) {
            FileType::Url(url) => Self::convert_url_to_png(file_path),
            FileType::Local(file) => Self::convert_file_to_png(&file),
        }
    }

    /// Converts passed file_path arg to FileType enum variant.
    fn convert_to_file_type(file_path: &String) -> FileType {
        if file_path.starts_with("http") || file_path.starts_with("https") {
            return FileType::Url(file_path.to_string());
        }

        FileType::Local(file_path.to_string())
    }

    /// Encodes the passed message into the png file
    /// located at the file_path arg.
    ///
    /// Writes the altered png file to disk at
    /// the passed output_path (if defined) or
    /// the original file_path.
    pub fn encode(
        file_path: &String,
        chunk_type: &String,
        message: &String,
        output_path: &Option<String>,
    ) -> Result<()> {
        let mut png = Self::convert_to_png(file_path)?;

        let b_chunk_type = Self::convert_to_4_byte_array(&chunk_type);
        let chunk_type = ChunkType::try_from(b_chunk_type)?;
        let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());

        png.append_chunk(chunk);

        let write_path = output_path.as_ref().unwrap_or(file_path);
        Self::write_file(write_path.to_string(), png.as_bytes())?;
        Ok(())
    }

    /// Searches a png file for a specific message type (ex. tEXt).
    ///
    /// Prints the text contained in each chunk or not found message.
    pub fn decode(file_path: &String, chunk_type: &String) -> Result<()> {
        let png = Self::convert_to_png(file_path)?;

        let found = png.chunk_by_type(chunk_type.as_str());

        if found.is_empty() {
            return Err(Box::new(CommandErrors::Decode(
                "chunk_type not found.",
            )));
        }

        let messages = found
            .iter()
            .map(|&chunk| chunk.data_as_string().unwrap())
            .collect::<Vec<String>>();

        println!(
            "Chunk type decoded as the following message: \n{}",
            messages.as_slice().join("\n")
        );

        Ok(())
    }

    /// Removes a specific chunk type from the png file and
    /// overwrites the original file.
    ///
    /// Prints removed chunk or not found message.
    pub fn remove(file_path: &String, chunk_type: &String) -> Result<()> {
        let mut png = Self::convert_file_to_png(file_path)?;

        if let Some(chunk) = png.remove_chunk(chunk_type) {
            Self::write_file(file_path.to_string(), png.as_bytes())?;
            println!("Removed the following chunk:\n{}", chunk.to_string());
        } else {
            return Err(Box::new(CommandErrors::Decode(
                "chunk_type not found.",
            )));
        }

        Ok(())
    }

    /// Prints the contents of png file as Chunks
    pub fn print(file_path: &String) -> Result<()> {
        let png = Self::convert_to_png(file_path)?;
        let formatted_png = format!("{}", png);

        println!("________________________________________________\n");
        println!("Total Chunks: {}\n", png.chunks.len());
        println!(
            "Png file at path {} has the following chunks:\n\n{}",
            file_path, formatted_png
        );
        println!("________________________________________________");

        Ok(())
    }
}
