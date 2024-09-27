use hermes_rs::{HermesHeader, HermesStruct};

use std::{fs::File, io};

// This is mostly just a test function as of right now.
// TODO: Scrap it and only export the lib
fn main() {
    // let filename = "./test_file.hbc";
    let filename = "./junk/eval.hbc";
    let f = File::open(filename).expect("no file found");
    let mut reader = io::BufReader::new(f);
    let header: HermesHeader = HermesStruct::deserialize(&mut reader, 0);

    println!("HBC Version: {:?}", header);

    header.function_headers.iter().for_each(|fh| {
        println!("function header: {:?}", fh);
    });

    /*
        let load_const_string =
            hermes::v94::Instruction::LoadConstString(hermes::v94::LoadConstString {
                op: hermes::v94::str_to_op("LoadConstString"),
                r0: 0,
                p0: 2,
            });

        // AsyncBreakCheck
        let async_break_check =
            hermes::v94::Instruction::AsyncBreakCheck(hermes::v94::AsyncBreakCheck {
                op: hermes::v94::str_to_op("AsyncBreakCheck"),
            });

        // Ret r0
        let ret = hermes::v94::Instruction::Ret(hermes::v94::Ret {
            op: hermes::v94::str_to_op("Ret"),
            r0: 0,
        });

        let instructions = vec![load_const_string, async_break_check, ret];
        let mut writer = Vec::new();
        for instr in instructions {
            instr.serialize(&mut writer);
        }

        assert!(
            writer == vec![115, 0, 2, 0, 98, 92, 0],
            "Bytecode is incorrect!"
        );
    */

    // if writer == vec![115, 0, 2, 0, 98, 92, 0] {
    //     println!("Bytecode is correct!");
    // } else {
    //     println!("Bytecode is incorrect!");
    // }
}
