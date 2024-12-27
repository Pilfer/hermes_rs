use hermes_rs::hermes_file::HermesFile;
use std::{env, fs::File, io};

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

    let hermes_file = HermesFile::deserialize(&mut reader);
    // let hex_string_bytes = hermes_file.string_storage_bytes.iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join(" ");
    // println!("String storage bytes: {}", hex_string_bytes);

    for (_sidx, s) in hermes_file.get_strings_by_kind().iter().enumerate() {
        println!("{:?}-{}", s.kind, s.string); // println!("[#{}] {}", _sidx, s); // Print index and string if you'd like.
    }
}
