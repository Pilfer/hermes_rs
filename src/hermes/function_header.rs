use std::{io, vec};

use crate::hermes::debug_info::DebugInfoOffsets;
use crate::hermes::decode::{decode_u32, decode_u8, read_bitfield};
use crate::hermes::encode::{align_writer, encode_u32, encode_u8, write_bitfield};
use crate::hermes::exception_handler::ExceptionHandlerInfo;
use crate::hermes::Serializable;

// For deserializing
pub fn get_large_info_offset(offset: u32, info_offset: u32) -> u32 {
    (info_offset << 16) | (offset & 0xffff)
}

// Returns offset and info_offset to populate into a SmallFunctionHeader.
// Essentially splits the value of the overflowed header offset into two variables.
// This is used for serializing.
pub fn get_large_info_offset_pair(real_large_info_offset: u32) -> (u32, u32) {
    (
        real_large_info_offset & 0xffff,
        real_large_info_offset >> 16,
    )
}

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct SmallFunctionHeader {
    pub offset: u32,
    pub param_count: u32,
    pub byte_size: u32,
    pub func_name: u32,
    pub info_offset: u32,
    pub frame_size: u32,
    pub env_size: u32,
    pub highest_read_cache_index: u32,
    pub highest_write_cache_index: u32,
    pub flags: FunctionHeaderFlag,
    pub exception_handlers: Vec<ExceptionHandlerInfo>,
    pub debug_info: Option<DebugInfoOffsets>,
}

impl SmallFunctionHeader {
    pub fn new() -> Self {
        SmallFunctionHeader {
            offset: 0,
            param_count: 0,
            byte_size: 0,
            func_name: 0,
            info_offset: 0,
            frame_size: 0,
            env_size: 0,
            highest_read_cache_index: 0,
            highest_write_cache_index: 0,
            flags: FunctionHeaderFlag {
                prohibit_invoke: FunctionHeaderFlagProhibitions::ProhibitNone,
                strict_mode: false,
                has_exception_handler: false,
                has_debug_info: false,
                overflowed: false,
            },
            exception_handlers: vec![],
            debug_info: None,
        }
    }

    pub fn set_size(&mut self, new_size: u32) {
        self.byte_size = new_size;
    }

    pub fn is_overflowed_check(&self) -> bool {
        let is_overflowed = |value: u32| -> bool { value > (1 << 17) - 1 };
        is_overflowed(self.offset)
            || is_overflowed(self.param_count)
            || is_overflowed(self.byte_size)
            || is_overflowed(self.func_name)
            || is_overflowed(self.info_offset)
            || is_overflowed(self.frame_size)
            || is_overflowed(self.env_size)
            || is_overflowed(self.highest_read_cache_index)
            || is_overflowed(self.highest_write_cache_index)
    }
}

impl Default for SmallFunctionHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl Serializable for SmallFunctionHeader {
    type Version = u32;
    fn size(&self) -> usize {
        16
    }

