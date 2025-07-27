use crate::chunk_type::{ChunkType, ChunkTypeError};
use crc::Crc;
use std::{
    fmt::Display,
    io::{self, BufReader, Read},
    string::FromUtf8Error,
};
use thiserror::Error;

const MIN_CHUNK_SIZE: u32 = 12;

#[derive(Debug, PartialEq, Eq)]
pub struct Chunk {
    data: Vec<u8>,
    chunk_type: ChunkType,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        const CRC_ALG: Crc<u32> = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);

        let crc_bytes: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .cloned()
            .collect();

        let crc = CRC_ALG.checksum(&crc_bytes);

        Self {
            data,
            chunk_type,
            crc,
        }
    }

    pub fn length(&self) -> u32 {
        self.data.len() as u32
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.data.clone())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let length = self.data.len() as u32;
        length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }
}

#[derive(Error, Debug)]
pub enum ChunkParserError {
    #[error(transparent)]
    ReaderError(#[from] io::Error),

    #[error("chunk did not contain all the required data")]
    Incomplete,

    #[error("invalid length field (expected {expected:?}, found {found:?})")]
    InvalidLengthField { expected: u32, found: u32 },

    #[error(transparent)]
    InvalidChunkType(#[from] ChunkTypeError),

    #[error("parsed checksum didn't match calculated checksum")]
    InvalidChecksum,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = ChunkParserError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // length (4) + type (4) + data (0) + crc (4) => 12
        // 12 is the smallest chunk that can exist. By checking the length
        // beforehand we can ensure that there will be no panics.
        if value.len() < 12 {
            return Err(ChunkParserError::Incomplete);
        }

        let mut reader = BufReader::new(value);
        let mut buffer: [u8; 4] = [0, 0, 0, 0];

        // The bytes are represented as follows:
        // +-------------+------------+-------------------+---------+
        // | Data Length | Chunk Type |       Data        |   CRC   |
        // +-------------+------------+-------------------+---------+
        // | 4 bytes     | 4 bytes    | Data Length bytes | 4 bytes |
        // +-------------+------------+-------------------+---------+

        // Read the first for byte that containes data lenght
        reader.read_exact(&mut buffer)?;
        let data_lenght = u32::from_be_bytes(buffer);

        if (value.len() as u32) - MIN_CHUNK_SIZE != data_lenght {
            return Err(ChunkParserError::InvalidLengthField {
                expected: value.len() as u32 - 12,
                found: data_lenght,
            });
        }

        // Then read the chunk type
        reader.read_exact(&mut buffer)?;
        let chunk_type = ChunkType::try_from(buffer)?;

        // We create a buffer of size Data Length
        let mut data_buffer = vec![0u8; data_lenght as usize];
        reader.read_exact(&mut data_buffer)?;

        // Then read the CRC
        reader.read_exact(&mut buffer)?;
        let crc = u32::from_be_bytes(buffer);

        let chunk = Self::new(chunk_type, data_buffer);

        if crc != chunk.crc() {
            return Err(ChunkParserError::InvalidChecksum);
        }

        Ok(chunk)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ length: {:4} type: {}, data: {}, crc {:10} }}",
            self.length(),
            self.chunk_type,
            self.data_as_string()
                .unwrap_or_else(|_| "<Invalid UTF-8>".to_string()),
            self.crc
        )
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
        let message_bytes = "This is where your secret message will be!".as_bytes();
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
        let expected_chunk_string = String::from("This is where your secret message will be!");
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
        let message_bytes = "This is where your secret message will be!".as_bytes();
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
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
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
        let message_bytes = "This is where your secret message will be!".as_bytes();
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
    }
}
