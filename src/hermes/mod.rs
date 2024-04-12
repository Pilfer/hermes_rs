pub mod decode;
pub mod encode;
use std::io;


use crate::hermes::decode::{decode_u32, decode_u64, decode_u8, read_bitfield};
use crate::hermes::encode::{encode_u32, encode_u64, encode_u8, write_bitfield};

pub type Reg8 = u8;
pub type Reg32 = u32;
pub type UInt8 = u8;
pub type UInt16 = u16;
pub type UInt32 = u32;
pub type Addr8 = i8;
pub type Addr32 = i32;
pub type Imm32 = i32;
pub type Double = f64;

pub trait InstructionParser {
    // fn new() -> Self;
    /// Decodes from binary format into rust struct.
    fn deserialize<R>(r: &mut R, op: u8) -> Self
    where
        R: io::Read + io::BufRead + io::Seek;

    /// Encodes struct to binary format.
    fn serialize<W>(&self, _w: &mut W)
    where
        W: io::Write;

    fn display(&self, hermes: &HermesHeader) -> String;

    /// Size of the struct when encoded.
    fn size(&self) -> usize;

    fn has_ret_target(&self) -> bool {
        false
    }
}


// Start macros

#[macro_export]
macro_rules! map_type {
  (Reg8) => { u8 };
  (Reg32) => { u32 };
  (UInt8) => { u8 };
  (UInt16) => { u16 };
  (UInt32) => { u32 };
  (Addr8) => { i8 };
  (Addr32) => { i32 };
  (Imm32) => { i32 };
  (Double) => { f64 };
  (StringIDUInt8) => { u8 };
  (StringIDUInt16) => { u16 };
  (StringIDUInt32) => { u32 };
  (FunctionIDUInt8) => { u8 };
  (FunctionIDUInt16) => { u16 };
  (FunctionIDUInt32) => { u32 };
  (BigIntIDUInt16) => { u16 };
  (BigIntIDUInt32) => { u32 };
}

#[macro_export]
macro_rules! map_size {
  (Reg8) => { 1 };
  (Reg32) => { 4 };
  (UInt8) => { 1 };
  (UInt16) => { 2 };
  (UInt32) => { 4 };
  (Addr8) => { 1 };
  (Addr32) => { 4 };
  (Imm32) => { 4 };
  (Double) => { 8 };
  (StringIDUInt8) => { 1 };
  (StringIDUInt16) => { 2 };
  (StringIDUInt32) => { 4 };
  (FunctionIDUInt8) => { 1 };
  (FunctionIDUInt16) => { 2 };
  (FunctionIDUInt32) => { 4 };
  (BigIntIDUInt16) => { 2 };
  (BigIntIDUInt32) => { 4 };
}

#[macro_export]
macro_rules! map_decode_fn {
  (Reg8) => { hermes::decode::decode_u8 };
  (Reg32) => { hermes::decode::decode_u32 };
  (UInt8) => { hermes::decode::decode_u8 };
  (UInt16) => { hermes::decode::decode_u16 };
  (UInt32) => { hermes::decode::decode_u32 };
  (Addr8) => { hermes::decode::decode_i8 };
  (Addr32) => { hermes::decode::decode_i32 };
  (Imm32) => { hermes::decode::decode_i32 };
  (Double) => { hermes::decode::decode_f64 };
  (StringIDUInt8) => { hermes::decode::decode_u8 };
  (StringIDUInt16) => { hermes::decode::decode_u16 };
  (StringIDUInt32) => { hermes::decode::decode_u32 };
  (FunctionIDUInt8) => { hermes::decode::decode_u8 };
  (FunctionIDUInt16) => { hermes::decode::decode_u16 };
  (FunctionIDUInt32) => { hermes::decode::decode_u32 };
  (BigIntIDUInt16) => { hermes::decode::decode_u16 };
  (BigIntIDUInt32) => { hermes::decode::decode_u32 };
}