    fn deserialize<R>(r: &mut R, _version: u32) -> Self
    where
        R: io::Read + io::BufRead + io::Seek,
    {
        let mut func_header_bytes = [0u8; 16];
        r.read_exact(&mut func_header_bytes)
            .expect("unable to read first word");

        let offset = read_bitfield(&func_header_bytes, 0, 25);
        let param_count = read_bitfield(&func_header_bytes, 25, 7);

        let byte_size = read_bitfield(&func_header_bytes, 32, 15);
        let func_name = read_bitfield(&func_header_bytes, 47, 17);

        let info_offset = read_bitfield(&func_header_bytes, 64, 25);
        let frame_size = read_bitfield(&func_header_bytes, 89, 7);

        let env_size = read_bitfield(&func_header_bytes, 96, 8);
        let highest_read_cache_index = read_bitfield(&func_header_bytes, 104, 8);
        let highest_write_cache_index = read_bitfield(&func_header_bytes, 112, 8);

        // last byte for flags
        let prohibit_invoke = read_bitfield(&func_header_bytes, 120, 2);
        let strict_mode = read_bitfield(&func_header_bytes, 122, 1);
        let has_exception_handler = read_bitfield(&func_header_bytes, 123, 1);
        let has_debug_info = read_bitfield(&func_header_bytes, 124, 1);
        let overflowed = read_bitfield(&func_header_bytes, 125, 1);

        let flags = FunctionHeaderFlag {
            prohibit_invoke: match prohibit_invoke as u8 {
                0 => FunctionHeaderFlagProhibitions::ProhibitCall,
                1 => FunctionHeaderFlagProhibitions::ProhibitConstruct,
                2 => FunctionHeaderFlagProhibitions::ProhibitNone,
                // Sometimes when a function is overflowed, we might get an incorrect value here
                // This obviously breaks the match statement, so we just default to ProhibitNone.
                // The true value will be overwritten in the LargeFunctionHeader.deserialize() call
                _ if overflowed == 1 => FunctionHeaderFlagProhibitions::ProhibitNone,
                _ => {
                    panic!(
                        "Unknown prohibit invoke on small function header: {:?} at position {:?}",
                        prohibit_invoke,
                        r.stream_position().unwrap()
                    );
                }
            },
            strict_mode: strict_mode == 1,
            has_exception_handler: has_exception_handler == 1,
            has_debug_info: has_debug_info == 1,
            overflowed: overflowed == 1,
        };

        // Reading the rest of this if the header is overflowed isn't necessary,
        // and will just cause an error. So we just return the header here.
        if overflowed == 1 {
            return SmallFunctionHeader {
                offset,
                param_count,
                byte_size,
                func_name,
                info_offset,
                frame_size,
                env_size,
                highest_read_cache_index,
                highest_write_cache_index,
                flags,
                exception_handlers: vec![],
                debug_info: None,
            };
        }

        SmallFunctionHeader {
            offset,
            param_count,
            byte_size,
            func_name,
            info_offset,
            frame_size,
            env_size,
            highest_read_cache_index,
            highest_write_cache_index,
            flags,
            exception_handlers: vec![],
            debug_info: None,
        }
    }

    fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write + io::Seek,
    {
        let mut func_header_bytes = [0u8; 16];

        write_bitfield(&mut func_header_bytes, 0, 25, self.offset);
        write_bitfield(&mut func_header_bytes, 25, 7, self.param_count);
        write_bitfield(&mut func_header_bytes, 32, 15, self.byte_size);
        write_bitfield(&mut func_header_bytes, 47, 17, self.func_name);
        write_bitfield(&mut func_header_bytes, 64, 25, self.info_offset);
        write_bitfield(&mut func_header_bytes, 89, 7, self.frame_size);
        write_bitfield(&mut func_header_bytes, 96, 8, self.env_size);
        write_bitfield(
            &mut func_header_bytes,
            104,
            8,
            self.highest_read_cache_index,
        );
        write_bitfield(
            &mut func_header_bytes,
            112,
            8,
            self.highest_write_cache_index,
        );

        // last byte for flags
        let mut flags_byte = [0u8];
        match self.flags.prohibit_invoke {
            FunctionHeaderFlagProhibitions::ProhibitCall => {
                write_bitfield(&mut flags_byte, 0, 2, 0)
            }
            FunctionHeaderFlagProhibitions::ProhibitConstruct => {
                write_bitfield(&mut flags_byte, 0, 2, 1)
            }
            FunctionHeaderFlagProhibitions::ProhibitNone => {
                write_bitfield(&mut flags_byte, 0, 2, 2)
            }
        }

        write_bitfield(&mut flags_byte, 2, 1, self.flags.strict_mode as u32);
        write_bitfield(
            &mut flags_byte,
            3,
            1,
            self.flags.has_exception_handler as u32,
        );
        write_bitfield(&mut flags_byte, 4, 1, self.flags.has_debug_info as u32);
        write_bitfield(&mut flags_byte, 5, 1, self.flags.overflowed as u32);

        func_header_bytes[15] = flags_byte[0];

        w.write_all(&func_header_bytes)
            .expect("unable to write first word");
    }
}

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct LargeFunctionHeader {
    pub offset: u32,
    pub param_count: u32,
    pub byte_size: u32,
    pub func_name: u32,
    pub info_offset: u32,
    pub frame_size: u32,
    pub env_size: u32,
    pub highest_read_cache_index: u32,
    pub highest_write_cache_index: u32,
    pub flags: FunctionHeaderFlag,
    pub exception_handlers: Vec<ExceptionHandlerInfo>,
    pub debug_info: Option<DebugInfoOffsets>,
}

