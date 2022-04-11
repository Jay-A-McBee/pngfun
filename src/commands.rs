use std::fs;
use std::io;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::Result;
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
        file_path: String,
        chunk_type: String,
        message: String,
        output_path: Option<String>,
    ) -> Result<()> {
        let contents = Self::read_file(&file_path);

        if contents.is_ok() {
            let b_chunk_type = Self::convert_to_4_byte_array(&chunk_type);
            let chunk_type = ChunkType::try_from(b_chunk_type)?;
            let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());
            let mut png = Png::try_from(contents.unwrap().as_slice())?;
            png.append_chunk(chunk);

            let write_path = output_path.unwrap_or(file_path);
            Self::write_file(write_path, png.as_bytes())?;
            Ok(())
        } else {
            Err(Box::new(contents.unwrap_err()))
        }
    }
}
