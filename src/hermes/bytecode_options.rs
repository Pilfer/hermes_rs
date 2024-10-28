use std::io;

use crate::hermes::decode::decode_u8;
use crate::hermes::encode::encode_u8;
use crate::hermes::Serializable;

#[derive(Debug)]
pub struct BytecodeOptions {
    pub static_builtins: bool,
    pub cjs_modules_statically_resolved: bool,
    pub has_async: bool,
    pub flags: bool,
}

impl BytecodeOptions {
    pub fn new() -> Self {
        Self {
            static_builtins: false,
            cjs_modules_statically_resolved: false,
            has_async: false,
            flags: false,
        }
    }
}

impl Default for BytecodeOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl Serializable for BytecodeOptions {
    type Version = u32;
    /// The size of a BytecodeOptions is 1 byte.  Bitfields are used to store the data.
    fn size(&self) -> usize {
        1
    }

    fn deserialize<R>(r: &mut R, _version: u32) -> Self
    where
        R: io::Read + io::BufRead + io::Seek,
    {
        let bytecode_options_byte: u8 = decode_u8(r);
        let static_builtins: bool = bytecode_options_byte >> 1 & 1 == 1;
        let cjs_modules_statically_resolved: bool = bytecode_options_byte >> 1 & 1 == 1;
        let has_async: bool = bytecode_options_byte >> 1 & 1 == 1;
        let flags: bool = bytecode_options_byte >> 1 & 1 == 1;

        BytecodeOptions {
            static_builtins,
            cjs_modules_statically_resolved,
            has_async,
            flags,
        }
    }

    fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write,
    {
        let mut bytecode_options_byte: u8 = 0;
        if self.static_builtins {
            bytecode_options_byte |= 1 << 1;
        }
        if self.cjs_modules_statically_resolved {
            bytecode_options_byte |= 1 << 1;
        }
        if self.has_async {
            bytecode_options_byte |= 1 << 1;
        }
        if self.flags {
            bytecode_options_byte |= 1 << 1;
        }

        encode_u8(w, bytecode_options_byte);
    }
}
