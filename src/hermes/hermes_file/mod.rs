pub mod builder;
pub mod reader;
pub mod writer;

use std::collections::HashMap;

use crate::hermes::big_int_table::BigIntTableEntry;
use crate::hermes::cjs_module::CJSModule;
use crate::hermes::debug_info::DebugInfo;
use crate::hermes::function_header::FunctionHeader;
use crate::hermes::function_sources::FunctionSourceEntry;
use crate::hermes::header::HermesHeader;
use crate::hermes::regexp_table::RegExpTableEntry;
use crate::hermes::string_kind::StringKindEntry;
use crate::hermes::string_table::{OverflowStringTableEntry, SmallStringTableEntry};

use super::{HermesInstruction, HermesStructReader};

// This struct should contain all the offsets for the different sections of the file
// It isn't part of the official spec, but we'll need it for writing different
// sections accurately and keeping track of offsets for the data section.
//
// Alternatively, we could just write the data section first and then write the
// header and offsets at the end of the file.
#[allow(dead_code)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct HermesOffsets {
    small_string_table_offsets: HashMap<u32, u32>, // index of string in string storage -> offset in file
    overflow_string_table_offsets: HashMap<u32, u32>, // index of string in string storage -> offset in file
    bytecode_offsets: HashMap<u32, u32>, // before serializing bytecode, write current stream position to this value
    debug_info_offset: u32, // before serializing debug info, write current stream position to this value
    file_length: u32,       // after serializing the footer, write the file length to this value
}

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct FunctionBytecode {
    pub func_index: u32,
    pub bytecode: Vec<u8>,
}

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct FunctionInstructions {
    pub func_index: u32,
    pub bytecode: Vec<HermesInstruction>,
}

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct HermesFile<R> {
    // Our reader
    _reader: R,

    // We use this to keep track of offsets for different sections of the file
    offsets: HermesOffsets,

    // We use this to keep track of the bytecode for each function
    function_bytecode: Vec<FunctionInstructions>,

    /*
     * Hermes file format
     */
    // Header - source of truth
    pub header: HermesHeader,

    // Function headers - Small or Large
    pub function_headers: Vec<FunctionHeader>,

    // String kinds - String, Identifier, or Empty
    pub string_kinds: Vec<StringKindEntry>,

    // Identifier hashes
    pub identifier_hashes: Vec<u32>,

    // String storage - StringTableEntry table that references offsets + length in string_storage_bytes
    pub string_storage: Vec<SmallStringTableEntry>,

    // String storage bytes - all strings concatenated together
    pub string_storage_bytes: Vec<u8>,

    // Overflow string storage - OverflowStringTableEntry table that references offsets + length in overflow_string_storage
    pub overflow_string_storage: Vec<OverflowStringTableEntry>,

    // Bytecode storage - all bytecode concatenated together
    pub bytecode_storage: Vec<u8>,

    // <Function Info goes here - DebugInfoOffsets + LargeFunctionHeader>

    // Array buffer storage - Array data stored here
    pub array_buffer_storage: Vec<u8>,

    // Object key buffer storage - Object key data stored here
    pub object_key_buffer: Vec<u8>,

    // Object value buffer storage - Object value data stored here
    pub object_val_buffer: Vec<u8>,

    // CJS module offset
    pub cjs_module_offset: u32,

    // BigInt table - BigIntTableEntry table that references offsets + length in big_int_storage
    pub big_int_table: Vec<BigIntTableEntry>,

    // BigInt storage - all BigInts concatenated together
    pub big_int_storage: Vec<u8>,

    // RegExp table - RegExpTableEntry table that references offsets + length in reg_exp_storage
    pub reg_exp_table: Vec<RegExpTableEntry>,

    // RegExp storage - all RegExps concatenated together. Format is bespoke bytecode.
    pub reg_exp_storage: Vec<u8>,

    // CommonJS modules
    pub cjs_modules: Vec<CJSModule>,

    // Function source entries
    pub function_source_entries: Vec<FunctionSourceEntry>,

    // Debug info (if not stripped - Hermes gives a blank header if stripped)
    pub debug_info: DebugInfo,

    // SHA1 of everything before the footer
    pub footer: [u8; 20],
}
