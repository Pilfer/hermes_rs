use hermes_rs::hermes_file::HermesFile;
use std::{env, fs::File, io};

fn main() {
    // Get first parameter passed to the program
    let args: Vec<String> = env::args().collect();
    let hbc_file = &args[1];

    if args.len() < 2 {
        println!("Usage: {} <hbc_file>", args[0]);
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

    println!("{:?}", hermes_file.header);
    println!("{:?}", hermes_file.function_source_entries);
    println!("{:?}", hermes_file.debug_info);
}
