use std::io;

use crate::hermes::bytecode_options::BytecodeOptions;

use crate::hermes::decode::{align_reader, decode_u32, decode_u64};
use crate::hermes::encode::{encode_u32, encode_u64};
use crate::hermes::Serializable;

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
    pub _padding: [u8; 19],
    // pub function_headers: Vec<FunctionHeader>,
    // pub string_kinds: Vec<StringKindEntry>,
    // pub identifier_hashes: Vec<u32>,
    // pub string_storage: Vec<SmallStringTableEntry>,
    // pub string_storage_bytes: Vec<u8>,
    // pub overflow_string_storage: Vec<OverflowStringTableEntry>,
    // pub array_buffer_storage: Vec<u8>,
    // pub object_key_buffer: Vec<u8>,
    // pub object_val_buffer: Vec<u8>,
    // pub big_int_table: Vec<BigIntTableEntry>,
    // pub reg_exp_table: Vec<RegExpTableEntry>,
    // pub cjs_modules: Vec<CJSModule>,
    // pub function_source_entries: Vec<FunctionSourceEntry>,
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
            _padding: [0; 19],
            // function_headers: vec![],
            // string_kinds: vec![],
            // identifier_hashes: vec![],
            // string_storage: vec![],
            // string_storage_bytes: vec![],
            // overflow_string_storage: vec![],
            // array_buffer_storage: vec![],
            // object_key_buffer: vec![],
            // object_val_buffer: vec![],
            // big_int_table: vec![],
            // reg_exp_table: vec![],
            // cjs_modules: vec![],
            // function_source_entries: vec![],
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

    // fn parse_bytecode<R>(&self, r: &mut R)
    // where
    //     R: io::Read + io::BufRead + io::Seek;

    // fn parse_bytecode_for_fn<R: io::Read + io::BufRead + io::Seek>(&self, idx: u32, r: &mut R)
    // where
    //     R: io::Read + io::BufRead + io::Seek;
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
        let big_int_count = decode_u32(r);
        let big_int_storage_size = decode_u32(r);
        let reg_exp_count = decode_u32(r);
        let reg_exp_storage_size = decode_u32(r);
        let array_buffer_size = decode_u32(r);
        let obj_key_buffer_size = decode_u32(r);
        let obj_value_buffer_size = decode_u32(r);

        let mut cjs_module_offset = 0;
        let mut segment_id = 0;

        if version < 78 {
            cjs_module_offset = decode_u32(r);
        } else {
            segment_id = decode_u32(r);
        }

        let mut cjs_module_count = 0;
        if version >= 84 {
            cjs_module_count = decode_u32(r);
        }

        let function_source_count = decode_u32(r);
        let debug_info_offset = decode_u32(r);

        let options = BytecodeOptions::deserialize(r, version);

        // Read padding bytes
        let mut _pad_bytes = [0u8; 19];
        r.read_exact(&mut _pad_bytes)
            .expect("error reading padding bytes");

        align_reader(r, 4).unwrap();

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
            _padding: _pad_bytes,
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

        // Write padding bytes
        w.write_all(&self._padding)
            .expect("unable to write padding bytes");
        /*
        // Write function headers
        for fh in &self.function_headers {
            match fh {
                FunctionHeader::Small(sfh) => {
                    align_writer(w, 4);
                    sfh.serialize(w)
                }
                FunctionHeader::Large(lfh) => lfh.serialize(w),
            }
        }

        // Write string kinds
        align_writer(w, 4);
        for sk in &self.string_kinds {
            sk.serialize(w);
        }

        // Write identifier hashes
        align_writer(w, 4);
        for ih in &self.identifier_hashes {
            encode_u32(w, *ih);
        }

        // Write string table entries
        align_writer(w, 4);
        for ss in &self.string_storage {
            ss.serialize(w);
        }

        // Write overflow string table entries
        align_writer(w, 4);
        for os in &self.overflow_string_storage {
            os.serialize(w);
        }

        // Write string storage bytes
        align_writer(w, 4);
        w.write_all(&self.string_storage_bytes)
            .expect("unable to write string storage bytes");

        // Write array buffer storage
        align_writer(w, 4);
        w.write_all(&self.array_buffer_storage)
            .expect("unable to write array buffer storage");

        // Write object key buffer
        align_writer(w, 4);
        w.write_all(&self.object_key_buffer)
            .expect("unable to write object key buffer");

        // Write object value buffer
        align_writer(w, 4);
        w.write_all(&self.object_val_buffer)
            .expect("unable to write object value buffer");

        // Write big int table
        align_writer(w, 4);
        for bi in &self.big_int_table {
            bi.serialize(w);
        }
        align_writer(w, 4);

        // Write reg exp table
        // TODO: actually look into this lol
        align_writer(w, 4);
        for re in &self.reg_exp_table {
            re.serialize(w);
        }
        align_writer(w, 4);

        // TODO: write reg_exp_storage bytes - needs to be added to the struct
        // reg_exp_storage

        // Write CJS modules
        align_writer(w, 4);
        for cjs in &self.cjs_modules {
            if self.options.cjs_modules_statically_resolved && self.version < 77 {
                match cjs {
                    CJSModule::CJSModuleInt(cjs_int) => cjs_int.serialize(w),
                    _ => panic!("CJSModuleInt expected, but got something else"),
                }
            } else {
                match cjs {
                    CJSModule::CJSModuleEntry(cjs_entry) => cjs_entry.serialize(w),
                    _ => panic!("CJSModuleEntry expected, but got something else"),
                }
            }
        }

        // Write function source entries
        align_writer(w, 4);
        for fse in &self.function_source_entries {
            fse.serialize(w);
        } */
    }

    // Read string
    // Read function
}
