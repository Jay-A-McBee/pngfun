use std::path::PathBuf;
use std::{error, fmt, fs, io};

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::Result;

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
pub struct Commands;

impl Commands {
    fn write_file(file_path: String, contents: Vec<u8>) -> io::Result<()> {
        let path_buf = PathBuf::from(file_path);
        fs::write(path_buf, contents)
    }

    fn convert_to_4_byte_array(val: &String) -> [u8; 4] {
        let bytes = val.as_bytes();
        [bytes[0], bytes[1], bytes[2], bytes[3]]
    }

    fn convert_file_to_png(file_path: &String) -> Result<Png> {
        let path_buf = PathBuf::from(file_path);
        Png::try_from(path_buf)
    }

    pub fn encode(
        file_path: &String,
        chunk_type: &String,
        message: &String,
        output_path: &Option<String>,
    ) -> Result<()> {
        let mut png = Self::convert_file_to_png(file_path)?;

        let b_chunk_type = Self::convert_to_4_byte_array(&chunk_type);
        let chunk_type = ChunkType::try_from(b_chunk_type)?;
        let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());

        png.append_chunk(chunk);

        let write_path = output_path.as_ref().unwrap_or(file_path);
        Self::write_file(write_path.to_string(), png.as_bytes())?;
        Ok(())
    }

    pub fn decode(file_path: &String, chunk_type: &String) -> Result<()> {
        let png = Self::convert_file_to_png(file_path)?;

        if let Some(chunk) = png.chunk_by_type(chunk_type.as_str()) {
            println!(
                "Chunk type decoded as the following message: \n{}",
                chunk.data_as_string().unwrap()
            );
            Ok(())
        } else {
            Err(Box::new(CommandErrors::Decode("chunk_type not found.")))
        }
    }

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

    pub fn print(file_path: &String) -> Result<()> {
        let png = Self::convert_file_to_png(file_path)?;
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
