use std::collections::HashMap;
use std::io::{self};

use crate::hermes::array_parser::ArrayTypes;
use crate::hermes::big_int_table::BigIntTableEntry;
use crate::hermes::debug_info::DebugInfoOffsets;
use crate::hermes::exception_handler::ExceptionHandlerInfo;
use crate::hermes::header::HermesHeader;

use crate::hermes::cjs_module::{CJSModule, CJSModuleEntry, CJSModuleInt};
use crate::hermes::debug_info::{DebugInfo, DebugInfoHeader};
use crate::hermes::decode::{align_reader, decode_u32};
use crate::hermes::function_header::{
    FunctionHeader, FunctionHeaderFlagProhibitions, LargeFunctionHeader, SmallFunctionHeader,
};
use crate::hermes::function_sources::FunctionSourceEntry;
use crate::hermes::regexp_table::RegExpTableEntry;
use crate::hermes::string_kind::StringKindEntry;
use crate::hermes::string_table::{OverflowStringTableEntry, SmallStringTableEntry};
use crate::hermes::{print_array_val, print_array_vals};
use crate::hermes::{HermesInstruction, InstructionParser, Serializable};
use crate::match_instruction;

use super::builder::StringTypePair;
use super::{FunctionBytecode, FunctionInstructions, HermesFile, HermesStructReader};

