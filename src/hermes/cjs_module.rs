use std::io;

use crate::hermes::decode::decode_u32;
use crate::hermes::encode::encode_u32;
use crate::hermes::Serializable;

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub enum CJSModule {
    CJSModuleInt(CJSModuleInt),
    CJSModuleEntry(CJSModuleEntry),
}

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct CJSModuleEntry {
    pub symbol_id: u32,
    pub offset: u32,
}

impl Serializable for CJSModuleEntry {
    type Version = u32;
    fn size(&self) -> usize {
        8
    }

    fn deserialize<R>(r: &mut R, _version: u32) -> Self
    where
        R: io::Read + io::BufRead + io::Seek,
    {
        CJSModuleEntry {
            symbol_id: decode_u32(r),
            offset: decode_u32(r),
        }
    }

    fn serialize<W>(&self, w: &mut W)
    where
        W: std::io::Write + std::io::Seek,
    {
        encode_u32(w, self.symbol_id);
        encode_u32(w, self.offset);
    }
}

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct CJSModuleInt {
    pub value: u32,
}

impl Serializable for CJSModuleInt {
    type Version = u32;
    fn size(&self) -> usize {
        4
    }

    fn deserialize<R>(r: &mut R, _version: u32) -> Self
    where
        R: io::Read + io::BufRead + io::Seek,
    {
        CJSModuleInt {
            value: decode_u32(r),
        }
    }

    fn serialize<W>(&self, w: &mut W)
    where
        W: std::io::Write + std::io::Seek,
    {
        encode_u32(w, self.value);
    }
}
