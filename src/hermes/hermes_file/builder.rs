use std::io;

use super::HermesFile;
use crate::hermes::OverflowStringTableEntry;
use crate::hermes::SmallStringTableEntry;

// See: https://github.com/facebook/hermes/blob/main/lib/BCGen/HBC/BytecodeGenerator.cpp#L376

impl<R> HermesFile<R>
where
    R: io::Read + io::BufRead + io::Seek,
{
    // Set the strings present in the HermesFile, build out String Table, etc...
    pub fn set_hermes_strings(&mut self, strings: Vec<String>) {
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
            string_storage_bytes.push(0);
        }
        self.string_storage = string_storage;
        self.string_storage_bytes = string_storage_bytes;
    }

    // Set the overflow strings, OverflowStringTable, ...
    pub fn set_hermes_overflow_string(&mut self, overflow_strings: Vec<String>) {
        let mut overflow_string_storage: Vec<OverflowStringTableEntry> = vec![];
        for string in overflow_strings {
            let offset = overflow_string_storage.len() as u32;
            let length = string.len() as u32;
            overflow_string_storage.push(OverflowStringTableEntry { offset, length });
        }
        self.overflow_string_storage = overflow_string_storage;
    }
}
