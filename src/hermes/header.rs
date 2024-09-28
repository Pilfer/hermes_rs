use core::panic;
use std::collections::HashMap;
use std::iter::Iterator;
use std::{io, u32};

use crate::hermes::big_int_table::BigIntTableEntry;
use crate::hermes::bytecode_options::BytecodeOptions;
use crate::hermes::cjs_module::{CJSModule, CJSModuleEntry, CJSModuleInt};
use crate::hermes::debug_info::DebugInfoOffsets;
use crate::hermes::decode::{decode_u32, decode_u64};
use crate::hermes::encode::{encode_u32, encode_u64};
use crate::hermes::exception_handler::ExceptionHandlerInfo;
use crate::hermes::function_header::FunctionHeader;
use crate::hermes::function_header::{LargeFunctionHeader, SmallFunctionHeader};
use crate::hermes::function_sources::FunctionSourceEntry;
use crate::hermes::regexp_table::RegExpTableEntry;
use crate::hermes::string_kind::StringKindEntry;
use crate::hermes::string_table::{OverflowStringTableEntry, SmallStringTableEntry};
use crate::hermes::{Instruction, InstructionParser, Serializable};

#[derive(Debug)]
pub struct HermesHeader {
    pub magic: u64,
    pub version: u32,
    pub sha1: [u8; 20],
    pub file_length: u32,
    pub global_code_index: u32,
    pub function_count: u32,
    pub string_kind_count: u32,
    pub identifier_count: u32,
    pub string_count: u32,
    pub overflow_string_count: u32,
    pub string_storage_size: u32,
    pub big_int_count: u32,
    pub big_int_storage_size: u32,
    pub reg_exp_count: u32,
    pub reg_exp_storage_size: u32,
    pub array_buffer_size: u32,
    pub obj_key_buffer_size: u32,
    pub obj_value_buffer_size: u32,
    pub segment_id: u32,
    pub cjs_module_count: u32,
    pub function_source_count: u32,
    pub debug_info_offset: u32,

    pub options: BytecodeOptions,
    pub _padding: [u8; 19],

    pub function_headers: Vec<FunctionHeader>,
    pub string_kinds: Vec<StringKindEntry>,
    pub identifier_hashes: Vec<u32>,

    pub string_storage: Vec<SmallStringTableEntry>,
    pub string_storage_bytes: Vec<u8>,
    pub overflow_string_storage: Vec<OverflowStringTableEntry>,
    pub array_buffer_storage: Vec<u8>,
    pub object_key_buffer: Vec<u8>,
    pub object_val_buffer: Vec<u8>,

    pub big_int_table: Vec<BigIntTableEntry>,
    pub reg_exp_table: Vec<RegExpTableEntry>,
    pub cjs_modules: Vec<CJSModule>,
    pub function_source_entries: Vec<FunctionSourceEntry>,
    // options - u8, pad 19 bytes after
}

impl HermesHeader {
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
}

pub trait HermesStruct {
    fn deserialize<R>(r: &mut R, _version: u32) -> Self
    where
        R: io::Read + io::BufRead + io::Seek;

    fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write;

    fn size(&self) -> usize;

    fn parse_bytecode<R>(&self, r: &mut R)
    where
        R: io::Read + io::BufRead + io::Seek;
}

impl HermesStruct for HermesHeader {
    fn size(&self) -> usize {
        128
    }

