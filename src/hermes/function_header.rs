use std::io;

use crate::hermes::debug_info::DebugInfoOffsets;
use crate::hermes::decode::{align_reader, decode_u32, decode_u8, read_bitfield};
use crate::hermes::encode::{align_writer, write_bitfield};
use crate::hermes::exception_handler::ExceptionHandlerInfo;
use crate::hermes::Serializable;

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

        //
        // https://github.com/facebook/hermes/blob/d964f6b125426f919ad30fb09ff09b9d5f041743/lib/BCGen/HBC/BytecodeDataProvider.cpp#L646
        //
        let mut exception_handlers: Vec<ExceptionHandlerInfo> = vec![];
        if flags.has_exception_handler {
            r.seek(io::SeekFrom::Start(info_offset as u64)).unwrap();
            align_reader(r, 4).unwrap();

            let exception_handler_count = decode_u32(r);
            for _ in 0..exception_handler_count {
                exception_handlers.push(ExceptionHandlerInfo::deserialize(r, _version));
            }
        }

        let debug_info = if flags.has_debug_info {
            let _current_pos = r.stream_position().unwrap();

            // Go to the debug info offset for this function header to read the debug info
            r.seek(io::SeekFrom::Start(info_offset as u64)).unwrap();

            // Read the debug info
            let dio = Some(DebugInfoOffsets::deserialize(r, _version));

            // Go back to the original position since we're done reading the debug info
            r.seek(io::SeekFrom::Start(_current_pos)).unwrap();
            dio
        } else {
            None
        };

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
            exception_handlers,
            debug_info,
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

impl Serializable for LargeFunctionHeader {
    type Version = u32;
    fn size(&self) -> usize {
        32
    }

    fn deserialize<R>(r: &mut R, _version: u32) -> Self
    where
        R: io::Read + io::BufRead + io::Seek,
    {
        let offset = decode_u32(r);
        let param_count = decode_u32(r);

        let byte_size = decode_u32(r);
        let func_name = decode_u32(r);

        let info_offset = decode_u32(r);
        let frame_size = decode_u32(r);

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

        let mut exception_handlers: Vec<ExceptionHandlerInfo> = vec![];
        if flags.has_exception_handler {
            align_reader(r, 4).unwrap();
            let exception_handler_count = decode_u32(r);
            for _ in 0..exception_handler_count {
                exception_handlers.push(ExceptionHandlerInfo::deserialize(r, _version));
            }
        }

        let debug_info = if flags.has_debug_info {
            let _current_pos = r.stream_position().unwrap();

            // Go to the debug info offset for this function header to read the debug info
            r.seek(io::SeekFrom::Start(info_offset as u64)).unwrap();

            // Read the debug info
            let dio = Some(DebugInfoOffsets::deserialize(r, _version));

            // Go back to the original position since we're done reading the debug info
            r.seek(io::SeekFrom::Start(_current_pos)).unwrap();
            dio
        } else {
            None
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
            exception_handlers,
            debug_info,
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

    pub fn func_name(&self) -> u32 {
        match self {
            FunctionHeader::Small(fh) => fh.func_name,
            FunctionHeader::Large(fh) => fh.func_name,
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

    pub fn exception_handlers(&self) -> Vec<ExceptionHandlerInfo> {
        match self {
            FunctionHeader::Small(fh) => fh.exception_handlers.clone(),
            FunctionHeader::Large(fh) => fh.exception_handlers.clone(),
        }
    }

    pub fn debug_info(&self) -> Option<DebugInfoOffsets> {
        match self {
            FunctionHeader::Small(fh) => fh.debug_info.clone(),
            FunctionHeader::Large(fh) => fh.debug_info.clone(),
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
}

#[derive(Debug, Clone)]
pub enum FunctionHeaderFlagProhibitions {
    ProhibitCall = 0,
    ProhibitConstruct = 1,
    ProhibitNone = 2,
}

#[derive(Debug, Clone)]
pub struct FunctionHeaderFlag {
    pub prohibit_invoke: FunctionHeaderFlagProhibitions, // 2
    pub strict_mode: bool,                               // 1
    pub has_exception_handler: bool,                     // 1
    pub has_debug_info: bool,                            // 1
    pub overflowed: bool,                                // 1
}
