use std::error;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkError(pub &'static str);

impl fmt::Display for ChunkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ChunkError(msg) = self;
        write!(f, "{}", msg)?;
        Ok(())
    }
}

impl error::Error for ChunkError {}
