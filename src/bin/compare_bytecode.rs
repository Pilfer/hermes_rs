use hermes_rs::{define_instructions, InstructionParser};

fn main() {
    // Use the define_instructions macro. The first parameter is the hermes version
    let instructions = define_instructions!(
        hermes_rs::v96,
        LoadConstString { r0: 0, p0: 1 },
        DirectEval {
            r0: 0,
            r1: 0,
            p0: 0
        },
        Ret { r0: 0 },
    )
    .unwrap();

    let mut writer = Vec::new();

    for instr in instructions {
        instr.serialize(&mut writer);
    }

    assert!(
        writer == vec![115, 0, 1, 0, 94, 0, 0, 0, 92, 0],
        "Bytecode is incorrect!"
    );
}
