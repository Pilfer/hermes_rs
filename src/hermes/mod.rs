pub mod array_parser;
pub mod big_int_table;
pub mod bytecode_options;
pub mod cjs_module;
pub mod debug_info;
pub mod decode;
pub mod encode;
pub mod exception_handler;
pub mod function_header;
pub mod function_sources;
pub mod header;
pub mod hermes_file;
pub mod jenkins;
pub mod regexp_table;
pub mod string_kind;
pub mod string_table;
pub mod types;
pub mod instructions;
pub(crate) mod util;

pub use util::Serializable;
pub use instructions::*;

use std::{fmt::Display, io};

pub use function_header::{
    FunctionHeaderFlag, FunctionHeaderFlagProhibitions, SmallFunctionHeader,
};
pub use header::{HermesHeader, HermesStructReader};
pub use hermes_file::HermesFile;
pub use string_kind::{StringKind, StringKindEntry};
pub use string_table::{OverflowStringTableEntry, SmallStringTableEntry};

use array_parser::ArrayTypes;

pub trait InstructionParser {
    // fn new() -> Self;
    /// Decodes from binary format into rust struct.
    fn deserialize<R>(r: &mut R, op: u8) -> Self
    where
        R: io::Read + io::BufRead + io::Seek;

    /// Encodes struct to binary format.
    fn serialize<W>(&self, _w: &mut W)
    where
        W: io::Write;

    fn get_string_field_names(&self) -> Vec<&str>;

    fn get_function_field_names(&self) -> Vec<&str>;

    fn display<R>(&self, hermes: &HermesFile<R>) -> String
    where
        R: io::Read + io::BufRead + io::Seek;

    /// Size of the struct when encoded.
    fn size(&self) -> usize;

    fn has_ret_target(&self) -> bool {
        false
    }

    fn is_jmp(&self) -> bool;

    fn get_address_field(&self) -> u32;
}

fn print_string<R>(idx: impl Into<usize>, hermes_file: &HermesFile<R>) -> impl Display
where
    R: io::Read + io::BufRead + io::Seek
{
	format!("{:?}", hermes_file.get_string_from_storage_by_index(idx.into()))
}

pub fn print_array_vals<R>(hermes_file: &HermesFile<R>, array_vals: &Vec<ArrayTypes>) -> String
where
    R: io::Read + io::BufRead + io::Seek,
{
    let mut js_values: Vec<String> = Vec::new();
    for arr_val in array_vals {
        let v = match arr_val {
            ArrayTypes::NullValue => {
                // JS null
                "null".to_string()
            }
            ArrayTypes::TrueValue { value: true } => {
                // JS true
                "true".to_string()
            }
            ArrayTypes::FalseValue { value: false } => {
                // JS false
                "false".to_string()
            }
            ArrayTypes::NumberValue { value } => {
                // JS number
                format!("{value}")
            }
            ArrayTypes::LongStringValue { value } => print_string(*value as usize, hermes_file).to_string(),
            ArrayTypes::ShortStringValue { value } => print_string(*value, hermes_file).to_string(),
            ArrayTypes::ByteStringValue { value } => print_string(*value, hermes_file).to_string(),
            ArrayTypes::IntegerValue { value } => {
                // JS number
                format!("{value}")
            }
            _ => {
                // fallback: JS undefined
                "undefined".to_string()
            }
        };
        js_values.push(v);
    }

    format!("Array Contents: [{}]", js_values.join(", "))
}

pub fn print_array_val<R>(hermes_file: &HermesFile<R>, array_val: &ArrayTypes) -> String
where
    R: io::Read + io::BufRead + io::Seek,
{
    match array_val {
        ArrayTypes::NullValue => {
            // JS null
            "null".to_string()
        }
        ArrayTypes::TrueValue { value: true } => {
            // JS true
            "true".to_string()
        }
        ArrayTypes::FalseValue { value: false } => {
            // JS false
            "false".to_string()
        }
        ArrayTypes::NumberValue { value } => {
            // JS number
            format!("{value}")
        }
		ArrayTypes::LongStringValue { value } => print_string(*value as usize, hermes_file).to_string(),
		ArrayTypes::ShortStringValue { value } => print_string(*value, hermes_file).to_string(),
		ArrayTypes::ByteStringValue { value } => print_string(*value, hermes_file).to_string(),
        ArrayTypes::IntegerValue { value } => {
            // JS number
            format!("{value}")
        }
        _ => {
            // fallback: JS undefined
            "undefined".to_string()
        }
    }
}
