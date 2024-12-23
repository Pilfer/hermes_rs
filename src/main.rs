#[macro_use]
pub mod hermes;

use hermes::{HermesHeader, HermesStructReader};

use std::{fs::File, io};

// This is mostly just a test function as of right now.
// TODO: Scrap it and only export the lib
fn main() {
    let filename = "./input_data/my_rn_hello_world.hbc.bundle";
    let f = File::open(filename).expect("no file found");
    let mut reader = io::BufReader::new(f);
    let header: HermesHeader = HermesStructReader::deserialize(&mut reader, 0);

    println!("HBC Version: {:?}", header.version);
    /*
    // works
    // header.function_headers.iter().for_each(|fh| {
    // println!("function header: {:?}", fh);
    // });

    // works
    // header.parse_bytecode(&mut reader);

    // works
    //for name in get_all_function_names(&header) {
    //    println!("Function name: {}", name);
    //}

    // Works - prints out the bytecode for the first function (Function<global>(1 params, 19 registers, 0 symbols)...)
    header.parse_bytecode_for_fn(0, &mut reader);

    // if writer == vec![115, 0, 2, 0, 98, 92, 0] {
    //     println!("Bytecode is correct!");
    // } else {
    //     println!("Bytecode is incorrect!");
    // } */
}

/*
#[allow(dead_code)]
fn get_all_function_names(header: &HermesHeader) -> Vec<String> {
    let mut function_names = Vec::new();
    for (index, function_header) in header.function_headers.iter().enumerate() {
        // let string_id = function_header.func_name().clone();
        let myfunc = &header
            .string_storage
            .get(function_header.func_name() as usize)
            .unwrap();
        // println!("------------------------------------------------");
        let func_start = myfunc.offset;
        let mut func_name = String::from_utf8(
            header.string_storage_bytes[func_start as usize..(func_start + myfunc.length) as usize]
                .to_vec(),
        )
        .unwrap();
        if func_name.is_empty() {
            func_name = format!("$FUNC_{}", index);
        }
        function_names.push(func_name);
    }
    function_names
}
 */
