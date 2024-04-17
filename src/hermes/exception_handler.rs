use std::io;

use crate::hermes::Serializable;
use crate::hermes::decode::decode_u32;
use crate::hermes::encode::encode_u32;

#[derive(Debug)]
pub struct ExceptionHandlerInfo {
    pub start: u32,
    pub end: u32,
    pub target: u32,
}


impl Serializable for ExceptionHandlerInfo {
    fn size(&self) -> usize {
        8
    }

    fn deserialize<R>(r: &mut R) -> Self
    where
        R: io::Read + io::BufRead + io::Seek,
    {
        let start = decode_u32(r);
        let end = decode_u32(r);
        let target = decode_u32(r);
        
        ExceptionHandlerInfo {
            start: start,
            end: end,
            target: target,
        }
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
