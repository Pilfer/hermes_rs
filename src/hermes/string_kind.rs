use std::io;

use crate::hermes::Serializable;
use crate::hermes::decode::read_bitfield;
use crate::hermes::encode::{write_bitfield, encode_u8};

#[derive(Debug)]
pub struct StringKindEntry {
    pub count: u32,
    pub kind: StringKind,
}

impl Serializable for StringKindEntry {
    /// The size of a StringKindEntry is 4 bytes, bitfields are used to store the data.
    fn size(&self) -> usize {
        4
    }

    fn deserialize<R>(r: &mut R) -> Self
    where
        R: io::Read + io::BufRead + io::Seek,
    {
        let mut string_kind_bytes: [u8; 4] = [0u8; 4];
        r.read_exact(&mut string_kind_bytes).expect("unable to read string kind bytes");
        let count = read_bitfield(&string_kind_bytes, 0, 31);
        let kind = read_bitfield(&string_kind_bytes, 31, 1);

        StringKindEntry {
            kind: match kind {
                0 => StringKind::String,
                1 => StringKind::Identifier,
                2 => StringKind::Predefined,
                _ => {
                    panic!("Unknown string kind");
                }
            },
            count: count,
        }
    }

    fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write,
    {
        let mut string_kind_bytes: [u8; 4] = [0u8; 4];
        write_bitfield(&mut string_kind_bytes, 0, 31, self.count);
        write_bitfield(&mut string_kind_bytes, 31, 1, match self.kind {
            StringKind::String => 0,
            StringKind::Identifier => 1,
            StringKind::Predefined => 2,
        });
        for byte in &string_kind_bytes {
            encode_u8(w, *byte);
        }
    }
}

#[derive(Debug)]
pub enum StringKind {
    String,
    Identifier,
    Predefined, // unused < 0.3.0, is now "Identifier"
}