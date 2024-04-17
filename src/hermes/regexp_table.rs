use std::io;

use crate::hermes::Serializable;
use crate::hermes::decode::decode_u32;
use crate::hermes::encode::encode_u32;

#[derive(Debug)]
pub struct RegExpTableEntry {
    pub offset: u32,
    pub length: u32,
}

impl Serializable for RegExpTableEntry {
    fn size(&self) -> usize {
        8
    }

    fn deserialize<R>(r: &mut R) -> Self
    where
        R: io::Read + io::BufRead + io::Seek,
    {
        RegExpTableEntry {
            offset: decode_u32(r),
            length: decode_u32(r),
        }
    }

    fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write,
    {
        encode_u32(w, self.offset);
        encode_u32(w, self.length);
    }
}