#[macro_export]
macro_rules! map_encode_fn {
  (Reg8) => { hermes::encode::encode_u8 };
  (Reg32) => { hermes::encode::encode_u32 };
  (UInt8) => { hermes::encode::encode_u8 };
  (UInt16) => { hermes::encode::encode_u16 };
  (UInt32) => { hermes::encode::encode_u32 };
  (Addr8) => { hermes::encode::encode_i8 };
  (Addr32) => { hermes::encode::encode_i32 };
  (Imm32) => { hermes::encode::encode_i32 };
  (Double) => { hermes::encode::encode_f64 };
  (StringIDUInt8) => { hermes::encode::encode_u8 };
  (StringIDUInt16) => { hermes::encode::encode_u16 };
  (StringIDUInt32) => { hermes::encode::encode_u32 };
  (FunctionIDUInt8) => { hermes::encode::encode_u8 };
  (FunctionIDUInt16) => { hermes::encode::encode_u16 };
  (FunctionIDUInt32) => { hermes::encode::encode_u32 };
  (BigIntIDUInt16) => { hermes::encode::encode_u16 };
  (BigIntIDUInt32) => { hermes::encode::encode_u32 };
}

#[macro_export]
macro_rules! define_opcode {
($name:ident, $($field:ident : $arg:tt),*) => {

  #[derive(Debug, Copy, Clone)]
  pub struct $name {
    pub op: u8,
    $(
      pub $field: map_type!($arg),
    )*
  }

  impl Default for $name {
    fn default() -> Self {
      Self {
          op: 0,
          $(
            $field: match stringify!(map_type!($arg)) {
              "Double" => 0.0 as _,
              _ => 0 as _,
            },
          )*
      }
    }
  }

  impl hermes::InstructionParser for $name {
    fn deserialize<R>(_r: &mut R, op: u8) -> Self
    where
        R: std::io::BufRead + std::io::Seek,
    {
      return Self{
        op: op,
        $(
          $field: map_decode_fn!($arg)(_r),
        )*
      }
    }

    // Returns true if the instruction has a return target.
    fn has_ret_target(&self) -> bool {
      match op_to_instr(self.op) {
        Instruction::Call(_) |
        Instruction::Construct(_) |
        Instruction::Call1(_) |
        Instruction::CallDirect(_) |
        Instruction::Call2(_) |
        Instruction::Call3(_) |
        Instruction::Call4(_) |
        Instruction::CallLong(_) |
        Instruction::ConstructLong(_) |
        Instruction::CallDirectLongIndex(_)  => true,
        _ => false,
      }
    }

    // TODO: implement
    fn serialize<W>(&self, _w: &mut W)
      where
          W: std::io::Write,
      {
        hermes::encode::encode_u8(_w, self.op);
        $(
          map_encode_fn!($arg)(_w, self.$field as _);
        )*
      }

    #[allow(unused_mut)]
    fn size(&self) -> usize {
      let mut total_size = 0;
      $(
        total_size += map_size!($arg);
      )*
      total_size
    }

    #[allow(unused_mut)]
    fn display(&self, _hermes: &hermes::HermesHeader) -> String {

      let mut display_string = format!("{} ", op_to_str(self.op));
      $(
          display_string = match stringify!($arg) {
            "StringIDUInt8" | "StringIDUInt16" | "StringIDUInt32" => format!("{} \"{}\"", display_string, _hermes.get_string_from_storage_by_index(self.$field as usize)),
            "FunctionIDUInt8" | "FunctionIDUInt16" | "FunctionIDUInt32" => {
              let target_function = _hermes.function_headers[self.$field as usize].func_name;
              let func_str = _hermes
                  .get_string_from_storage_by_index(target_function as usize)
                  .to_string();
              format!("{} Function<{}>", display_string, func_str)
            },
            "BigIntIDUInt16" | "BigIntIDUInt32" => {
              // TODO: Make this work
              let bigint_addr = self.$field as usize;
              format!("{} {} ", display_string, bigint_addr)
            },
            _ => {
              let tmp = match stringify!($arg) {
                "Reg8" | "Reg32" => {
                  format!("{} r{},", display_string, self.$field )
                },
                _ => {
                  format!("{} {},", display_string, self.$field )
                }
              };
              // prepend "," to tmp
              format!("{} ", tmp)
              // format!("{} {} ", display_string,  stringify!($arg))//, self.$field )
            }
        };

        // stip trailing comma

      )*
      // format!("{} // has_ret_target {}", display_string.trim_end_matches(", ").trim().to_string(), self.has_ret_target())
      format!("{}", display_string.trim_end_matches(", ").trim().to_string())

    }
  }
};
}

