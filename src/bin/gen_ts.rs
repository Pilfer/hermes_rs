/**
 * Kitchen sink project to generate TypeScript type definitions for the Hermes bytecode format.
 */
fn main() {
    generate_typescript_types();
}

#[allow(dead_code)]
fn write_def_file(filename: &str, contents: String) {
    use std::fs::{create_dir_all, File};
    use std::io::Write;
    use std::path::Path;

    let path = Path::new(filename);

    if let Some(parent) = path.parent() {
        create_dir_all(parent).unwrap();
    }

    let mut file = File::create(filename).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}

/**
 * Generate TypeScript type definitions for the Hermes bytecode format.
 * Note: You can only have one version of Hermes bytecode feature enabled at a time due to a limitation in
 * the `specta` crate. You'll get DuplicateTypeName errors if other versions are enabled.
 * The specta maintainers are working on this and it should be fixed soon (PR #72).
 */
fn generate_typescript_types() {
    #[cfg(feature = "specta")]
    {
        use specta::ts::{self, ExportConfiguration};

        use hermes_rs::{
            big_int_table::BigIntTableEntry,
            bytecode_options::BytecodeOptions,
            cjs_module::{CJSModule, CJSModuleEntry, CJSModuleInt},
            debug_info::{
                DebugFileRegion, DebugInfo, DebugInfoHeader, DebugInfoOffsets, DebugInfoOffsetsNew,
                DebugInfoOffsetsOld, DebugStringTable, FunctionDebugInfoDeserializer,
            },
            exception_handler::ExceptionHandlerInfo,
            function_header::{FunctionHeader, LargeFunctionHeader},
            function_sources::FunctionSourceEntry,
            hermes_file::{FunctionBytecode, FunctionInstructions, HermesFile, HermesOffsets},
            regexp_table::RegExpTableEntry,
            string_kind::{StringKindEntryNew, StringKindEntryOld},
            FunctionHeaderFlag, FunctionHeaderFlagProhibitions, HermesHeader,
            OverflowStringTableEntry, SmallFunctionHeader, SmallStringTableEntry, StringKind,
            StringKindEntry,
        };

        let conf = &ExportConfiguration::default().bigint(ts::BigIntExportBehavior::BigInt);

        #[cfg(feature = "v89")]
        {
            let _v89_insns = ts::export::<hermes_rs::v89::Instruction>(conf).unwrap();
            write_def_file("./ts/v89/instructions.d.ts", _v89_insns);
        }

        #[cfg(feature = "v90")]
        {
            let _v90_insns = ts::export::<hermes_rs::v90::Instruction>(conf).unwrap();
            write_def_file("./ts/v90/instructions.d.ts", _v90_insns);
        }

        #[cfg(feature = "v93")]
        {
            let _v93_insns = ts::export::<hermes_rs::v93::Instruction>(conf).unwrap();
            write_def_file("./ts/v93/instructions.d.ts", _v93_insns);
        }

        #[cfg(feature = "v94")]
        {
            let _v94_insns = ts::export::<hermes_rs::v94::Instruction>(conf).unwrap();
            write_def_file("./ts/v94/instructions.d.ts", _v94_insns);
        }

        #[cfg(feature = "v95")]
        {
            let _v95_insns = ts::export::<hermes_rs::v95::Instruction>(conf).unwrap();
            write_def_file("./ts/v95/instructions.d.ts", _v95_insns);
        }

        #[cfg(feature = "v96")]
        {
            let _v96_insns = ts::export::<hermes_rs::v96::Instruction>(conf).unwrap();
            write_def_file("./ts/v96/instructions.d.ts", _v96_insns);
        }

        let exports = vec![
            ts::export::<HermesFile<()>>(conf).unwrap(),
            ts::export::<HermesHeader>(conf).unwrap(),
            ts::export::<BytecodeOptions>(conf).unwrap(),
            ts::export::<BigIntTableEntry>(conf).unwrap(),
            ts::export::<CJSModule>(conf).unwrap(),
            ts::export::<CJSModuleEntry>(conf).unwrap(),
            ts::export::<CJSModuleInt>(conf).unwrap(),
            ts::export::<DebugInfo>(conf).unwrap(),
            ts::export::<DebugInfoOffsets>(conf).unwrap(),
            ts::export::<DebugInfoOffsetsNew>(conf).unwrap(),
            ts::export::<DebugInfoOffsetsOld>(conf).unwrap(),
            ts::export::<DebugInfoHeader>(conf).unwrap(),
            ts::export::<DebugStringTable>(conf).unwrap(),
            ts::export::<DebugFileRegion>(conf).unwrap(),
            ts::export::<FunctionDebugInfoDeserializer>(conf).unwrap(),
            ts::export::<ExceptionHandlerInfo>(conf).unwrap(),
            ts::export::<SmallFunctionHeader>(conf).unwrap(),
            ts::export::<LargeFunctionHeader>(conf).unwrap(),
            ts::export::<FunctionHeader>(conf).unwrap(),
            ts::export::<FunctionHeaderFlag>(conf).unwrap(),
            ts::export::<FunctionHeaderFlagProhibitions>(conf).unwrap(),
            ts::export::<FunctionSourceEntry>(conf).unwrap(),
            ts::export::<RegExpTableEntry>(conf).unwrap(),
            ts::export::<StringKind>(conf).unwrap(),
            ts::export::<StringKindEntry>(conf).unwrap(),
            ts::export::<StringKindEntryNew>(conf).unwrap(),
            ts::export::<StringKindEntryOld>(conf).unwrap(),
            ts::export::<SmallStringTableEntry>(conf).unwrap(),
            ts::export::<OverflowStringTableEntry>(conf).unwrap(),
            ts::export::<HermesOffsets>(conf).unwrap(),
            ts::export::<FunctionBytecode>(conf).unwrap(),
            ts::export::<FunctionInstructions>(conf).unwrap(),
        ];

        let exports_str = exports.join("\n");

        write_def_file("./ts/hermes.d.ts", exports_str);
    }
}
