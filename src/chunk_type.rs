use std::fmt;
use std::str::FromStr;
use std::convert::TryFrom;
use crate::{Error, Result};

/// 
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChunkType {
    chunk_type: [u8; 4],    //块类型
}


impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self> {
        for char in value {
            if char < b'A' || char > b'z' || (char > b'Z' && char < b'a') {
                return Err("Out of range".into());
            }
        }
        let chunk = ChunkType { chunk_type: value };
        return Ok(chunk);
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut chunk_type: Vec<u8> = Vec::with_capacity(4);
        if s.len() != 4 {
            let err = format!("Out of range, need len 4 found len {}", s.len());
            return Err(err.into());
        }
        for s in s.chars() {
            let s = s as u8;
            if s < b'A' || s > b'z' || (s > b'Z' && s < b'a')  {
                return Err("Unrecognized type".into());
            }
            chunk_type.push(s as u8);
        }
        let chunk_type = chunk_type.try_into().unwrap();
        let chunk = ChunkType { chunk_type: chunk_type };
        return Ok(chunk);
    }
}

impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let chunk_type = self.chunk_type;
        let res;
        unsafe{
            res = std::str::from_utf8_unchecked(&chunk_type);
        }
        write!(f, "{}", res)
    }
}

/// 实现 PNG 块检测功能
///   bLOb <-- 以文本形式表示的 32 位块类型代码
///   |||| 
///   |||+- 安全复制位为 1（小写字母；位 5 为 1）
///   ||+-- 保留位为 0（大写字母；位 5 为 0）
///   |+--- 私有位为 0（大写字母；第 5 位为 0) 
///   +---- 辅助位为 1（小写字母；第 5 位为 1）
/// 详情参考: `http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html`
/// 
impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        return self.chunk_type;
    }

    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    fn is_critical(&self) -> bool {
        let critical: u8 = self.chunk_type[0];
        u8::is_ascii_uppercase(&critical)
    }

    fn is_public(&self) -> bool {
        let public: u8 = self.chunk_type[1];
        u8::is_ascii_uppercase(&public)
    }

    fn is_reserved_bit_valid(&self) -> bool {
        let reserved: u8 = self.chunk_type[2];
        u8::is_ascii_uppercase(&reserved)
    }

    fn is_safe_to_copy(&self) -> bool {
        let safe: u8 = self.chunk_type[3];
        u8::is_ascii_lowercase(&safe)
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

    #[test]
    fn test_stract() {
        let x = -100;
        let y = i32::abs(x);
        println!("{}", y);

        let m: i32 = -100;
        let n = m.abs();
        println!("{}", n);
    }
}