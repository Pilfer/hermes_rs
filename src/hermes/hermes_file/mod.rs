pub mod builder;
pub mod reader;
pub mod writer;

use std::collections::HashMap;
use std::io;

use crate::hermes::big_int_table::BigIntTableEntry;
use crate::hermes::cjs_module::CJSModule;
use crate::hermes::debug_info::DebugInfo;
use crate::hermes::function_header::FunctionHeader;
use crate::hermes::function_sources::FunctionSourceEntry;
use crate::hermes::header::HermesHeader;
use crate::hermes::regexp_table::RegExpTableEntry;
use crate::hermes::string_kind::StringKindEntry;
use crate::hermes::string_table::{OverflowStringTableEntry, SmallStringTableEntry};

use super::HermesStructReader;

// This struct should contain all the offsets for the different sections of the file
// It isn't part of the official spec, but we'll need it for writing different
// sections accurately and keeping track of offsets for the data section.
//
// Alternatively, we could just write the data section first and then write the
// header and offsets at the end of the file.
#[allow(dead_code)]
#[derive(Debug)]
pub struct HermesOffsets {
    // header - not required
    // function_headers - not required - always after header
    // string kind not required - doesn't have offsets
    // identifier hashes not required
    bytecode_offset: u32, // before serializing bytecode, write current stream position to this value

    small_string_table_offsets: HashMap<u32, u32>, // index of string in string storage -> offset in file
    overflow_string_table_offsets: HashMap<u32, u32>, // index of string in string storage -> offset in file
    // maybe BigIntTableEntry offsets???
    // maybe RegExpTableEntry offsets???

    // Probably will also need to keep track of these
    // CJSModuleInt
    // CJSModuleEntry
    debug_info_offset: u32, // before serializing debug info, write current stream position to this value
    file_length: u32,       // after serializing the footer, write the file length to this value
}

#[derive(Debug)]
pub struct HermesFile<R>
where
    R: io::Read + io::BufRead + io::Seek,
{
    _reader: R,
    pub header: HermesHeader,
    pub function_headers: Vec<FunctionHeader>,
    pub string_kinds: Vec<StringKindEntry>,
    pub identifier_hashes: Vec<u32>,
    pub string_storage: Vec<SmallStringTableEntry>,
    pub string_storage_bytes: Vec<u8>,
    pub bytecode_storage: Vec<u8>,
    pub overflow_string_storage: Vec<OverflowStringTableEntry>,
    pub array_buffer_storage: Vec<u8>,
    pub object_key_buffer: Vec<u8>,
    pub object_val_buffer: Vec<u8>,
    pub cjs_module_offset: u32,
    pub big_int_table: Vec<BigIntTableEntry>,
    pub big_int_storage: Vec<u8>,
    pub reg_exp_table: Vec<RegExpTableEntry>,
    pub reg_exp_storage: Vec<u8>,
    pub cjs_modules: Vec<CJSModule>,
    pub function_source_entries: Vec<FunctionSourceEntry>,
    pub debug_info: DebugInfo,

    // SHA1 of everything before the footer
    pub footer: [u8; 20],
}