    fn deserialize<R>(r: &mut R, _version: u32) -> Self
    where
        R: io::Read + io::BufRead + io::Seek,
    {
        let magic: u64 = decode_u64(r);

        let version = decode_u32(r);
        let mut sha1_bytes = [0u8; 20];
        r.read_exact(&mut sha1_bytes)
            .expect("Could not read sha1 bytes");
        let sha1 = sha1_bytes;
        let file_length = decode_u32(r);
        let global_code_index = decode_u32(r);
        let function_count = decode_u32(r);
        let string_kind_count = decode_u32(r);
        let identifier_count = decode_u32(r);
        let string_count = decode_u32(r);
        let overflow_string_count = decode_u32(r);
        let string_storage_size = decode_u32(r);
        let big_int_count = decode_u32(r);
        let big_int_storage_size = decode_u32(r);
        let reg_exp_count = decode_u32(r);
        let reg_exp_storage_size = decode_u32(r);
        let array_buffer_size = decode_u32(r);
        let obj_key_buffer_size = decode_u32(r);
        let obj_value_buffer_size = decode_u32(r);
        let segment_id = decode_u32(r);
        let cjs_module_count = decode_u32(r);
        let function_source_count = decode_u32(r);
        let debug_info_offset = decode_u32(r);

        let options = BytecodeOptions::deserialize(r, version);

        // Read padding bytes
        let mut _pad_bytes = [0u8; 19];
        r.read_exact(&mut _pad_bytes)
            .expect("error reading padding bytes");

        // ============================= //
        // Read the function headers
        // Two storage sizes - small and large
        // Read small first to see if it overflows
        // If it overflows, seek back and read large struct
        // else, carry on.
        // ============================= //
        let mut function_headers: Vec<FunctionHeader> = vec![];
        for _ in 0..function_count {
            let sfh = SmallFunctionHeader::deserialize(r, version);

            #[allow(clippy::seek_from_current)]
            let _current_pos = r.seek(io::SeekFrom::Current(0)).unwrap();
            let function_header_val: FunctionHeader;

            if !sfh.flags.overflowed {
                function_header_val = FunctionHeader::Small(sfh);
            } else {
                let new_offset = sfh.info_offset << 16 | sfh.offset;

                // Go back to the start of the LFH to deserialize it properly
                r.seek(io::SeekFrom::Start(new_offset as u64))
                    .expect("unable to seek to overflowed function header");

                // Large function header reading
                let fh = LargeFunctionHeader::deserialize(r, version);
                function_header_val = FunctionHeader::Large(fh);
            }

            let mut fhv = function_header_val.clone();

            function_headers.push(function_header_val);

            // Read the ExceptionInfo
            if fhv.flags().has_exception_handler {
                let mut exception_handlers: Vec<ExceptionHandlerInfo> = vec![];
                let exc_headers_count = decode_u32(r);
                for _ in 0..exc_headers_count {
                    exception_handlers.push(ExceptionHandlerInfo::deserialize(r, version));
                }

                match fhv {
                    FunctionHeader::Small(ref mut sfh) => {
                        sfh.exception_handlers = exception_handlers;
                    }
                    FunctionHeader::Large(ref mut lfh) => {
                        lfh.exception_handlers = exception_handlers;
                    }
                }
            }

            if fhv.flags().has_debug_info {
                let debug_info = DebugInfoOffsets::deserialize(r, version);
                match fhv {
                    FunctionHeader::Small(ref mut sfh) => {
                        sfh.debug_info = debug_info;
                    }
                    FunctionHeader::Large(ref mut lfh) => {
                        lfh.debug_info = debug_info;
                    }
                }
            }

            r.seek(io::SeekFrom::Start(_current_pos as u64))
                .expect("unable to seek to function header");
        } // End of function headers

        // Read string kinds
        let mut string_kinds: Vec<StringKindEntry> = vec![];
        for _string_kind_idx in 0..string_kind_count {
            let string_kind = StringKindEntry::deserialize(r, version);
            string_kinds.push(string_kind);
        }

        // Read identifier hashes
        let mut identifier_hashes: Vec<u32> = vec![];
        for _ in 0..identifier_count {
            identifier_hashes.push(decode_u32(r));
        }

        // Read small string table entry
        let mut string_storage: Vec<SmallStringTableEntry> = vec![];
        for _ in 0..string_count {
            let string_item = SmallStringTableEntry::deserialize(r, version);
            string_storage.push(string_item);
        }

        // Read overflow string table entries
        let mut overflow_string_storage: Vec<OverflowStringTableEntry> = vec![];
        for _ in 0..overflow_string_count {
            overflow_string_storage.push(OverflowStringTableEntry::deserialize(r, version));
        }

        // Read string storage bytes
        let mut string_storage_bytes_real = vec![0u8; string_storage_size as usize];
        r.read_exact(&mut string_storage_bytes_real)
            .expect("unable to read string storage");

        // Read array buffer storage
        let mut array_buffer_storage = vec![0u8; array_buffer_size as usize];
        r.read_exact(&mut array_buffer_storage)
            .expect("unable to read array buffer storage");

        // Read object key buffer
        let mut object_key_buffer = vec![0u8; obj_key_buffer_size as usize];
        r.read_exact(&mut object_key_buffer)
            .expect("unable to read object key buffer storage");

        // Read object value buffer
        let mut object_val_buffer = vec![0u8; obj_value_buffer_size as usize];
        r.read_exact(&mut object_val_buffer)
            .expect("unable to read object value buffer storage");

        // Read big int table
        let mut big_int_table = vec![];
        if big_int_count > 0 && version >= 87 {
            for _ in 0..big_int_count {
                big_int_table.push(BigIntTableEntry::deserialize(r, version));
            }
        }

        // Read regexp table
        let mut reg_exp_table = vec![];
        if reg_exp_count > 0 {
            for _ in 0..reg_exp_count {
                reg_exp_table.push(RegExpTableEntry::deserialize(r, version));
            }

            // Read the regexp storage buffer
            // TODO: make parser for this. The RegExp stuff has a bespoke bytecode as well.
            // I'm not sure how useful it'd be for people to be able to dig through this, but for the sake
            // of completeness, it should be done.
            let mut reg_exp_storage = vec![0u8; reg_exp_storage_size as usize];
            r.read_exact(&mut reg_exp_storage)
                .expect("unable to read regexp storage");
        }

        // Read CJS modules
        let mut cjs_modules: Vec<CJSModule> = vec![];
        if cjs_module_count > 0 {
            if options.cjs_modules_statically_resolved && version < 77 {
                for _ in 0..cjs_module_count {
                    let cjs_module = CJSModuleInt::deserialize(r, version);
                    cjs_modules.push(CJSModule::CJSModuleInt(cjs_module));
                }
            } else {
                for _ in 0..cjs_module_count {
                    let cjs_module = CJSModuleEntry::deserialize(r, version);
                    cjs_modules.push(CJSModule::CJSModuleEntry(cjs_module));
                }
            }
        }

        let mut function_source_entries: Vec<FunctionSourceEntry> = vec![];
        if function_source_count > 0 {
            for _ in 0..function_source_count {
                function_source_entries.push(FunctionSourceEntry::deserialize(r, version));
            }
        }

        Self {
            magic,
            version,
            sha1,
            file_length,
            global_code_index,
            function_count,
            string_kind_count,
            identifier_count,
            string_count,
            overflow_string_count,
            string_storage_size,
            big_int_count,
            big_int_storage_size,
            reg_exp_count,
            reg_exp_storage_size,
            array_buffer_size,
            obj_key_buffer_size,
            obj_value_buffer_size,
            segment_id,
            cjs_module_count,
            function_source_count,
            debug_info_offset,
            options,
            _padding: _pad_bytes,
            function_headers,
            string_kinds,
            identifier_hashes,
            string_storage,
            string_storage_bytes: string_storage_bytes_real,
            overflow_string_storage,
            array_buffer_storage,
            object_key_buffer,
            object_val_buffer,
            big_int_table,
            reg_exp_table,
            cjs_modules,
            function_source_entries,
        }
    }

