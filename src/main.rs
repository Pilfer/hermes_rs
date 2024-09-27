#[macro_use]
pub mod hermes;

use hermes::{HermesHeader, HermesStruct};

use std::{fs::File, io};

// This is mostly just a test function as of right now.
// TODO: Scrap it and only export the lib
fn main() {
    let filename = "./input_data/test_file.hbc";
    let f = File::open(filename).expect("no file found");
    let mut reader = io::BufReader::new(f);
    let header: HermesHeader = HermesStruct::deserialize(&mut reader, 0);

    println!("HBC Version: {:?}", header);

    header.function_headers.iter().for_each(|fh| {
        println!("function header: {:?}", fh);
    });

    header.parse_bytecode(&mut reader);

    // if writer == vec![115, 0, 2, 0, 98, 92, 0] {
    //     println!("Bytecode is correct!");
    // } else {
    //     println!("Bytecode is incorrect!");
    // }
}
