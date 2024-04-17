use std::io;

use crate::hermes::debug_info::DebugInfoOffsets;
use crate::hermes::Serializable;
use crate::hermes::decode::read_bitfield;
use crate::hermes::exception_handler::ExceptionHandlerInfo;

#[derive(Debug)]
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
    pub debug_info: DebugInfoOffsets,
}

impl Serializable for SmallFunctionHeader {
    fn size(&self) -> usize {
        1337
    }

    fn deserialize<R>(r: &mut R) -> Self
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
                _ => {
                    panic!("Unknown prohibit invoke on small function header");
                }
            },
            strict_mode: strict_mode == 1,
            has_exception_handler: has_exception_handler == 1,
            has_debug_info: has_debug_info == 1,
            overflowed: overflowed == 1,
        };

        return SmallFunctionHeader {
            offset: offset,
            param_count: param_count,
            byte_size: byte_size,
            func_name: func_name,
            info_offset: info_offset,
            frame_size: frame_size,
            env_size: env_size,
            highest_read_cache_index: highest_read_cache_index,
            highest_write_cache_index: highest_write_cache_index,
            flags: flags,
            exception_handlers: vec![],
            debug_info: DebugInfoOffsets{src: 0, scope_desc: 0, callee: 0},
        };
    }

    fn serialize<W>(&self, _w: &mut W)
    where
        W: io::Write,
    {
        // encode_u32(w, self.string_data_off);
    }
}

#[derive(Debug)]
pub enum FunctionHeaderFlagProhibitions {
    ProhibitCall = 0,
    ProhibitConstruct = 1,
    ProhibitNone = 2,
}

#[derive(Debug)]
pub struct FunctionHeaderFlag {
    pub prohibit_invoke: FunctionHeaderFlagProhibitions, // 2
    pub strict_mode: bool,                               // 1
    pub has_exception_handler: bool,                     // 1
    pub has_debug_info: bool,                            // 1
    pub overflowed: bool,                                // 1
}