/// Implement the InstructionParser trait for all instructions so we can use
/// the `deserialize` method to read instructions from a file/stream in a more
/// generic way. Also implements `serialize`, `display` and `size` methods.
#[macro_export]
macro_rules! impl_instruction_parser {
($($variant:ident => $insn:ident),*) => {
  #[allow(unused_variables, non_snake_case)]
  impl hermes::InstructionParser for Instruction {
    fn deserialize<R: std::io::Read + std::io::BufRead + std::io::Seek>(reader: &mut R, op: u8) -> Self {
        match op_to_instr(op) {
            $(
            Instruction::$variant($insn) => {
              let insn: $insn = $insn::deserialize(reader, op);
              Instruction::$variant(insn)
            },
          )*
        }
      }

      fn serialize<W: std::io::Write>(&self, _w: &mut W) {
        match self {
          $(
              Instruction::$variant($insn) => {
                $insn.serialize(_w);
              }
          )*
        }
      }

      fn display(&self, _hermes: &hermes::HermesHeader) -> String {
        match self {
          $(
            Instruction::$insn(insn) => insn.display(_hermes),
          )*
        }
      }

      fn size(&self) -> usize {
        match self {
          $(
            Instruction::$insn(insn) => insn.size(),
          )*
        }
      }
  }
};
}

#[macro_export]
macro_rules! build_instructions {
($(($opcode:expr, $instruction:ident, $($operand:ident : $type:ident),*)),*) => {

    $(define_opcode!($instruction, $($operand : $type),*);)*

    #[derive(Debug, Copy)]
    #[repr(u8)]
    pub enum Instruction {
      $(
        $instruction($instruction),
      )*
    }

    impl Clone for Instruction {
      fn clone(&self) -> Self {
        match self {
          $(
            Instruction::$instruction(insn) => Instruction::$instruction(insn.clone()),
          )*
        }
      }
    }

    pub fn op_to_str(op: u8) -> &'static str {
      match op {
        $(
          $opcode => stringify!($instruction),
        )*
        _ => "unknown",
      }
    }

    pub fn op_to_instr(op: u8) -> Instruction {
      match op {
        $(
          $opcode => Instruction::$instruction($instruction{op, ..Default::default()}),
        )*
        _ => Instruction::Unreachable(Unreachable{ op }),
      }
    }

    pub fn str_to_op(instr: &str) -> u8 {
      match instr {
        $(
          stringify!($instruction) => $opcode,
        )*
        _ => 0,
      }
    }

    pub fn instr_to_op(instr: Instruction) -> u8 {
      match instr {
        $(
          Instruction::$instruction(_) => $opcode,
        )*
      }
    }

    impl_instruction_parser!($($instruction => $instruction),*);

  };
}

// End macros

#[cfg(feature = "v89")]
#[macro_use]
pub mod v89;


#[macro_use]
#[cfg(feature = "v90")]
pub mod v90;
#[macro_use]
#[cfg(feature = "v93")]
pub mod v93;
#[macro_use]
#[cfg(feature = "v94")]
pub mod v94;
#[macro_use]
#[cfg(feature = "v95")]
pub mod v95;


#[derive(Debug, Clone)]
pub enum Instruction {
  #[cfg(feature = "v89")]
  V89(v89::Instruction),
  #[cfg(feature = "v90")]
  V90(v90::Instruction),
  #[cfg(feature = "v93")]
  V93(v93::Instruction),
  #[cfg(feature = "v94")]
  V94(v94::Instruction),
  #[cfg(feature = "v95")]
  V95(v95::Instruction),
}


