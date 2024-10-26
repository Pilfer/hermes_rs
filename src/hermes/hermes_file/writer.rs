use std::io;

use super::{HermesFile, HermesStructReader};
use crate::hermes::cjs_module::CJSModule;
use crate::hermes::encode::align_writer;
use crate::hermes::encode::encode_u32;
use crate::hermes::Serializable;

impl<R> HermesFile<R>
where
    R: io::Read + io::BufRead + io::Seek,
{
    pub fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write + io::Seek,
    {
        // Relevant code from Hermes:
        // https://github.com/facebook/hermes/blob/main/include/hermes/BCGen/HBC/BytecodeStream.h#L37
        self.write_header(w);
        self.write_function_headers(w);
        self.write_string_kinds(w);
        self.write_identifier_hashes(w);
        self.write_small_string_table(w);
        self.write_overflow_string_table(w);
        self.write_string_storage(w);
        self.write_array_buffer(w);
        self.write_object_key_buffer(w);
        self.write_object_value_buffer(w);

        if self.header.version >= 87 {
            self.write_big_int_table(w);
            self.write_big_int_storage(w);
        }

        self.write_reg_exp_table(w);
        self.write_cjs_module_table(w);

        if self.header.version >= 84 {
            self.write_function_source_table(w);
        }

        self.write_debug_info(w);
        self.write_footer(w);
    }

    // pub fn write_<W>(&self, w: &mut W) where W: io::Write + io::Seek, {}
    pub fn write_header<W>(&self, w: &mut W)
    where
        W: io::Write + io::Seek,
    {
        self.header.serialize(w);
        align_writer(w, 4);
    }

    pub fn write_function_headers<W>(&self, w: &mut W)
    where
        W: io::Write + io::Seek,
    {
        // Relevant code from Hermes:
        // https://github.com/facebook/hermes/blob/main/lib/BCGen/HBC/BytecodeStream.cpp#L233

        // Populator code
        // https://github.com/facebook/hermes/blob/main/lib/BCGen/HBC/BytecodeDataProvider.cpp#L211

        // TODO: Make this functional. This is a temporary solution.
        for func_header in &self.function_headers {
            // This is where we're going to go back to if it's a LargeFunctionHeader
            let _current_pos = w.stream_position().unwrap();

            if !func_header.flags().overflowed {
                func_header.serialize(w);
            } else {
                align_writer(w, 4);
                // move to the info offset
                let new_offset = func_header.info_offset() << 16 | func_header.offset();
                w.seek(io::SeekFrom::Start(new_offset as u64)).unwrap();

                // Serialize the LargeFunctionHeader here
                func_header.serialize(w);
            }

            // TODO: Tag bytecode to a function
        }
    }

    pub fn write_string_kinds<W>(&self, w: &mut W)
    where
        W: io::Write + io::Seek,
    {
        align_writer(w, 4);
        for kind in &self.string_kinds {
            kind.serialize(w);
        }
    }

    pub fn write_identifier_hashes<W>(&self, w: &mut W)
    where
        W: io::Write + io::Seek,
    {
        align_writer(w, 4);
        for hash in &self.identifier_hashes {
            encode_u32(w, *hash);
        }
    }

    pub fn write_small_string_table<W>(&self, w: &mut W)
    where
        W: io::Write + io::Seek,
    {
        align_writer(w, 4);
        for string in &self.string_storage {
            string.serialize(w);
        }
    }

    pub fn write_overflow_string_table<W>(&self, w: &mut W)
    where
        W: io::Write + io::Seek,
    {
        align_writer(w, 4);
        for overflow in &self.overflow_string_storage {
            overflow.serialize(w);
        }
    }

    pub fn write_string_storage<W>(&self, w: &mut W)
    where
        W: io::Write + io::Seek,
    {
        align_writer(w, 4);
        w.write_all(&self.string_storage_bytes).unwrap();
    }

    pub fn write_array_buffer<W>(&self, w: &mut W)
    where
        W: io::Write + io::Seek,
    {
        align_writer(w, 4);
        w.write_all(&self.array_buffer_storage).unwrap();
    }

    pub fn write_object_key_buffer<W>(&self, w: &mut W)
    where
        W: io::Write + io::Seek,
    {
        align_writer(w, 4);
        w.write_all(&self.object_key_buffer).unwrap();
    }

    pub fn write_object_value_buffer<W>(&self, w: &mut W)
    where
        W: io::Write + io::Seek,
    {
        align_writer(w, 4);
        w.write_all(&self.object_val_buffer).unwrap();
    }

    pub fn write_big_int_table<W>(&self, w: &mut W)
    where
        W: io::Write + io::Seek,
    {
        align_writer(w, 4);
        for big_int in &self.big_int_table {
            big_int.serialize(w);
        }
    }

    pub fn write_big_int_storage<W>(&self, w: &mut W)
    where
        W: io::Write + io::Seek,
    {
        align_writer(w, 4);
        w.write_all(&self.big_int_storage).unwrap();
    }

    pub fn write_reg_exp_table<W>(&self, w: &mut W)
    where
        W: io::Write + io::Seek,
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
        W: io::Write + io::Seek,
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
        W: io::Write + io::Seek,
    {
        align_writer(w, 4);
        if !self.function_source_entries.is_empty() {
            for source in &self.function_source_entries {
                source.serialize(w);
            }
        }
    }

    pub fn write_debug_info<W>(&self, w: &mut W)
    where
        W: io::Write + io::Seek,
    {
        // TODO: set the header debug info offset to w.stram_position().unwrap() here
        self.debug_info.serialize(w);
    }

    pub fn write_footer<W>(&self, w: &mut W)
    where
        W: io::Write + io::Seek,
    {
        w.write_all(&self.footer).unwrap();
    }
}
