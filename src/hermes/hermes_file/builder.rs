use std::io;

use super::FunctionInstructions;
use super::HermesFile;
use crate::hermes::big_int_table::BigIntTableEntry;
use crate::hermes::debug_info::DebugStringTable;
use crate::hermes::function_header::{FunctionHeader, LargeFunctionHeader, SmallFunctionHeader};
use crate::hermes::IntoParentInstruction;
use crate::hermes::OverflowStringTableEntry;
use crate::hermes::SmallStringTableEntry;
use crate::hermes::{HermesInstruction, InstructionParser};

impl<R> HermesFile<R>
where
    R: io::Read + io::BufRead + io::Seek,
{
    // Set the strings present in the HermesFile, build out String Table, etc...
    pub fn set_strings(&mut self, strings: Vec<String>) {
        let mut string_storage: Vec<SmallStringTableEntry> = vec![];
        let mut string_storage_bytes = vec![];
        for string in strings {
            // check if string is utf-16
            let is_utf_16 = string.chars().any(|c| c as u32 > 0x10000);
            let offset = string_storage_bytes.len() as u32;
            let length = string.len() as u32;
            string_storage.push(SmallStringTableEntry {
                is_utf_16,
                offset,
                length,
            });
            string_storage_bytes.extend(string.as_bytes());
        }

        // pad an extra 10000000 bytes to the end of the string storage for testing.
        // string_storage_bytes.extend(vec![0; 10000000]);
        self.header.string_count = string_storage.len() as u32;
        self.string_storage = string_storage;

        self.string_storage_bytes = string_storage_bytes;
    }

    // Set the overflow strings, OverflowStringTable, ...
    pub fn set_overflow_string(&mut self, overflow_strings: Vec<String>) {
        let mut overflow_string_storage: Vec<OverflowStringTableEntry> = vec![];
        for string in overflow_strings {
            let offset = overflow_string_storage.len() as u32;
            let length = string.len() as u32;
            overflow_string_storage.push(OverflowStringTableEntry { offset, length });
        }
        self.overflow_string_storage = overflow_string_storage;
    }

    pub fn set_big_ints(&mut self, big_ints: Vec<u64>) {
        let mut big_int_table: Vec<BigIntTableEntry> = vec![];
        let mut big_int_storage: Vec<u8> = vec![];
        for big_int in big_ints {
            let offset = big_int_storage.len() as u32;
            let length = big_int.to_be_bytes().len() as u32;
            big_int_table.push(BigIntTableEntry { offset, length });
            big_int_storage.extend(big_int.to_be_bytes());
        }
        self.big_int_table = big_int_table;
        self.big_int_storage = big_int_storage;
    }

    pub fn set_debug_strings(&mut self, debug_strings: Vec<String>) {
        let mut debug_string_table: Vec<DebugStringTable> = vec![];
        let mut debug_string_storage: Vec<u8> = vec![];
        for string in debug_strings {
            let offset = debug_string_storage.len() as u32;
            let length = string.len() as u32;
            debug_string_table.push(DebugStringTable { offset, length });
            debug_string_storage.extend(string.as_bytes());
        }
        self.debug_info.string_table = debug_string_table;
        self.debug_info.string_storage = debug_string_storage;
    }

    pub fn is_overflowed_value(&self, value: u32) -> bool {
        value > (1 << 17) - 1
    }

    pub fn add_function<I>(&mut self, func_header: &mut FunctionHeader, bytecode: &mut Vec<I>)
    where
        I: InstructionParser + IntoParentInstruction + Clone,
    {
        let func: &mut FunctionHeader = match func_header {
            FunctionHeader::Small(f) => {
                if f.is_overflowed_check() {
                    f.flags.overflowed = true;
                    let large = LargeFunctionHeader::from(f.clone());
                    &mut FunctionHeader::Large(large)
                } else {
                    func_header
                }
            }
            FunctionHeader::Large(f) => {
                if f.is_overflowed_check() {
                    f.flags.overflowed = true;
                    let small = SmallFunctionHeader::from(f.clone());
                    &mut FunctionHeader::Small(small)
                } else {
                    func_header
                }
            }
        };

        let bytecode_clone = bytecode.clone();

        let mut bc_size = 0;
        for insn in bytecode {
            bc_size += insn.size();
        }

        func.set_byte_size(bc_size as u32);
        self.function_headers.push(func.clone());

        let parent_bytecode: Vec<HermesInstruction> = bytecode_clone
            .into_iter()
            .map(|insn| insn.into_parent())
            .collect();

        self.function_bytecode.push(FunctionInstructions {
            func_index: self.function_headers.len() as u32 - 1,
            bytecode: parent_bytecode,
        });
    }

    pub fn update_header(&mut self) {
        self.header.function_count = self.function_headers.len() as u32;

        self.header.big_int_count = self.big_int_table.len() as u32;
        self.header.big_int_storage_size = self.big_int_storage.len() as u32;
        self.header.reg_exp_count = self.reg_exp_table.len() as u32;
        self.header.reg_exp_storage_size = self.reg_exp_storage.len() as u32;
        self.header.array_buffer_size = self.array_buffer_storage.len() as u32;
        self.header.obj_key_buffer_size = self.object_key_buffer.len() as u32;
        self.header.obj_value_buffer_size = self.object_val_buffer.len() as u32;
        self.header.cjs_module_offset = self.cjs_module_offset;
        self.header.cjs_module_count = self.cjs_modules.len() as u32;
        self.header.function_source_count = self.function_source_entries.len() as u32;
        self.header.debug_info_offset = self.offsets.debug_info_offset;
    }
}
