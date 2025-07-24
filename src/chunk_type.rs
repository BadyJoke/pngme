use core::{convert::TryFrom, str::FromStr};
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChunkTypeError {
    #[error("The chunk type is not ASCII letters")]
    NotASCIILetters,

    #[error("Name type is too long (expected {expected} bytes, got {actual}")]
    InvalidNameLenght { expected: u8, actual: usize },
}

enum ChunkTypeProperties {
    Ancillary = 0,
    Private = 1,
    Reserved = 2,
    SafeToCopy = 3,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl ChunkType {
    const CHUNK_PROPERTY_SET_MASK: u8 = 1 << 5;

    fn bit_is_zero(byte: u8) -> bool {
        // If the result is 0 means the bit is not set
        // As example:
        //
        //   1000 0111 = 135
        // & 0001 0000 = 32
        // -----------
        //   0000 0000 = 0
        byte & ChunkType::CHUNK_PROPERTY_SET_MASK == 0
    }

    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {
        Self::bit_is_zero(self.bytes[ChunkTypeProperties::Ancillary as usize])
    }

    pub fn is_public(&self) -> bool {
        Self::bit_is_zero(self.bytes[ChunkTypeProperties::Private as usize])
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        Self::bit_is_zero(self.bytes[ChunkTypeProperties::Reserved as usize])
    }

    pub fn is_safe_to_copy(&self) -> bool {
        !Self::bit_is_zero(self.bytes[ChunkTypeProperties::SafeToCopy as usize])
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = crate::Error;

    fn try_from(value: [u8; 4]) -> crate::Result<Self> {
        match value.into_iter().all(|val| val.is_ascii_alphabetic()) {
            true => Ok(ChunkType { bytes: value }),
            false => Err(Box::new(ChunkTypeError::NotASCIILetters)),
        }
    }
}

impl FromStr for ChunkType {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        let bytes: [u8; 4] = s.as_bytes().try_into().map_err(|_| {
            Box::new(ChunkTypeError::InvalidNameLenght {
                expected: 4,
                actual: s.len(),
            })
        })?;
        ChunkType::try_from(bytes)
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let repr: String = self.bytes.iter().map(|b| char::from(*b)).collect();
        write!(f, "{}", repr)
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
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
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
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

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
