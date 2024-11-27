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

        // Mostly using this as a debugging value. Compiler will remove it.
        let _function_header_offset = c;

        // Now that we have all the offset locations, we can get to writing the actual data at those locations.
        // This whole process will be rewritten once all the bugs are hammered out.
        w.seek(io::SeekFrom::Start(string_kind_offset)).unwrap();
        self.write_string_kinds(w);

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

        if self.header.function_source_count > 0 {
            w.seek(io::SeekFrom::Start(function_source_table_offset))
                .unwrap();
            self.write_function_source_table(w);
        }

        w.seek(io::SeekFrom::Start(bytecode_offset)).unwrap();

        // Debug...
        // println!("string_kind_offset: {:?}", string_kind_offset);
        // println!("identifier_hash_offset: {:?}", identifier_hash_offset);
        // println!("string_table_offset: {:?}", string_table_offset);
        // println!("overflow_string_table_offset: {:?}", overflow_string_table_offset);
        // println!("string_storage_offset: {:?}", string_storage_offset);
        // println!("array_buffer_offset: {:?}", array_buffer_offset);
        // println!("object_key_buffer_offset: {:?}", object_key_buffer_offset);
        // println!("object_val_buffer_offset: {:?}", object_val_buffer_offset);
        // println!("big_int_table_offset: {:?}", big_int_table_offset);
        // println!("reg_exp_table_offset: {:?}", reg_exp_table_offset);
        // println!("cjs_module_table_offset: {:?}", cjs_module_table_offset);
        // println!("function_source_table_offset: {:?}", function_source_table_offset);
        // println!("bytecode_offset: {:?}", bytecode_offset);

        align_writer(w, 4);

        // Function index : offset of the bytecode insns
        let mut function_bytecode_offsets = vec![];

        // Write the bytecode for each function here, and keep a record of the offset
        // so we can write it to the function header later
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

        // This is the offset where LargeFunctionHeaders will be written.
        let large_function_header_offset = w.stream_position().unwrap() as u64;

        // Manually override any LargeFunctionHeaders that exist to be SmallFunctionHeaders
        for fh in &mut self.function_headers {
            let _current_offset = w.stream_position().unwrap();

            // fh.set_offset(bytecode_offset as u32); // bytecode
            // fh.set_info_offset(current_offset as u32); // exception handlers, debug info
            match fh {
                FunctionHeader::Large(lfh) => {
                    // If we catch a Large, we need to convert it to a small.
                    // The data, at this point in the code, is still valid as it hasn't been modified yet.
                    let mut sfh: SmallFunctionHeader = SmallFunctionHeader::from(lfh.clone());
                    sfh.flags.overflowed = true;

                    // We're reassigning the Large to a Small here.
                    // Below this section in the code, we're going to check for overflows
                    // and actually write the LargeFunctionHeader based on the data within this.
                    fh.set_overflowed(true);
                    *fh = FunctionHeader::Small(sfh);
                }
                _ => { /* Do nothing */ }
            }
        }

        let mut large_write_offset = large_function_header_offset;
        for (_fidx, func_pair) in &mut self.function_bytecode.iter().enumerate() {
            let current_pos = w.stream_position().unwrap();
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

            match fh {
                FunctionHeader::Small(sfh) => {
                    let is_overflowed = sfh.flags.overflowed || sfh.is_overflowed_check();
                    if is_overflowed {
                        let mut lfh = LargeFunctionHeader::from(sfh.clone());

                        // Seek to large_write_offset and serialize the LargeFunctionHeader + exception handlers + debug info
                        w.seek(io::SeekFrom::Start(large_write_offset)).unwrap();

                        // Update the offsets of this LargeFunctionHeader to reflect the correct values.
                        // Possibly superfluous, but it's here for now.
                        lfh.offset = bytecode_offset as u32;
                        lfh.info_offset = large_write_offset as u32;

                        // Keep track of where this is being written to so we can calculate the overflow offsets
                        // for the SmallFunctionHeader
                        let large_offset = w.stream_position().unwrap();

                        // Serialize the LargeFunctionHeader struct
                        lfh.serialize(w);

                        // Serialize the exception handlers and debug info
                        if lfh.flags.has_exception_handler {
                            align_writer(w, 4);
                            encode_u32(w, lfh.exception_handlers.len() as u32);
                            for eh in lfh.exception_handlers.iter() {
                                eh.serialize(w);
                            }
                        }

                        if lfh.flags.has_debug_info && lfh.debug_info.is_some() {
                            lfh.debug_info.as_mut().unwrap().serialize(w);
                        }

                        // Update the offset for large writes so we don't let them step on one another
                        large_write_offset = w.stream_position().unwrap() as u64;

                        // Seek back to current_offset to write the small header
                        w.seek(io::SeekFrom::Start(current_pos)).unwrap();

                        // Update SFH offset + info_offset
                        let (new_large_offset, new_large_info_offset) =
                            get_large_info_offset_pair(large_offset as u32);
                        sfh.offset = new_large_offset as u32;
                        sfh.info_offset = new_large_info_offset as u32;
                        sfh.flags.overflowed = true;
                        sfh.serialize(w);
                    } else {
                        w.seek(io::SeekFrom::Start(sfh_offset)).unwrap(); // string kind off fbbe0
                                                                          // Write the SmallFunctionHeader as per usual
                        sfh.offset = bytecode_offset as u32;
                        sfh.info_offset = large_write_offset as u32;
                        sfh.serialize(w);

                        let current_offset = w.stream_position().unwrap();
                        w.seek(io::SeekFrom::Start(sfh.info_offset as u64)).unwrap();
                        if !sfh.flags.overflowed {
                            // Write the exception handlers and debug info for true SmallFunctionHeaders.
                            // At this point we've already written the LargeFunctionHeader exceptions and debug info
                            if sfh.flags.has_exception_handler {
                                // println!("writing exception/debug info for {:?} at {:?}", _fidx, w.stream_position().unwrap());
                                align_writer(w, 4);
                                encode_u32(w, sfh.exception_handlers.len() as u32);
                                for eh in sfh.exception_handlers.iter() {
                                    eh.serialize(w);
                                    large_write_offset = w.stream_position().unwrap() as u64;
                                }
                            }

                            if sfh.flags.has_debug_info && sfh.debug_info.is_some() {
                                sfh.debug_info.as_mut().unwrap().serialize(w);
                                large_write_offset = w.stream_position().unwrap() as u64;
                            }
                        }
                        // Go back to the position of the SmallFunctionHeader after the write
                        w.seek(io::SeekFrom::Start(current_offset)).unwrap();
                    }
                }
                _ => {
                    panic!("Function header is not a SmallFunctionHeader. Use the correct type.");
                }
            }
        }

        // Seek to the large_write_offset, as thats the last place we wrote a LargeFunctionHeader or SmallFunctionHeader Exception info/debug info
        w.seek(io::SeekFrom::Start(large_write_offset)).unwrap();
        println!("1111111 large_write_offset: {:?}", large_write_offset);

        // Write large function headers in a nicer way
        // for (offset, mut lfh) in large_headers {
        //     w.seek(io::SeekFrom::Start(offset)).unwrap();
        //     let current_offset = w.stream_position().unwrap();
        //     println!("Writing large function header at {:?}", offset);
        //     lfh.info_offset = current_offset as u32;
        //     println!("LFH: {:?}", lfh);
        //     lfh.serialize(w);

        //     if lfh.flags.has_exception_handler {
        //         align_writer(w, 4);

        //         // Well this was a stupid fucking bug.
        //         encode_u32(w, lfh.exception_handlers.len() as u32);

        //         for eh in lfh.exception_handlers {
        //             eh.serialize(w);
        //         }
        //     }

        //     if lfh.flags.has_debug_info && lfh.debug_info.is_some() {
        //         // align_writer(w, 4);
        //         lfh.debug_info.unwrap().serialize(w);
        //     }
        // }

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
    /*
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
        }
    }

    */

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
