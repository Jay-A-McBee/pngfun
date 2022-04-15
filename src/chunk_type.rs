use std::convert::TryFrom;
use std::error;
use std::fmt;
use std::str::FromStr;

use crate::chunk_error::ChunkError;
use crate::{Error, Result};

/// A validated PNG chunk type. See the PNG spec for more details.
/// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    chunk: [u8; 4],
}
#[allow(dead_code)]
impl ChunkType {
    /// returns Chunktype.chunk
    pub fn bytes(&self) -> [u8; 4] {
        self.chunk
    }
    /// ChunkType.chunk[0] is_ascii_uppercase
    pub fn is_critical(&self) -> bool {
        self.chunk[0].is_ascii_uppercase()
    }
    /// ChunkType.chunk[1] is_ascii_uppercase
    pub fn is_public(&self) -> bool {
        self.chunk[1].is_ascii_uppercase()
    }
    /// ChunkType.chunk[3] is_ascii_lowercase
    pub fn is_safe_to_copy(&self) -> bool {
        self.chunk[3].is_ascii_lowercase()
    }

    pub fn is_reserved_bit_valid(byte: u8) -> bool {
        byte.is_ascii_uppercase()
    }

    pub fn is_valid_byte(byte: &u8) -> bool {
        byte.is_ascii_lowercase() || byte.is_ascii_uppercase()
    }

    pub fn is_valid(chunk: &[u8]) -> bool {
        ChunkType::is_reserved_bit_valid(chunk[2])
            && !chunk.iter().any(|b| !ChunkType::is_valid_byte(b))
    }
}

impl error::Error for ChunkType {}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;
    fn try_from(chunk: [u8; 4]) -> Result<Self> {
        if !ChunkType::is_valid(&chunk) {
            return Err(Box::new(ChunkError("Chunk bytes are invalid")));
        }

        Ok(ChunkType { chunk })
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(chunk_str: &str) -> Result<Self> {
        let len = chunk_str.len();

        if len != 4 {
            return Err(Box::new(ChunkError(
                "The string must be 4 characters long.",
            )));
        }

        let bytes = chunk_str.as_bytes();

        if !ChunkType::is_reserved_bit_valid(bytes[2]) {
            return Err(Box::new(ChunkError(
                "Error: Third character must be uppercase.",
            )));
        }

        Ok(ChunkType {
            chunk: [bytes[0], bytes[1], bytes[2], bytes[3]],
        })
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.chunk).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = "RuSt".as_bytes();
        assert!(ChunkType::is_reserved_bit_valid(chunk[2]));
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt");
        assert!(chunk.is_ok());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);

        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
