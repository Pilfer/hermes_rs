use hermes_rs::debug_info::{
    DebugFileRegion, DebugInfo, DebugInfoHeader, DebugInfoOffsets, DebugStringTable,
};

use hermes_rs::{define_instructions, InstructionParser};
use sha1::{Digest, Sha1};
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

use hermes_rs::bytecode_options::BytecodeOptions;
use hermes_rs::function_header::FunctionHeader;
use hermes_rs::string_kind::StringKindEntryNew;
use hermes_rs::{
    FunctionHeaderFlag, HermesHeader, HermesStruct, Serializable, SmallFunctionHeader,
    SmallStringTableEntry,
};

use hermes_rs::encode::align_writer;

use std::{fs::File, io, vec};

fn main() {
    // identical to the old version, sans bytecode fields it looks like.
    let mut newheader: HermesHeader = HermesHeader {
        magic: 2240826417119764422,
        version: 96,
        sha1: [
            169, 124, 131, 2, 218, 185, 11, 236, 113, 132, 169, 24, 59, 34, 180, 59, 173, 213, 122,
            101,
        ],
        file_length: 286,
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
        function_headers: vec![FunctionHeader::Small(SmallFunctionHeader {
            offset: 176,
            param_count: 1,
            byte_size: 10,
            func_name: 0,
            info_offset: 188,
            frame_size: 1,
            env_size: 0,
            highest_read_cache_index: 0,
            highest_write_cache_index: 0,
            flags: FunctionHeaderFlag {
                prohibit_invoke: hermes_rs::FunctionHeaderFlagProhibitions::ProhibitNone,
                strict_mode: false,
                has_exception_handler: false,
                has_debug_info: true,
                overflowed: false,
            },
            exception_handlers: vec![],
            debug_info: DebugInfoOffsets {
                src: 0,
                scope_desc: 0,
                callee: 0,
            },
        })],
        options: BytecodeOptions {
            static_builtins: false,
            cjs_modules_statically_resolved: false,
            has_async: false,
            flags: false,
        },
        string_kinds: vec![hermes_rs::StringKindEntry::New(StringKindEntryNew {
            count: 2,
            kind: hermes_rs::StringKind::String,
        })],
        identifier_hashes: vec![],
        string_storage: vec![
            SmallStringTableEntry {
                length: 6,
                offset: 0,
                is_utf_16: false,
            },
            SmallStringTableEntry {
                length: 11,
                offset: 6,
                is_utf_16: false,
            },
        ],
        string_storage_bytes: vec![
            103, 108, 111, 98, 97, 108, 112, 114, 105, 110, 116, 40, 49, 50, 51, 41, 59,
        ],
        overflow_string_storage: vec![],
        array_buffer_storage: vec![],
        object_key_buffer: vec![],
        object_val_buffer: vec![],
        big_int_table: vec![],
        reg_exp_table: vec![],
        cjs_modules: vec![],
        function_source_entries: vec![],
        _padding: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    };

    // Define some instructions to write to the file
    let instructions = define_instructions!(
        hermes_rs::v96,
        LoadConstString { r0: 0, p0: 1 },
        DirectEval {
            r0: 0,
            r1: 0,
            p0: 0
        },
        Ret { r0: 0 },
    )
    .unwrap();

    // Now we can serialize the header, then write the instructions to the file at offset 176
    // make a new file writer
    let mut writer: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    newheader.serialize(&mut writer);
    align_writer(&mut writer, 32);

    writer
        .seek(io::SeekFrom::Start(
            newheader.function_headers[0].offset() as u64
        ))
        .expect("unable to seek to offset");

    for instr in instructions {
        instr.serialize(&mut writer);
    }

    // pad with 4 bytes after the instructions
    // This was in the SimpleBytecode-whatever file
    writer
        .write_all(&vec![0; 4])
        .expect("unable to write padding");

    // I have no idea why this is necessary, but it is.
    writer
        .write_all(&vec![0; 6])
        .expect("unable to write padding");

    // Pad again at the start of debug info
    writer
        .write_all(&vec![0; 4])
        .expect("unable to write padding");

    // create DebugInfo struct
    let debug_info: DebugInfo = DebugInfo {
        header: DebugInfoHeader {
            filename_count: 1,
            filename_storage_size: 6,
            file_region_count: 1,
            scope_desc_data_offset: 13,
            textified_callee_offset: 16,
            string_table_offset: 17,
            debug_data_size: 17,
        },
        string_table: vec![DebugStringTable {
            offset: 0,
            length: 6,
        }],
        string_storage: "yes.js".as_bytes().to_vec(),
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

    // serialize the debug info
    debug_info.serialize(&mut writer);

    let _footer_cursor_pos = writer.position();

    // Get all bytes between start of file and _footer_cursor_pos
    let executable_bytes = writer.get_ref().as_slice()[0.._footer_cursor_pos as usize].to_vec();

    let file_length = executable_bytes.len() + 20;

    writer
        .seek(io::SeekFrom::Start(0))
        .expect("unable to seek to start");

    newheader.file_length = file_length as u32;
    let mut header_buffer = Cursor::new(Vec::new());
    newheader.serialize(&mut header_buffer);

    // write the header to the writer at the start
    writer
        .write_all(&header_buffer.get_ref())
        .expect("unable to write header");

    // come back
    writer
        .seek(io::SeekFrom::Start(_footer_cursor_pos))
        .expect("unable to seek to start");

    let updated_bytes = writer.get_ref().as_slice()[0.._footer_cursor_pos as usize].to_vec();

    // calculate the footer hash
    let mut hasher = Sha1::new();
    hasher.update(&updated_bytes);
    let footer_hash = hasher.finalize();

    let _footer_hash_hex = footer_hash
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    writer
        .write_all(&footer_hash)
        .expect("unable to write padding");

    // write the new bytecode to the file
    let mut file = File::create("eval_print.hbc").expect("unable to create file");
    writer
        .seek(SeekFrom::Start(0))
        .expect("unable to seek to start");

    // print all bytes in writer as hex, for debug purposes
    // let hex_string3 = writer.clone().get_ref().iter().map(|b| format!("{:02x}", b)).collect::<String>();
    // println!("hex_string3: {}", hex_string3);

    writer.bytes().for_each(|b| {
        file.write(&[b.unwrap()]).expect("unable to write to file");
    });

    println!("File 'eval_print.hbc' was created.");
}
