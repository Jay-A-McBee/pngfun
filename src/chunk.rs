use crate::chunk_error::ChunkError;
use crate::chunk_type::ChunkType;
use crc::{Crc, CRC_32_ISO_HDLC};
use std::fmt;
use std::io;
use std::io::Read;
use std::str;

use crate::{Error, Result};

#[derive(Debug, Clone)]
pub struct Chunk {
    pub chunk_type: ChunkType,
    pub data: Vec<u8>,
    pub crc: u32,
}

const CRC_GEN: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

impl Chunk {
    /// creates a new Chunk struct
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let all_bytes = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect::<Vec<u8>>();

        Chunk {
            chunk_type,
            data,
            crc: CRC_GEN.checksum(&all_bytes),
        }
    }

    /// return the length of Chunk.data
    pub fn length(&self) -> u32 {
        self.data.len() as u32
    }

    /// returns a reference to Chunk.chunk_type
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    /// returns a reference to Chunk.data
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// returns chunk.crc
    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        let data = str::from_utf8(&self.data).unwrap();
        Ok(data.to_string())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let len_bytes = self.data.len() as u32;
        // println!("CRC::{}", self.crc);
        len_bytes
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type().bytes().iter())
            .chain(self.data.iter())
            .chain((self.crc as u32).to_be_bytes().iter())
            .copied()
            .collect()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        println!("bytes{:?}", bytes);

        let mut reader = io::BufReader::new(bytes);
        let mut b_length = [0; 4];
        let mut b_type = [0; 4];
        let mut b_crc = [0; 4];
        // subtract known 4 byte sections (length, chunk_type, crc) from total length
        // to determine data length.
        let mut b_data = vec![0; bytes.len() - 12];

        reader.read_exact(&mut b_length)?;
        reader.read_exact(&mut b_type)?;
        reader.read_exact(&mut b_data)?;
        reader.read_exact(&mut b_crc)?;

        println!("type{:?}::crc{:?}", b_type, b_crc);

        match ChunkType::try_from(b_type) {
            Ok(chunk_type) => {
                let chunk = Chunk::new(chunk_type, b_data);

                if chunk.crc() != u32::from_be_bytes(b_crc) {
                    // println!("in crc failure {:?}", chunk);
                    return Err(Box::new(ChunkError("Crc does not match")));
                }

                Ok(chunk)
            }
            Err(msg) => Err(msg),
        }
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes =
            "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string =
            String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes =
            "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string =
            String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes =
            "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes =
            "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        let _chunk_string = format!("{}", chunk);
        // println!("{}", _chunk_string);
    }
}