    fn parse_bytecode<R: io::Read + io::BufRead + io::Seek>(&self, r: &mut R) {
        // Function body goes here
        {
            for fh in &self.function_headers {
                r.seek(io::SeekFrom::Start(fh.offset() as u64)).unwrap();
                let mut bytecode_buf = vec![0u8; fh.byte_size() as usize];
                r.read_exact(&mut bytecode_buf)
                    .expect("unable to read first functions bytecode");

                let myfunc = self.string_storage.get(fh.func_name() as usize).unwrap();
                println!("------------------------------------------------");
                let func_start = myfunc.offset;
                let func_name = String::from_utf8(
                    self.string_storage_bytes
                        [func_start as usize..(func_start + myfunc.length) as usize]
                        .to_vec(),
                )
                .unwrap();

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
                    let ins_obj: Option<Instruction> = match self.version {
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
                        _ => {
                            panic!("Unsupported HBC version: {:?}. Check Cargo.toml features to see if this HBC version is enabled.", self.version);
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
                            // I want to get the struct members of `ins` here.
                            // println!("==========={}: {} // {}", byte_index, display_str, addy);
                        }

                        if labels.get(&byte_index).is_some() {
                            println!("    L{}:", labels.get(&byte_index).unwrap());
                        }
                        // build_instructions

                        println!("{}\t{}", byte_index, display_str);
                        let size = ins.size();
                        instructions_list.push(ins);
                        // println!("--------- index is {}", index);
                        // println!("--------- byte_index is {}", byte_index);
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

    fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write,
    {
        encode_u64(w, self.magic);
        encode_u32(w, self.version);
        w.write_all(&self.sha1).expect("unable to write sha1");
        encode_u32(w, self.file_length);
        encode_u32(w, self.global_code_index);
        encode_u32(w, self.function_count);
        encode_u32(w, self.string_kind_count);
        encode_u32(w, self.identifier_count);
        encode_u32(w, self.string_count);
        encode_u32(w, self.overflow_string_count);
        encode_u32(w, self.string_storage_size);
        encode_u32(w, self.big_int_count);
        encode_u32(w, self.big_int_storage_size);
        encode_u32(w, self.reg_exp_count);
        encode_u32(w, self.reg_exp_storage_size);
        encode_u32(w, self.array_buffer_size);
        encode_u32(w, self.obj_key_buffer_size);
        encode_u32(w, self.obj_value_buffer_size);
        encode_u32(w, self.segment_id);
        encode_u32(w, self.cjs_module_count);
        encode_u32(w, self.function_source_count);
        encode_u32(w, self.debug_info_offset);

        self.options.serialize(w);

        // Write padding bytes
        w.write_all(&self._padding)
            .expect("unable to write padding bytes");

        // Write function headers
        for fh in &self.function_headers {
            fh.serialize(w);
        }

        // Write string kinds
        for sk in &self.string_kinds {
            sk.serialize(w);
        }

        // Write identifier hashes
        for ih in &self.identifier_hashes {
            encode_u32(w, *ih);
        }

        // Write string table entries
        for ss in &self.string_storage {
            ss.serialize(w);
        }

        // Write overflow string table entries
        for os in &self.overflow_string_storage {
            os.serialize(w);
        }

        // Write string storage bytes
        w.write_all(&self.string_storage_bytes)
            .expect("unable to write string storage bytes");

        // Write array buffer storage
        w.write_all(&self.array_buffer_storage)
            .expect("unable to write array buffer storage");

        // Write object key buffer
        w.write_all(&self.object_key_buffer)
            .expect("unable to write object key buffer");

        // Write object value buffer
        w.write_all(&self.object_val_buffer)
            .expect("unable to write object value buffer");
    }

    // Read string
    // Read function
}
