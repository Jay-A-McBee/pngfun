use std::path::PathBuf;
use std::{error, fmt, fs, io};

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::Result;

#[derive(Debug)]
pub enum CommandErrors {
    Encode(&'static str),
    Decode(&'static str),
    Remove(&'static str),
    Print(&'static str),
}

impl error::Error for CommandErrors {}

impl fmt::Display for CommandErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = match self {
            CommandErrors::Encode(msg) => msg,
            CommandErrors::Decode(msg) => msg,
            CommandErrors::Remove(msg) => msg,
            CommandErrors::Print(msg) => msg,
        };

        write!(f, "{}", error)
    }
}
#[derive(Debug)]
pub struct Commands;

impl Commands {
    fn read_file(file_path: &String) -> io::Result<Vec<u8>> {
        let path = PathBuf::from(file_path);
        fs::read(path)
    }

    fn write_file(file_path: String, contents: Vec<u8>) -> io::Result<()> {
        let path_buf = PathBuf::from(file_path);
        fs::write(path_buf, contents)?;
        Ok(())
    }

    fn convert_to_4_byte_array(val: &String) -> [u8; 4] {
        let bytes = val.as_bytes();
        [bytes[0], bytes[1], bytes[2], bytes[3]]
    }

    pub fn encode(
        file_path: &String,
        chunk_type: &String,
        message: &String,
        output_path: &Option<String>,
    ) -> Result<()> {
        let contents = Self::read_file(&file_path)?;

        let b_chunk_type = Self::convert_to_4_byte_array(&chunk_type);
        let chunk_type = ChunkType::try_from(b_chunk_type)?;
        let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());

        let mut png = Png::try_from(contents.as_slice())?;
        png.append_chunk(chunk);

        let write_path = output_path.as_ref().unwrap_or(file_path);
        Self::write_file(write_path.to_string(), png.as_bytes())?;
        Ok(())
    }

    pub fn decode(file_path: &String, chunk_type: &String) -> Result<()> {
        let contents = Self::read_file(&file_path)?;
        let png = Png::try_from(contents.as_slice())?;

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
        let contents = Self::read_file(&file_path)?;
        let mut png = Png::try_from(contents.as_slice())?;

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
        let contents = Self::read_file(&file_path)?;
        let png = Png::try_from(contents.as_slice())?;
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
