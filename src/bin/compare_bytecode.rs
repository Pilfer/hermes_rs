use hermes_rs::{define_instructions, InstructionParser};

fn main() {
    #[cfg(feature = "v96")]
    // Use the define_instructions macro. The first parameter is the hermes version
    let instructions = define_instructions!(
        hermes_rs::v96,
        LoadConstString {
            r0: 0.into(),
            p0: 1.into()
        },
        DirectEval {
            r0: 0.into(),
            r1: 0.into(),
            p0: 0.into()
        },
        Ret { r0: 0.into() },
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
