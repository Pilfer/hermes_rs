use std::io;
use std::vec;

use super::FunctionInstructions;
use super::HermesFile;
use crate::hermes::big_int_table::BigIntTableEntry;
use crate::hermes::debug_info::DebugStringTable;
use crate::hermes::function_header::{FunctionHeader, LargeFunctionHeader, SmallFunctionHeader};
use crate::hermes::jenkins::hash_string;
use crate::hermes::string_kind::{
    StringKind, StringKindEntry, StringKindEntryNew, StringKindEntryOld,
};
use crate::hermes::IntoParentInstruction;
use crate::hermes::OverflowStringTableEntry;
use crate::hermes::SmallStringTableEntry;
use crate::hermes::{HermesInstruction, InstructionParser};

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct StringTypePair {
    pub string: String,
    pub kind: StringKind,
}

impl<R> HermesFile<R>
where
    R: io::Read + io::BufRead + io::Seek,
{
    pub fn push_string_kind(&mut self, kind: StringKind, count: u32) {
        if count > 0 {
            let sk = if self.header.version <= 71 {
                StringKindEntry::Old(StringKindEntryOld { kind, count })
            } else {
                StringKindEntry::New(StringKindEntryNew { kind, count })
            };
            self.string_kinds.push(sk);
        }
    }

    pub fn set_string_pairs_unordered(&mut self, mut pairs: Vec<StringTypePair>) {
        let mut string_storage: Vec<SmallStringTableEntry> = vec![];
        let mut overflow_string_storage: Vec<OverflowStringTableEntry> = vec![];
        let mut string_storage_bytes: Vec<u8> = vec![];
        let mut identifier_hashes: Vec<u32> = vec![];

        let mut string_count = 0;
        let mut identifier_count = 0;
        let mut predefined_count = 0;

        let mut last_kind = StringKind::String;

        for (pidx, pair) in pairs.iter_mut().enumerate() {
            // Set the last_kind to the first kind in the list to build out the sections
            if pidx == 0 {
                last_kind = pair.kind;
            }

            let string = &pair.string;
            let is_utf_16 = string.chars().any(|c| c as u32 > 0x10000);
            let offset = string_storage_bytes.len() as u32;
            let mut length = string.len() as u32;

            if !is_utf_16 {
                string_storage_bytes.extend(string.as_bytes());
            } else {
                let utf16_bytes = string.encode_utf16().collect::<Vec<u16>>();
                let mut byte_length = 0;
                for code_unit in &utf16_bytes {
                    let b = &code_unit.to_le_bytes();
                    string_storage_bytes.extend_from_slice(b);
                    byte_length += 1;
                }
                length = byte_length as u32;
            }

            // If the string is >= 255 characters long, it needs to be stored in
            // the overflow string storage area. The offset gets updated to the index
            // of this string in the OverflowStringTable, and the length is set to 255
            // so that the runtime knows to look in the overflow string storage area.
            if length >= 255 {
                // set the current string length to 255
                let small_length = 255;
                let small_offset = overflow_string_storage.len() as u32;

                string_storage.push(SmallStringTableEntry {
                    is_utf_16,
                    offset: small_offset,
                    length: small_length,
                });

                overflow_string_storage.push(OverflowStringTableEntry { offset, length });
            } else {
                string_storage.push(SmallStringTableEntry {
                    is_utf_16,
                    offset,
                    length,
                });
            };

            /*
             * This logic looks kind of wonky, but I promise it needs to be here.
             * Some strings get loaded up in chunks based on their kind.
             * This is more common in larger React Native bundles, which is why it's here.
             * Example:
             *  - We have 10 strings of kind String, then 10 strings of kind Identifier.
             *  - The first 5 Identifiers get defined first, then 10 Strings, followed by
             *   the last 5 Identifiers.
             *  I had assumed it was always String -> Identifier -> Predefined, but that's not the case.
             */
            match pair.kind {
                StringKind::String => {
                    match last_kind {
                        StringKind::String => {
                            string_count += 1;
                            last_kind = StringKind::String;
                        }
                        _ => {
                            if string_count > 0 {
                                self.push_string_kind(last_kind, string_count);
                                string_count = 0;
                                continue;
                            }
                            string_count = 1;
                        }
                    };
                }
                StringKind::Identifier => {
                    let ihash = hash_string(string.as_str());
                    if !identifier_hashes.contains(&ihash) {
                        identifier_hashes.push(ihash);
                    }
                    match last_kind {
                        StringKind::Identifier => {
                            identifier_count += 1;
                            last_kind = StringKind::Identifier;
                        }
                        _ => {
                            if identifier_count > 0 {
                                self.push_string_kind(last_kind, identifier_count);
                                identifier_count = 0;
                                continue;
                            }
                            identifier_count = 1;
                        }
                    };
                }
                StringKind::Predefined => {
                    match last_kind {
                        StringKind::Predefined => {
                            predefined_count += 1;
                            last_kind = StringKind::Predefined;
                        }
                        _ => {
                            if predefined_count > 0 {
                                self.push_string_kind(last_kind, predefined_count);
                                predefined_count = 0;
                                continue;
                            }
                            predefined_count = 1;
                        }
                    };
                }
            }
        }

        self.string_storage = string_storage;
        self.overflow_string_storage = overflow_string_storage;

        self.header.string_count = self.string_storage.len() as u32;
        self.header.overflow_string_count = self.overflow_string_storage.len() as u32;

        // self.identifier_hashes = identifier_hashes;

        self.string_storage_bytes = string_storage_bytes;
    }

    // TODO: need to append identifiers
    pub fn set_strings_ordered(
        &mut self,
        strings: Vec<StringTypePair>,
        identifiers: Vec<StringTypePair>,
        predefined: Vec<StringTypePair>,
    ) {
        let mut string_storage: Vec<SmallStringTableEntry> = vec![];
        let mut string_storage_bytes: Vec<u8> = vec![];

        let mut string_count = 0;
        let mut identifier_count = 0;
        let mut predefined_count = 0;

        for pair in strings {
            let string = pair.string;
            let is_utf_16 = string.chars().any(|c| c as u32 > 0x10000);
            let offset = string_storage_bytes.len() as u32;
            let length = string.len() as u32;

            string_storage.push(SmallStringTableEntry {
                is_utf_16,
                offset,
                length,
            });

            string_storage_bytes.extend(string.as_bytes());

            string_count += 1;
        }

        for pair in identifiers {
            let string = pair.string;
            let is_utf_16 = string.chars().any(|c| c as u32 > 0x10000);
            let offset = string_storage_bytes.len() as u32;
            let length = string.len() as u32;

            string_storage.push(SmallStringTableEntry {
                is_utf_16,
                offset,
                length,
            });

            string_storage_bytes.extend(string.as_bytes());

            identifier_count += 1;
        }

        for pair in predefined {
            let string = pair.string;
            let is_utf_16 = string.chars().any(|c| c as u32 > 0x10000);
            let offset = string_storage_bytes.len() as u32;
            let length = string.len() as u32;

            string_storage.push(SmallStringTableEntry {
                is_utf_16,
                offset,
                length,
            });

            string_storage_bytes.extend(string.as_bytes());

            predefined_count += 1;
        }

        self.string_kinds = vec![];

        self.push_string_kind(StringKind::String, string_count);
        self.push_string_kind(StringKind::Identifier, identifier_count);
        self.push_string_kind(StringKind::Predefined, predefined_count);

        let ssl = string_storage.len() as u32;
        self.string_storage = string_storage;
        self.header.string_count = ssl;

        self.string_storage_bytes = string_storage_bytes;

        // self.set_strings(strings);
        // self.set_overflow_string(overflow_strings);
    }

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
        let ssl = string_storage.len() as u32;
        self.string_storage = string_storage;
        self.header.string_count = ssl;
        for sk in self.string_kinds.iter_mut() {
            match sk {
                StringKindEntry::Old(old) => match old.kind {
                    StringKind::String => {
                        old.count = self.header.string_count;
                    }
                    _ => {}
                },
                StringKindEntry::New(new) => match new.kind {
                    StringKind::String => {
                        new.count = self.header.string_count;
                    }
                    _ => {}
                },
            }
        }

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
            is_large: match func {
                FunctionHeader::Small(_) => false,
                FunctionHeader::Large(_) => true,
            },
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