impl Instruction {
  // implement the methods of the trait
  fn display(&self, _hermes: &HermesHeader) -> String{
      match self {
          #[cfg(feature = "v89")]
          Instruction::V89(instruction) => instruction.display(_hermes),
          #[cfg(feature = "v90")]
          Instruction::V90(instruction) => instruction.display(_hermes),
          #[cfg(feature = "v93")]
          Instruction::V93(instruction) => instruction.display(_hermes),
          #[cfg(feature = "v94")]
          Instruction::V94(instruction) => instruction.display(_hermes),
          #[cfg(feature = "v95")]
          Instruction::V95(instruction) => instruction.display(_hermes),
      }
  }

  fn size(&self) -> usize {
      match self {
          #[cfg(feature = "v89")]
          Instruction::V89(instruction) => instruction.size(),
          #[cfg(feature = "v90")]
          Instruction::V90(instruction) => instruction.size(),
          #[cfg(feature = "v93")]
          Instruction::V93(instruction) => instruction.size(),
          #[cfg(feature = "v94")]
          Instruction::V94(instruction) => instruction.size(),
          #[cfg(feature = "v95")]
          Instruction::V95(instruction) => instruction.size(),
      }
  }
}

#[derive(Debug)]
pub struct BytecodeOptions {
    pub static_builtins: bool,
    pub cjs_modules_statically_resolved: bool,
    pub has_async: bool,
    pub flags: bool,
}

#[derive(Debug)]
pub struct HermesHeader {
    pub magic: u64,
    pub version: u32,
    pub sha1: [u8; 20],
    pub file_length: u32,
    pub global_code_index: u32,
    pub function_count: u32,
    pub string_kind_count: u32,
    pub identifier_count: u32,
    pub string_count: u32,
    pub overflow_string_count: u32,
    pub string_storage_size: u32,
    pub big_int_count: u32,
    pub big_int_storage_size: u32,
    pub reg_exp_count: u32,
    pub reg_exp_storage_size: u32,
    pub array_buffer_size: u32,
    pub obj_key_buffer_size: u32,
    pub obj_value_buffer_size: u32,
    pub segment_id: u32,
    pub cjs_module_count: u32,
    pub function_source_count: u32,
    pub debug_info_offset: u32,

    pub options: BytecodeOptions,

    pub function_headers: Vec<SmallFunctionHeader>,
    pub string_kinds: Vec<StringKindEntry>,
    pub string_storage: Vec<SmallStringTableEntry>,
    pub string_storage_bytes: Vec<u8>,
    pub overflow_string_storage: Vec<OverflowStringTableEntry>,
    // options - u8, pad 19 bytes after
}

impl HermesHeader {
    pub fn get_string_from_storage_by_index(&self, index: usize) -> String {
        // return "placeholder".to_string();
        // println!("get_string_storage_by_index: {} - {}",insn, index);
        // println!("get_string_storage_by_index as u16: {}", index as u16);
        // println!("self.string_storage size is: {}", self.string_storage.len());
        let myfunc = self.string_storage.get(index).unwrap();
        // println!("{:?}", myfunc);
        return String::from_utf8(
            self.string_storage_bytes
                [myfunc.offset as usize..(myfunc.offset + myfunc.length) as usize]
                .to_vec(),
        )
        .unwrap();
    }
}

pub trait HermesStruct {
    /// Padding requirement from DEX spec.

    /// Decodes from binary format into rust struct.
    fn deserialize<R>(r: &mut R) -> Self
    where
        R: io::Read + io::BufRead + io::Seek;

    /// Encodes struct to binary format.
    fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write;

    /// Size of the struct when encoded.
    fn size(&self) -> usize;

    fn parse_bytecode<R>(&self, r: &mut R)
    where
        R: io::Read + io::BufRead + io::Seek;
}

#[derive(Debug)]
pub struct SmallFunctionHeader {
    pub offset: u32,
    pub param_count: u32,
    pub byte_size: u32,
    pub func_name: u32,
    pub info_offset: u32,
    pub frame_size: u32,
    pub env_size: u32,
    pub highest_read_cache_index: u32,
    pub highest_write_cache_index: u32,
    pub flags: FunctionHeaderFlag,
}

#[derive(Debug)]
pub struct DebugInfoOffsets {
    pub src: u32,
    pub scope_desc: u32,
    pub callee: u32,
}

