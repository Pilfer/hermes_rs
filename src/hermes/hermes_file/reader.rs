use std::collections::HashMap;
use std::io;

use crate::hermes::big_int_table::BigIntTableEntry;
use crate::hermes::header::HermesHeader;

use crate::hermes::cjs_module::{CJSModule, CJSModuleEntry, CJSModuleInt};
use crate::hermes::debug_info::{DebugInfo, DebugInfoHeader};
use crate::hermes::decode::{align_reader, decode_u32};
use crate::hermes::function_header::FunctionHeader;
use crate::hermes::function_header::{LargeFunctionHeader, SmallFunctionHeader};
use crate::hermes::function_sources::FunctionSourceEntry;
use crate::hermes::regexp_table::RegExpTableEntry;
use crate::hermes::string_kind::StringKindEntry;
use crate::hermes::string_table::{OverflowStringTableEntry, SmallStringTableEntry};
use crate::hermes::{Instruction, InstructionParser, Serializable};

use super::{HermesFile, HermesStructReader};

impl<R> HermesFile<R>
where
    R: io::Read + io::BufRead + io::Seek,
{
    pub fn new(r: R) -> Self {
        Self {
            _reader: r,
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
        // The bytecode of all of the functions are in this section. When reading, we have the offsets so we know where to start reading from.
        // The debug_info for all of the functions follow the bytecode. Same as above - the info offset is in the function header, so we simply just read that.
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
            let sfh = SmallFunctionHeader::deserialize(&mut self._reader, self.header.version);

            // #[allow(clippy::seek_from_current)]
            let _current_pos = self._reader.stream_position().unwrap();

            // Check if we're dealing with a Small or Large Function Header.
            // Overflowed = Large Function Header
            let function_header_val: FunctionHeader = if !sfh.flags.overflowed {
                FunctionHeader::Small(sfh)
            } else {
                let new_offset = sfh.info_offset << 16 | sfh.offset;

                // Go back to the start of the LFH to deserialize it properly
                self._reader
                    .seek(io::SeekFrom::Start(new_offset as u64))
                    .expect("unable to seek to overflowed function header");

                let lfh = LargeFunctionHeader::deserialize(&mut self._reader, self.header.version);
                FunctionHeader::Large(lfh.clone())
            };

            self._reader
                .seek(io::SeekFrom::Start(_current_pos))
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

            // Get storage bytes
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

    /*
     * Returns the bytecode for each function in the Hermes file. HashMap is indexed by function index.
     */
    pub fn get_bytecode(&mut self) -> HashMap<u32, Vec<u8>> {
        let mut out = HashMap::new();
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

            out.insert(idx as u32, buf);
        }

        out
    }

    // ------------------------------------------ //
    // helper methods start
    // ------------------------------------------ //

    /*
     * Returns a vector of all the strings from the string storage
     */
    pub fn get_strings(&self) -> Vec<String> {
        let mut out = vec![];
        for (idx, _) in self.string_storage.iter().enumerate() {
            out.push(self.get_string_from_storage_by_index(idx));
        }
        out
    }

    pub fn get_string_from_storage_by_index(&self, index: usize) -> String {
        let myfunc = self.string_storage.get(index).unwrap();
        if myfunc.is_utf_16 {
            {
                let bytes = self.string_storage_bytes
                    [myfunc.offset as usize..(myfunc.offset + (myfunc.length * 2)) as usize]
                    .to_vec();
                let utf16_values: Vec<u16> = bytes
                    .chunks(2)
                    .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                    .collect();

                String::from_utf16(&utf16_values).unwrap()
            }
        } else {
            String::from_utf8(
                self.string_storage_bytes
                    [myfunc.offset as usize..(myfunc.offset + myfunc.length) as usize]
                    .to_vec(),
            )
            .unwrap()
        }
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

        // println!("bytecode as hex: {:?}", bytecode_buf);

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
            let ins_obj: Option<Instruction> = match self.header.version {
                #[cfg(feature = "v89")]
                89 => Some(Instruction::V89(
                    crate::hermes::v89::Instruction::deserialize(&mut r_cursor, op),
                )),
                #[cfg(feature = "v90")]
                90 => Some(Instruction::V90(
                    crate::hermes::v90::Instruction::deserialize(&mut r_cursor, op),
                )),
                #[cfg(feature = "v93")]
                93 => Some(Instruction::V93(
                    crate::hermes::v93::Instruction::deserialize(&mut r_cursor, op),
                )),
                #[cfg(feature = "v94")]
                94 => Some(Instruction::V94(
                    crate::hermes::v94::Instruction::deserialize(&mut r_cursor, op),
                )),
                #[cfg(feature = "v95")]
                95 => Some(Instruction::V95(
                    crate::hermes::v95::Instruction::deserialize(&mut r_cursor, op),
                )),
                #[cfg(feature = "v96")]
                96 => Some(Instruction::V96(
                    crate::hermes::v96::Instruction::deserialize(&mut r_cursor, op),
                )),
                _ => {
                    panic!("Unsupported HBC version: {:?}. Check Cargo.toml features to see if this HBC version is enabled.", self.header.version);
                    // None
                }
            };

            // let ins: Instruction = ins_obj.unwrap();
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

                let myfunc = self.string_storage.get(fh.func_name() as usize).unwrap();
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

                println!(
                    "Function<{}>({:?} params, {:?} registers, {:?} symbols):",
                    func_name,
                    fh.param_count(),
                    fh.frame_size(),
                    fh.env_size()
                );

                // println!("bytecode as hex: {:?}", bytecode_buf);

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
                    let ins_obj: Option<Instruction> = match self.header.version {
                        #[cfg(feature = "v89")]
                        89 => Some(Instruction::V89(
                            crate::hermes::v89::Instruction::deserialize(&mut r_cursor, op),
                        )),
                        #[cfg(feature = "v90")]
                        90 => Some(Instruction::V90(
                            crate::hermes::v90::Instruction::deserialize(&mut r_cursor, op),
                        )),
                        #[cfg(feature = "v93")]
                        93 => Some(Instruction::V93(
                            crate::hermes::v93::Instruction::deserialize(&mut r_cursor, op),
                        )),
                        #[cfg(feature = "v94")]
                        94 => Some(Instruction::V94(
                            crate::hermes::v94::Instruction::deserialize(&mut r_cursor, op),
                        )),
                        #[cfg(feature = "v95")]
                        95 => Some(Instruction::V95(
                            crate::hermes::v95::Instruction::deserialize(&mut r_cursor, op),
                        )),
                        #[cfg(feature = "v96")]
                        96 => Some(Instruction::V96(
                            crate::hermes::v96::Instruction::deserialize(&mut r_cursor, op),
                        )),
                        _ => {
                            panic!("Unsupported HBC version: {:?}. Check Cargo.toml features to see if this HBC version is enabled.", self.header.version);
                            // None
                        }
                    };

                    // let ins: Instruction = ins_obj.unwrap();
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
                            println!("    L{}:", labels.get(&byte_index).unwrap());
                        }

                        // build_instructions
                        println!("{}\t{}", byte_index, display_str);
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
        }
    }

    // ------------------------------------------ //
    // helper methods end
    // ------------------------------------------ //
}
