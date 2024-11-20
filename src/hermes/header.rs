use std::io;

use crate::hermes::bytecode_options::BytecodeOptions;

use crate::hermes::decode::{align_reader, decode_u32, decode_u64};
use crate::hermes::encode::{align_writer, encode_u32, encode_u64};
use crate::hermes::Serializable;

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct HermesHeader {
    // file: &'a HermesFile<'a>,
    pub magic: u64,
    pub version: u32,
    pub sha1: [u8; 20], // sha1 is the sha1 hash of the input source  JS file. Example: "eval(`print(123);`)" = a97c8302dab90bec7184a9183b22b43badd57a65
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
    pub cjs_module_offset: u32,
    pub function_source_count: u32,
    pub debug_info_offset: u32,

    pub options: BytecodeOptions,
}

impl Default for HermesHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl HermesHeader {
    pub fn new() -> Self {
        Self {
            magic: 0,
            version: 0,
            sha1: [0; 20],
            file_length: 0,
            global_code_index: 0,
            function_count: 0,
            string_kind_count: 0,
            identifier_count: 0,
            string_count: 0,
            overflow_string_count: 0,
            string_storage_size: 0,
            big_int_count: 0,
            big_int_storage_size: 0,
            reg_exp_count: 0,
            reg_exp_storage_size: 0,
            array_buffer_size: 0,
            obj_key_buffer_size: 0,
            obj_value_buffer_size: 0,
            segment_id: 0,
            cjs_module_count: 0,
            cjs_module_offset: 0,
            function_source_count: 0,
            debug_info_offset: 0,
            options: BytecodeOptions::new(),
        }
    }
}

pub trait HermesStructReader {
    fn deserialize<R>(r: &mut R, _version: u32) -> Self
    where
        R: io::Read + io::BufRead + io::Seek;

    fn serialize<W>(&self, w: &mut W)
    where
        W: std::io::Write + std::io::Seek;

    fn size(&self) -> usize;
}

impl HermesStructReader for HermesHeader {
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

        // Big int count and storage size are only present in version >= 87
        let big_int_count = if version >= 87 { decode_u32(r) } else { 0 };
        let big_int_storage_size = if version >= 87 { decode_u32(r) } else { 0 };

        let reg_exp_count = decode_u32(r);
        let reg_exp_storage_size = decode_u32(r);
        let array_buffer_size = decode_u32(r);
        let obj_key_buffer_size = decode_u32(r);
        let obj_value_buffer_size = decode_u32(r);

        let mut cjs_module_offset = 0;
        let mut segment_id = 0;

        // cjs_module_offset is only present in version < 78, otherwise it's segment_id
        if version < 78 {
            cjs_module_offset = decode_u32(r);
        } else {
            segment_id = decode_u32(r);
        }

        // cjs_module_count is only present in version >= 84
        let mut cjs_module_count = 0;
        if version >= 84 {
            cjs_module_count = decode_u32(r);
        }

        let function_source_count = decode_u32(r);
        let debug_info_offset = decode_u32(r);

        let options = BytecodeOptions::deserialize(r, version);

        // Align to 32 bytes
        align_reader(r, 32).unwrap();

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
            cjs_module_offset,
            segment_id,
            cjs_module_count,
            function_source_count,
            debug_info_offset,
            options,
        }
    }

    fn serialize<W>(&self, w: &mut W)
    where
        W: std::io::Write + std::io::Seek,
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

        if self.version < 78 {
            encode_u32(w, self.cjs_module_offset);
        } else {
            encode_u32(w, self.segment_id);
        }

        encode_u32(w, self.cjs_module_count);
        if self.version >= 84 {
            encode_u32(w, self.cjs_module_count);
        }

        encode_u32(w, self.debug_info_offset);

        self.options.serialize(w);

        // Align to 32 bytes
        align_writer(w, 32);
    }

    // Read string
    // Read function
}
