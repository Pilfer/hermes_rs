/*

Dumps the metro loader modules' function definitions.

This is useful for finding the function definitions for a given module for further analysis.

Ideally one one dump the function ids and then check the bytecode to see what they contain.

Usage:

    cargo run --bin modules <hbc_file>

*/

use core::panic;
use hermes_rs::{array_parser::ArrayTypes, hermes_file::HermesFile, HermesInstruction};
use std::{env, fs::File, io};

// Simple macro to handle version-specific instruction matching
macro_rules! match_instruction {
    ($instruction:expr, $insn_var:ident, $code:block) => {
        #[allow(unused_imports)]
        match $instruction {
            #[cfg(feature = "v76")]
            HermesInstruction::V76($insn_var) => {
                use hermes_rs::v76::*;
                $code
            }
            #[cfg(feature = "v84")]
            HermesInstruction::V84($insn_var) => {
                use hermes_rs::v84::*;
                $code
            }
            #[cfg(feature = "v89")]
            HermesInstruction::V89($insn_var) => {
                use hermes_rs::v89::*;
                $code
            }
            #[cfg(feature = "v90")]
            HermesInstruction::V90($insn_var) => {
                use hermes_rs::v90::*;
                $code
            }
            #[cfg(feature = "v93")]
            HermesInstruction::V93($insn_var) => {
                use hermes_rs::v93::*;
                $code
            }
            #[cfg(feature = "v94")]
            HermesInstruction::V94($insn_var) => {
                use hermes_rs::v94::*;
                $code
            }
            #[cfg(feature = "v95")]
            HermesInstruction::V95($insn_var) => {
                use hermes_rs::v95::*;
                $code
            }
            #[cfg(feature = "v96")]
            HermesInstruction::V96($insn_var) => {
                use hermes_rs::v96::*;
                $code
            }
        }
    };
}

