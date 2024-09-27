use std::io;

use crate::hermes::decode::{decode_u32, read_bitfield};
use crate::hermes::encode::{encode_u32, encode_u8, write_bitfield};
use crate::hermes::Serializable;

#[derive(Debug)]
pub struct SmallStringTableEntry {
    pub is_utf_16: bool,
    pub offset: u32,
    pub length: u32,
}

impl Serializable for SmallStringTableEntry {
    type Version = u32;
    /// The size of a SmallStringTableEntry is 4 bytes, bitfields are used to store the data.
    fn size(&self) -> usize {
        4
    }

    fn deserialize<R>(r: &mut R, _version: u32) -> Self
    where
        R: io::Read + io::BufRead + io::Seek,
    {
        let mut string_storage_bytes = [0u8; 4];
        r.read_exact(&mut string_storage_bytes)
            .expect("unable to read string storage bytes");
        let is_utf_16 = read_bitfield(&string_storage_bytes, 0, 1);
        let offset = read_bitfield(&string_storage_bytes, 1, 23);
        let length = read_bitfield(&string_storage_bytes, 24, 8);

        SmallStringTableEntry {
            is_utf_16: is_utf_16 == 1,
            offset,
            length,
        }
    }

    fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write,
    {
        let mut string_storage_bytes = [0u8; 4];
        write_bitfield(
            &mut string_storage_bytes,
            0,
            1,
            if self.is_utf_16 { 1 } else { 0 },
        );
        write_bitfield(&mut string_storage_bytes, 1, 23, self.offset);
        write_bitfield(&mut string_storage_bytes, 24, 8, self.length);
        for byte in &string_storage_bytes {
            encode_u8(w, *byte);
        }
    }
}

#[derive(Debug)]
pub struct OverflowStringTableEntry {
    pub offset: u32,
    pub length: u32,
}

impl Serializable for OverflowStringTableEntry {
    type Version = u32;
    fn size(&self) -> usize {
        8
    }

    fn deserialize<R>(r: &mut R, _version: u32) -> Self
    where
        R: io::Read + io::BufRead + io::Seek,
    {
        OverflowStringTableEntry {
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
