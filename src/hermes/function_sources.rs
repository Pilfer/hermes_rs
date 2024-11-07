use std::io;

use crate::hermes::decode::decode_u32;
use crate::hermes::encode::encode_u32;
use crate::hermes::Serializable;

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct FunctionSourceEntry {
    pub function_id: u32,
    pub string_id: u32,
}

impl Serializable for FunctionSourceEntry {
    type Version = u32;
    fn size(&self) -> usize {
        8
    }

    fn deserialize<R>(r: &mut R, _version: u32) -> Self
    where
        R: io::Read + io::BufRead + io::Seek,
    {
        let function_id = decode_u32(r);
        let string_id = decode_u32(r);
        FunctionSourceEntry {
            function_id,
            string_id,
        }
    }

    fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write,
    {
        encode_u32(w, self.function_id);
        encode_u32(w, self.string_id);
    }
}