#[derive(Debug)]
pub struct DebugInfoHeader {
    pub filename_count: u32,
    pub filename_storage_size: u32,
    pub file_region_count: u32,
    pub scope_desc_data_offset: u32,
    pub textified_callee_offset: u32,
    pub string_table_offset: u32,
    pub debug_data_size: u32,
}

#[derive(Debug)]
pub struct DebugFileRegion {
    pub from_address: u32,
    pub filename_id: u32,
    pub source_mapping_url_id: u32,
}

#[derive(Debug)]
pub struct BigIntTableEntry {
    pub offset: u32,
    pub length: u32,
}

#[derive(Debug)]
pub struct RegExpTableEntry {
    pub offset: u32,
    pub length: u32,
}

impl SmallFunctionHeader {
    fn _size(&self) -> usize {
        1337
    }

    fn _deserialize<R>(_r: &mut R) -> Self
    where
        R: io::Read + io::BufRead,
    {
        return SmallFunctionHeader {
            offset: 0,
            param_count: 0,
            byte_size: 0,
            func_name: 0,
            info_offset: 0,
            frame_size: 0,
            env_size: 0,
            highest_read_cache_index: 0,
            highest_write_cache_index: 0,
            flags: FunctionHeaderFlag {
                prohibit_invoke: FunctionHeaderFlagProhibitions::ProhibitNone,
                strict_mode: false,
                has_exception_handler: false,
                has_debug_info: false,
                overflowed: false,
            },
        };
    }

    fn _serialize<W>(&self, _w: &mut W)
    where
        W: io::Write,
    {
        // encode_u32(w, self.string_data_off);
    }
}

#[derive(Debug)]
pub enum FunctionHeaderFlagProhibitions {
    ProhibitCall = 0,
    ProhibitConstruct = 1,
    ProhibitNone = 2,
}

#[derive(Debug)]
pub struct FunctionHeaderFlag {
    pub prohibit_invoke: FunctionHeaderFlagProhibitions, // 2
    pub strict_mode: bool,                               // 1
    pub has_exception_handler: bool,                     // 1
    pub has_debug_info: bool,                            // 1
    pub overflowed: bool,                                // 1
}

#[derive(Debug)]
pub struct ExceptionHandlerInfo {
    pub start: u32,
    pub end: u32,
    pub target: u32,
}

#[derive(Debug)]
pub struct StringKindEntry {
    pub count: u32,
    pub kind: StringKind,
}

#[derive(Debug)]
pub enum StringKind {
    String,
    Identifier,
    Predefined, // unused < 0.3.0, is now "Identifier"
}

#[derive(Debug)]
pub struct SmallStringTableEntry {
    pub is_utf_16: bool,
    pub offset: u32,
    pub length: u32,
}

#[derive(Debug)]
pub struct OverflowStringTableEntry {
    pub offset: u32,
    pub length: u32,
}

impl HermesStruct for HermesHeader {
    fn size(&self) -> usize {
        1337
    }

