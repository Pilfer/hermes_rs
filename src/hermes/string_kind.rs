use std::io;

use crate::hermes::decode::read_bitfield;
use crate::hermes::encode::{encode_u8, write_bitfield};
use crate::hermes::Serializable;

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct StringKindEntryNew {
    pub count: u32,
    pub kind: StringKind,
}

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct StringKindEntryOld {
    pub count: u32,
    pub kind: StringKind,
}

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub enum StringKindEntry {
    Old(StringKindEntryOld),
    New(StringKindEntryNew),
}

impl Serializable for StringKindEntry {
    type Version = u32;
    /// The size of a StringKindEntry is 4 bytes, bitfields are used to store the data.
    fn size(&self) -> usize {
        match self {
            StringKindEntry::Old(entry) => entry.size(),
            StringKindEntry::New(entry) => entry.size(),
        }
    }

    fn deserialize<R>(r: &mut R, _version: u32) -> Self
    where
        R: io::Read + io::BufRead + io::Seek,
    {
        match _version {
            0..=71 => StringKindEntry::Old(StringKindEntryOld::deserialize(r, _version)),
            72_u32..=u32::MAX => StringKindEntry::New(StringKindEntryNew::deserialize(r, _version)),
        }
    }

    fn serialize<W>(&self, w: &mut W)
    where
        W: std::io::Write + std::io::Seek,
    {
        match self {
            StringKindEntry::Old(entry) => entry.serialize(w),
            StringKindEntry::New(entry) => entry.serialize(w),
        }
    }
}

impl Serializable for StringKindEntryNew {
    type Version = u32;
    /// The size of a StringKindEntry is 4 bytes, bitfields are used to store the data.
    fn size(&self) -> usize {
        4
    }

    fn deserialize<R>(r: &mut R, _version: u32) -> Self
    where
        R: io::Read + io::BufRead + io::Seek,
    {
        let mut string_kind_bytes: [u8; 4] = [0u8; 4];
        r.read_exact(&mut string_kind_bytes)
            .expect("unable to read string kind bytes");
        let count = read_bitfield(&string_kind_bytes, 0, 31);
        let kind = read_bitfield(&string_kind_bytes, 31, 1);

        StringKindEntryNew {
            kind: match kind {
                0 => StringKind::String,
                1 => StringKind::Identifier,
                2 => StringKind::Predefined,
                _ => {
                    panic!("Unknown string kind");
                }
            },
            count,
        }
    }

    fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write,
    {
        let mut string_kind_bytes: [u8; 4] = [0u8; 4];
        write_bitfield(&mut string_kind_bytes, 0, 31, self.count);
        write_bitfield(
            &mut string_kind_bytes,
            31,
            1,
            match self.kind {
                StringKind::String => 0,
                StringKind::Identifier => 1,
                StringKind::Predefined => 2,
            },
        );
        for byte in &string_kind_bytes {
            encode_u8(w, *byte);
        }
    }
}

impl Serializable for StringKindEntryOld {
    type Version = u32;

    /// The size of a StringKindEntry is 4 bytes, bitfields are used to store the data.
    fn size(&self) -> usize {
        4
    }

    fn deserialize<R>(r: &mut R, _version: u32) -> Self
    where
        R: io::Read + io::BufRead + io::Seek,
    {
        let mut string_kind_bytes: [u8; 4] = [0u8; 4];
        r.read_exact(&mut string_kind_bytes)
            .expect("unable to read string kind bytes");
        let count = read_bitfield(&string_kind_bytes, 0, 30);
        let kind = read_bitfield(&string_kind_bytes, 30, 2);

        StringKindEntryOld {
            kind: match kind {
                0 => StringKind::String,
                1 => StringKind::Identifier,
                2 => StringKind::Predefined,
                _ => {
                    panic!("Unknown string kind");
                }
            },
            count,
        }
    }

    fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write,
    {
        let mut string_kind_bytes: [u8; 4] = [0u8; 4];
        write_bitfield(&mut string_kind_bytes, 0, 30, self.count);
        write_bitfield(
            &mut string_kind_bytes,
            30,
            2,
            match self.kind {
                StringKind::String => 0,
                StringKind::Identifier => 1,
                StringKind::Predefined => 2,
            },
        );
        for byte in &string_kind_bytes {
            encode_u8(w, *byte);
        }
    }
}

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub enum StringKind {
    String,
    Identifier,
    Predefined, // unused < 0.3.0, is now "Identifier"
}
