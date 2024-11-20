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

    let mut hermes_file = HermesFile::deserialize(&mut reader);

    hermes_file.print_bytecode();
}