    fn deserialize<R>(r: &mut R) -> Self
    where
        R: io::Read + io::BufRead + io::Seek,
    {
        let magic: u64 = decode_u64(r);
        println!("magic: {:?}", magic);

        let version = decode_u32(r);
        let mut sha1_bytes = [0u8; 20];
        r.read_exact(&mut sha1_bytes)
            .expect("Could not read sha1 bytes");
        let sha1 = sha1_bytes;
        let file_length = decode_u32(r);
        let global_code_index = decode_u32(r);
        let function_count = decode_u32(r);
        let string_kind_count = decode_u32(r);
        let identifier_count = decode_u32(r);
        let string_count = decode_u32(r);
        let overflow_string_count = decode_u32(r);
        let string_storage_size = decode_u32(r);
        let big_int_count = decode_u32(r);
        let big_int_storage_size = decode_u32(r);
        let reg_exp_count = decode_u32(r);
        let reg_exp_storage_size = decode_u32(r);
        let array_buffer_size = decode_u32(r);
        let obj_key_buffer_size = decode_u32(r);
        let obj_value_buffer_size = decode_u32(r);
        let segment_id = decode_u32(r);
        let cjs_module_count = decode_u32(r);
        let function_source_count = decode_u32(r);
        let debug_info_offset = decode_u32(r);

        let options_u8: u8 = decode_u8(r);
        let static_builtins: bool = options_u8 >> 1 & 1 == 1;
        let cjs_modules_statically_resolved: bool = options_u8 >> 1 & 1 == 1;
        let has_async: bool = options_u8 >> 1 & 1 == 1;
        let flags: bool = options_u8 >> 1 & 1 == 1;

        let options: BytecodeOptions = BytecodeOptions {
            static_builtins: static_builtins,
            cjs_modules_statically_resolved: cjs_modules_statically_resolved,
            has_async: has_async,
            flags: flags,
        };

        {
            // Read padding bytes
            let mut _pad_bytes = [0u8; 19];
            let _pad = r
                .read_exact(&mut _pad_bytes)
                .expect("Error reading padding bytes");
        }

        // Read the function headers
        let mut function_headers: Vec<SmallFunctionHeader> = vec![];
        for _ in 0..function_count {
            let mut func_header_bytes = [0u8; 16];
            r.read_exact(&mut func_header_bytes)
                .expect("unable to read first word");
            // dbg_hex!("{:?}", &func_header_bytes);

            let offset = read_bitfield(&func_header_bytes, 0, 25);
            let param_count = read_bitfield(&func_header_bytes, 25, 7);
            let byte_size = read_bitfield(&func_header_bytes, 32, 15);
            let func_name = read_bitfield(&func_header_bytes, 47, 17);
            let info_offset = read_bitfield(&func_header_bytes, 64, 25);
            let frame_size = read_bitfield(&func_header_bytes, 89, 7);
            let env_size = read_bitfield(&func_header_bytes, 96, 8);
            let highest_read_cache_index = read_bitfield(&func_header_bytes, 104, 8);
            let highest_write_cache_index = read_bitfield(&func_header_bytes, 112, 8);

            // last byte for flags
            let prohibit_invoke = read_bitfield(&func_header_bytes, 120, 2);
            let strict_mode = read_bitfield(&func_header_bytes, 122, 1);
            let has_exception_handler = read_bitfield(&func_header_bytes, 123, 1);
            let has_debug_info = read_bitfield(&func_header_bytes, 124, 1);
            let overflowed = read_bitfield(&func_header_bytes, 125, 1);

            let flags = FunctionHeaderFlag {
                prohibit_invoke: match prohibit_invoke as u8 {
                    0 => FunctionHeaderFlagProhibitions::ProhibitCall,
                    1 => FunctionHeaderFlagProhibitions::ProhibitConstruct,
                    2 => FunctionHeaderFlagProhibitions::ProhibitNone,
                    _ => {
                        panic!("Unknown prohibit invoke on small function header");
                    }
                },
                strict_mode: strict_mode == 1,
                has_exception_handler: has_exception_handler == 1,
                has_debug_info: has_debug_info == 1,
                overflowed: overflowed == 1,
            };

            if flags.has_exception_handler {
                println!("Function has exception handler");
                
                // temporarily go to the info offset but come back
                let current_pos = r.seek(io::SeekFrom::Current(0)).unwrap();
                r.seek(io::SeekFrom::Start(info_offset as u64))
                    .expect("unable to seek to exception handler info");

                // Align to 4 bytes
                // let mut _pad_bytes = [0u8; 4];
                // r.read_exact(&mut _pad_bytes)
                //     .expect("unable to read exception handler padding");
                
                let exc_headers_count = decode_u32(r);
                println!("Exception handler count: {:?}", exc_headers_count);

                let mut exception_handlers = vec![];
                for _ in 0..exc_headers_count {
                    let mut exception_handler_bytes = [0u8; 8];
                    r.read_exact(&mut exception_handler_bytes)
                        .expect("unable to read exception handler bytes");
                    let start = decode_u32(&mut &exception_handler_bytes[0..4]);
                    let end = decode_u32(&mut &exception_handler_bytes[4..8]);
                    let target = decode_u32(r);
                    let exception_handler = ExceptionHandlerInfo {
                        start: start,
                        end: end,
                        target: target,
                    };
                    println!("Exception handler: {:?}", exception_handler);
                    exception_handlers.push(exception_handler);
                }

  
                // go back to where we were
                r.seek(io::SeekFrom::Start(current_pos))
                    .expect("unable to seek back to original position");
            }

            if flags.has_debug_info {
                println!("Function has debug info");
                // go to the debug_info_offset
                let current_pos = r.seek(io::SeekFrom::Current(0)).unwrap();
                r.seek(io::SeekFrom::Start(debug_info_offset as u64)).expect("unable to seek to debug info");
                
                let debug_info = DebugInfoOffsets {
                    src: decode_u32(r),
                    scope_desc: decode_u32(r),
                    callee: decode_u32(r),
                };

                println!("Debug info ({}): {:?}", func_name, debug_info);

                // go back to where we were
                r.seek(io::SeekFrom::Start(current_pos)).expect("unable to seek back to original position");
                // DebugInfo
            }

            let sfh: SmallFunctionHeader = SmallFunctionHeader {
                offset: offset,
                param_count: param_count,
                byte_size: byte_size,
                func_name: func_name,
                info_offset: info_offset,
                frame_size: frame_size,
                env_size: env_size,
                highest_read_cache_index: highest_read_cache_index,
                highest_write_cache_index: highest_write_cache_index,
                flags: flags,
            };
            // println!("sfh byte size: {:?}", sfh.byte_size);
            function_headers.push(sfh);
        }

        // Read string kinds
        let mut string_kinds: Vec<StringKindEntry> = vec![];
        for _string_kind_idx in 0..string_kind_count {
            // println!("iterating over string kind count");
            // read 4 bytes
            let mut string_kind_bytes: [u8; 4] = [0u8; 4];
            r.read_exact(&mut string_kind_bytes)
                .expect("unable to read string kind bytes");
            // dbg_hex!("{:?}", &string_kind_bytes);
            let count = read_bitfield(&string_kind_bytes, 0, 31);
            let kind = read_bitfield(&string_kind_bytes, 31, 1);

            let string_kind = StringKindEntry {
                kind: match kind {
                    0 => StringKind::String,
                    1 => StringKind::Identifier,
                    2 => StringKind::Predefined,
                    _ => {
                        panic!("Unknown string kind");
                    }
                },
                count: count,
            };

            // println!("String kind {}: {:?}", string_kind_idx, &string_kind);
            string_kinds.push(string_kind);
        }

        // Read identifier hashes
        let mut identifier_hashes: Vec<u32> = vec![];
        for _ in 0..identifier_count {
            identifier_hashes.push(decode_u32(r));
        }

        // Read small string table entry
        let mut string_storage: Vec<SmallStringTableEntry> = vec![];
        for _ in 0..string_count {
            // println!("iterating over string table entries now");
            let mut string_storage_bytes = [0u8; 4];
            r.read_exact(&mut string_storage_bytes)
                .expect("unable to read string storage bytes");
            let is_utf_16 = read_bitfield(&string_storage_bytes, 0, 1);
            let offset = read_bitfield(&string_storage_bytes, 1, 23);
            let length = read_bitfield(&string_storage_bytes, 24, 8);
            let string_item = SmallStringTableEntry {
                is_utf_16: is_utf_16 == 1,
                offset: offset,
                length: length,
            };

            // dbg_hex!("{:?}", &string_item);
            string_storage.push(string_item);
        }

        let mut overflow_string_storage: Vec<OverflowStringTableEntry> = vec![];
        for _ in 0..overflow_string_count {
            // println!("iterating over overflow string table entries now");
            let overflow_string_item = OverflowStringTableEntry {
                offset: decode_u32(r),
                length: decode_u32(r),
            };
            // dbg_hex!("{:?}", &overflow_string_item);
            overflow_string_storage.push(overflow_string_item);
        }

        let mut string_storage_bytes_real = vec![0u8; string_storage_size as usize];
        r.read_exact(&mut string_storage_bytes_real)
            .expect("unable to read string storage");
        let string_storage_as_str = String::from_utf8(string_storage_bytes_real.clone()).unwrap();
        println!("string_storage: {:?}", string_storage_as_str);

        let mut array_buffer_storage = vec![0u8; array_buffer_size as usize];
        r.read_exact(&mut array_buffer_storage)
            .expect("unable to read array buffer storage");

        let mut object_key_buffer = vec![0u8; obj_key_buffer_size as usize];
        r.read_exact(&mut object_key_buffer)
            .expect("unable to read object key buffer storage");

        let mut object_val_buffer = vec![0u8; obj_value_buffer_size as usize];
        r.read_exact(&mut object_val_buffer)
            .expect("unable to read object value buffer storage");

        // TODO: Add debug info stuff
        // https://github.com/facebook/hermes/blob/main/lib/BCGen/HBC/DebugInfo.cpp#L181
        println!("Hermes version: {:?}", version);
        return Self {
            magic: magic,
            version: version,
            sha1: sha1,
            file_length: file_length,
            global_code_index: global_code_index,
            function_count: function_count,
            string_kind_count: string_kind_count,
            identifier_count: identifier_count,
            string_count: string_count,
            overflow_string_count: overflow_string_count,
            string_storage_size: string_storage_size,
            big_int_count: big_int_count,
            big_int_storage_size: big_int_storage_size,
            reg_exp_count: reg_exp_count,
            reg_exp_storage_size: reg_exp_storage_size,
            array_buffer_size: array_buffer_size,
            obj_key_buffer_size: obj_key_buffer_size,
            obj_value_buffer_size: obj_value_buffer_size,
            segment_id: segment_id,
            cjs_module_count: cjs_module_count,
            function_source_count: function_source_count,
            debug_info_offset: debug_info_offset,
            options: options,
            function_headers: function_headers,
            string_kinds: string_kinds,
            string_storage: string_storage,
            string_storage_bytes: string_storage_bytes_real,
            overflow_string_storage: overflow_string_storage,
        };
    }