impl LargeFunctionHeader {
    pub fn new() -> Self {
        LargeFunctionHeader {
            offset: 0,
            param_count: 0,
            byte_size: 0,
            func_name: 0,
            info_offset: 0,
            frame_size: 0,
            env_size: 0,
            highest_read_cache_index: 0,
            highest_write_cache_index: 0,
            flags: FunctionHeaderFlag {
                prohibit_invoke: FunctionHeaderFlagProhibitions::ProhibitNone,
                strict_mode: false,
                has_exception_handler: false,
                has_debug_info: false,
                overflowed: false,
            },
            exception_handlers: vec![],
            debug_info: None,
        }
    }

    pub fn set_size(&mut self, new_size: u32) {
        self.byte_size = new_size;
    }

    pub fn is_overflowed_check(&self) -> bool {
        let is_overflowed = |value: u32| -> bool { value > (1 << 17) - 1 };
        is_overflowed(self.offset)
            || is_overflowed(self.param_count)
            || is_overflowed(self.byte_size)
            || is_overflowed(self.func_name)
            || is_overflowed(self.info_offset)
            || is_overflowed(self.frame_size)
            || is_overflowed(self.env_size)
            || is_overflowed(self.highest_read_cache_index)
            || is_overflowed(self.highest_write_cache_index)
    }
}

impl Default for LargeFunctionHeader {
    fn default() -> Self {
        Self::new()
    }
}
impl From<LargeFunctionHeader> for SmallFunctionHeader {
    fn from(lfh: LargeFunctionHeader) -> Self {
        let (calculated_offset, calculated_info_offset) =
            get_large_info_offset_pair(lfh.info_offset);
        SmallFunctionHeader {
            // offset: lfh.offset,
            offset: calculated_offset,
            param_count: lfh.param_count,
            byte_size: lfh.byte_size,
            func_name: lfh.func_name,
            // info_offset: lfh.info_offset,
            info_offset: calculated_info_offset,
            frame_size: lfh.frame_size,
            env_size: lfh.env_size,
            highest_read_cache_index: lfh.highest_read_cache_index,
            highest_write_cache_index: lfh.highest_write_cache_index,
            flags: lfh.flags,
            exception_handlers: lfh.exception_handlers,
            debug_info: lfh.debug_info,
        }
    }
}

impl From<SmallFunctionHeader> for LargeFunctionHeader {
    fn from(sfh: SmallFunctionHeader) -> Self {
        LargeFunctionHeader {
            offset: sfh.offset,
            param_count: sfh.param_count,
            byte_size: sfh.byte_size,
            func_name: sfh.func_name,
            info_offset: sfh.info_offset,
            frame_size: sfh.frame_size,
            env_size: sfh.env_size,
            highest_read_cache_index: sfh.highest_read_cache_index,
            highest_write_cache_index: sfh.highest_write_cache_index,
            flags: sfh.flags,
            exception_handlers: sfh.exception_handlers,
            debug_info: sfh.debug_info,
        }
    }
}

impl Serializable for LargeFunctionHeader {
    type Version = u32;
    fn size(&self) -> usize {
        32
    }

    fn deserialize<R>(r: &mut R, _version: u32) -> Self
    where
        R: io::Read + io::BufRead + io::Seek,
    {
        // 4
        let offset = decode_u32(r);
        let param_count = decode_u32(r);

        // 8
        let byte_size = decode_u32(r);
        let func_name = decode_u32(r);

        // 12
        let info_offset = decode_u32(r);
        let frame_size = decode_u32(r);

        // 16
        let env_size = decode_u32(r); // 28
        let highest_read_cache_index = decode_u8(r);
        let highest_write_cache_index = decode_u8(r); // 30

        let flags_byte = vec![decode_u8(r)]; // 31

        // last byte for flags
        let prohibit_invoke = read_bitfield(&flags_byte, 0, 2);
        let strict_mode = read_bitfield(&flags_byte, 2, 1);
        let has_exception_handler = read_bitfield(&flags_byte, 3, 1);
        let has_debug_info = read_bitfield(&flags_byte, 4, 1);
        let overflowed = read_bitfield(&flags_byte, 5, 1);

        let flags = FunctionHeaderFlag {
            prohibit_invoke: match prohibit_invoke as u8 {
                0 => FunctionHeaderFlagProhibitions::ProhibitCall,
                1 => FunctionHeaderFlagProhibitions::ProhibitConstruct,
                2 => FunctionHeaderFlagProhibitions::ProhibitNone,
                _ => {
                    panic!("Unknown prohibit invoke on small function header");
                }
            },
            strict_mode: strict_mode == 1,
            has_exception_handler: has_exception_handler == 1,
            has_debug_info: has_debug_info == 1,
            overflowed: overflowed == 1,
        };

        LargeFunctionHeader {
            offset,
            param_count,
            byte_size,
            func_name,
            info_offset,
            frame_size,
            env_size,
            highest_read_cache_index: highest_read_cache_index as u32,
            highest_write_cache_index: highest_write_cache_index as u32,
            flags,
            exception_handlers: vec![],
            debug_info: None,
        }
    }

    fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write + io::Seek,
    {
        // assert!(1 == 2, "Do a check here to see if self.func_name overflows");
        // let mut func_header_bytes = [0u8; 32];

        encode_u32(w, self.offset);
        encode_u32(w, self.param_count);
        encode_u32(w, self.byte_size);
        encode_u32(w, self.func_name);
        encode_u32(w, self.info_offset);
        encode_u32(w, self.frame_size);
        encode_u32(w, self.env_size);
        encode_u8(w, self.highest_read_cache_index as u8);
        encode_u8(w, self.highest_write_cache_index as u8);

        // last byte for flags
        let mut flag_byte = [0u8];
        match self.flags.prohibit_invoke {
            FunctionHeaderFlagProhibitions::ProhibitCall => write_bitfield(&mut flag_byte, 0, 2, 0),
            FunctionHeaderFlagProhibitions::ProhibitConstruct => {
                write_bitfield(&mut flag_byte, 0, 2, 1)
            }
            FunctionHeaderFlagProhibitions::ProhibitNone => write_bitfield(&mut flag_byte, 0, 2, 2),
        }
        write_bitfield(&mut flag_byte, 2, 1, self.flags.strict_mode as u32);
        write_bitfield(
            &mut flag_byte,
            3,
            1,
            self.flags.has_exception_handler as u32,
        );
        write_bitfield(&mut flag_byte, 4, 1, self.flags.has_debug_info as u32);
        write_bitfield(&mut flag_byte, 5, 1, self.flags.overflowed as u32);

        // func_header_bytes[15] = flags_byte[0];

        w.write_all(&flag_byte).expect("unable to write first word");

        if self.flags.has_exception_handler {
            align_writer(w, 4);
            for handler in &self.exception_handlers {
                handler.serialize(w);
            }
        }

        if self.flags.has_debug_info && self.debug_info.is_some() {
            align_writer(w, 4);
            self.debug_info.as_ref().unwrap().serialize(w);
        }
    }
}

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub enum FunctionHeader {
    Small(SmallFunctionHeader),
    Large(LargeFunctionHeader),
}

pub trait FunctionHeaderImpl {
    fn offset(&self) -> u32;
    fn param_count(&self) -> u32;
    fn byte_size(&self) -> u32;
    fn func_name(&self) -> u32;
    fn info_offset(&self) -> u32;
    fn frame_size(&self) -> u32;
    fn env_size(&self) -> u32;
    fn highest_read_cache_index(&self) -> u32;
    fn highest_write_cache_index(&self) -> u32;
    fn flags(&self) -> FunctionHeaderFlag;
    fn exception_handlers(&self) -> Vec<ExceptionHandlerInfo>;
    fn debug_info(&self) -> DebugInfoOffsets;
}

impl FunctionHeader {
    pub fn offset(&self) -> u32 {
        match self {
            FunctionHeader::Small(fh) => fh.offset,
            FunctionHeader::Large(fh) => fh.offset,
        }
    }

    pub fn set_offset(&mut self, new_offset: u32) {
        match self {
            FunctionHeader::Small(fh) => fh.offset = new_offset,
            FunctionHeader::Large(fh) => fh.offset = new_offset,
        }
    }

    pub fn set_size(&mut self, byte_size: u32) {
        match self {
            FunctionHeader::Small(fh) => fh.byte_size = byte_size,
            FunctionHeader::Large(fh) => fh.byte_size = byte_size,
        }
    }

    pub fn param_count(&self) -> u32 {
        match self {
            FunctionHeader::Small(fh) => fh.param_count,
            FunctionHeader::Large(fh) => fh.param_count,
        }
    }

    pub fn byte_size(&self) -> u32 {
        match self {
            FunctionHeader::Small(fh) => fh.byte_size,
            FunctionHeader::Large(fh) => fh.byte_size,
        }
    }

