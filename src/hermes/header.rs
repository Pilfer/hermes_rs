use std::collections::HashMap;
use std::io;

use crate::hermes::bytecode_options::BytecodeOptions;
use crate::hermes::debug_info::DebugInfoOffsets;
use crate::hermes::decode::{decode_u32, decode_u64};
use crate::hermes::exception_handler::ExceptionHandlerInfo;
use crate::hermes::small_function_header::SmallFunctionHeader;
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

    pub function_headers: Vec<SmallFunctionHeader>,
    pub string_kinds: Vec<StringKindEntry>,
    pub string_storage: Vec<SmallStringTableEntry>,
    pub string_storage_bytes: Vec<u8>,
    pub overflow_string_storage: Vec<OverflowStringTableEntry>,
    // options - u8, pad 19 bytes after
}

impl HermesHeader {
    pub fn get_string_from_storage_by_index(&self, index: usize) -> String {
        let myfunc = self.string_storage.get(index).unwrap();
        return String::from_utf8(
            self.string_storage_bytes
                [myfunc.offset as usize..(myfunc.offset + myfunc.length) as usize]
                .to_vec(),
        )
        .unwrap();
    }
}

pub trait HermesStruct {
    fn deserialize<R>(r: &mut R) -> Self
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
        // 80 - u32 fields
        // 8  - u64 field
        // 20 - raw u8s
        // 1  - byte for bytecode options
        // 19 - padding bytes
        128
    }

    fn deserialize<R>(r: &mut R) -> Self
    where
        R: io::Read + io::BufRead + io::Seek,
    {
        let magic: u64 = decode_u64(r);
        println!("magic: {:?}", magic);

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

        let options = BytecodeOptions::deserialize(r);

        // Read padding bytes
        let mut _pad_bytes = [0u8; 19];
        let _pad = r
            .read_exact(&mut _pad_bytes)
            .expect("error reading padding bytes");

        // Read the function headers
        let mut function_headers: Vec<SmallFunctionHeader> = vec![];
        for _ in 0..function_count {
            let mut fh = SmallFunctionHeader::deserialize(r);

            if fh.flags.has_exception_handler {
                println!("Function has exception handler");
                let current_pos = r.seek(io::SeekFrom::Current(0)).unwrap();
                r.seek(io::SeekFrom::Start(fh.info_offset as u64))
                    .expect("unable to seek to exception handler info");

                let exc_headers_count = decode_u32(r);
                println!("Exception handler count: {:?}", exc_headers_count);

                // let mut exception_handlers = vec![];
                for _ in 0..exc_headers_count {
                    fh.exception_handlers
                        .push(ExceptionHandlerInfo::deserialize(r));
                }

                // go back to where we were
                r.seek(io::SeekFrom::Start(current_pos))
                    .expect("unable to seek back to original position");
            }

            if fh.flags.has_debug_info {
                println!("Function has debug info");
                // go to the debug_info_offset
                let current_pos = r.seek(io::SeekFrom::Current(0)).unwrap();
                r.seek(io::SeekFrom::Start(debug_info_offset as u64))
                    .expect("unable to seek to debug info");

                let debug_info = DebugInfoOffsets::deserialize(r);
                fh.debug_info = debug_info;

                println!("the Debug info ({}): {:?}", fh.func_name, &fh.debug_info);

                // go back to where we were
                r.seek(io::SeekFrom::Start(current_pos))
                    .expect("unable to seek back to original position");
                // DebugInfo
            }

            // println!("sfh byte size: {:?}", sfh.byte_size);
            function_headers.push(fh);
        }

        // Read string kinds
        let mut string_kinds: Vec<StringKindEntry> = vec![];
        for _string_kind_idx in 0..string_kind_count {
            let string_kind = StringKindEntry::deserialize(r);
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
            let string_item = SmallStringTableEntry::deserialize(r);
            string_storage.push(string_item);
        }

        let mut overflow_string_storage: Vec<OverflowStringTableEntry> = vec![];
        for _ in 0..overflow_string_count {
            overflow_string_storage.push(OverflowStringTableEntry::deserialize(r));
        }

        let mut string_storage_bytes_real = vec![0u8; string_storage_size as usize];
        r.read_exact(&mut string_storage_bytes_real)
            .expect("unable to read string storage");
        let string_storage_as_str = String::from_utf8(string_storage_bytes_real.clone()).unwrap();
        println!("string_storage: {:?}", string_storage_as_str);

        let mut array_buffer_storage = vec![0u8; array_buffer_size as usize];
        r.read_exact(&mut array_buffer_storage)
            .expect("unable to read array buffer storage");

        let mut object_key_buffer = vec![0u8; obj_key_buffer_size as usize];
        r.read_exact(&mut object_key_buffer)
            .expect("unable to read object key buffer storage");

        let mut object_val_buffer = vec![0u8; obj_value_buffer_size as usize];
        r.read_exact(&mut object_val_buffer)
            .expect("unable to read object value buffer storage");

        // TODO: Add debug info stuff
        // https://github.com/facebook/hermes/blob/main/lib/BCGen/HBC/DebugInfo.cpp#L181
        println!("Hermes version: {:?}", version);
        return Self {
            magic: magic,
            version: version,
            sha1: sha1,
            file_length: file_length,
            global_code_index: global_code_index,
            function_count: function_count,
            string_kind_count: string_kind_count,
            identifier_count: identifier_count,
            string_count: string_count,
            overflow_string_count: overflow_string_count,
            string_storage_size: string_storage_size,
            big_int_count: big_int_count,
            big_int_storage_size: big_int_storage_size,
            reg_exp_count: reg_exp_count,
            reg_exp_storage_size: reg_exp_storage_size,
            array_buffer_size: array_buffer_size,
            obj_key_buffer_size: obj_key_buffer_size,
            obj_value_buffer_size: obj_value_buffer_size,
            segment_id: segment_id,
            cjs_module_count: cjs_module_count,
            function_source_count: function_source_count,
            debug_info_offset: debug_info_offset,
            options: options,
            _padding: _pad_bytes,
            function_headers: function_headers,
            string_kinds: string_kinds,
            string_storage: string_storage,
            string_storage_bytes: string_storage_bytes_real,
            overflow_string_storage: overflow_string_storage,
        };
    }

    fn parse_bytecode<R: io::Read + io::BufRead + io::Seek>(&self, r: &mut R) {
        // Function body goes here
        {
            for fh in &self.function_headers {
                // println!("function header: {:?}", fh);
                r.seek(io::SeekFrom::Start(fh.offset as u64)).unwrap();
                let mut bytecode_buf = vec![0u8; fh.byte_size as usize];
                r.read_exact(&mut bytecode_buf)
                    .expect("unable to read first functions bytecode");

                let myfunc = self.string_storage.get(fh.func_name as usize).unwrap();
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
                    func_name, fh.param_count, fh.frame_size, fh.env_size
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
                    let op = op_byte.clone();
                    // Make a new Cursor to print the remaining bytes
                    let mut r_cursor = io::Cursor::new(byte_iter.as_slice());

                    // Deserialize the instruction
                    let ins_obj: Option<Instruction> = match self.version {
                        #[cfg(feature = "v89")]
                        89 => Some(Instruction::V89(v89::Instruction::deserialize(
                            &mut r_cursor,
                            op,
                        ))),
                        #[cfg(feature = "v90")]
                        90 => Some(Instruction::V90(v90::Instruction::deserialize(
                            &mut r_cursor,
                            op,
                        ))),
                        #[cfg(feature = "v93")]
                        93 => Some(Instruction::V93(v93::Instruction::deserialize(
                            &mut r_cursor,
                            op,
                        ))),
                        #[cfg(feature = "v94")]
                        94 => Some(Instruction::V94(
                            crate::hermes::v94::Instruction::deserialize(&mut r_cursor, op),
                        )),
                        #[cfg(feature = "v95")]
                        95 => Some(Instruction::V95(v95::Instruction::deserialize(
                            &mut r_cursor,
                            op,
                        ))),
                        _ => None,
                    };

                    // let ins: Instruction = ins_obj.unwrap();
                    if let Some(ins) = ins_obj {
                        // This label code may be the worst code I've ever written
                        let mut label_idx = 0;

                        // Exception handler logic here
                        if fh.flags.has_exception_handler {
                            for (idx, eh) in fh.exception_handlers.iter().enumerate() {
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

    fn serialize<W>(&self, _w: &mut W)
    where
        W: io::Write,
    {
        todo!()
    }

    // Read string
    // Read function
}