    fn parse_bytecode<R: io::Read + io::BufRead + io::Seek>(&self, r: &mut R){
        // Function body goes here
        {
            for fh in &self.function_headers {
                // println!("function header: {:?}", fh);
                r.seek(io::SeekFrom::Start(fh.offset as u64)).unwrap();
                let mut bytecode_buf = vec![0u8; fh.byte_size as usize];
                r.read_exact(&mut bytecode_buf).expect("unable to read first functions bytecode");

                let myfunc = self.string_storage.get(fh.func_name as usize).unwrap();
                println!("------------------------------------------------");
                let func_start = myfunc.offset;
                let func_name = String::from_utf8(self.string_storage_bytes[func_start as usize..(func_start + myfunc.length) as usize].to_vec()).unwrap();
                println!(
                    "Function<{}>({:?} params, {:?} registers, {:?} symbols):",
                    func_name, fh.param_count, fh.frame_size, fh.env_size
                );

                println!("bytecode as hex: {:?}", bytecode_buf);

                #[allow(unused_mut)]
                let mut instructions_list = vec![];

                let mut byte_iter = bytecode_buf.iter();
                // Iterate over bytecode_buf and parse the instructions
                while let Some(&op_byte) = byte_iter.next() {
                    #[allow(unused_variables)]
                    let op = op_byte.clone();
                    // Make a new Cursor to print the remaining bytes
                    #[allow(unused_mut, unused_variables)]
                    let mut r_cursor = io::Cursor::new(byte_iter.as_slice());

                    // Deserialize the instruction
                    let ins_obj: Option<Instruction> = match self.version {
                      #[cfg(feature = "v89")]
                      // 89 => Some(Instruction::V89(hermes::v89::Instruction::deserialize(&mut r_cursor, op))),
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


                    // let ins: Instruction = ins_obj.unwrap();
                    if let Some(ins) = ins_obj {
                      println!("\t{}", ins.display(self));
                      let size = ins.size();
                      instructions_list.push(ins);
                      for _ in 0..size {
                        if byte_iter.next().is_none() {
                          break;
                        }
                      }
                    }
    
                }
                
                if func_name == "foo" {

                  // Iterate over the instructions_list
                  for ins in instructions_list {
                      println!("{:?}", ins);
  
                  }
                }
            }
        }
    }

    fn serialize<W>(&self, _w: &mut W)
    where
        W: io::Write,
    {
        todo!()
    }

    // Read string
    // Read function
}

