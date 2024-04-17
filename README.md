# hermes-rs  

A dependency-free disassembler and assembler for the Hermes bytecode, written in Rust.   

A special thanks to [P1sec](https://github.com/P1sec/hermes-dec) for digging through the Hermes git repo, pulling all of the BytecodeDef files out, and tagging them. This made writing this tool much easier.  

- [hermes-rs](#hermes-rs)
    - [Supported HBC Versions](#supported-hbc-versions)
      - [Project Goals](#project-goals)
        - [Potential Use cases](#potential-use-cases)
      - [Features](#features)
  - [Installation](#installation)
  - [Usage](#usage)
      - [Reading File Header](#reading-file-header)
      - [Reading Function Headers](#reading-function-headers)
      - [Parsing Bytecode](#parsing-bytecode)
      - [Encoding Instructions](#encoding-instructions)
      - [Using specific HBC Versions](#using-specific-hbc-versions)
- [Hermes Resources](#hermes-resources)
- [Development](#development)
    - [Supporting new versions of Hermes](#supporting-new-versions-of-hermes)
  - [TODO](#todo)



### Supported HBC Versions    

| HBC Version | Disassembler | (Binary) Assembler | (Textual) Assembler | Decompiler |    
|--|--|--|--|--|    
| 89 | ✅ | ✅ | ❌ | ❌ |  
| 90 | ✅ | ✅ | ❌ | ❌ |  
| 93 | ✅ | ✅ | ❌ | ❌ |  
| 94 | ✅ | ✅ | ❌ | ❌ |  
| 95 | ✅ | ✅ | ❌ | ❌ |  



#### Project Goals  

- Full coverage for all public HBC versions  
- The ability to inject code stubs directly into the .hbc file for instrumentation  
- Textual HBC assembly  

##### Potential Use cases   

- Find which functions reference specific strings    
- Generate frida hooks for mobile implementations   
  - hermes loader -> hook loading the package -> feed to hermes-rs -> patch code  
for bidirectional communication or even just logging  
- Writing fuzzers  

#### Features  
- bLaZiNgLy FaSt  
- Export strings  
- Export bytecode  
- Encoding instructions piecemeal  
- Reduce binary size by [only enabling certain versions of HBC](#using-specific-hbc-versions)  


## Installation  

`cargo add hermes_rs`


## Usage  

#### Reading File Header  

```rust
let f = File::open("./test_file.hbc").expect("no file found");
let mut reader = io::BufReader::new(f);
let header: HermesHeader = HermesStruct::deserialize(&mut reader);

println!("Header: {:?}", header);
```

Output:  

```go
{
  magic: 2240826417119764422, 
  version: 94, 
  sha1: [ 13, 37, 133, 71, 337, 17, 182, 139, 155, 223, 133, 7, 132, 109, 21, 96, 3, 12, 19, 56], 
  file_length: 1102, 
  global_code_index: 0, 
  function_count: 3, 
  string_kind_count: 2, 
  identifier_count: 5, 
  string_count: 14, 
  overflow_string_count: 0, 
  string_storage_size: 88, 
  big_int_count: 0, 
  big_int_storage_size: 0, 
  reg_exp_count: 0, 
  reg_exp_storage_size: 0, 
  array_buffer_size: 0, 
  obj_key_buffer_size: 0, 
  obj_value_buffer_size: 0, 
  segment_id: 0, 
  cjs_module_count: 0, 
  function_source_count: 0, 
  debug_info_offset: 628, 
  options: BytecodeOptions { 
    static_builtins: false, 
    cjs_modules_statically_resolved: false, 
    has_async: false, 
    flags: false
  },
  function_headers: [
    SmallFunctionHeader { 
      offset: 348, 
      param_count: 1, 
      byte_size: 69, 
      func_name: 5, 
      info_offset: 576, 
      frame_size: 15,
      env_size: 2,
      highest_read_cache_index: 2,
      highest_write_cache_index: 0,
      flags: FunctionHeaderFlag {
        prohibit_invoke: ProhibitNone, 
        strict_mode: false, 
        has_exception_handler: true, 
        has_debug_info: true, overflowed: false
        }
    }, 
    // ...
  ], 
  string_kinds: [
    StringKindEntry { count: 9, kind: String }, 
    StringKindEntry { count: 5, kind: Identifier }
  ],
  string_storage: [
    SmallStringTableEntry { is_utf_16: false, offset: 0, length: 4}, 
    // ...
  ],
  string_storage_bytes: [ 119, 101, 101, ... ], 
  overflow_string_storage: []
}
```

#### Reading Function Headers  

```rust
header.function_headers.iter().for_each(|fh| {
  println!("function header: {:?}", fh);
});

// Prints the following:
// function header: SmallFunctionHeader { ... } }
// ...
```

#### Parsing Bytecode  

```rust
header.parse_bytecode(&mut reader);

// By default, prints the following. It is assumed that the end user will 
// bring their own functionality to play with the instructions as-needed

/*
Function<foo>(1 params, 1 registers, 0 symbols):
  LoadConstString  r0,  "bar"
  AsyncBreakCheck
  Ret  r0
*/        
```


#### Encoding Instructions  

Encoding instructions is trivial - each `Instruction` implements a trait with `deserialize` and `serialize` methods.

You can alias the imports for your version to shorten the code, but fully-expanded it may look something like this:  

```rust
let load_const_string = hermes::v94::Instruction::LoadConstString(hermes::v94::LoadConstString {
  op: hermes::v94::str_to_op("LoadConstString"),
  r0: 0,
  p0: 2,
});

let async_break_check = hermes::v94::Instruction::AsyncBreakCheck(hermes::v94::AsyncBreakCheck {
  op: hermes::v94::str_to_op("AsyncBreakCheck"),
});

let ret = hermes::v94::Instruction::Ret(hermes::v94::Ret {
    op: hermes::v94::str_to_op("Ret"),
    r0: 0,
});

let instructions = vec![load_const_string, async_break_check, ret];

let mut writer = Vec::new();
for instr in instructions {
    instr.serialize(&mut writer);
}

// Make sure the encoded bytes are valid
assert!(writer == vec![115, 0, 2, 0, 98, 92, 0], "Bytecode is incorrect!");
```

#### Using specific HBC Versions  

Want to use a specific version of the Hermes bytecode and reduce your binary size?  

In Cargo.toml, find the `hermes_rs` dependency and select which HBC version(s) you'd like to use in your application.  

Example:  
```toml
[dependencies]
hermes_rs = { features = ["v89", "v90", "v93", "v94", "v95"] }
```

# Hermes Resources    

I leaned heavily on the following projects and resources to develop this package.  

- **Official docs**: https://hermesengine.dev/  
  - Source: https://github.com/facebook/hermes  
- **hermes-dec** disassembler/decompiler:  
  - https://github.com/P1sec/hermes-dec  
  - Opcode Docs: https://p1sec.github.io/hermes-dec/opcodes_table.html  
- **hbctool**: https://github.com/bongtrop/hbctool  
- **hasmer** (stale): https://github.com/lucasbaizer2/hasmer  



--- 

# Development  


### Supporting new versions of Hermes  

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
  ... instructions here
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

pub enum Instruction {
  // ...
  #[cfg(feature = "v100")]
  V100(v100::Instruction),
}

// ...

impl Instruction {
  // implement the methods of the trait
  fn display(&self, _hermes: &HermesHeader) -> String{
      match self {
        // ...
        #[cfg(feature = "v100")]
        Instruction::V100(instruction) => instruction.display(_hermes),
      }
  }

  fn size(&self) -> usize {
      match self {
          // ...
          #[cfg(feature = "v100")]
          Instruction::V100(instruction) => instruction.size(),
      }
  }
}


// ...

// In parse_bytecode there's currently a match statement that will also need to be populated...
let ins_obj: Option<Instruction> = match self.version {
  #[cfg(feature = "v89")]
  89 => Some(Instruction::V89(v89::Instruction::deserialize(&mut r_cursor, op))),
  #[cfg(feature = "v90")]
  90 => Some(Instruction::V90(v90::Instruction::deserialize(&mut r_cursor, op))),
  #[cfg(feature = "v93")]
  93 => Some(Instruction::V93(v93::Instruction::deserialize(&mut r_cursor, op))),
  #[cfg(feature = "v94")]
  94 => Some(Instruction::V94(v94::Instruction::deserialize(&mut r_cursor, op))),
  #[cfg(feature = "v95")]
  95 => Some(Instruction::V95(v95::Instruction::deserialize(&mut r_cursor, op))),
  _ => None,
};
```

Finally, add the `feature` (`v100 = []`) to Cargo.toml.

---  

## TODO  

- [X] Break header structs out into their own files  
- [ ] Add correct logic for FunctionHeader/SmallFunctionHeader  
- [X] Exception definition stuff  
  - Code quality for this is pretty bad - should probably go back over it later  
- [ ] DebugInfo definition stuff  
- [ ] Add comments  
- [ ] Docs  
- [ ] `Serializer` implementations    

| Struct | Deserialize | Serialize | Size |  
|--|--|--|--|  
| HermesHeader | ✅ | ❌ | ❌ |  
| SmallFunctionHeader | ✅ | ❌ | ❌ |  
| StringKindEntry | ✅ | ✅ | ✅ |  
| SmallStringTableEntry | ✅ | ✅ | ✅ |  
| OverflowStringTableEntry | ✅ | ✅ | ✅ | 
| BigIntTableEntry | ✅ | ❌ | ❌ |  
| BytecodeOptions | ✅ | ✅ | ✅ |  
| DebugInfoOffsets | ✅ | ✅ | ✅ |  
| DebugInfoHeader | ✅ | ✅ | ✅ |  
| DebugFileRegion | ✅ | ✅ | ✅ |  
| ExceptionHandlerInfo | ✅ | ✅ | ✅ |  
| RegExpTableEntry | ✅ | ❌ | ❌ |  
| FunctionHeaderFlag | ✅ | ❌ | ❌ |  

- [ ] Parse in the correct order:

```cpp
// From official Hermes source code  
void visitBytecodeSegmentsInOrder(Visitor &visitor) {
  visitor.visitFunctionHeaders();
  visitor.visitStringKinds();
  visitor.visitIdentifierHashes();
  visitor.visitSmallStringTable();
  visitor.visitOverflowStringTable();
  visitor.visitStringStorage();
  visitor.visitArrayBuffer();
  visitor.visitObjectKeyBuffer();
  visitor.visitObjectValueBuffer();
  visitor.visitBigIntTable();
  visitor.visitBigIntStorage();
  visitor.visitRegExpTable();
  visitor.visitRegExpStorage();
  visitor.visitCJSModuleTable();
  visitor.visitFunctionSourceTable();
}
```