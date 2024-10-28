use hermes_rs::hermes_file::HermesFile;
use hermes_rs::{define_instructions, InstructionParser};
use std::{fs::File, io};

fn main() {
    // let filename = "./input_data/yeet.bundle";
    // let filename = "./input_data/eval.hbc";
    let filename = "./junk/oof/yes2.hbc";
    let f = File::open(filename).expect("no file found");
    let mut reader = io::BufReader::new(f);
    let mut hermes_file = HermesFile::deserialize(&mut reader);

    hermes_file.print_bytecode();

    /*
    // println!("Time to get the bytecode for the functions!");
    let bc = hermes_file.get_bytecode();

    // Print out the bytecode bytes in a pretty table
    for func in bc {
        println!("{:?}", func);
        // println!("Function #{:?} - {:?}", func.func_index, func.bytecode);
    }
     */

    // println!("Strings: {:?}", hermes_file.get_strings());
}
