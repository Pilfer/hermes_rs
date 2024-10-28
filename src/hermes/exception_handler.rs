use std::io;

use crate::hermes::decode::decode_u32;
use crate::hermes::encode::encode_u32;
use crate::hermes::Serializable;

#[derive(Debug, Clone)]
pub struct ExceptionHandlerInfo {
    pub start: u32,
    pub end: u32,
    pub target: u32,
}

impl Serializable for ExceptionHandlerInfo {
    type Version = u32;

    fn size(&self) -> usize {
        12
    }

    fn deserialize<R>(r: &mut R, _version: u32) -> Self
    where
        R: io::Read + io::BufRead + io::Seek,
    {
        let start = decode_u32(r);
        let end = decode_u32(r);
        let target = decode_u32(r);

        ExceptionHandlerInfo { start, end, target }
    }

    fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write,
    {
        encode_u32(w, self.start);
        encode_u32(w, self.end);
        encode_u32(w, self.target);
    }
}
