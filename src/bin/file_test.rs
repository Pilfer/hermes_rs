use hermes_rs::hermes_file::HermesFile;
use std::{fs::File, io};

fn main() {
    // let filename = "./junk/oof/yes2.hbc";
    let filename = "./input_data/index.android.bundle";
    let f = File::open(filename).expect("no file found");
    let mut reader = io::BufReader::new(f);
    let mut hermes_file = HermesFile::deserialize(&mut reader);

    println!("\n\n\nFile: {:?}", hermes_file);

    println!("Time to get the bytecode for the functions!");

    let bc = hermes_file.get_bytecode();

    println!("{0: <10} | {1: <10} ", "Func IDX", "Code");
    for func in bc.iter().enumerate() {
        println!("{0: <10} | {1: <?}", func.0, func.1);
    }

    println!("Strings: {:?}", hermes_file.get_strings());

    // hermes_file.print_bytecode();
}