    pub fn set_byte_size(&mut self, new_size: u32) {
        match self {
            FunctionHeader::Small(fh) => fh.byte_size = new_size,
            FunctionHeader::Large(fh) => fh.byte_size = new_size,
        };
    }

    pub fn func_name(&self) -> u32 {
        match self {
            FunctionHeader::Small(fh) => fh.func_name,
            FunctionHeader::Large(fh) => fh.func_name,
        }
    }

    pub fn set_info_offset(&mut self, offset: u32) {
        match self {
            FunctionHeader::Small(fh) => fh.info_offset = offset,
            FunctionHeader::Large(fh) => fh.info_offset = offset,
        }
    }
    pub fn info_offset(&self) -> u32 {
        match self {
            FunctionHeader::Small(fh) => fh.info_offset,
            FunctionHeader::Large(fh) => fh.info_offset,
        }
    }

    pub fn frame_size(&self) -> u32 {
        match self {
            FunctionHeader::Small(fh) => fh.frame_size,
            FunctionHeader::Large(fh) => fh.frame_size,
        }
    }

    pub fn env_size(&self) -> u32 {
        match self {
            FunctionHeader::Small(fh) => fh.env_size,
            FunctionHeader::Large(fh) => fh.env_size,
        }
    }

    pub fn highest_read_cache_index(&self) -> u32 {
        match self {
            FunctionHeader::Small(fh) => fh.highest_read_cache_index,
            FunctionHeader::Large(fh) => fh.highest_read_cache_index,
        }
    }

    pub fn highest_write_cache_index(&self) -> u32 {
        match self {
            FunctionHeader::Small(fh) => fh.highest_write_cache_index,
            FunctionHeader::Large(fh) => fh.highest_write_cache_index,
        }
    }

    pub fn flags(&self) -> FunctionHeaderFlag {
        match self {
            FunctionHeader::Small(fh) => fh.flags.clone(),
            FunctionHeader::Large(fh) => fh.flags.clone(),
        }
    }

    pub fn set_overflowed(&mut self, overflowed: bool) {
        match self {
            FunctionHeader::Small(fh) => fh.flags.overflowed = overflowed,
            FunctionHeader::Large(fh) => fh.flags.overflowed = overflowed,
        }
    }

    pub fn exception_handlers(&self) -> Vec<ExceptionHandlerInfo> {
        match self {
            FunctionHeader::Small(fh) => fh.exception_handlers.clone(),
            FunctionHeader::Large(fh) => fh.exception_handlers.clone(),
        }
    }

    pub fn set_exception_handlers(&mut self, new_handlers: Vec<ExceptionHandlerInfo>) {
        match self {
            FunctionHeader::Small(fh) => fh.exception_handlers = new_handlers,
            FunctionHeader::Large(fh) => fh.exception_handlers = new_handlers,
        }
    }

    pub fn debug_info(&self) -> Option<DebugInfoOffsets> {
        match self {
            FunctionHeader::Small(fh) => fh.debug_info.clone(),
            FunctionHeader::Large(fh) => fh.debug_info.clone(),
        }
    }

    pub fn set_debug_info(&mut self, new_debug_info: Option<DebugInfoOffsets>) {
        match self {
            FunctionHeader::Small(fh) => fh.debug_info = new_debug_info,
            FunctionHeader::Large(fh) => fh.debug_info = new_debug_info,
        }
    }

    pub fn serialize<W>(&self, w: &mut W)
    where
        W: std::io::Write + std::io::Seek,
    {
        match self {
            FunctionHeader::Small(fh) => fh.serialize(w),
            FunctionHeader::Large(fh) => fh.serialize(w),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            FunctionHeader::Small(fh) => fh.size(),
            FunctionHeader::Large(fh) => fh.size(),
        }
    }

    pub fn is_overflowed_check(&mut self) -> bool {
        match self {
            FunctionHeader::Small(fh) => fh.is_overflowed_check(),
            FunctionHeader::Large(fh) => fh.is_overflowed_check(),
        }
    }
}

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub enum FunctionHeaderFlagProhibitions {
    ProhibitCall = 0,
    ProhibitConstruct = 1,
    ProhibitNone = 2,
}

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct FunctionHeaderFlag {
    pub prohibit_invoke: FunctionHeaderFlagProhibitions, // 2
    pub strict_mode: bool,                               // 1
    pub has_exception_handler: bool,                     // 1
    pub has_debug_info: bool,                            // 1
    pub overflowed: bool,                                // 1
}