fn dump_array_vals(
    hermes_file: &HermesFile<&mut io::BufReader<File>>,
    next_idx: usize,
    array_vals: &Vec<ArrayTypes>,
) -> String {
    let mut js_values: Vec<String> = Vec::new();
    for arr_val in array_vals {
        let v = match arr_val {
            ArrayTypes::EmptyValueSized { value: val } => {
                format!("...new Array({})", val)
            }
            ArrayTypes::NullValue {} => {
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
            ArrayTypes::NumberValue { value: val } => {
                // JS number
                format!("{}", val)
            }
            ArrayTypes::LongStringValue { value: val } => {
                // JS string literal, quoted
                let s = hermes_file.get_string_from_storage_by_index(*val as usize);
                format!("{:?}", s)
            }
            ArrayTypes::ShortStringValue { value: val } => {
                let s = hermes_file.get_string_from_storage_by_index(*val as usize);
                format!("{:?}", s)
            }
            ArrayTypes::ByteStringValue { value: val } => {
                let s = hermes_file.get_string_from_storage_by_index(*val as usize);
                format!("{:?}", s)
            }
            ArrayTypes::IntegerValue { value: val } => {
                // JS number
                format!("{}", val)
            }
            _ => {
                // fallback: JS undefined
                "undefined".to_string()
            }
        };
        js_values.push(v);
    }
    // Print as a JS array
    format!("[{}] /* Arr IDX: {} */", js_values.join(", "), next_idx)
}

fn main() {
    // Get first parameter passed to the program
    let args: Vec<String> = env::args().collect();
    let hbc_file = &args[1];

    if args.len() < 2 {
        println!("Usage: modules <hbc_file>");
        std::process::exit(1);
    }

    // check if file exists
    if !std::path::Path::new(hbc_file).exists() {
        println!("File not found: {}", hbc_file);
        std::process::exit(1);
    }

    let f = File::open(hbc_file).expect("no file found");

    let mut reader = io::BufReader::new(f);

    let mut hermes_file = HermesFile::deserialize(&mut reader);

    // The index of the __d string - __d() is the "define" function for the metro loader.
    // The metro loader will call __d() to define a module and export it to the global scope.
    let mut __d_string_index = 0;
    for (i, s) in hermes_file.get_strings().iter().enumerate() {
        if s == "__d" {
            __d_string_index = i;
            break;
        }
    }

    let _define_str = hermes_file.get_string_from_storage_by_index(__d_string_index);

    if _define_str != "__d" {
        panic!("__d string is not __d");
    }

    let first_bc = hermes_file.get_func_bytecode(0);

    for (idx, bc) in first_bc.iter().enumerate() {
        // Process TryGetById instructions to find __d
        match_instruction!(bc, insn, {
            if let Instruction::TryGetById(try_get) = insn {
                if try_get.p1.0 == __d_string_index as u16 {
                    /*
                    The metro "__d" function is used to define a module.
                    Arguments:
                        - factory function (closure reference)
                        - module id (how the metro bundler will reference it)
                        - dependency map (which modules this module depends on)
                        - note: if __DEV__ is defined, we get a fancy fourth parameter called "inverse dependency map"
                     */
                    let mut module_id = -1 as i32;
                    let mut module_factory = -1 as i32;
                    let mut func_name = String::new();
                    let mut dependency_map = String::new();

                    let _return_reg = try_get.r0;

                    // Fetch the factory function id from the CreateClosure instruction
                    if idx + 1 < first_bc.len() {
                        match_instruction!(&first_bc[idx + 1], next_insn, {
                            if let Instruction::CreateClosure(create_closure) = next_insn {
                                module_factory = create_closure.p0.0 as i32;
                                let func = hermes_file.function_headers
                                    [create_closure.p0.0 as usize]
                                    .clone();
                                func_name = hermes_file
                                    .get_string_from_storage_by_index(func.func_name() as usize)
                                    .to_string();

                                if func_name.is_empty() {
                                    func_name = format!("$FUNC_{}", create_closure.p0.0);
                                }
                            }
                        });
                    }

                    // Fetch the module id from the possible LoadConst* instructions
                    if idx + 2 < first_bc.len() {
                        match_instruction!(&first_bc[idx + 2], third_insn, {
                            let val = match third_insn {
                                Instruction::LoadConstZero(_) => {
                                    0 // this should always be zero
                                }
                                Instruction::LoadConstInt(load_int) => load_int.p0.0 as i32,
                                Instruction::LoadConstUInt8(load_uint8) => load_uint8.p0.0 as i32,
                                _ => 0 as i32,
                            };
                            module_id = val;
                        });
                    }

                    // Fetch the dependency map from the possible NewArray* instructions
                    // Note: NewArrayWithBuffer and NewArrayWithBufferLong have contents stored in the array_buffer_storage table
                    if idx + 3 < first_bc.len() {
                        match_instruction!(&first_bc[idx + 3], fourth_insn, {
                            dependency_map = match fourth_insn {
                                Instruction::NewArray(new_array) => dump_array_vals(
                                    &hermes_file,
                                    0,
                                    &vec![ArrayTypes::EmptyValueSized {
                                        value: new_array.p0.0 as u32,
                                    }],
                                ),
                                Instruction::NewArrayWithBuffer(new_array_with_buffer) => {
                                    let _arr_size = new_array_with_buffer.p1.0 as usize;
                                    let arr_idx = new_array_with_buffer.p2.0 as usize;
                                    let (_, arr_vals) = hermes_file.get_array_buffer(arr_idx, 0);
                                    dump_array_vals(&hermes_file, arr_idx, &arr_vals)
                                }
                                Instruction::NewArrayWithBufferLong(new_array_with_buffer_long) => {
                                    let _arr_size = new_array_with_buffer_long.p1.0 as usize;
                                    let arr_idx = new_array_with_buffer_long.p2.0 as usize;
                                    let (_, arr_vals) = hermes_file.get_array_buffer(arr_idx, 0);

                                    dump_array_vals(&hermes_file, arr_idx, &arr_vals)
                                }
                                _ => "Unknown - probably a bug".to_string(),
                            };
                        });
                    }

                    if module_id != -1 && module_factory != -1 {
                        println!(
                            "// Function {:?} being registered as a Metro module with a moduleId of {:?}\n__d({:?}, {:?}, {});",
                            func_name, module_id, module_factory, module_id, dependency_map
                        );
                    }
                }
            }
        });
    }
}
