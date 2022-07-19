use crate::chunk_type::ChunkType;
use crate::{Error, Result};
use crc;
use std::fmt;

#[derive(Debug)]
pub struct Chunk {
    len: u32,
    chunk_type: ChunkType,
    pub chunk_data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let crc: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .cloned()
            .chain(data.iter().cloned())
            .collect();
        let crc: u32 = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(&crc);
        Chunk {
            len: data.len() as u32,
            chunk_type: chunk_type,
            chunk_data: data,
            crc: crc,
        }
    }

    fn length(&self) -> u32 {
        self.len
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.chunk_data
    }

    fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        let data = self.chunk_data.to_owned();
        let res = match std::str::from_utf8(data.as_slice()) {
            Ok(s) => s.to_owned(),
            Err(_) => return Err("转换UTF8 错误!".into()),
        };
        Ok(res)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let len = self.len;
        let chunk_type = self.chunk_type.bytes();
        let chunk_data = self.chunk_data.as_slice();
        let crc = self.crc;
        len.to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(chunk_data.iter())
            .chain(crc.to_be_bytes().iter())
            .cloned()
            .collect()
    }
}

impl std::convert::TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let slice_len: [u8; 4] = value[0..4].try_into()?;
        let len: u32 = u32::from_be_bytes(slice_len);

        let slice_chunk_type: [u8; 4] =
            value[4..8].try_into().expect("slice with incorrect length");
        let chunk_type = ChunkType::try_from(slice_chunk_type)?;

        let chunk_data = if (value.len() - 4) <= 8 {
            Vec::with_capacity(0)
        } else {
            value[8..value.len() - 4].try_into()?
        };

        let slice_crc: [u8; 4] = value[(value.len() - 4)..].try_into()?;
        let crc: u32 = u32::from_be_bytes(slice_crc);

        let crc_check: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .cloned()
            .chain(chunk_data.iter().cloned())
            .collect();
        let valid_crc = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(&crc_check);

        if crc != valid_crc {
            return Err("crc check fail!".into());
        }

        let chunk = Chunk {
            len,
            chunk_type,
            chunk_data,
            crc,
        };
        Ok(chunk)
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let chunk_type = self.chunk_type.bytes();
        let res;
        unsafe {
            res = std::str::from_utf8_unchecked(&chunk_type);
        }
        write!(f, "{}", res)
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
