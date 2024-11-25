# hermes_rs

Note: Still a WIP - A PR is always welcome.  

The API is subject to change as I iterate over use cases and improve the design. 


A *nearly* dependency-free disassembler and assembler for the Hermes bytecode, written in Rust.  

For the sake transparency, the current dependencies are:  

- `sha1`  
  - This is required for generating the footer hash. 
- `serde` - Optional  
  - So you can serialize/deserialize in your app  
- `specta` - Optional  
  - To generate TS types for tauri or wasm use cases  
- `specta-util` - Optional  
  - Same as above  

A special thanks to [P1sec](https://github.com/P1sec/hermes-dec) for digging through the Hermes git repo, pulling all of the BytecodeDef files out, and tagging them. This made writing this tool much easier.

- [hermes\_rs](#hermes_rs)
    - [Supported HBC Versions](#supported-hbc-versions)
      - [Project Goals](#project-goals)
        - [Potential Use cases](#potential-use-cases)
      - [Features](#features)
  - [Installation](#installation)
  - [Usage](#usage)
      - [Reading File Header](#reading-file-header)
      - [Reading Strings](#reading-strings)
      - [Reading Function Headers](#reading-function-headers)
      - [Dumping Bytecode](#dumping-bytecode)
      - [Encoding Instructions](#encoding-instructions)
      - [Creating Binaries From Scratch](#creating-binaries-from-scratch)
      - [Using specific HBC Versions](#using-specific-hbc-versions)
- [Hermes Resources](#hermes-resources)
- [Development](#development)
    - [Debugging Existing Functionality](#debugging-existing-functionality)
    - [Supporting new versions of Hermes](#supporting-new-versions-of-hermes)

### Supported HBC Versions

| HBC Version | Disassembler | (Binary) Assembler | (Textual) Assembler | Decompiler |
| ----------- | ------------ | ------------------ | ------------------- | ---------- |
| 89          | ✅           | ✅                 | ❌                  | ❌         |
| 90          | ✅           | ✅                 | ❌                  | ❌         |
| 93          | ✅           | ✅                 | ❌                  | ❌         |
| 94          | ✅           | ✅                 | ❌                  | ❌         |
| 95          | ✅           | ✅                 | ❌                  | ❌         |
| 96          | ✅           | ✅                 | ❌                  | ❌         |

A couple of features are missing currently, as they're low priority for me at the moment.

- Regular Expression deserialization and serialization*  
- Debug Info  deserialization and serialization*  

\* _Supports u8 buffer for manual population_  


#### Project Goals

- Full coverage for all public HBC versions  
- The ability to inject code stubs directly into the .hbc file for instrumentation  
- Textual HBC assembly  
- Eventually a halfway decent decompiler, but that may be another project that uses this one  

##### Potential Use cases

- Find which functions reference specific strings
- Generate frida hooks for mobile implementations
  - hermes loader -> hook loading the package -> feed to hermes_rs -> patch code
    for bidirectional communication or even just logging
- Writing fuzzers

#### Features

- Disassemble Hermes Bytecode (HBC)  
- Assemble Hermes Bytecode (HBC)  
- Type-safe instruction building across multiple versions of HBC  
- The ability to reduce binary size by [only enabling certain versions of HBC](#using-specific-hbc-versions)
- **Utilities**  
  - Dump Bytecode  
    - `cargo run --bin bytecode index.android.bundle`  
  - Dump Strings  
    - `cargo run --bin strings index.android.bundle`  

## Installation

To add hermes_rs to your project, simply run:  

`cargo add hermes_rs`  

- **Specific HBC Versions** - enable any of `["v89","v90","v93","v94","v95", "v96"]`  
  - **Default**: `["v94","v95", "v96"]`  
- **Serde Support** (Optional) - enable the `serde` feature  
- **Generate TS Types** (Optional) - enable the `specta` feature  
  - `cargo run --bin gen_ts --features specta` will output `*.d.ts` to `./ts`.  
  - Note: Only one HBC version can be used at a time for this due to a limitiation in `specta`.  



## Usage

#### Reading File Header

```rust
let filename = "./input_data/index.android.bundle";

let f = File::open(filename).expect("no file found");
let mut reader = io::BufReader::new(f);

let mut hermes_file = HermesFile::deserialize(&mut reader);

println!("{:?}", hermes_file.header);
```

Output:

```go
HermesHeader {
  magic: 2240826417119764422,
  version: 94,
  sha1: [20, 178, 139, 133, 105, 198, 134, 29, 58, 101, 194, 248, 210, 173, 84, 79, 162, 174, 43, 205],
  file_length: 11059884,
  global_code_index: 0,
  function_count: 54483,
  string_kind_count: 3,
  identifier_count: 35878,
  string_count: 65091,
  overflow_string_count: 425,
  string_storage_size: 2238216,
  big_int_count: 0,
  big_int_storage_size: 0,
  reg_exp_count: 448,
  reg_exp_storage_size: 49719,
  array_buffer_size: 132510,
  obj_key_buffer_size: 43517,
  obj_value_buffer_size: 137207,
  segment_id: 0,
  cjs_module_count: 0,
  cjs_module_offset: 0,
  function_source_count: 1361,
  debug_info_offset: 11059836,
  options: BytecodeOptions {
    static_builtins: false,
    cjs_modules_statically_resolved: false,
    has_async: false,
    flags: false
  },
}
```

#### Reading Strings  

```rust
println!("Strings: {:?}", hermes_file.get_strings());
```

Output:  

```sh
Strings: ["$$typeof", "type", "API", "isArray", "Array", ... ]
```

#### Reading Function Headers

```rust
for func in hermes_file.function_headers {
    println!("{:?}", func);
}

// Prints the following:
Small(SmallFunctionHeader { offset: 252641, param_count: 3, byte_size: 63, func_name: 5168, info_offset: 842244, frame_size: 16, env_size: 0, highest_read_cache_index: 1, highest_write_cache_index: 0, flags: FunctionHeaderFlag { prohibit_invoke: ProhibitNone, strict_mode: false, has_exception_handler: false, has_debug_info: false, overflowed: false }, exception_handlers: [], debug_info: None })
Small(LargeFunctionHeader { offset: 252704, param_count: 2, byte_size: 41, func_name: 3756, info_offset: 842244, frame_size: 14, env_size: 0, highest_read_cache_index: 1, highest_write_cache_index: 0, flags: FunctionHeaderFlag { prohibit_invoke: ProhibitNone, strict_mode: false, has_exception_handler: false, has_debug_info: false, overflowed: false }, exception_handlers: [], debug_info: None })
...
```

#### Dumping Bytecode

**Single Function**

Call `print_bytecode_for_fn(fidx)`, where `fidx` is the function index of the element in `hermes_file.function_headers`.

```rust
hermes_file.parse_bytecode_for_fn(1337);
```

Output:

```asm
Function<setCurrentTarget>(3 params, 11 registers, 0 symbols):
0x00000000	GetEnvironment  r0,  0
0x00000001	LoadFromEnvironment  r2,  r0,  6
0x00000002	LoadConstUndefined  r0
0x00000003	LoadParam  r1,  1
0x00000004	Call2  r2,  r2,  r0,  r1
0x00000005	LoadConstNull  r1
0x00000006	PutById  r2,  r1,  1,  "currentTarget"
0x00000007	Ret  r0
```

**Entire File**  

Printing out the bytecode for the entire executable in a human-readable format is possible by calling `print_bytecode`.  

```rust
hermes_file.print_bytecode();
```

Which outputs:  

```smali
Function<global>(1 params, 19 registers, 0 symbols):
0x00000000	DeclareGlobalVar  "__BUNDLE_START_TIME__"
0x00000001	DeclareGlobalVar  "__DEV__"
0x00000002	DeclareGlobalVar  "process"
0x00000003	DeclareGlobalVar  "__METRO_GLOBAL_PREFIX__"
0x00000004	CreateEnvironment  r3
0x00000005	LoadThisNS  r4
0x00000006	GetById  r1,  r4,  1,  "nativePerformanceNow"
0x00000007	GetGlobalObject  r0
0x00000008	JmpTrue  2L1,  r1
0x00000009	TryGetById  r2,  r0,  2,  "Date"
0x0000000A	GetByIdShort  r1,  r2,  3,  "now"
0x0000000B	Call1  r1,  r1,  r2
0x0000000C	Jmp  L1
0x0000000D	TryGetById  r5,  r0,  1,  "nativePerformanceNow"
    L1:
0x0000000E	LoadConstUndefined  r2
0x0000000F	Call1  r1,  r5,  r2
0x00000010	PutById  r0,  r1,  1,  "__BUNDLE_START_TIME__"
0x00000011	LoadConstFalse  r1
0x00000012	PutById  r0,  r1,  2,  "__DEV__"
0x00000013	GetByIdShort  r1,  r4,  4,  "process"
0x00000014	JmpTrue  5,  r1
...
```

**Raw Bytes**  

In the event that you want to access *just* the raw bytes for a specific function, you can use `hermes_file.get_bytecode()` and iterate.
The function returns a `Vec<FunctionBytecode>`, which has the function index and bytecode (`Vec<u8>`) pairing.

```rust
let bc = hermes_file.get_bytecode();

for func in bc {
  println!("{:?}", func);
}
```

Output:  

```sh
FunctionBytecode { func_index: 0, bytecode: [52, 2, 11, 0, 0, 52, 3, 11, 0, 0, 52, 217, 0, 0, 0, 52, ..<truncated>... ] }
FunctionBytecode { func_index: 1, bytecode: [50, 2, 108, 8, 1, 42, 2, 0, 8, 100, 7, 2, 2, 0, 100, 4, ..<truncated>... ] }
FunctionBytecode { func_index: 2, bytecode: [48, 0, 57, 0, 0, 1, 19, 0, 54, 1, 0, 2, 219, 106, 1, 1, ..<truncated>... ] }
FunctionBytecode { func_index: 3, bytecode: [108, 3, 2, 41, 0, 0, 46, 2, 0, 1, 54, 1, 2, 1, 163, 83, ..<truncated>... ] }
FunctionBytecode { func_index: 4, bytecode: [108, 3, 1, 41, 0, 0, 46, 2, 0, 1, 54, 1, 2, 1, 47, 83,  ..<truncated>... ] }
FunctionBytecode { func_index: 5, bytecode: [108, 4, 1, 41, 2, 0, 46, 1, 2, 1, 54, 0, 1, 1, 47, 83,  ..<truncated>... ] }
FunctionBytecode { func_index: 6, bytecode: [108, 4, 1, 41, 2, 0, 46, 1, 2, 1, 54, 0, 1, 1, 47, 83,  ..<truncated>... ] }
```

#### Encoding Instructions

Encoding instructions is trivial - each `Instruction` implements a trait with `deserialize` and `serialize` methods.


```rust
use hermes_rs::{define_instructions, InstructionParser};

/*
 * Use the define_instructions macro to get a vec of the correct instructions
 * for the version of Hermes you're targeting.
 * The first parameter is the hermes version you'd like to use.
 *
 * The bytecode below represents: eval(`print(123);`)
 * The `print(123)` string is the second (index 1) string in the string table.
*/
let instructions = define_instructions!(
    hermes_rs::v96,
    LoadConstString { r0: 0.into(), p0: 1.into() },   // Load `print(123);` into r0
    DirectEval { r0: 0.into(), r1: 0.into(), p0: 0.into() }, // Evaluate the string
    Ret { r0: 0.into() },                      // Return
).unwrap();

let mut writer = Vec::new();

for instr in instructions {
    instr.serialize(&mut writer);
}

// Make sure the encoded bytes are valid
assert!(writer == vec![115, 0, 1, 0, 94, 0, 0, 0, 92, 0], "Bytecode is incorrect!");
```

#### Creating Binaries From Scratch  

Take a look at the [Creating Binaries](./CreatingBinaries.md) example.

Working example: `cargo run --example create`  


#### Using specific HBC Versions

Want to use a specific version of the Hermes bytecode and reduce your binary size?

In Cargo.toml, find the `hermes_rs` dependency and select which HBC version(s) you'd like to use in your application.

Example:

```toml
[dependencies]
hermes_rs = { features = ["v89", "v90", "v93", "v94", "v95", "v96"] }
```

# Hermes Resources

**My Stuff**  

- **Github\.io Page**: https://pilfer.github.io/mobile-reverse-engineering/react-native/  

**Other Resources**  
- **Official docs**: https://hermesengine.dev/  
  - Source: https://github.com/facebook/hermes  
- **hermes-dec** disassembler/decompiler:  
  - https://github.com/P1sec/hermes-dec  
  - Opcode Docs: https://p1sec.github.io/hermes-dec/opcodes_table.html  
- **hbctool**: https://github.com/bongtrop/hbctool  
- **hasmer**: https://github.com/lucasbaizer2/hasmer  

---

# Development

### Debugging Existing Functionality  

My code isn't perfect, and this project is a fairly large undertaking. Because of this, you may run into some bugs. Aside from using an actual debugger with this project, the best tool to use for inspecting Hermes Bytecode files is [ImHex](https://github.com/WerWolv/ImHex) by [WerWolv](https://github.com/WerWolv).  

It features an expressive pattern language that allows you to visualize binary structures with ease.

I've included a [scratchpad pattern](./helpers/hermes.hexpat) to speed things up. It isn't 1:1 with this repository, but it's close enough to inspect most bits of memory to identify where things may be going wrong and why.  

If you need something more verbose that includes _examples_ of instructions, see: [https://github.com/Pilfer/hermes-imhex-pattern/](https://github.com/Pilfer/hermes-imhex-pattern/). Please note that parts of that file are wrong and you may need to do some hevay lifting or code generation to support the HBC version you're targeting.
 
### Supporting new versions of Hermes

This section assumes that only instructions have been modified, and not core parsing logic (struct fields, RegExp bytecode, Debug Info fields, etc). If the latter has a diff, obviously we'll need to implement those changes.  

There is a script in `./def_versions/_gen_macros.js` that reads and parses a Bytecode Definition file passed to it as the first argument and outputs a file containing the macro body to support the updated instructions.


```sh
# How I generated them

cd ./def_versions

node _gen_macros.js 89.def > ../src/hermes/v89/mod.rs
node _gen_macros.js 90.def > ../src/hermes/v90/mod.rs
node _gen_macros.js 93.def > ../src/hermes/v93/mod.rs
node _gen_macros.js 94.def > ../src/hermes/v94/mod.rs
node _gen_macros.js 95.def > ../src/hermes/v95/mod.rs
```

Example with a hypothetical `v100` version :

```sh
node _gen_macros.js v100.def
```

Which outputs:

```rust
use crate::hermes;

build_instructions!(
  (0, Unreachable, ),
  (1, NewObjectWithBuffer, r0: Reg8, p0: UInt16, p1: UInt16, p2: UInt16, p3: UInt16),
  (2, NewObjectWithBufferLong, r0: Reg8, p0: UInt16, p1: UInt16, p2: UInt32, p3: UInt32),
  (3, NewObject, r0: Reg8),
  (4, NewObjectWithParent, r0: Reg8, r1: Reg8),
  ... other instructions here
);
```

From here, you'll add a new directory and `mod.rs` file for this version (`./src/hermes/v100/mod.rs`) and paste the output from the script into it.

This could (and probably should) be a `build.rs` process.

After creating this file, open up `./src/hermes/mod.rs` and navigate to the Instruction module imports and add the import, then populate the Instruction enum + trait + other functions' match statements with the new version. You'll likely need to rely on the compiler to complain about missing match branches - there's only a few, though.

As this codebase evolves, you may need add branch arms in different matches.

```rust
#[macro_use]
#[cfg(feature = "v100")]
pub mod v100;

// ...

pub enum HermesInstruction {
  // ...
  #[cfg(feature = "v100")]
  V100(v100::Instruction),
}

// ...

impl HermesInstruction {
  // implement the methods of the trait
  fn display(&self, _hermes: &HermesHeader) -> String{
      match self {
        // ...
        #[cfg(feature = "v100")]
        HermesInstruction::V100(instruction) => instruction.display(_hermes),
      }
  }

  fn size(&self) -> usize {
      match self {
          // ...
          #[cfg(feature = "v100")]
          HermesInstruction::V100(instruction) => instruction.size(),
      }
  }
}


// ...

// In parse_bytecode there's currently a match statement that will also need to be populated...
let ins_obj: Option<HermesInstruction> = match self.version {
  #[cfg(feature = "v89")]
  89 => Some(HermesInstruction::V89(v89::Instruction::deserialize(&mut r_cursor, op))),
  #[cfg(feature = "v90")]
  90 => Some(HermesInstruction::V90(v90::Instruction::deserialize(&mut r_cursor, op))),
  #[cfg(feature = "v93")]
  93 => Some(HermesInstruction::V93(v93::Instruction::deserialize(&mut r_cursor, op))),
  #[cfg(feature = "v94")]
  94 => Some(HermesInstruction::V94(v94::Instruction::deserialize(&mut r_cursor, op))),
  #[cfg(feature = "v95")]
  95 => Some(HermesInstruction::V95(v95::Instruction::deserialize(&mut r_cursor, op))),
  _ => None,
};
```

Finally, add the `feature` (`v100 = []`) to Cargo.toml.

