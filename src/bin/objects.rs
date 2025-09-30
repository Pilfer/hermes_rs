use hermes_rs::array_parser::ArrayTypes;
use hermes_rs::hermes_file::HermesFile;
use std::io::{self};
use std::{env, fs::File};

fn main() {
    // Get first parameter passed to the program
    let args: Vec<String> = env::args().collect();
    let hbc_file = &args[1];

    if args.len() < 2 {
        println!("Usage: strings <hbc_file>");
        std::process::exit(1);
    }

    // check if file exists
    if !std::path::Path::new(hbc_file).exists() {
        println!("File not found: {}", hbc_file);
        std::process::exit(1);
    }

    let f = File::open(hbc_file).expect("no file found");

    let mut reader = io::BufReader::new(f);

    let mut hermes_file: HermesFile<&mut io::BufReader<File>> =
        HermesFile::deserialize(&mut reader);

    // print object key storage as hex
    // println!("object key storage: {:x?}", hermes_file.object_key_buffer);

    fn print_array_vals(
        prefix: &str,
        hermes_file: &HermesFile<&mut io::BufReader<File>>,
        next_idx: usize,
        array_vals: &Vec<ArrayTypes>,
    ) {
        let mut js_values: Vec<String> = Vec::new();
        for arr_val in array_vals {
            let v = match arr_val {
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
        println!(
            "const obj{}_{} = [{}];",
            prefix,
            next_idx,
            js_values.join(", ")
        );
    }
    // println!("object key buffer: {:?}", o);

    let mut next_idx = 0;
    while next_idx < hermes_file.object_key_buffer.len() {
        let o = hermes_file.get_object_key_buffer(next_idx, 0);
        let new_idx = o.0;
        print_array_vals("keys", &hermes_file, new_idx, &o.1);
        if new_idx <= next_idx {
            println!("Warning: Index didn't advance, breaking to prevent infinite loop");
            break;
        }

        next_idx = new_idx;
    }

    // do the same with object_val_buffer
    let mut next_idx = 0;
    while next_idx < hermes_file.object_val_buffer.len() {
        let o = hermes_file.get_object_val_buffer(next_idx, 0);
        let new_idx = o.0;
        print_array_vals("vals", &hermes_file, new_idx, &o.1);
        if new_idx <= next_idx {
            println!("Warning: Index didn't advance, breaking to prevent infinite loop");
            break;
        }

        next_idx = new_idx;
    }
}
