use sha1::{Digest, Sha1};
use std::io::{self, BufRead, Read, Seek, Write};
use std::vec;

use super::{HermesFile, HermesStructReader};
use crate::hermes::cjs_module::CJSModule;
use crate::hermes::encode::align_writer;
use crate::hermes::encode::encode_u32;
use crate::hermes::function_header::get_large_info_offset_pair;
use crate::hermes::function_header::{FunctionHeader, LargeFunctionHeader, SmallFunctionHeader};
use crate::hermes::Serializable;

// pad the pseudo offset to 4
fn align_offset(v: u64) -> u64 {
    if v % 4 == 0 {
        v
    } else {
        v + (4 - v % 4)
    }
}

impl<R> HermesFile<R>
where
    R: Read + BufRead + Seek,
{
    pub fn serialize<W>(&mut self, w: &mut W)
    where
        W: Write + io::Seek + Read + BufRead,
    {
        // This function could definitely use some refactoring.
        // Currently we're manually calculating the offsets of each section of the file and using that as
        // a jumping point. This obviously isn't ideal, but we're still hammering out bugs.
        // It's also worth noting that I'm still learning rust, so excuse any bad practices.
        // Please open an issue or PR with suggestions!

        let _base_offset = w.stream_position().unwrap() + self.header.size() as u64; // should always be 128

        let mut c = _base_offset;

        c = align_offset(c);
        let function_headers_offset = c;
        c += 16 * self.function_headers.len() as u64; // 16 bytes per (small) function header

        c = align_offset(c);
        let string_kind_offset = c;
        c += 4 * self.header.string_kind_count as u64;

        c = align_offset(c);
        let identifier_hash_offset = c;
        c += 4 * self.header.identifier_count as u64;

        c = align_offset(c);
        let string_table_offset = c;
        c += 4 * self.header.string_count as u64;

        c = align_offset(c);
        let overflow_string_table_offset = c;
        c += 8 * self.overflow_string_storage.len() as u64;

        c = align_offset(c);
        let string_storage_offset = c;
        c += self.string_storage_bytes.len() as u64;

        c = align_offset(c);
        let array_buffer_offset = c;
        c += self.array_buffer_storage.len() as u64;

        c = align_offset(c);
        let object_key_buffer_offset = c;
        c += self.object_key_buffer.len() as u64;

        c = align_offset(c);
        let object_val_buffer_offset = c;
        c += self.object_val_buffer.len() as u64;

        let mut big_int_table_offset = 0;
        if self.header.version >= 87 {
            c = align_offset(c);
            big_int_table_offset = c;
            c += 8 * self.big_int_table.len() as u64;
        }

        c = align_offset(c);
        let reg_exp_table_offset = c;
        c += 8 * self.header.reg_exp_count as u64;
        let _reg_exp_storage_offset = c;
        c += self.reg_exp_storage.len() as u64;

        c = align_offset(c);
        let cjs_size = match self.cjs_modules.first() {
            Some(CJSModule::CJSModuleEntry(_)) => 8,
            Some(CJSModule::CJSModuleInt(_)) => 4,
            None => 0,
        };
        let cjs_module_table_offset = c;
        c += cjs_size * self.header.cjs_module_count as u64;

        c = align_offset(c);
        let function_source_table_offset = c;

        if self.header.version >= 84 {
            c += 8 * self.header.function_source_count as u64;
        }

        c = align_offset(c);
        let bytecode_offset = c;
        let bytecode_length: u64 = self
            .function_bytecode
            .iter()
            .flat_map(|pair| pair.bytecode.iter())
            .map(|instruction| instruction.size() as u64)
            .sum();

        c += bytecode_length;

        c = align_offset(c);
        // this is where LFH and/or debug info will go
        let debug_info_offset = c;

        // iterate over all function headers, check if overflowed, and calculate the size of the large function headers
        let mut overflowed_function_headers_len = 0;
        for fh in &self.function_headers {
            if fh.flags().overflowed {
                // should always increment by 32
                overflowed_function_headers_len += fh.size() as u64;
            }
            // check if they have debug info and exception handlers
            for eh in fh.exception_handlers() {
                overflowed_function_headers_len = align_offset(overflowed_function_headers_len);
                overflowed_function_headers_len += eh.size() as u64;
            }

            if fh.debug_info().is_some() {
                overflowed_function_headers_len = align_offset(overflowed_function_headers_len);
                overflowed_function_headers_len += fh.debug_info().unwrap().size() as u64;
            }
        }

        // Now that we have all the offset locations, we can get to writing the actual data at those locations.
        // This whole process will be rewritten once all the bugs are hammered out.
        w.seek(io::SeekFrom::Start(string_kind_offset)).unwrap();
        self.write_string_kinds(w);
        // TODO: add assertions here to ensure we're at the correct positon

        w.seek(io::SeekFrom::Start(identifier_hash_offset)).unwrap();
        self.write_identifier_hashes(w);

        w.seek(io::SeekFrom::Start(string_table_offset)).unwrap();
        self.write_small_string_table(w);

        w.seek(io::SeekFrom::Start(overflow_string_table_offset))
            .unwrap();
        self.write_overflow_string_table(w);

        w.seek(io::SeekFrom::Start(string_storage_offset)).unwrap();
        self.write_string_storage(w);

        w.seek(io::SeekFrom::Start(array_buffer_offset)).unwrap();
        self.write_array_buffer(w);

        w.seek(io::SeekFrom::Start(object_key_buffer_offset))
            .unwrap();
        self.write_object_key_buffer(w);

        w.seek(io::SeekFrom::Start(object_val_buffer_offset))
            .unwrap();
        self.write_object_value_buffer(w);

        if self.header.version >= 87 {
            w.seek(io::SeekFrom::Start(big_int_table_offset)).unwrap();
            self.write_big_int_table(w);
            align_writer(w, 4);
            self.write_big_int_storage(w);
        }

        w.seek(io::SeekFrom::Start(reg_exp_table_offset)).unwrap();
        self.write_reg_exp_table(w); // This writes the reg_exp_storage value as well

        if self.header.version >= 84 {
            w.seek(io::SeekFrom::Start(cjs_module_table_offset))
                .unwrap();
            self.write_cjs_module_table(w);
        }

        w.seek(io::SeekFrom::Start(function_source_table_offset))
            .unwrap();
        self.write_function_source_table(w);

        w.seek(io::SeekFrom::Start(bytecode_offset)).unwrap();

        align_writer(w, 4);

        // Function index : offset of the bytecode insns
        let mut function_bytecode_offsets = vec![];

        for func_pair in &mut self.function_bytecode {
            let current_offset = w.stream_position().unwrap();
            function_bytecode_offsets.push((func_pair.func_index, current_offset));
            // Update the SmallFunctionHeader offset and info_offset with the new bytecode offset
            for instruction in &func_pair.bytecode {
                instruction.serialize(w);
            }
        }

        align_writer(w, 4);

        // Store the individual SmallFunctionHeader offsets for each function header
        // We need to know where they are so we can go back to them and update values
        let mut small_func_header_offsets = vec![];
        let mut sfhoc = function_headers_offset;
        for func_pair in &mut self.function_bytecode {
            small_func_header_offsets.push((func_pair.func_index, sfhoc));
            sfhoc += 16;
        }

        let mut function_debug_info_offsets: Vec<(u32, bool, u64)> = vec![];

        w.seek(io::SeekFrom::Start(debug_info_offset)).unwrap();
        for func_pair in &mut self.function_bytecode {
            let current_offset = w.stream_position().unwrap();

            // Check if the function header is overflowed
            let fh = self
                .function_headers
                .get_mut(func_pair.func_index as usize)
                .unwrap();

            // Write the SmallFunctionHeader
            let sfh_offset = small_func_header_offsets
                .iter()
                .find(|(index, _)| *index == func_pair.func_index)
                .unwrap()
                .1;

            // Get bytecode offset for this function
            let bytecode_offset = function_bytecode_offsets
                .iter()
                .find(|(index, _)| *index == func_pair.func_index)
                .unwrap()
                .1;

            // Do the overflow check _after_ we have the offsets.
            let is_overflowed = fh.flags().overflowed || fh.is_overflowed_check();
            if is_overflowed {
                fh.set_overflowed(true);
                let (calculated_offset, calculated_info_offset) =
                    get_large_info_offset_pair(current_offset as u32);
                fh.set_info_offset(calculated_info_offset as u32);
                fh.set_offset(calculated_offset as u32);
            } else {
                fh.set_info_offset(current_offset as u32);
                fh.set_offset(bytecode_offset as u32);
            }

            if !is_overflowed {
                w.seek(io::SeekFrom::Start(sfh_offset)).unwrap();
                if func_pair.func_index == 719 {}
                fh.serialize(w);
                w.seek(io::SeekFrom::Start(current_offset)).unwrap();
            } else {
                // FH is definitely overflowed, so handle appropriately
                match fh {
                    FunctionHeader::Small(sfh) => {
                        // Convert to LargeFunctionHeader
                        let lfh: LargeFunctionHeader = LargeFunctionHeader::from(sfh.clone());
                        w.seek(io::SeekFrom::Start(sfh_offset)).unwrap();
                        sfh.serialize(w);

                        *fh = FunctionHeader::Large(lfh);
                    }
                    FunctionHeader::Large(lfh) => {
                        let sfh: SmallFunctionHeader = SmallFunctionHeader::from(lfh.clone());
                        w.seek(io::SeekFrom::Start(sfh_offset)).unwrap();
                        sfh.serialize(w);
                    }
                }

                // Write what _should_ only ever be a LargeFunctionHeader
                w.seek(io::SeekFrom::Start(current_offset)).unwrap();
                match fh {
                    FunctionHeader::Large(lfh) => {
                        lfh.offset = bytecode_offset as u32;
                        lfh.info_offset = current_offset as u32;
                        lfh.flags.overflowed = true;
                        lfh.serialize(w);
                    }
                    _ => {
                        panic!(
                            "Function header is not a LargeFunctionHeader. Use the correct type."
                        );
                    }
                }
            }

            w.seek(io::SeekFrom::Start(current_offset)).unwrap();

            // Update the SmallFunctionHeader offset and info_offset with the new debug info offset
            // Write the exception handlers and debug info offset
            if fh.flags().has_exception_handler {
                align_writer(w, 4);

                // Well this was a stupid fucking bug.
                encode_u32(w, fh.exception_handlers().len() as u32);

                for eh in fh.exception_handlers() {
                    eh.serialize(w);
                }
            }

            if fh.flags().has_debug_info && fh.debug_info().is_some() {
                // align_writer(w, 4);
                fh.debug_info().unwrap().serialize(w);
            }

            function_debug_info_offsets.push((func_pair.func_index, is_overflowed, current_offset));
        }

        self.offsets.debug_info_offset = w.stream_position().unwrap() as u32;
        self.write_debug_info(w);

        let footer_offset = w.stream_position().unwrap();

        self.header.file_length = w.stream_position().unwrap() as u32 + 20; // plus sha1 footer size

        w.seek(io::SeekFrom::Start(0)).unwrap();
        self.update_header();
        self.write_header(w);

        w.seek(io::SeekFrom::Start(footer_offset)).unwrap();
        self.write_footer(w);
    }

    // pub fn write_<W>(&self, w: &mut W) where W: Write + io::Seek, {}
    pub fn write_header<W>(&self, w: &mut W)
    where
        W: Write + io::Seek,
    {
        self.header.serialize(w);
        align_writer(w, 4);
    }

    // Don't use this.
    pub fn write_function_headers<W>(&mut self, w: &mut W)
    where
        W: Write + io::Seek,
    {
        // Relevant code from Hermes:
        // https://github.com/facebook/hermes/blob/main/lib/BCGen/HBC/BytecodeStream.cpp#L233

        // Populator code
        // https://github.com/facebook/hermes/blob/main/lib/BCGen/HBC/BytecodeDataProvider.cpp#L211

        // TODO: Make this functional. This is a temporary solution.
        for mut func_header in self.function_headers.clone() {
            // This is where we're going to go back to if it's a LargeFunctionHeader
            let _current_pos = w.stream_position().unwrap();

            // https://github.com/facebook/hermes/blob/d964f6b125426f919ad30fb09ff09b9d5f041743/include/hermes/BCGen/HBC/BytecodeFileFormat.h#L341

            // Probably can delete this - lost in sauce atm.
            if func_header.is_overflowed_check() {
                func_header.set_overflowed(true);
            }

            if !func_header.flags().overflowed {
                func_header.serialize(w);
            } else {
                align_writer(w, 4);

                // We're definitely overflowed and dealing with a large function header here
                // --> _assuming_ the add_function function was used in builder.rs <--
                // There's always a chance that someone will try to feed this a SmallFunctionHeader that is overflowed.

                // So we need to write a SmallFunctionHeader with the overflowed flag set to true
                // The offset and info offset are updated in the main serialize function
                match func_header {
                    FunctionHeader::Large(lfh) => {
                        let nsfh: SmallFunctionHeader = lfh.into();
                        nsfh.serialize(w);
                    }
                    _ => {
                        panic!(
                            "Function header is not a LargeFunctionHeader. Use the correct type."
                        );
                    }
                }
            }

            // TODO: Tag bytecode to a function
        }
    }

    pub fn write_string_kinds<W>(&self, w: &mut W)
    where
        W: Write + io::Seek,
    {
        align_writer(w, 4);
        for kind in &self.string_kinds {
            kind.serialize(w);
        }
    }

    pub fn write_identifier_hashes<W>(&self, w: &mut W)
    where
        W: Write + io::Seek,
    {
        align_writer(w, 4);
        for hash in &self.identifier_hashes {
            encode_u32(w, *hash);
        }
    }

    pub fn write_small_string_table<W>(&mut self, w: &mut W)
    where
        W: Write + io::Seek,
    {
        align_writer(w, 4);

        for (sidx, string) in self.string_storage.iter().enumerate() {
            let offset = w.stream_position().unwrap();
            self.offsets
                .small_string_table_offsets
                .insert(sidx as u32, offset as u32);
            string.serialize(w);
        }
        self.header.string_count = self.string_storage.len() as u32;
    }

    pub fn write_overflow_string_table<W>(&mut self, w: &mut W)
    where
        W: Write + io::Seek,
    {
        align_writer(w, 4);
        for (oidx, overflow) in self.overflow_string_storage.iter().enumerate() {
            let offset = w.stream_position().unwrap();
            self.offsets
                .overflow_string_table_offsets
                .insert(oidx as u32, offset as u32);
            overflow.serialize(w);
        }

        self.header.overflow_string_count = self.overflow_string_storage.len() as u32;
    }

    pub fn write_string_storage<W>(&mut self, w: &mut W)
    where
        W: Write + io::Seek,
    {
        align_writer(w, 4);
        w.write_all(&self.string_storage_bytes).unwrap();

        self.header.string_storage_size = self.string_storage_bytes.len() as u32;
    }

    pub fn write_array_buffer<W>(&self, w: &mut W)
    where
        W: Write + io::Seek,
    {
        align_writer(w, 4);
        w.write_all(&self.array_buffer_storage).unwrap();
    }

    pub fn write_object_key_buffer<W>(&self, w: &mut W)
    where
        W: Write + io::Seek,
    {
        align_writer(w, 4);
        w.write_all(&self.object_key_buffer).unwrap();
    }

    pub fn write_object_value_buffer<W>(&self, w: &mut W)
    where
        W: Write + io::Seek,
    {
        align_writer(w, 4);
        w.write_all(&self.object_val_buffer).unwrap();
    }

    pub fn write_big_int_table<W>(&self, w: &mut W)
    where
        W: Write + io::Seek,
    {
        align_writer(w, 4);
        for big_int in &self.big_int_table {
            big_int.serialize(w);
        }
    }

    pub fn write_big_int_storage<W>(&self, w: &mut W)
    where
        W: Write + io::Seek,
    {
        align_writer(w, 4);
        w.write_all(&self.big_int_storage).unwrap();
    }

    pub fn write_reg_exp_table<W>(&self, w: &mut W)
    where
        W: Write + io::Seek,
    {
        align_writer(w, 4);
        if !self.reg_exp_table.is_empty() {
            for reg_exp in &self.reg_exp_table {
                reg_exp.serialize(w);
            }

            w.write_all(&self.reg_exp_storage).unwrap();
        }
    }

    pub fn write_cjs_module_table<W>(&self, w: &mut W)
    where
        W: Write + io::Seek,
    {
        align_writer(w, 4);
        for cjs in &self.cjs_modules {
            match cjs {
                CJSModule::CJSModuleEntry(cjs) => cjs.serialize(w),
                CJSModule::CJSModuleInt(cjs) => cjs.serialize(w),
            }
        }
    }

    pub fn write_function_source_table<W>(&self, w: &mut W)
    where
        W: Write + io::Seek,
    {
        align_writer(w, 4);
        if !self.function_source_entries.is_empty() {
            for source in &self.function_source_entries {
                source.serialize(w);
            }
        }
    }

    pub fn write_debug_info<W>(&mut self, w: &mut W)
    where
        W: Write + io::Seek,
    {
        // TODO: set the header debug info offset to w.stram_position().unwrap() here
        let offset = w.stream_position().unwrap();
        self.offsets.debug_info_offset = offset as u32;
        self.debug_info.serialize(w);
    }

    pub fn write_footer<W>(&mut self, w: &mut W)
    where
        W: Write + io::Seek + Read + BufRead,
    {
        // align_writer(w, 4);
        let pos = w.stream_position().unwrap();
        let mut executable_bytes = vec![];
        w.seek(io::SeekFrom::Start(0)).unwrap();
        w.take(pos).read_to_end(&mut executable_bytes).unwrap();

        // calculate the footer hash
        let mut hasher = Sha1::new();
        hasher.update(&executable_bytes);
        self.footer = hasher.finalize().into();

        // Debug
        let _footer_hash_hex = self
            .footer
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();

        w.write_all(&self.footer).unwrap();
    }
}
