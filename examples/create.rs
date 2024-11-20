use hermes_rs::debug_info::{
    DebugFileRegion, DebugInfo, DebugInfoHeader, DebugInfoOffsets, DebugInfoOffsetsNew,
};

use hermes_rs::string_kind::StringKindEntryNew;
use hermes_rs::{define_instructions, HermesFile};

use std::io::{Cursor, Read, Seek, SeekFrom, Write};

use hermes_rs::bytecode_options::BytecodeOptions;
use hermes_rs::function_header::FunctionHeader;
use hermes_rs::{FunctionHeaderFlag, HermesHeader, SmallFunctionHeader};

use std::{fs::File, io, vec};

fn main() {
    let bytebuff = vec![];
    let mut hermes_file = HermesFile::new(io::BufReader::new(Cursor::new(bytebuff)));

    // identical to the old version, sans bytecode fields it looks like.
    hermes_file.header = HermesHeader {
        magic: 2240826417119764422,
        version: 96,
        sha1: [
            169, 124, 131, 2, 218, 185, 11, 236, 113, 132, 169, 24, 59, 34, 180, 59, 173, 213, 122,
            101,
        ],
        file_length: 1337,
        global_code_index: 0,
        function_count: 1,
        string_kind_count: 1,
        identifier_count: 0,
        string_count: 2,
        overflow_string_count: 0,
        string_storage_size: 17,
        big_int_count: 0,
        big_int_storage_size: 0,
        reg_exp_count: 0,
        reg_exp_storage_size: 0,
        array_buffer_size: 0,
        obj_key_buffer_size: 0,
        obj_value_buffer_size: 0,
        cjs_module_offset: 0,
        segment_id: 0,
        cjs_module_count: 0,
        function_source_count: 0,
        debug_info_offset: 200,
        options: BytecodeOptions {
            static_builtins: false,
            cjs_modules_statically_resolved: false,
            has_async: false,
            flags: false,
        },
    };

    hermes_file.set_strings(vec![
        "global".to_string(),
        "print(123);".to_string(),
        "print(1+1);".to_string(),
    ]);

    // Create a function with a large highest_write_cache_index to overflow it
    // This will be a LargeFunctionHeader during serialization.
    hermes_file.add_function(
        &mut FunctionHeader::Small(SmallFunctionHeader {
            offset: 0,
            param_count: 1,
            byte_size: 10,
            func_name: 0,
            info_offset: 188,
            frame_size: 95,
            env_size: 96,
            highest_read_cache_index: 97,
            highest_write_cache_index: 98, // intentionally set to something large to overflow it
            flags: FunctionHeaderFlag {
                prohibit_invoke: hermes_rs::FunctionHeaderFlagProhibitions::ProhibitNone,
                strict_mode: false,
                has_exception_handler: false,
                has_debug_info: true,
                overflowed: false,
            },
            exception_handlers: vec![],
            debug_info: Some(DebugInfoOffsets::New(DebugInfoOffsetsNew {
                src: 1,
                scope_desc: 2,
                callee: 3,
            })),
        }),
        &mut define_instructions!(
            hermes_rs::v96,
            LoadConstString {
                r0: 0.into(),
                p0: 1.into()
            }, // load the string "print(123);" into r0
            DirectEval {
                r0: 0.into(),
                r1: 0.into(),
                p0: 0.into()
            },
            LoadConstString {
                r0: 0.into(),
                p0: 2.into()
            }, // load the string "print(1+1);" into r0
            DirectEval {
                r0: 0.into(),
                r1: 0.into(),
                p0: 0.into()
            },
            AsyncBreakCheck {},
            Ret { r0: 0.into() },
        )
        .unwrap(),
    );

    // Create a second function that'll end up being a SmallFunctionHeader
    hermes_file.add_function(
        &mut FunctionHeader::Small(SmallFunctionHeader {
            offset: 0,
            param_count: 1,
            byte_size: 10,
            func_name: 2, // set this to a large value to overflow it
            info_offset: 188,
            frame_size: 95,
            env_size: 96,
            highest_read_cache_index: 97,
            highest_write_cache_index: 1,
            flags: FunctionHeaderFlag {
                prohibit_invoke: hermes_rs::FunctionHeaderFlagProhibitions::ProhibitNone,
                strict_mode: false,
                has_exception_handler: false,
                has_debug_info: true,
                overflowed: false,
            },
            exception_handlers: vec![],
            debug_info: None,
        }),
        &mut define_instructions!(
            hermes_rs::v96,
            LoadConstString {
                r0: 0.into(),
                p0: 2.into()
            }, // load the string "print(123);" into r0
            LoadConstString {
                r0: 0.into(),
                p0: 1.into()
            }, // load the string "print(123);" into r0
            DirectEval {
                r0: 0.into(),
                r1: 0.into(),
                p0: 0.into()
            },
            AsyncBreakCheck {},
            Ret { r0: 0.into() },
        )
        .unwrap(),
    );

    hermes_file.string_kinds = vec![hermes_rs::StringKindEntry::New(StringKindEntryNew {
        count: hermes_file.string_storage.len() as u32,
        kind: hermes_rs::StringKind::String,
    })];

    // create DebugInfo struct
    let debug_info: DebugInfo = DebugInfo {
        header: DebugInfoHeader {
            filename_count: 1,
            filename_storage_size: 6,
            file_region_count: 1,
            scope_desc_data_offset: 13,
            textified_callee_offset: Some(16),
            string_table_offset: Some(17),
            debug_data_size: 17,
        },
        string_table: vec![],
        string_storage: vec![],
        file_regions: vec![DebugFileRegion {
            from_address: 0,
            filename_id: 0,
            source_mapping_url_id: 0,
        }],
        sources_data_storage: vec![
            0x00, 0x01, 0x01, 0x04, 0x00, 0x04, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x0F, 0x7F, 0x7F,
            0x00, 0x00,
        ],
        scope_desc_data_storage: vec![0x00, 0xE0],
        textified_callee_storage: vec![],
        string_table_storage: vec![],
    };

    hermes_file.debug_info = debug_info;
    hermes_file.set_debug_strings(vec!["yes.js".to_string()]);

    let mut writer: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    hermes_file.serialize(&mut writer);

    // write the new bytecode to the file
    let mut file = File::create("out.hbc").expect("unable to create file");

    writer
        .seek(SeekFrom::Start(0))
        .expect("unable to seek to start");

    file.write_all(&writer.bytes().map(|b| b.unwrap()).collect::<Vec<u8>>())
        .expect("unable to write to file");

    println!("File 'out.hbc' was created.");
}