impl<R> HermesFile<R>
where
    R: io::Read + io::BufRead + io::Seek,
{
    pub fn new(r: R) -> Self {
        Self {
            _reader: r,
            offsets: super::HermesOffsets {
                small_string_table_offsets: HashMap::new(),
                overflow_string_table_offsets: HashMap::new(),
                bytecode_offsets: HashMap::new(),
                debug_info_offset: 0,
                file_length: 0,
            },
            function_bytecode: vec![],
            header: HermesHeader::new(),
            function_headers: vec![],
            string_kinds: vec![],
            identifier_hashes: vec![],
            string_storage: vec![],
            string_storage_bytes: vec![],
            bytecode_storage: vec![],
            overflow_string_storage: vec![],
            array_buffer_storage: vec![],
            object_key_buffer: vec![],
            object_val_buffer: vec![],
            cjs_module_offset: 0,
            big_int_table: vec![],
            big_int_storage: vec![],
            reg_exp_table: vec![],
            reg_exp_storage: vec![],

            cjs_modules: vec![],
            function_source_entries: vec![],
            debug_info: DebugInfo {
                header: DebugInfoHeader {
                    filename_count: 0,
                    filename_storage_size: 0,
                    file_region_count: 0,
                    scope_desc_data_offset: 0,
                    textified_callee_offset: Some(0),
                    string_table_offset: Some(0),
                    debug_data_size: 0,
                },
                string_table: vec![],
                string_storage: vec![],
                file_regions: vec![],
                sources_data_storage: vec![],
                scope_desc_data_storage: vec![],
                textified_callee_storage: vec![],
                string_table_storage: vec![],
            },
            footer: [0; 20],
        }
    }

    pub fn deserialize(r: &mut R) -> HermesFile<&mut R> {
        let mut hermes_file: HermesFile<&mut R> = HermesFile::new(r);
        hermes_file.visit_header();
        hermes_file.visit_function_headers();
        // The bytecode of all of the functions are in this section.
        // When reading, we have the offsets so we know where to start
        // reading from.
        // The debug_info for all of the functions follow the bytecode.
        // Same as above - the info offset is in the function header, so
        // we simply just read that.
        hermes_file.visit_string_kinds();
        hermes_file.visit_identifier_hashes();
        hermes_file.visit_small_string_table();
        hermes_file.visit_overflow_string_table();
        hermes_file.visit_string_storage();
        hermes_file.visit_array_buffer();
        hermes_file.visit_object_key_buffer();
        hermes_file.visit_object_value_buffer();

        if hermes_file.header.version >= 87 {
            hermes_file.visit_big_int_table();
            hermes_file.visit_big_int_storage();
        }

        hermes_file.visit_reg_exp_table();
        hermes_file.visit_cjs_module_table();

        if hermes_file.header.version >= 84 {
            hermes_file.visit_function_source_table();
        }

        hermes_file.visit_debug_info();
        hermes_file.visit_footer();

        hermes_file
    }

    pub fn visit_header(&mut self) {
        self.header = HermesHeader::deserialize(&mut self._reader, 0);
    }

    pub fn visit_function_headers(&mut self) {
        for _ in 0..self.header.function_count {
            let _initpos = self._reader.stream_position().unwrap();
            let mut sfh = SmallFunctionHeader::deserialize(&mut self._reader, self.header.version);

            let anchor_pos = _initpos + sfh.size() as u64;
            // Check if we're dealing with a Small or Large Function Header.
            // Overflowed = Large Function Header
            let function_header_val: FunctionHeader = if !sfh.flags.overflowed {
                // if has_exception_handler and debug_info, navigate to infooffset
                if sfh.flags.has_exception_handler || sfh.flags.has_debug_info {
                    self._reader
                        .seek(io::SeekFrom::Start(sfh.info_offset as u64))
                        .expect("unable to seek to overflowed function header");
                }

                // read exception info and debug_info here
                let mut exception_handlers: Vec<ExceptionHandlerInfo> = vec![];
                if sfh.flags.has_exception_handler {
                    align_reader(&mut self._reader, 4).unwrap();

                    let exception_handler_count = decode_u32(&mut self._reader);
                    for _ in 0..exception_handler_count {
                        exception_handlers.push(ExceptionHandlerInfo::deserialize(
                            &mut self._reader,
                            self.header.version,
                        ));
                    }
                };

                sfh.exception_handlers = exception_handlers;

                let debug_info = if sfh.flags.has_debug_info {
                    let _current_pos = self._reader.stream_position().unwrap();

                    // Read the debug info
                    let dio = Some(DebugInfoOffsets::deserialize(
                        &mut self._reader,
                        self.header.version,
                    ));

                    // Go back to the original position since we're done reading the debug info
                    self._reader
                        .seek(io::SeekFrom::Start(_current_pos))
                        .unwrap();
                    dio
                } else {
                    None
                };

                sfh.debug_info = debug_info;

                FunctionHeader::Small(sfh)
            } else {
                let new_offset = (sfh.info_offset << 16) | (sfh.offset & 0xffff);
                // Go back to the start of the LFH to deserialize it properly
                self._reader
                    .seek(io::SeekFrom::Start(new_offset as u64))
                    .expect("unable to seek to overflowed function header");

                let mut lfh = FunctionHeader::Large(LargeFunctionHeader::deserialize(
                    &mut self._reader,
                    self.header.version,
                ));

                let mut exception_handlers: Vec<ExceptionHandlerInfo> = vec![];
                if lfh.flags().has_exception_handler {
                    align_reader(&mut self._reader, 4).unwrap();

                    let exception_handler_count = decode_u32(&mut self._reader);
                    for _ in 0..exception_handler_count {
                        exception_handlers.push(ExceptionHandlerInfo::deserialize(
                            &mut self._reader,
                            self.header.version,
                        ));
                    }
                };

                lfh.set_exception_handlers(exception_handlers);

                let debug_info = if lfh.flags().has_debug_info {
                    let _current_pos = self._reader.stream_position().unwrap();

                    // Read the debug info
                    let dio = Some(DebugInfoOffsets::deserialize(
                        &mut self._reader,
                        self.header.version,
                    ));

                    // Go back to the original position since we're done reading the debug info
                    self._reader
                        .seek(io::SeekFrom::Start(_current_pos))
                        .unwrap();
                    dio
                } else {
                    None
                };

                lfh.set_debug_info(debug_info);
                lfh
            };

            self._reader
                .seek(io::SeekFrom::Start(anchor_pos))
                .expect("unable to seek to function header");

            self.function_headers.push(function_header_val);
        }
    }

    pub fn visit_string_kinds(&mut self) {
        align_reader(&mut self._reader, 4).unwrap();
        for _ in 0..self.header.string_kind_count {
            self.string_kinds.push(StringKindEntry::deserialize(
                &mut self._reader,
                self.header.version,
            ));
        }
    }

    pub fn visit_identifier_hashes(&mut self) {
        align_reader(&mut self._reader, 4).unwrap();
        for _ in 0..self.header.identifier_count {
            self.identifier_hashes.push(decode_u32(&mut self._reader));
        }
    }

    pub fn visit_small_string_table(&mut self) {
        for _ in 0..self.header.string_count {
            align_reader(&mut self._reader, 4).unwrap();
            self.string_storage.push(SmallStringTableEntry::deserialize(
                &mut self._reader,
                self.header.version,
            ));
        }
    }

    pub fn visit_overflow_string_table(&mut self) {
        align_reader(&mut self._reader, 4).unwrap();
        for _ in 0..self.header.overflow_string_count {
            self.overflow_string_storage
                .push(OverflowStringTableEntry::deserialize(
                    &mut self._reader,
                    self.header.version,
                ));
        }
    }

    pub fn visit_string_storage(&mut self) {
        align_reader(&mut self._reader, 4).unwrap();
        let mut buf = vec![0; self.header.string_storage_size as usize];
        self._reader.read_exact(&mut buf).unwrap();
        self.string_storage_bytes = buf;
    }

    pub fn visit_array_buffer(&mut self) {
        align_reader(&mut self._reader, 4).unwrap();
        let mut buf = vec![0; self.header.array_buffer_size as usize];
        self._reader.read_exact(&mut buf).unwrap();
        self.array_buffer_storage = buf;
    }

    pub fn visit_object_key_buffer(&mut self) {
        align_reader(&mut self._reader, 4).unwrap();
        let mut buf = vec![0; self.header.obj_key_buffer_size as usize];
        self._reader.read_exact(&mut buf).unwrap();
        self.object_key_buffer = buf;
    }

    pub fn visit_object_value_buffer(&mut self) {
        align_reader(&mut self._reader, 4).unwrap();
        let mut buf = vec![0; self.header.obj_value_buffer_size as usize];
        self._reader.read_exact(&mut buf).unwrap();
        self.object_val_buffer = buf;
    }

    pub fn visit_big_int_table(&mut self) {
        align_reader(&mut self._reader, 4).unwrap();
        if self.header.big_int_count > 0 && self.header.version >= 87 {
            for _ in 0..self.header.big_int_count {
                self.big_int_table.push(BigIntTableEntry::deserialize(
                    &mut self._reader,
                    self.header.version,
                ));
            }
        }
    }

    pub fn visit_big_int_storage(&mut self) {
        align_reader(&mut self._reader, 4).unwrap();
        let mut buf = vec![0; self.header.big_int_storage_size as usize];
        self._reader.read_exact(&mut buf).unwrap();
        self.big_int_storage = buf;
    }

    pub fn visit_reg_exp_table(&mut self) {
        align_reader(&mut self._reader, 4).unwrap();
        if self.header.reg_exp_count > 0 {
            for _ in 0..self.header.reg_exp_count {
                self.reg_exp_table.push(RegExpTableEntry::deserialize(
                    &mut self._reader,
                    self.header.version,
                ));
            }

            // Get RegExp storage bytes
            self.visit_reg_exp_storage();
        }
    }

    pub fn visit_reg_exp_storage(&mut self) {
        let reg_exp_storage_size = self.header.reg_exp_storage_size as usize;
        let mut buf = vec![0; reg_exp_storage_size];
        self._reader.read_exact(&mut buf).unwrap();
        self.reg_exp_storage = buf;
    }

    pub fn visit_cjs_module_table(&mut self) {
        align_reader(&mut self._reader, 4).unwrap();
        if self.header.cjs_module_count > 0 {
            if self.header.options.cjs_modules_statically_resolved && self.header.version < 77 {
                for _ in 0..self.header.cjs_module_count {
                    let cjs_module =
                        CJSModuleInt::deserialize(&mut self._reader, self.header.version);
                    self.cjs_modules.push(CJSModule::CJSModuleInt(cjs_module));
                }
            } else {
                for _ in 0..self.header.cjs_module_count {
                    let cjs_module =
                        CJSModuleEntry::deserialize(&mut self._reader, self.header.version);
                    self.cjs_modules.push(CJSModule::CJSModuleEntry(cjs_module));
                }
            }
        }
    }

    pub fn visit_function_source_table(&mut self) {
        align_reader(&mut self._reader, 4).unwrap();
        if self.header.function_source_count > 0 && self.header.version >= 84 {
            for _ in 0..self.header.function_source_count {
                self.function_source_entries
                    .push(FunctionSourceEntry::deserialize(
                        &mut self._reader,
                        self.header.version,
                    ));
            }
        }
    }

    pub fn visit_debug_info(&mut self) {
        self._reader
            .seek(io::SeekFrom::Start(self.header.debug_info_offset as u64))
            .expect("unable to seek to debug info offset");

        self.debug_info = DebugInfo::deserialize(&mut self._reader, self.header.version);
    }

    pub fn visit_footer(&mut self) {
        let mut buf = [0; 20];
        self._reader.read_exact(&mut buf).unwrap();
        self.footer = buf;
    }

    pub fn get_array_buffer(&mut self, idx: usize, slice_size: usize) -> (usize, Vec<ArrayTypes>) {
        let buf_size = if slice_size > 0 {
            slice_size + idx
        } else {
            self.array_buffer_storage.len()
        };

        let buf = &self.array_buffer_storage[idx..buf_size];
        let mut current_idx = 0; // Start at 0 for relative indexing within buf
        let tag = buf[current_idx];

        let left_in_seq: u32;

        if (tag & 0x80) != 0 {
            left_in_seq = (((tag & 0x0f) as u32) << 8) | (buf[current_idx + 1] as u32);
        } else {
            left_in_seq = (tag & 0x0f) as u32;
        }

        // Define tag constants
        const NULL_TAG: u8 = 0 << 4;
        const TRUE_TAG: u8 = 1 << 4;
        const FALSE_TAG: u8 = 2 << 4;
        const NUMBER_TAG: u8 = 3 << 4;
        const LONG_STRING_TAG: u8 = 4 << 4;
        const SHORT_STRING_TAG: u8 = 5 << 4;
        const BYTE_STRING_TAG: u8 = 6 << 4;
        const INTEGER_TAG: u8 = 7 << 4;

        // Store parsed elements
        let mut elements_parsed = 0;

        let mut parsed_values: Vec<ArrayTypes> = vec![];

        // Parse sequences until we've parsed all left_in_seq elements
        while elements_parsed < left_in_seq && current_idx < buf.len() {
            // Get the tag for this sequence
            let sequence_tag = buf[current_idx];
            let tag_type = sequence_tag & 0x70;
            let sequence_length: u32;

            // Check if this sequence has overflow (extended length)
            if (sequence_tag & 0x80) != 0 {
                if current_idx + 1 >= buf.len() {
                    println!("Warning: Not enough bytes for overflow sequence length");
                    break;
                }
                sequence_length =
                    (((sequence_tag & 0x0f) as u32) << 8) | (buf[current_idx + 1] as u32);
                current_idx += 2;
            } else {
                sequence_length = (sequence_tag & 0x0f) as u32;
                current_idx += 1;
            }

            // Parse 'sequence_length' number of elements of this type
            for _seq_idx in 0..sequence_length {
                if elements_parsed >= left_in_seq {
                    break;
                }

                if current_idx >= buf.len() {
                    println!("Warning: Reached end of buffer before parsing all elements");
                    break;
                }

                match tag_type {
                    NULL_TAG => {
                        parsed_values.push(ArrayTypes::NullValue {});
                    }
                    TRUE_TAG => {
                        parsed_values.push(ArrayTypes::TrueValue { value: true });
                        // No additional bytes to read for true
                    }
                    FALSE_TAG => {
                        parsed_values.push(ArrayTypes::FalseValue { value: false });
                        // No additional bytes to read for false
                    }
                    NUMBER_TAG => {
                        if current_idx + 8 <= buf.len() {
                            let val = u64::from_le_bytes(
                                buf[current_idx..current_idx + 8].try_into().unwrap(),
                            );
                            parsed_values.push(ArrayTypes::NumberValue { value: val });
                            current_idx += 8; // 8 bytes for u64
                        } else {
                            println!("  Warning: Not enough bytes for NUMBER_TAG");
                            break;
                        }
                    }
                    LONG_STRING_TAG => {
                        if current_idx + 4 <= buf.len() {
                            let val = u32::from_le_bytes(
                                buf[current_idx..current_idx + 4].try_into().unwrap(),
                            );
                            parsed_values.push(ArrayTypes::LongStringValue { value: val });
                            current_idx += 4;
                        } else {
                            println!("  Warning: Not enough bytes for LONG_STRING_TAG");
                            break;
                        }
                    }
                    SHORT_STRING_TAG => {
                        if current_idx + 2 <= buf.len() {
                            let val = u16::from_le_bytes(
                                buf[current_idx..current_idx + 2].try_into().unwrap(),
                            );
                            parsed_values.push(ArrayTypes::ShortStringValue { value: val });
                            current_idx += 2;
                        } else {
                            println!("  Warning: Not enough bytes for SHORT_STRING_TAG");
                            break;
                        }
                    }
                    BYTE_STRING_TAG => {
                        if current_idx + 1 <= buf.len() {
                            let val = buf[current_idx];
                            parsed_values.push(ArrayTypes::ByteStringValue { value: val });
                            current_idx += 1;
                        } else {
                            println!("  Warning: Not enough bytes for BYTE_STRING_TAG");
                            break;
                        }
                    }
                    INTEGER_TAG => {
                        if current_idx + 4 <= buf.len() {
                            let val = u32::from_le_bytes(
                                buf[current_idx..current_idx + 4].try_into().unwrap(),
                            );
                            parsed_values.push(ArrayTypes::IntegerValue { value: val });
                            current_idx += 4;
                        } else {
                            println!("  Warning: Not enough bytes for INTEGER_TAG");
                            break;
                        }
                    }
                    _ => {
                        println!(
                            "  Element {}: Unknown tag: {:02x}",
                            elements_parsed, tag_type
                        );
                        current_idx += 1;
                    }
                }

                elements_parsed += 1;
            }
        }

        (idx + current_idx, parsed_values) // Return absolute index
    }

    pub fn get_object_key_buffer(
        &mut self,
        idx: usize,
        slice_size: usize,
    ) -> (usize, Vec<ArrayTypes>) {
        let buf_size = if slice_size > 0 {
            slice_size + idx
        } else {
            self.object_key_buffer.len()
        };

        let buf = &self.object_key_buffer[idx..buf_size];
        let mut current_idx = 0; // Start at 0 for relative indexing within buf
        let tag = buf[current_idx];

        let left_in_seq: u32;
        if (tag & 0x80) != 0 {
            left_in_seq = (((tag & 0x0f) as u32) << 8) | (buf[current_idx + 1] as u32);
        } else {
            left_in_seq = (tag & 0x0f) as u32;
        }

        // Define tag constants
        const NULL_TAG: u8 = 0 << 4;
        const TRUE_TAG: u8 = 1 << 4;
        const FALSE_TAG: u8 = 2 << 4;
        const NUMBER_TAG: u8 = 3 << 4;
        const LONG_STRING_TAG: u8 = 4 << 4;
        const SHORT_STRING_TAG: u8 = 5 << 4;
        const BYTE_STRING_TAG: u8 = 6 << 4;
        const INTEGER_TAG: u8 = 7 << 4;

        // Store parsed elements
        let mut elements_parsed = 0;

        let mut parsed_values: Vec<ArrayTypes> = vec![];

        // Parse sequences until we've parsed all left_in_seq elements
        while elements_parsed < left_in_seq && current_idx < buf.len() {
            let sequence_tag = buf[current_idx];
            let tag_type = sequence_tag & 0x70;
            let sequence_length: u32;

            if (sequence_tag & 0x80) != 0 {
                if current_idx + 1 >= buf.len() {
                    println!("Warning: Not enough bytes for overflow sequence length");
                    break;
                }
                sequence_length =
                    (((sequence_tag & 0x0f) as u32) << 8) | (buf[current_idx + 1] as u32);
                current_idx += 2;
            } else {
                sequence_length = (sequence_tag & 0x0f) as u32;
                current_idx += 1;
            }

            // Parse 'sequence_length' number of elements of this type
            for _seq_idx in 0..sequence_length {
                if elements_parsed >= left_in_seq {
                    break;
                }

                if current_idx >= buf.len() {
                    println!("Warning: Reached end of buffer before parsing all elements");
                    break;
                }

                match tag_type {
                    NULL_TAG => {
                        parsed_values.push(ArrayTypes::NullValue {});
                    }
                    TRUE_TAG => {
                        parsed_values.push(ArrayTypes::TrueValue { value: true });
                    }
                    FALSE_TAG => {
                        parsed_values.push(ArrayTypes::FalseValue { value: false });
                    }
                    NUMBER_TAG => {
                        if current_idx + 8 <= buf.len() {
                            let val = u64::from_le_bytes(
                                buf[current_idx..current_idx + 8].try_into().unwrap(),
                            );
                            parsed_values.push(ArrayTypes::NumberValue { value: val });
                            current_idx += 8;
                        } else {
                            println!("  Warning: Not enough bytes for NUMBER_TAG");
                            break;
                        }
                    }
                    LONG_STRING_TAG => {
                        if current_idx + 4 <= buf.len() {
                            let val = u32::from_le_bytes(
                                buf[current_idx..current_idx + 4].try_into().unwrap(),
                            );
                            parsed_values.push(ArrayTypes::LongStringValue { value: val });
                            current_idx += 4;
                        } else {
                            println!("  Warning: Not enough bytes for LONG_STRING_TAG");
                            break;
                        }
                    }
                    SHORT_STRING_TAG => {
                        if current_idx + 2 <= buf.len() {
                            let val = u16::from_le_bytes(
                                buf[current_idx..current_idx + 2].try_into().unwrap(),
                            );
                            parsed_values.push(ArrayTypes::ShortStringValue { value: val });
                            current_idx += 2;
                        } else {
                            println!("  Warning: Not enough bytes for SHORT_STRING_TAG");
                            break;
                        }
                    }
                    BYTE_STRING_TAG => {
                        if current_idx + 1 <= buf.len() {
                            let val = buf[current_idx];
                            parsed_values.push(ArrayTypes::ByteStringValue { value: val });
                            current_idx += 1;
                        } else {
                            println!("  Warning: Not enough bytes for BYTE_STRING_TAG");
                            break;
                        }
                    }
                    INTEGER_TAG => {
                        if current_idx + 4 <= buf.len() {
                            let val = u32::from_le_bytes(
                                buf[current_idx..current_idx + 4].try_into().unwrap(),
                            );
                            parsed_values.push(ArrayTypes::IntegerValue { value: val });
                            current_idx += 4;
                        } else {
                            println!("  Warning: Not enough bytes for INTEGER_TAG");
                            break;
                        }
                    }
                    _ => {
                        println!(
                            "  Element {}: Unknown tag: {:02x}",
                            elements_parsed, tag_type
                        );

                        current_idx += 1;
                    }
                }

                elements_parsed += 1;
            }
        }

        (idx + current_idx, parsed_values) // Return absolute index
    }

    pub fn get_object_val_buffer(
        &mut self,
        idx: usize,
        slice_size: usize,
    ) -> (usize, Vec<ArrayTypes>) {
        let buf_size = if slice_size > 0 {
            slice_size + idx
        } else {
            self.object_val_buffer.len()
        };

        let buf = &self.object_val_buffer[idx..buf_size];
        let mut current_idx = 0; // Start at 0 for relative indexing within buf
        let tag = buf[current_idx];

        let left_in_seq: u32;
        if (tag & 0x80) != 0 {
            left_in_seq = (((tag & 0x0f) as u32) << 8) | (buf[current_idx + 1] as u32);
        } else {
            left_in_seq = (tag & 0x0f) as u32;
        }

        // Define tag constants
        const NULL_TAG: u8 = 0 << 4;
        const TRUE_TAG: u8 = 1 << 4;
        const FALSE_TAG: u8 = 2 << 4;
        const NUMBER_TAG: u8 = 3 << 4;
        const LONG_STRING_TAG: u8 = 4 << 4;
        const SHORT_STRING_TAG: u8 = 5 << 4;
        const BYTE_STRING_TAG: u8 = 6 << 4;
        const INTEGER_TAG: u8 = 7 << 4;

        // Store parsed elements
        let mut elements_parsed = 0;

        let mut parsed_values: Vec<ArrayTypes> = vec![];

        // Parse sequences until we've parsed all left_in_seq elements
        while elements_parsed < left_in_seq && current_idx < buf.len() {
            let sequence_tag = buf[current_idx];
            let tag_type = sequence_tag & 0x70;
            let sequence_length: u32;

            if (sequence_tag & 0x80) != 0 {
                if current_idx + 1 >= buf.len() {
                    println!("Warning: Not enough bytes for overflow sequence length");
                    break;
                }
                sequence_length =
                    (((sequence_tag & 0x0f) as u32) << 8) | (buf[current_idx + 1] as u32);
                current_idx += 2;
            } else {
                sequence_length = (sequence_tag & 0x0f) as u32;
                current_idx += 1;
            }

            // Parse 'sequence_length' number of elements of this type
            for _seq_idx in 0..sequence_length {
                if elements_parsed >= left_in_seq {
                    break;
                }

                if current_idx >= buf.len() {
                    println!("Warning: Reached end of buffer before parsing all elements");
                    break;
                }

                match tag_type {
                    NULL_TAG => {
                        parsed_values.push(ArrayTypes::NullValue {});
                    }
                    TRUE_TAG => {
                        parsed_values.push(ArrayTypes::TrueValue { value: true });
                    }
                    FALSE_TAG => {
                        parsed_values.push(ArrayTypes::FalseValue { value: false });
                    }
                    NUMBER_TAG => {
                        if current_idx + 8 <= buf.len() {
                            let val = u64::from_le_bytes(
                                buf[current_idx..current_idx + 8].try_into().unwrap(),
                            );
                            parsed_values.push(ArrayTypes::NumberValue { value: val });
                            current_idx += 8;
                        } else {
                            println!("  Warning: Not enough bytes for NUMBER_TAG");
                            break;
                        }
                    }
                    LONG_STRING_TAG => {
                        if current_idx + 4 <= buf.len() {
                            let val = u32::from_le_bytes(
                                buf[current_idx..current_idx + 4].try_into().unwrap(),
                            );
                            parsed_values.push(ArrayTypes::LongStringValue { value: val });
                            current_idx += 4;
                        } else {
                            println!("  Warning: Not enough bytes for LONG_STRING_TAG");
                            break;
                        }
                    }
                    SHORT_STRING_TAG => {
                        if current_idx + 2 <= buf.len() {
                            let val = u16::from_le_bytes(
                                buf[current_idx..current_idx + 2].try_into().unwrap(),
                            );
                            parsed_values.push(ArrayTypes::ShortStringValue { value: val });
                            current_idx += 2;
                        } else {
                            println!("  Warning: Not enough bytes for SHORT_STRING_TAG");
                            break;
                        }
                    }
                    BYTE_STRING_TAG => {
                        if current_idx + 1 <= buf.len() {
                            let val = buf[current_idx];
                            parsed_values.push(ArrayTypes::ByteStringValue { value: val });
                            current_idx += 1;
                        } else {
                            println!("  Warning: Not enough bytes for BYTE_STRING_TAG");
                            break;
                        }
                    }
                    INTEGER_TAG => {
                        if current_idx + 4 <= buf.len() {
                            let val = u32::from_le_bytes(
                                buf[current_idx..current_idx + 4].try_into().unwrap(),
                            );
                            parsed_values.push(ArrayTypes::IntegerValue { value: val });
                            current_idx += 4;
                        } else {
                            println!("  Warning: Not enough bytes for INTEGER_TAG");
                            break;
                        }
                    }
                    _ => {
                        println!(
                            "  Element {}: Unknown tag: {:02x}",
                            elements_parsed, tag_type
                        );

                        current_idx += 1;
                    }
                }

                elements_parsed += 1;
            }
        }

        (idx + current_idx, parsed_values) // Return absolute index
    }

    /*
     * Returns the bytecode for each function in the Hermes file.
     */
    pub fn get_bytecode(&mut self) -> Vec<FunctionBytecode> {
        let mut output: Vec<FunctionBytecode> = vec![];

        for (idx, _) in self.function_headers.iter().enumerate() {
            let fh = &self.function_headers.get(idx).unwrap();

            // Create a buffer for the function bytecode
            let mut buf = vec![0u8; fh.byte_size() as usize];

            // Seek to the start of the function bytecode
            self._reader
                .seek(io::SeekFrom::Start(fh.offset() as u64))
                .unwrap();

            // Read it into the buffer
            self._reader.read_exact(&mut buf).unwrap();
            output.push(FunctionBytecode {
                func_index: idx as u32,
                bytecode: buf,
            });
        }
        output
    }

    pub fn get_instructions(&mut self) -> Vec<FunctionInstructions> {
        let function_headers: Vec<_> = self
            .function_headers
            .iter()
            .enumerate()
            .map(|(idx, fh)| (idx, fh.clone()))
            .collect();
        for (idx, fh) in function_headers {
            let bytecode = self.get_func_bytecode(idx as u32);
            self.function_bytecode.push(FunctionInstructions {
                func_index: idx as u32,
                is_large: match fh {
                    FunctionHeader::Small(_) => false,
                    FunctionHeader::Large(_) => true,
                },
                bytecode,
            });
        }

        // Remove the clone
        let vec = &self.function_bytecode;
        vec.to_vec()
        // self.function_bytecode.clone()
    }

    // ------------------------------------------ //
    // helper methods start
    // ------------------------------------------ //

    /*
     * Returns a vector of all the strings from the string storage - this isn't technically ordered/tagged by the string kind
     */
    pub fn get_strings(&self) -> Vec<String> {
        let mut out = vec![];
        for (idx, _) in self.string_storage.iter().enumerate() {
            out.push(self.get_string_from_storage_by_index(idx));
        }
        out
    }

    /*
     * Returns a vector of all the strings from the string storage - ordered by the string kind
     * as Hermes expects them. String -> Identifier -> Predefined.
     */
    pub fn get_strings_by_kind(&self) -> Vec<StringTypePair> {
        let mut out: Vec<StringTypePair> = vec![];
        let mut string_id = 0; // anchor
        for kind in self.string_kinds.iter() {
            match kind {
                StringKindEntry::New(sk) => {
                    for _idx in 0..sk.count {
                        out.push(StringTypePair {
                            string: self.get_string_from_storage_by_index(string_id),
                            kind: sk.kind,
                        });
                        string_id += 1;
                    }
                }
                StringKindEntry::Old(sk) => {
                    for _idx in 0..sk.count {
                        out.push(StringTypePair {
                            string: self.get_string_from_storage_by_index(string_id),
                            kind: sk.kind,
                        });
                        string_id += 1;
                    }
                }
            }
        }
        out
    }

    /*
     * Returns a string from the string storage by index - UTF-16 or UTF-8
     */
    pub fn get_string_from_storage_by_index(&self, index: usize) -> String {
        let myfunc = self.string_storage.get(index).unwrap();

        let is_utf16 = myfunc.is_utf_16;

        let mut real_offset = myfunc.offset;
        let mut real_length = myfunc.length;

        // String is overflowed, so we have to read the real offsets and length from the overflow table
        if myfunc.length == 255 {
            let overflow_entry = self
                .overflow_string_storage
                .get(myfunc.offset as usize)
                .unwrap();
            real_offset = overflow_entry.offset;
            real_length = overflow_entry.length;
        }

        if is_utf16 {
            let bytes = &self.string_storage_bytes
                [real_offset as usize..(real_offset + (real_length * 2)) as usize];

            let utf16_values: Vec<u16> = bytes
                .chunks_exact(2)
                .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                .collect();

            // Commenting out due to: https://github.com/Pilfer/hermes_rs/issues/9
            // String::from_utf16(&utf16_values).expect("Invalid UTF-16")
            // in favor of from_utf16_lossy, which allows for "invalid" UTF-16.
            // TODO: Actually fix this.
            String::from_utf16_lossy(&utf16_values).to_string()
        } else {
            String::from_utf8(
                self.string_storage_bytes
                    [real_offset as usize..(real_offset + real_length) as usize]
                    .to_vec(),
            )
            .expect("Invalid UTF-8")
        }
    }

    /*
     * Returns the instructions for a function by index
     */
    pub fn get_func_bytecode(&mut self, idx: u32) -> Vec<HermesInstruction> {
        let fh = &self.function_headers.get(idx as usize).unwrap();
        self._reader
            .seek(io::SeekFrom::Start(fh.offset() as u64))
            .unwrap();

        let mut bytecode_buf = vec![0u8; fh.byte_size() as usize];
        self._reader
            .read_exact(&mut bytecode_buf)
            .expect("unable to read first functions bytecode");

        let mut instructions_list = vec![];

        let mut byte_iter = bytecode_buf.iter();

        // Iterate over bytecode_buf and parse the instructions
        while let Some(&op_byte) = byte_iter.next() {
            let op = op_byte;
            // Make a new Cursor to print the remaining bytes
            let mut r_cursor = io::Cursor::new(byte_iter.as_slice());

            let ins_obj: Option<HermesInstruction> = match self.header.version {
                #[cfg(feature = "v76")]
                76 => Some(HermesInstruction::V76(
                    crate::hermes::v76::Instruction::deserialize(&mut r_cursor, op),
                )),
                #[cfg(feature = "v84")]
                84 => Some(HermesInstruction::V84(
                    crate::hermes::v84::Instruction::deserialize(&mut r_cursor, op),
                )),
                #[cfg(feature = "v89")]
                89 => Some(HermesInstruction::V89(
                    crate::hermes::v89::Instruction::deserialize(&mut r_cursor, op),
                )),
                #[cfg(feature = "v90")]
                90 => Some(HermesInstruction::V90(
                    crate::hermes::v90::Instruction::deserialize(&mut r_cursor, op),
                )),
                #[cfg(feature = "v93")]
                93 => Some(HermesInstruction::V93(
                    crate::hermes::v93::Instruction::deserialize(&mut r_cursor, op),
                )),
                #[cfg(feature = "v94")]
                94 => Some(HermesInstruction::V94(
                    crate::hermes::v94::Instruction::deserialize(&mut r_cursor, op),
                )),
                #[cfg(feature = "v95")]
                95 => Some(HermesInstruction::V95(
                    crate::hermes::v95::Instruction::deserialize(&mut r_cursor, op),
                )),
                #[cfg(feature = "v96")]
                96 => Some(HermesInstruction::V96(
                    crate::hermes::v96::Instruction::deserialize(&mut r_cursor, op),
                )),
                _ => {
                    panic!("Unsupported HBC version: {:?}. Check Cargo.toml features to see if this HBC version is enabled.", self.header.version);
                    // None
                }
            };

            if let Some(ins) = ins_obj {
                // We have to subtract by 1 to account for the opcode byte, as
                // we include it in the instruction size method, but it's
                // already been read at this point.
                let size = ins.size() - 1;
                instructions_list.push(ins);

                for _ in 0..size {
                    if byte_iter.next().is_none() {
                        break;
                    }
                }
            }
        }

        instructions_list
        // ---------------------------------------------------------------------------------------- //
    }

    pub fn parse_bytecode_for_fn(&mut self, idx: u32) {
        let fh = &self.function_headers.get(idx as usize).unwrap();
        self._reader
            .seek(io::SeekFrom::Start(fh.offset() as u64))
            .unwrap();
        let mut bytecode_buf = vec![0u8; fh.byte_size() as usize];
        self._reader
            .read_exact(&mut bytecode_buf)
            .expect("unable to read first functions bytecode");

        let myfunc = self.string_storage.get(fh.func_name() as usize).unwrap();
        println!("------------------------------------------------");
        let func_start = myfunc.offset;
        let mut func_name = String::from_utf8(
            self.string_storage_bytes[func_start as usize..(func_start + myfunc.length) as usize]
                .to_vec(),
        )
        .unwrap();

        if func_name.is_empty() {
            func_name = format!("$FUNC_{}", idx);
        }

        println!(
            "Function<{}>({:?} params, {:?} registers, {:?} symbols):",
            func_name,
            fh.param_count(),
            fh.frame_size(),
            fh.env_size()
        );

        // #[allow(unused_mut)]
        let mut instructions_list = vec![];

        let mut byte_iter = bytecode_buf.iter();
        let mut index = 0;
        let mut byte_index = 0;

        let mut labels: HashMap<u32, u32> = HashMap::new();

        // Iterate over bytecode_buf and parse the instructions
        while let Some(&op_byte) = byte_iter.next() {
            let op = op_byte;
            // Make a new Cursor to print the remaining bytes
            let mut r_cursor = io::Cursor::new(byte_iter.as_slice());

            // Deserialize the instruction
            let ins_obj: Option<HermesInstruction> = match self.header.version {
                #[cfg(feature = "v89")]
                89 => Some(HermesInstruction::V89(
                    crate::hermes::v89::Instruction::deserialize(&mut r_cursor, op),
                )),
                #[cfg(feature = "v90")]
                90 => Some(HermesInstruction::V90(
                    crate::hermes::v90::Instruction::deserialize(&mut r_cursor, op),
                )),
                #[cfg(feature = "v93")]
                93 => Some(HermesInstruction::V93(
                    crate::hermes::v93::Instruction::deserialize(&mut r_cursor, op),
                )),
                #[cfg(feature = "v94")]
                94 => Some(HermesInstruction::V94(
                    crate::hermes::v94::Instruction::deserialize(&mut r_cursor, op),
                )),
                #[cfg(feature = "v95")]
                95 => Some(HermesInstruction::V95(
                    crate::hermes::v95::Instruction::deserialize(&mut r_cursor, op),
                )),
                #[cfg(feature = "v96")]
                96 => Some(HermesInstruction::V96(
                    crate::hermes::v96::Instruction::deserialize(&mut r_cursor, op),
                )),
                _ => {
                    panic!("Unsupported HBC version: {:?}. Check Cargo.toml features to see if this HBC version is enabled.", self.header.version);
                    // None
                }
            };

            if let Some(ins) = ins_obj {
                // This label code may be the worst code I've ever written
                let mut label_idx = 0;

                // Exception handler logic here
                if fh.flags().has_exception_handler {
                    for (idx, eh) in fh.exception_handlers().iter().enumerate() {
                        let ehidx = idx + 1;
                        let has_label = if index == eh.start as usize {
                            label_idx += ehidx + 1;
                            true
                        } else if index == eh.end as usize {
                            label_idx += ehidx + 2;
                            true
                        } else if index == eh.target as usize {
                            label_idx += ehidx;
                            true
                        } else {
                            false
                        };

                        if has_label {
                            println!("    L{}:", label_idx);
                        }
                    }
                }

                // Check if the instruction is a jump target
                let mut display_str = ins.display(self);
                if ins.is_jmp() {
                    let addy = ins.get_address_field();
                    label_idx += 1;
                    labels.insert(addy, label_idx as u32);

                    let from = format!("{}", addy).to_string();
                    let to = format!("L{}", label_idx).to_string();
                    display_str = display_str.replace(&from, &to);
                }

                if labels.contains_key(&byte_index) {
                    println!("          \tL{}:", labels.get(&byte_index).unwrap());
                }

                // build_instructions
                println!("{:#010X}\t{}", byte_index, display_str);
                let size = ins.size();
                instructions_list.push(ins);

                index += size + 1;
                byte_index += 1;

                for _ in 0..size {
                    if byte_iter.next().is_none() {
                        break;
                    }
                }
            }
        }
    }

    pub fn print_bytecode(&mut self) {
        // Function body goes here
        {
            for (fidx, fh) in self.function_headers.iter().enumerate() {
                self._reader
                    .seek(io::SeekFrom::Start(fh.offset() as u64))
                    .unwrap();
                let mut bytecode_buf = vec![0u8; fh.byte_size() as usize];

                self._reader
                    .read_exact(&mut bytecode_buf)
                    .expect("unable to read first functions bytecode");

                let func_name_idx = fh.func_name() as usize;

                let myfunc = self.string_storage.get(func_name_idx).unwrap();
                println!("------------------------------------------------");
                let func_start = myfunc.offset;
                let mut func_name = String::from_utf8(
                    self.string_storage_bytes
                        [func_start as usize..(func_start + myfunc.length) as usize]
                        .to_vec(),
                )
                .unwrap();

                if func_name.is_empty() {
                    func_name = format!("$FUNC_{}", fidx);
                }

                // Print out the FunctionHeader type - this makes things easier to debug.
                // There's no real spec, so I can get away with dropping a # comment here.
                let is_large = match fh {
                    FunctionHeader::Small(_) => false,
                    FunctionHeader::Large(_) => true,
                };

                println!(
                    "{}<{}>({:?} params, {:?} registers, {:?} symbols): # Type: {}FunctionHeader - funcID: {} ({} bytes @ {})\n",
                    match fh.flags().prohibit_invoke {
                        FunctionHeaderFlagProhibitions::ProhibitCall => "Constructor",
                        FunctionHeaderFlagProhibitions::ProhibitConstruct => "NCFunction",
                        FunctionHeaderFlagProhibitions::ProhibitNone => "Function",
                    },
                    func_name,
                    fh.param_count(),
                    fh.frame_size(),
                    fh.env_size(),
                    if is_large { "Large" } else { "Small" },
                    fidx,
                    fh.byte_size(),
                    self._reader.stream_position().unwrap(),
                );

                // #[allow(unused_mut)]
                let mut instructions_list = vec![];

                let mut byte_iter = bytecode_buf.iter();
                let mut index = 0;
                let mut byte_index = 0;

                let mut labels: HashMap<u32, u32> = HashMap::new();
                let mut label_idx = 0;
                let mut output = vec![];

                // Iterate over bytecode_buf and parse the instructions
                while let Some(&op_byte) = byte_iter.next() {
                    let op = op_byte;
                    // Make a new Cursor to print the remaining bytes
                    let mut r_cursor = io::Cursor::new(byte_iter.as_slice());

                    // Deserialize the instruction
                    let ins_obj: Option<HermesInstruction> = match self.header.version {
                        #[cfg(feature = "v76")]
                        76 => Some(HermesInstruction::V76(
                            crate::hermes::v76::Instruction::deserialize(&mut r_cursor, op),
                        )),
                        #[cfg(feature = "v84")]
                        84 => Some(HermesInstruction::V84(
                            crate::hermes::v84::Instruction::deserialize(&mut r_cursor, op),
                        )),
                        #[cfg(feature = "v89")]
                        89 => Some(HermesInstruction::V89(
                            crate::hermes::v89::Instruction::deserialize(&mut r_cursor, op),
                        )),
                        #[cfg(feature = "v90")]
                        90 => Some(HermesInstruction::V90(
                            crate::hermes::v90::Instruction::deserialize(&mut r_cursor, op),
                        )),
                        #[cfg(feature = "v93")]
                        93 => Some(HermesInstruction::V93(
                            crate::hermes::v93::Instruction::deserialize(&mut r_cursor, op),
                        )),
                        #[cfg(feature = "v94")]
                        94 => Some(HermesInstruction::V94(
                            crate::hermes::v94::Instruction::deserialize(&mut r_cursor, op),
                        )),
                        #[cfg(feature = "v95")]
                        95 => Some(HermesInstruction::V95(
                            crate::hermes::v95::Instruction::deserialize(&mut r_cursor, op),
                        )),
                        #[cfg(feature = "v96")]
                        96 => Some(HermesInstruction::V96(
                            crate::hermes::v96::Instruction::deserialize(&mut r_cursor, op),
                        )),
                        _ => {
                            panic!("Unsupported HBC version: {:?}. Check Cargo.toml features to see if this HBC version is enabled.", self.header.version);
                            // None
                        }
                    };

                    if let Some(ins) = ins_obj {
                        // This label code may be the worst code I've ever written
                        // TODO: Need to handle jumps better here i think
                        // Exception handler logic here
                        if fh.flags().has_exception_handler {
                            for (idx, eh) in fh.exception_handlers().iter().enumerate() {
                                let ehidx = idx + 1;
                                let has_label = if index == eh.start as usize {
                                    label_idx += ehidx + 1;
                                    true
                                } else if index == eh.end as usize {
                                    label_idx += ehidx + 2;
                                    true
                                } else if index == eh.target as usize {
                                    label_idx += ehidx;
                                    true
                                } else {
                                    false
                                };

                                if has_label {
                                    output.push(format!("\tL{}:", label_idx));
                                }
                            }
                        }

                        // Check if the instruction is a jump target
                        let mut display_str = ins.display(self);
                        if ins.is_jmp() {
                            let addy = ins.get_address_field();

                            // Track labels properly
                            if !labels.contains_key(&addy) {
                                label_idx += 1;
                                labels.insert(addy, label_idx as u32);
                            }

                            let original = format!("{}", addy).to_string();
                            let enriched_label =
                                format!("L{}, r0 # Go to Addr: {}", label_idx, addy);
                            display_str = display_str.replace(&original, &enriched_label);
                        }

                        // build_instructions
                        output.push(format!("{:?}\t{}", byte_index, display_str));
                        // println!("{:#010X}\t{}", byte_index, display_str);
                        let size = ins.size() - 1; // Have to subtract - 1 for the opcode
                        instructions_list.push(ins);

                        index += size + 1;
                        byte_index += 1;

                        for _ in 0..size {
                            if byte_iter.next().is_none() {
                                break;
                            }
                        }
                    }
                }

                // populate labels into output
                for (lidx, line) in output.iter().enumerate() {
                    if labels.contains_key(&(lidx as u32)) {
                        println!("\t\tL{}:", labels.get(&(lidx as u32)).unwrap());
                    }
                    println!("{}", line);
                }
            }
        }
    }

    pub fn print_bytecode_new(&mut self) {
        {
            let function_headers: Vec<_> = self
                .function_headers
                .iter()
                .enumerate()
                .map(|(i, fh)| (i, fh.clone()))
                .collect();

            for (fidx, fh) in function_headers {
                let bytecode = self.get_func_bytecode(fidx as u32);
                self._reader
                    .seek(io::SeekFrom::Start(fh.offset() as u64))
                    .unwrap();

                let func_name_idx = fh.func_name() as usize;

                let myfunc = self.string_storage.get(func_name_idx).unwrap();
                println!("------------------------------------------------");
                let func_start = myfunc.offset;
                let mut func_name = String::from_utf8(
                    self.string_storage_bytes
                        [func_start as usize..(func_start + myfunc.length) as usize]
                        .to_vec(),
                )
                .unwrap();

                if func_name.is_empty() {
                    func_name = format!("$FUNC_{}", fidx);
                }

                // Print out the FunctionHeader type - this makes things easier to debug.
                // There's no real spec, so I can get away with dropping a # comment here.
                let is_large = match fh {
                    FunctionHeader::Small(_) => false,
                    FunctionHeader::Large(_) => true,
                };

                println!(
                    "{}<{}>({:?} params, {:?} registers, {:?} symbols): # Type: {}FunctionHeader - funcID: {} ({} bytes @ {})\n",
                    match fh.flags().prohibit_invoke {
                        FunctionHeaderFlagProhibitions::ProhibitCall => "Constructor",
                        FunctionHeaderFlagProhibitions::ProhibitConstruct => "NCFunction",
                        FunctionHeaderFlagProhibitions::ProhibitNone => "Function",
                    },
                    func_name,
                    fh.param_count(),
                    fh.frame_size(),
                    fh.env_size(),
                    if is_large { "Large" } else { "Small" },
                    fidx,
                    fh.byte_size(),
                    self._reader.stream_position().unwrap(),
                );

                let mut labels: HashMap<u32, u32> = HashMap::new();
                let mut label_idx = 0;
                let mut output = vec![];

                for (_idx, ins) in bytecode.iter().enumerate() {
                    if fh.flags().has_exception_handler {
                        for (idx, eh) in fh.exception_handlers().iter().enumerate() {
                            let ehidx = idx + 1;
                            let has_label = if _idx == eh.start as usize {
                                label_idx += ehidx + 1;
                                true
                            } else if _idx == eh.end as usize {
                                label_idx += ehidx + 2;
                                true
                            } else if _idx == eh.target as usize {
                                label_idx += ehidx;
                                true
                            } else {
                                false
                            };

                            if has_label {
                                output.push(format!("\tL{}:", label_idx));
                            }
                        }
                    }

                    // Check if the instruction is a jump target
                    let mut display_str = ins.display(self);
                    if ins.is_jmp() {
                        let addy = ins.get_address_field();

                        // Track labels properly
                        if !labels.contains_key(&addy) {
                            label_idx += 1;
                            labels.insert(addy, label_idx as u32);
                        }

                        let original = format!("{}", addy).to_string();
                        let enriched_label = format!("L{}, r0 # Go to Addr: {}", label_idx, addy);
                        display_str = display_str.replace(&original, &enriched_label);
                    }

                    let extra = match_instruction!(ins, target, {
                        match target {
                            Instruction::NewArrayWithBuffer(t) => {
                                let arrbuf = self.get_array_buffer(t.p2.0 as usize, 0);
                                format!("{}", print_array_vals(self, &arrbuf.1))
                            }
                            Instruction::NewArrayWithBufferLong(t) => {
                                let arrbuf = self.get_array_buffer(t.p2.0 as usize, 0);
                                format!("{}", print_array_vals(self, &arrbuf.1))
                            }
                            Instruction::NewArray(t) => {
                                format!("new Array({})", t.p0.0)
                            }
                            Instruction::NewObjectWithBuffer(t) => {
                                let keybuf = self.get_object_key_buffer(t.p2.0 as usize, 0);
                                let valbuf = self.get_object_val_buffer(t.p3.0 as usize, 0);
                                let mut joined = vec![];
                                for (key, value) in keybuf.1.iter().zip(valbuf.1.iter()) {
                                    joined.push(format!(
                                        "{}: {}",
                                        print_array_val(self, key),
                                        print_array_val(self, value)
                                    ));
                                }
                                format!("{{{}}}", joined.join(", "))
                            }
                            Instruction::NewObjectWithBufferLong(t) => {
                                let keybuf = self.get_object_key_buffer(t.p2.0 as usize, 0);
                                let valbuf = self.get_object_val_buffer(t.p3.0 as usize, 0);
                                let mut joined = vec![];
                                for (key, value) in keybuf.1.iter().zip(valbuf.1.iter()) {
                                    joined.push(format!(
                                        "{}: {}",
                                        print_array_val(self, key),
                                        print_array_val(self, value)
                                    ));
                                }
                                format!("{{{}}}", joined.join(", "))
                            }
                            _ => "".to_string(),
                        }
                    });

                    // build_instructions
                    if extra.is_empty() {
                        output.push(format!("{:?}\t{}", _idx, display_str));
                    } else {
                        output.push(format!("{:?}\t{} // {}", _idx, display_str, extra));
                    }
                }

                // populate labels into output
                for (lidx, line) in output.iter().enumerate() {
                    if labels.contains_key(&(lidx as u32)) {
                        println!("\t\tL{}:", labels.get(&(lidx as u32)).unwrap());
                    }
                    println!("{}", line);
                }
            }
        }
    }

    pub fn get_disassembled_bytecode(&mut self) -> String {
        {
            let mut output_buf: Vec<String> = vec![];
            let function_headers: Vec<_> = self
                .function_headers
                .iter()
                .enumerate()
                .map(|(i, fh)| (i, fh.clone()))
                .collect();

            for (fidx, fh) in function_headers {
                let bytecode = self.get_func_bytecode(fidx as u32);
                self._reader
                    .seek(io::SeekFrom::Start(fh.offset() as u64))
                    .unwrap();

                let func_name_idx = fh.func_name() as usize;

                let myfunc = self.string_storage.get(func_name_idx).unwrap();
                output_buf.push("------------------------------------------------\n".to_string());
                let func_start = myfunc.offset;
                let mut func_name = String::from_utf8(
                    self.string_storage_bytes
                        [func_start as usize..(func_start + myfunc.length) as usize]
                        .to_vec(),
                )
                .unwrap();

                if func_name.is_empty() {
                    func_name = format!("$FUNC_{}", fidx);
                }

                // Print out the FunctionHeader type - this makes things easier to debug.
                // There's no real spec, so I can get away with dropping a # comment here.
                let is_large = match fh {
                    FunctionHeader::Small(_) => false,
                    FunctionHeader::Large(_) => true,
                };

                output_buf.push(format!(
                    "{}<{}>({:?} params, {:?} registers, {:?} symbols): # Type: {}FunctionHeader - funcID: {} ({} bytes @ {})\n",
                    match fh.flags().prohibit_invoke {
                        FunctionHeaderFlagProhibitions::ProhibitCall => "Constructor",
                        FunctionHeaderFlagProhibitions::ProhibitConstruct => "NCFunction",
                        FunctionHeaderFlagProhibitions::ProhibitNone => "Function",
                    },
                    func_name,
                    fh.param_count(),
                    fh.frame_size(),
                    fh.env_size(),
                    if is_large { "Large" } else { "Small" },
                    fidx,
                    fh.byte_size(),
                    self._reader.stream_position().unwrap(),
                ).to_string());

                let mut labels: HashMap<u32, u32> = HashMap::new();
                let mut label_idx = 0;
                let mut output = vec![];

                for (_idx, ins) in bytecode.iter().enumerate() {
                    if fh.flags().has_exception_handler {
                        for (idx, eh) in fh.exception_handlers().iter().enumerate() {
                            let ehidx = idx + 1;
                            let has_label = if _idx == eh.start as usize {
                                label_idx += ehidx + 1;
                                true
                            } else if _idx == eh.end as usize {
                                label_idx += ehidx + 2;
                                true
                            } else if _idx == eh.target as usize {
                                label_idx += ehidx;
                                true
                            } else {
                                false
                            };

                            if has_label {
                                output.push(format!("\tL{}:", label_idx));
                            }
                        }
                    }

                    // Check if the instruction is a jump target
                    let mut display_str = ins.display(self);
                    if ins.is_jmp() {
                        let addy = ins.get_address_field();

                        // Track labels properly
                        if !labels.contains_key(&addy) {
                            label_idx += 1;
                            labels.insert(addy, label_idx as u32);
                        }

                        let original = format!("{}", addy).to_string();
                        let enriched_label = format!("L{}, r0 # Go to Addr: {}", label_idx, addy);
                        display_str = display_str.replace(&original, &enriched_label);
                    }

                    let extra = match_instruction!(ins, target, {
                        match target {
                            Instruction::NewArrayWithBuffer(t) => {
                                let arrbuf = self.get_array_buffer(t.p2.0 as usize, 0);
                                format!("{}", print_array_vals(self, &arrbuf.1))
                            }
                            Instruction::NewArrayWithBufferLong(t) => {
                                let arrbuf = self.get_array_buffer(t.p2.0 as usize, 0);
                                format!("{}", print_array_vals(self, &arrbuf.1))
                            }
                            Instruction::NewArray(t) => {
                                format!("new Array({})", t.p0.0)
                            }
                            Instruction::NewObjectWithBuffer(t) => {
                                let keybuf = self.get_object_key_buffer(t.p2.0 as usize, 0);
                                let valbuf = self.get_object_val_buffer(t.p3.0 as usize, 0);
                                let mut joined = vec![];
                                for (key, value) in keybuf.1.iter().zip(valbuf.1.iter()) {
                                    joined.push(format!(
                                        "{}: {}",
                                        print_array_val(self, key),
                                        print_array_val(self, value)
                                    ));
                                }
                                format!("{{{}}}", joined.join(", "))
                            }
                            Instruction::NewObjectWithBufferLong(t) => {
                                let keybuf = self.get_object_key_buffer(t.p2.0 as usize, 0);
                                let valbuf = self.get_object_val_buffer(t.p3.0 as usize, 0);
                                let mut joined = vec![];
                                for (key, value) in keybuf.1.iter().zip(valbuf.1.iter()) {
                                    joined.push(format!(
                                        "{}: {}",
                                        print_array_val(self, key),
                                        print_array_val(self, value)
                                    ));
                                }
                                format!("{{{}}}", joined.join(", "))
                            }
                            _ => "".to_string(),
                        }
                    });

                    // build_instructions
                    if extra.is_empty() {
                        output.push(format!("{:?}\t{}", _idx, display_str));
                    } else {
                        output.push(format!("{:?}\t{} // {}", _idx, display_str, extra));
                    }
                }

                // populate labels into output
                for (lidx, line) in output.iter().enumerate() {
                    if labels.contains_key(&(lidx as u32)) {
                        if let Some(label) = labels.get(&(lidx as u32)) {
                            output_buf.push(format!("\t\tL{}:", label));
                        }
                    }
                    output_buf.push(line.clone());
                }
            }
            output_buf.join("\n")
        }
        //end
    }

    pub fn get_disassembled_bytecode_for_function(&mut self, fidx: usize) -> String {
        {
            let mut output_buf: Vec<String> = vec![];
            let fh = self.function_headers.get(fidx).unwrap().clone();

            let bytecode = self.get_func_bytecode(fidx as u32);
            self._reader
                .seek(io::SeekFrom::Start(fh.offset() as u64))
                .unwrap();

            let func_name_idx = fh.func_name() as usize;

            let myfunc = self.string_storage.get(func_name_idx).unwrap();
            output_buf.push("------------------------------------------------\n".to_string());
            let func_start = myfunc.offset;
            let mut func_name = String::from_utf8(
                self.string_storage_bytes
                    [func_start as usize..(func_start + myfunc.length) as usize]
                    .to_vec(),
            )
            .unwrap();

            if func_name.is_empty() {
                func_name = format!("$FUNC_{}", fidx);
            }

            // Print out the FunctionHeader type - this makes things easier to debug.
            // There's no real spec, so I can get away with dropping a # comment here.
            let is_large = match fh {
                FunctionHeader::Small(_) => false,
                FunctionHeader::Large(_) => true,
            };

            output_buf.push(format!(
                    "{}<{}>({:?} params, {:?} registers, {:?} symbols): # Type: {}FunctionHeader - funcID: {} ({} bytes @ {})\n",
                    match fh.flags().prohibit_invoke {
                        FunctionHeaderFlagProhibitions::ProhibitCall => "Constructor",
                        FunctionHeaderFlagProhibitions::ProhibitConstruct => "NCFunction",
                        FunctionHeaderFlagProhibitions::ProhibitNone => "Function",
                    },
                    func_name,
                    fh.param_count(),
                    fh.frame_size(),
                    fh.env_size(),
                    if is_large { "Large" } else { "Small" },
                    fidx,
                    fh.byte_size(),
                    self._reader.stream_position().unwrap(),
                ).to_string());

            let mut labels: HashMap<u32, u32> = HashMap::new();
            let mut label_idx = 0;
            let mut output = vec![];

            for (_idx, ins) in bytecode.iter().enumerate() {
                if fh.flags().has_exception_handler {
                    for (idx, eh) in fh.exception_handlers().iter().enumerate() {
                        let ehidx = idx + 1;
                        let has_label = if _idx == eh.start as usize {
                            label_idx += ehidx + 1;
                            true
                        } else if _idx == eh.end as usize {
                            label_idx += ehidx + 2;
                            true
                        } else if _idx == eh.target as usize {
                            label_idx += ehidx;
                            true
                        } else {
                            false
                        };

                        if has_label {
                            output.push(format!("\tL{}:", label_idx));
                        }
                    }
                }

                // Check if the instruction is a jump target
                let mut display_str = ins.display(self);
                if ins.is_jmp() {
                    let addy = ins.get_address_field();

                    // Track labels properly
                    if !labels.contains_key(&addy) {
                        label_idx += 1;
                        labels.insert(addy, label_idx as u32);
                    }

                    let original = format!("{}", addy).to_string();
                    let enriched_label = format!("L{}, r0 # Go to Addr: {}", label_idx, addy);
                    display_str = display_str.replace(&original, &enriched_label);
                }

                let extra = match_instruction!(ins, target, {
                    match target {
                        Instruction::NewArrayWithBuffer(t) => {
                            let arrbuf = self.get_array_buffer(t.p2.0 as usize, 0);
                            format!("{}", print_array_vals(self, &arrbuf.1))
                        }
                        Instruction::NewArrayWithBufferLong(t) => {
                            let arrbuf = self.get_array_buffer(t.p2.0 as usize, 0);
                            format!("{}", print_array_vals(self, &arrbuf.1))
                        }
                        Instruction::NewArray(t) => {
                            format!("new Array({})", t.p0.0)
                        }
                        Instruction::NewObjectWithBuffer(t) => {
                            let keybuf = self.get_object_key_buffer(t.p2.0 as usize, 0);
                            let valbuf = self.get_object_val_buffer(t.p3.0 as usize, 0);
                            let mut joined = vec![];
                            for (key, value) in keybuf.1.iter().zip(valbuf.1.iter()) {
                                joined.push(format!(
                                    "{}: {}",
                                    print_array_val(self, key),
                                    print_array_val(self, value)
                                ));
                            }
                            format!("{{{}}}", joined.join(", "))
                        }
                        Instruction::NewObjectWithBufferLong(t) => {
                            let keybuf = self.get_object_key_buffer(t.p2.0 as usize, 0);
                            let valbuf = self.get_object_val_buffer(t.p3.0 as usize, 0);
                            let mut joined = vec![];
                            for (key, value) in keybuf.1.iter().zip(valbuf.1.iter()) {
                                joined.push(format!(
                                    "{}: {}",
                                    print_array_val(self, key),
                                    print_array_val(self, value)
                                ));
                            }
                            format!("{{{}}}", joined.join(", "))
                        }
                        _ => "".to_string(),
                    }
                });

                // build_instructions
                if extra.is_empty() {
                    output.push(format!("{:?}\t{}", _idx, display_str));
                } else {
                    output.push(format!("{:?}\t{} // {}", _idx, display_str, extra));
                }
            }

            // populate labels into output
            for (lidx, line) in output.iter().enumerate() {
                if labels.contains_key(&(lidx as u32)) {
                    if let Some(label) = labels.get(&(lidx as u32)) {
                        output_buf.push(format!("\t\tL{}:", label));
                    }
                }
                output_buf.push(line.clone());
            }
            output_buf.join("\n")
        }
    }
    // ------------------------------------------ //
    // helper methods end
    // ------------------------------------------ //
}
