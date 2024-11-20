pub mod big_int_table;
pub mod bytecode_options;
pub mod cjs_module;
pub mod debug_info;
pub mod decode;
pub mod encode;
pub mod exception_handler;
pub mod function_header;
pub mod function_sources;
pub mod header;
pub mod hermes_file;
pub mod jenkins;
pub mod regexp_table;
pub mod string_kind;
pub mod string_table;
pub mod types;

use std::io;

pub use function_header::{
    FunctionHeaderFlag, FunctionHeaderFlagProhibitions, SmallFunctionHeader,
};
pub use header::{HermesHeader, HermesStructReader};
pub use hermes_file::HermesFile;
pub use string_kind::{StringKind, StringKindEntry};
pub use string_table::{OverflowStringTableEntry, SmallStringTableEntry};

// pub type Reg8 = u8;
/*

#[derive(Debug, Copy, Clone)]
pub struct Reg8(u8);
impl Default for Reg8 { fn default() -> Self { Reg8(0) } }

#[derive(Debug, Copy, Clone)]
pub struct Reg32(u32);
impl Default for Reg32 { fn default() -> Self { Reg32(0) } }

#[derive(Debug, Copy, Clone)]
pub struct UInt8(u8);
impl Default for UInt8 { fn default() -> Self { UInt8(0) } }

#[derive(Debug, Copy, Clone)]
pub struct UInt16(u16);
impl Default for UInt16 { fn default() -> Self { UInt16(0) } }

#[derive(Debug, Copy, Clone)]
pub struct UInt32(u32);
impl Default for UInt32 { fn default() -> Self { UInt32(0) } }

#[derive(Debug, Copy, Clone)]
pub struct Addr8(i8);
impl Default for Addr8 { fn default() -> Self { Addr8(0) } }

#[derive(Debug, Copy, Clone)]
pub struct Addr32(i32);
impl Default for Addr32 { fn default() -> Self { Addr32(0) } }

#[derive(Debug, Copy, Clone)]
pub struct Imm32(i32);
impl Default for Imm32 { fn default() -> Self { Imm32(0) } }

#[derive(Debug, Copy, Clone)]
pub struct Double(f64);
impl Default for Double { fn default() -> Self { Double(0.0) } }

// Index types - the values reference the values at {index} in StringTable, FunctionTable, and BigIntTable
#[derive(Debug, Copy, Clone)]
pub struct StringIDUInt8(u8);
impl Default for StringIDUInt8 { fn default() -> Self { StringIDUInt8(0) } }

#[derive(Debug, Copy, Clone)]
pub struct StringIDUInt16(u16);
impl Default for StringIDUInt16 { fn default() -> Self { StringIDUInt16(0) } }

#[derive(Debug, Copy, Clone)]
pub struct StringIDUInt32(u32);
impl Default for StringIDUInt32 { fn default() -> Self { StringIDUInt32(0) } }

#[derive(Debug, Copy, Clone)]
pub struct FunctionIDUInt8(u8);
impl Default for FunctionIDUInt8 { fn default() -> Self { FunctionIDUInt8(0) } }

#[derive(Debug, Copy, Clone)]
pub struct FunctionIDUInt16(u16);
impl Default for FunctionIDUInt16 { fn default() -> Self { FunctionIDUInt16(0) } }

#[derive(Debug, Copy, Clone)]
pub struct FunctionIDUInt32(u32);
impl Default for FunctionIDUInt32 { fn default() -> Self { FunctionIDUInt32(0) } }

#[derive(Debug, Copy, Clone)]
pub struct BigIntIDUInt16(u16);
impl Default for BigIntIDUInt16 { fn default() -> Self { BigIntIDUInt16(0) } }

#[derive(Debug, Copy, Clone)]
pub struct BigIntIDUInt32(u32);
impl Default for BigIntIDUInt32 { fn default() -> Self { BigIntIDUInt32(0) } }

impl From<u8> for Reg8 {
  fn from(value: u8) -> Self {
      Reg8(value)
  }
}

impl From<Reg8> for u8 {
  fn from(value: Reg8) -> Self {
      value.0
  }
}

impl From<u16> for UInt16 {
  fn from(value: u16) -> Self {
      UInt16(value)
  }
}

impl From<UInt16> for u16 {
  fn from(value: UInt16) -> Self {
      value.0
  }
}

impl From<u32> for UInt32 {
  fn from(value: u32) -> Self {
      UInt32(value)
  }
}

impl From<UInt32> for u32 {
  fn from(value: UInt32) -> Self {
      value.0
  }
}

impl From<i8> for Addr8 {
  fn from(value: i8) -> Self {
      Addr8(value)
  }
}

impl From<Addr8> for i8 {
  fn from(value: Addr8) -> Self {
      value.0
  }
}

impl From<i32> for Addr32 {
  fn from(value: i32) -> Self {
      Addr32(value)
  }
}

impl From<Addr32> for i32 {
  fn from(value: Addr32) -> Self {
      value.0
  }
}

impl From<i32> for Imm32 {
  fn from(value: i32) -> Self {
      Imm32(value)
  }
}

impl From<Imm32> for i32 {
  fn from(value: Imm32) -> Self {
      value.0
  }
}

impl From<f64> for Double {
  fn from(value: f64) -> Self {
      Double(value)
  }
}

impl From<Double> for f64 {
  fn from(value: Double) -> Self {
      value.0
  }
}
*/
// -- end hermes types

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

    fn get_string_field_names(&self) -> Vec<&str>;

    fn get_function_field_names(&self) -> Vec<&str>;

    fn display<R>(&self, hermes: &HermesFile<R>) -> String
    where
        R: io::Read + io::BufRead + io::Seek;

    /// Size of the struct when encoded.
    fn size(&self) -> usize;

    fn has_ret_target(&self) -> bool {
        false
    }

    fn is_jmp(&self) -> bool;

    fn get_address_field(&self) -> u32;
}

// Start macros
#[macro_export]
macro_rules! map_type {
    (Reg8) => {
        u8
    };
    (Reg32) => {
        u32
    };
    (UInt8) => {
        u8
    };
    (UInt16) => {
        u16
    };
    (UInt32) => {
        u32
    };
    (Addr8) => {
        i8
    };
    (Addr32) => {
        i32
    };
    (Imm32) => {
        i32
    };
    (Double) => {
        f64
    };
    (StringIDUInt8) => {
        u8
    };
    (StringIDUInt16) => {
        u16
    };
    (StringIDUInt32) => {
        u32
    };
    (FunctionIDUInt8) => {
        u8
    };
    (FunctionIDUInt16) => {
        u16
    };
    (FunctionIDUInt32) => {
        u32
    };
    (BigIntIDUInt16) => {
        u16
    };
    (BigIntIDUInt32) => {
        u32
    };
}

#[macro_export]
macro_rules! map_size {
    (Reg8) => {
        1
    };
    (Reg32) => {
        4
    };
    (UInt8) => {
        1
    };
    (UInt16) => {
        2
    };
    (UInt32) => {
        4
    };
    (Addr8) => {
        1
    };
    (Addr32) => {
        4
    };
    (Imm32) => {
        4
    };
    (Double) => {
        8
    };
    (StringIDUInt8) => {
        1
    };
    (StringIDUInt16) => {
        2
    };
    (StringIDUInt32) => {
        4
    };
    (FunctionIDUInt8) => {
        1
    };
    (FunctionIDUInt16) => {
        2
    };
    (FunctionIDUInt32) => {
        4
    };
    (BigIntIDUInt16) => {
        2
    };
    (BigIntIDUInt32) => {
        4
    };
}

#[macro_export]
macro_rules! map_decode_fn {
    (Reg8) => {
        hermes::decode::decode_u8
    };
    (Reg32) => {
        hermes::decode::decode_u32
    };
    (UInt8) => {
        hermes::decode::decode_u8
    };
    (UInt16) => {
        hermes::decode::decode_u16
    };
    (UInt32) => {
        hermes::decode::decode_u32
    };
    (Addr8) => {
        hermes::decode::decode_i8
    };
    (Addr32) => {
        hermes::decode::decode_i32
    };
    (Imm32) => {
        hermes::decode::decode_i32
    };
    (Double) => {
        hermes::decode::decode_f64
    };
    (StringIDUInt8) => {
        hermes::decode::decode_u8
    };
    (StringIDUInt16) => {
        hermes::decode::decode_u16
    };
    (StringIDUInt32) => {
        hermes::decode::decode_u32
    };
    (FunctionIDUInt8) => {
        hermes::decode::decode_u8
    };
    (FunctionIDUInt16) => {
        hermes::decode::decode_u16
    };
    (FunctionIDUInt32) => {
        hermes::decode::decode_u32
    };
    (BigIntIDUInt16) => {
        hermes::decode::decode_u16
    };
    (BigIntIDUInt32) => {
        hermes::decode::decode_u32
    };
}

#[macro_export]
macro_rules! map_encode_fn {
    (Reg8) => {
        hermes::encode::encode_u8
    };
    (Reg32) => {
        hermes::encode::encode_u32
    };
    (UInt8) => {
        hermes::encode::encode_u8
    };
    (UInt16) => {
        hermes::encode::encode_u16
    };
    (UInt32) => {
        hermes::encode::encode_u32
    };
    (Addr8) => {
        hermes::encode::encode_i8
    };
    (Addr32) => {
        hermes::encode::encode_i32
    };
    (Imm32) => {
        hermes::encode::encode_i32
    };
    (Double) => {
        hermes::encode::encode_f64
    };
    (StringIDUInt8) => {
        hermes::encode::encode_u8
    };
    (StringIDUInt16) => {
        hermes::encode::encode_u16
    };
    (StringIDUInt32) => {
        hermes::encode::encode_u32
    };
    (FunctionIDUInt8) => {
        hermes::encode::encode_u8
    };
    (FunctionIDUInt16) => {
        hermes::encode::encode_u16
    };
    (FunctionIDUInt32) => {
        hermes::encode::encode_u32
    };
    (BigIntIDUInt16) => {
        hermes::encode::encode_u16
    };
    (BigIntIDUInt32) => {
        hermes::encode::encode_u32
    };
}

#[macro_export]
macro_rules! get_field {
    ($struct:expr, $field:ident) => {
        $struct.$field
    };
}

#[macro_export]
macro_rules! set_field {
    ($struct:expr, $field:ident, $value:expr) => {
        $struct.$field = $value;
    };
}

#[macro_export]
macro_rules! define_opcode {
($name:ident, $($field:ident : $arg:tt),*) => {

  #[cfg_attr(feature = "specta", derive(specta::Type))]
  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug, Copy, Clone)]
  pub struct $name {
    pub op: u8,
    $(
      // pub $field: map_type!($arg),
      pub $field: $arg,
    )*
  }

  // impl Default for $name {
  //   fn default() -> Self {
  //     Self {
  //         op: 0,
  //         $(
  //           // $field: match stringify!(map_type!($arg)) {
  //             // "Double" => 0.0 as _,
  //             // _ => 0 as $arg,
  //           // },
  //         )*
  //     }
  //   }
  // }
  impl Default for $name {
    fn default() -> Self {
        Self {
            op: 0,
            $(
                $field: Default::default(),
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
        op,
        $(
          // $field: map_decode_fn!($arg)(_r),
          $field: map_decode_fn!($arg)(_r).into(),
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

    fn is_jmp(&self) -> bool {
      match op_to_instr(self.op) {
        Instruction::Jmp(_) |
        Instruction::JmpLong(_) |
        Instruction::JmpTrue(_) |
        Instruction::JmpTrueLong(_) |
        Instruction::JmpFalse(_) |
        Instruction::JmpFalseLong(_) |
        Instruction::JmpUndefined(_) |
        Instruction::JmpUndefinedLong(_) |
        Instruction::SaveGenerator(_) |
        Instruction::SaveGeneratorLong(_) |
        Instruction::JLess(_) |
        Instruction::JLessLong(_) |
        Instruction::JNotLess(_) |
        Instruction::JNotLessLong(_) |
        Instruction::JLessN(_) |
        Instruction::JLessNLong(_) |
        Instruction::JNotLessN(_) |
        Instruction::JNotLessNLong(_) |
        Instruction::JLessEqual(_) |
        Instruction::JLessEqualLong(_) |
        Instruction::JNotLessEqual(_) |
        Instruction::JNotLessEqualLong(_) |
        Instruction::JLessEqualN(_) |
        Instruction::JLessEqualNLong(_) |
        Instruction::JNotLessEqualN(_) |
        Instruction::JNotLessEqualNLong(_) |
        Instruction::JGreater(_) |
        Instruction::JGreaterLong(_) |
        Instruction::JNotGreater(_) |
        Instruction::JNotGreaterLong(_) |
        Instruction::JGreaterN(_) |
        Instruction::JGreaterNLong(_) |
        Instruction::JNotGreaterN(_) |
        Instruction::JNotGreaterNLong(_) |
        Instruction::JGreaterEqual(_) |
        Instruction::JGreaterEqualLong(_) |
        Instruction::JNotGreaterEqual(_) |
        Instruction::JNotGreaterEqualLong(_) |
        Instruction::JGreaterEqualN(_) |
        Instruction::JGreaterEqualNLong(_) |
        Instruction::JNotGreaterEqualN(_) |
        Instruction::JNotGreaterEqualNLong(_) |
        Instruction::JEqual(_) |
        Instruction::JEqualLong(_) |
        Instruction::JNotEqual(_) |
        Instruction::JNotEqualLong(_) |
        Instruction::JStrictEqual(_) |
        Instruction::JStrictEqualLong(_) |
        Instruction::JStrictNotEqual(_) |
        Instruction::JStrictNotEqualLong(_) => true,

        // This is a special case for the switch instruction
        // p1 = Addr32, so we can assume it's an address
        Instruction::SwitchImm(_) => true,
        _ => false,
      }
    }

    fn serialize<W>(&self, _w: &mut W)
      where
          W: std::io::Write,
      {
        hermes::encode::encode_u8(_w, self.op);
        $(
          // map_encode_fn!($arg)(_w, self.$field as _);
          map_encode_fn!($arg)(_w, self.$field.into());
        )*
      }

    #[allow(unused_mut)]
    fn size(&self) -> usize {
      let mut total_size = 0;
      total_size += 1; // opcode
      $(
        total_size += map_size!($arg);
      )*
      total_size
    }

    #[allow(unused_mut, unused_assignments)]
    fn get_address_field(&self) -> u32 {
      let mut val: u32 = 0;

      $(
        val = match stringify!($arg) {
          // "Addr8" | "Addr32" => self.$field as u32,
          "Addr8" | "Addr32" => self.$field.into(),
          _ => 0
        };
      )*
      val as u32

    }

    fn get_string_field_names(&self) -> Vec<&str> {
      #[allow(unused_mut)]
      let mut fields = vec![];
      $(
        if stringify!($arg).contains("StringID") {
          fields.push(stringify!($field));
        }
      )*
      fields
    }

    fn get_function_field_names(&self) -> Vec<&str> {
      #[allow(unused_mut)]
      let mut fields = vec![];
      $(
        if stringify!($arg).contains("FunctionID") {
          fields.push(stringify!($field));
        }
      )*
      fields
    }

    #[allow(unused_mut)]
    fn display<R>(&self, _hermes: &hermes::hermes_file::HermesFile<R>) -> String
    where R: io::Read + io::BufRead + io::Seek {

      let mut display_string = format!("{} ", op_to_str(self.op));
      $(
          display_string = match stringify!($arg) {
            "StringIDUInt8" | "StringIDUInt16" | "StringIDUInt32" => format!("{} \"{}\"", display_string, _hermes.get_string_from_storage_by_index(self.$field.into())),
            "FunctionIDUInt8" | "FunctionIDUInt16" | "FunctionIDUInt32" => {
              let index: usize = Into::<usize>::into(self.$field);
              let target_function = _hermes.function_headers[index].func_name();
              let func_str = _hermes
                  .get_string_from_storage_by_index(target_function as usize)
                  .to_string();

              // functions don't always have names, so in that case we'll return the function index
              if func_str.is_empty() {
                  format!("{} Function<$FUNC_{}>", display_string, self.$field)
              } else {
                  format!("{} Function<{}>", display_string, func_str)
              }
            },
            "BigIntIDUInt16" | "BigIntIDUInt32" => {
              // TODO: Make this work
              // let bigint_addr = self.$field.into() as usize;
              let bigint_addr: usize = Into::<usize>::into(self.$field);
              format!("{} {} ", display_string, bigint_addr)
            },
            _ => {
              let tmp = match stringify!($arg) {
                "Reg8" | "Reg32" => {
                  format!("{} r{},", display_string, self.$field )
                },
                "Addr8" | "Addr32" => {
                  format!("{} {},", display_string, self.$field )
                }
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

      fn display<R>(&self, _hermes: &hermes::hermes_file::HermesFile<R>) -> String
      where
      R: io::Read + io::BufRead + io::Seek
      {
        match self {
          $(
            Instruction::$insn(insn) => insn.display(_hermes),
          )*
        }
      }

      fn get_string_field_names(&self) -> Vec<&str> {
        match self {
          $(
            Instruction::$insn(insn) => insn.get_string_field_names(),
          )*
        }
      }

      fn get_function_field_names(&self) -> Vec<&str> {
        match self {
          $(
            Instruction::$insn(insn) => insn.get_string_field_names(),
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

      fn is_jmp(&self) -> bool {
        match self {
          $(
            Instruction::$insn(insn) => insn.is_jmp(),
          )*
        }
      }

      fn get_address_field(&self) -> u32 {
        match self {
          $(
            Instruction::$insn(insn) => insn.get_address_field(),
          )*
        }
      }
  }
}
}

#[macro_export]
macro_rules! build_instructions {
($(($opcode:expr, $instruction:ident, $($operand:ident : $type:ident),*)),*) => {
    use std::io;
    #[allow(unused_imports)]
    use hermes::types::{Reg8, Reg32, UInt8, UInt16, UInt32, Addr8, Addr32, Imm32, Double, StringIDUInt8, StringIDUInt16, StringIDUInt32, FunctionIDUInt8, FunctionIDUInt16, FunctionIDUInt32, BigIntIDUInt16, BigIntIDUInt32};

    $(define_opcode!($instruction, $($operand : $type),*);)*

    #[cfg_attr(feature = "specta", derive(specta::Type))]
    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    #[derive(Debug, Copy)]
    #[repr(u8)]
    pub enum Instruction {
      $(
        $instruction($instruction),
      )*
    }

    impl Clone for Instruction {
      fn clone(&self) -> Self {
        *self
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
      #[allow(clippy::needless_update)]
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

// End macro definitions
#[cfg(feature = "v84")]
#[macro_use]
pub mod v84;

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

#[macro_use]
#[cfg(feature = "v96")]
pub mod v96;

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Copy, Clone)]
pub enum HermesInstruction {
    #[cfg(feature = "v84")]
    V84(v84::Instruction),
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
    #[cfg(feature = "v96")]
    V96(v96::Instruction),
}

impl HermesInstruction {
    pub fn display<R>(&self, _hermes: &HermesFile<R>) -> String
    where
        R: io::Read + io::BufRead + io::Seek,
    {
        match self {
            #[cfg(feature = "v84")]
            HermesInstruction::V84(instruction) => instruction.display(_hermes),
            #[cfg(feature = "v89")]
            HermesInstruction::V89(instruction) => instruction.display(_hermes),
            #[cfg(feature = "v90")]
            HermesInstruction::V90(instruction) => instruction.display(_hermes),
            #[cfg(feature = "v93")]
            HermesInstruction::V93(instruction) => instruction.display(_hermes),
            #[cfg(feature = "v94")]
            HermesInstruction::V94(instruction) => instruction.display(_hermes),
            #[cfg(feature = "v95")]
            HermesInstruction::V95(instruction) => instruction.display(_hermes),
            #[cfg(feature = "v96")]
            HermesInstruction::V96(instruction) => instruction.display(_hermes),
        }
    }

    pub fn is_jmp(&self) -> bool {
        match self {
            #[cfg(feature = "v84")]
            HermesInstruction::V84(instruction) => instruction.is_jmp(),
            #[cfg(feature = "v89")]
            HermesInstruction::V89(instruction) => instruction.is_jmp(),
            #[cfg(feature = "v90")]
            HermesInstruction::V90(instruction) => instruction.is_jmp(),
            #[cfg(feature = "v93")]
            HermesInstruction::V93(instruction) => instruction.is_jmp(),
            #[cfg(feature = "v94")]
            HermesInstruction::V94(instruction) => instruction.is_jmp(),
            #[cfg(feature = "v95")]
            HermesInstruction::V95(instruction) => instruction.is_jmp(),
            #[cfg(feature = "v96")]
            HermesInstruction::V96(instruction) => instruction.is_jmp(),
        }
    }

    pub fn get_address_field(&self) -> u32 {
        match self {
            #[cfg(feature = "v84")]
            HermesInstruction::V84(instruction) => instruction.get_address_field(),
            #[cfg(feature = "v89")]
            HermesInstruction::V89(instruction) => instruction.get_address_field(),
            #[cfg(feature = "v90")]
            HermesInstruction::V90(instruction) => instruction.get_address_field(),
            #[cfg(feature = "v93")]
            HermesInstruction::V93(instruction) => instruction.get_address_field(),
            #[cfg(feature = "v94")]
            HermesInstruction::V94(instruction) => instruction.get_address_field(),
            #[cfg(feature = "v95")]
            HermesInstruction::V95(instruction) => instruction.get_address_field(),
            #[cfg(feature = "v96")]
            HermesInstruction::V96(instruction) => instruction.get_address_field(),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            #[cfg(feature = "v84")]
            HermesInstruction::V84(instruction) => instruction.size(),
            #[cfg(feature = "v89")]
            HermesInstruction::V89(instruction) => instruction.size(),
            #[cfg(feature = "v90")]
            HermesInstruction::V90(instruction) => instruction.size(),
            #[cfg(feature = "v93")]
            HermesInstruction::V93(instruction) => instruction.size(),
            #[cfg(feature = "v94")]
            HermesInstruction::V94(instruction) => instruction.size(),
            #[cfg(feature = "v95")]
            HermesInstruction::V95(instruction) => instruction.size(),
            #[cfg(feature = "v96")]
            HermesInstruction::V96(instruction) => instruction.size(),
        }
    }

    pub fn serialize<W>(&self, w: &mut W)
    where
        W: std::io::Write,
    {
        match self {
            #[cfg(feature = "v84")]
            HermesInstruction::V84(instruction) => instruction.serialize(w),
            #[cfg(feature = "v89")]
            HermesInstruction::V89(instruction) => instruction.serialize(w),
            #[cfg(feature = "v90")]
            HermesInstruction::V90(instruction) => instruction.serialize(w),
            #[cfg(feature = "v93")]
            HermesInstruction::V93(instruction) => instruction.serialize(w),
            #[cfg(feature = "v94")]
            HermesInstruction::V94(instruction) => instruction.serialize(w),
            #[cfg(feature = "v95")]
            HermesInstruction::V95(instruction) => instruction.serialize(w),
            #[cfg(feature = "v96")]
            HermesInstruction::V96(instruction) => instruction.serialize(w),
        }
    }

    pub fn deserialize<R>(r: &mut R, op: u8) -> Self
    where
        R: io::Read + io::BufRead + io::Seek,
    {
        match op {
            #[cfg(feature = "v84")]
            89 => HermesInstruction::V84(v84::Instruction::deserialize(r, op)),
            #[cfg(feature = "v89")]
            89 => HermesInstruction::V89(v89::Instruction::deserialize(r, op)),
            #[cfg(feature = "v90")]
            90 => HermesInstruction::V90(v90::Instruction::deserialize(r, op)),
            #[cfg(feature = "v93")]
            93 => HermesInstruction::V93(v93::Instruction::deserialize(r, op)),
            #[cfg(feature = "v94")]
            94 => HermesInstruction::V94(v94::Instruction::deserialize(r, op)),
            #[cfg(feature = "v95")]
            95 => HermesInstruction::V95(v95::Instruction::deserialize(r, op)),
            #[cfg(feature = "v96")]
            96 => HermesInstruction::V96(v96::Instruction::deserialize(r, op)),
            _ => HermesInstruction::V96(v96::Instruction::deserialize(r, op)),
        }
    }
}

pub trait IntoParentInstruction {
    fn into_parent(self) -> HermesInstruction;
}

#[cfg(feature = "v84")]
impl IntoParentInstruction for v84::Instruction {
    fn into_parent(self) -> HermesInstruction {
        HermesInstruction::V84(self)
    }
}

#[cfg(feature = "v89")]
impl IntoParentInstruction for v89::Instruction {
    fn into_parent(self) -> HermesInstruction {
        HermesInstruction::V89(self)
    }
}

#[cfg(feature = "v90")]
impl IntoParentInstruction for v90::Instruction {
    fn into_parent(self) -> HermesInstruction {
        HermesInstruction::V90(self)
    }
}

#[cfg(feature = "v93")]
impl IntoParentInstruction for v93::Instruction {
    fn into_parent(self) -> HermesInstruction {
        HermesInstruction::V93(self)
    }
}

#[cfg(feature = "v94")]
impl IntoParentInstruction for v94::Instruction {
    fn into_parent(self) -> HermesInstruction {
        HermesInstruction::V94(self)
    }
}

#[cfg(feature = "v95")]
impl IntoParentInstruction for v95::Instruction {
    fn into_parent(self) -> HermesInstruction {
        HermesInstruction::V95(self)
    }
}

#[cfg(feature = "v96")]
impl IntoParentInstruction for v96::Instruction {
    fn into_parent(self) -> HermesInstruction {
        HermesInstruction::V96(self)
    }
}

pub trait Serializable {
    type Version;

    // deserialize from binary format into rust struct.
    // Note: _version is passed as there's some differences in
    // behavior depending on which version of hermes is used.
    fn deserialize<R>(r: &mut R, _version: u32) -> Self
    where
        R: io::Read + io::BufRead + io::Seek;

    fn serialize<W>(&self, w: &mut W)
    where
        W: std::io::Write + std::io::Seek;

    fn size(&self) -> usize;
}

#[macro_export]
/**
 * This macro is used to define a list of instructions for a given version of Hermes.
 * It will return a `Vec<Instruction>` that can be serialized into the bytecode section
 * of a Hermes executable.
```rust
let instructions = instructions!(
    hermes_rs::v96, // Version is required
    LoadConstString { r0: 0, p0: 1 },
    DirectEval {
        r0: 0,
        r1: 0,
        p0: 0
    },
    Ret { r0: 0 },
).unwrap();
```
 */
macro_rules! define_instructions {
  ($version:path, $($instr:ident { $($field:ident: $value:expr),* }),* $(,)?) => {
      {
          use $version::{str_to_op, Instruction};
          use $version::*;

          Some(vec![
            $(
                Instruction::$instr ($instr{
                    op: str_to_op(stringify!($instr)),
                    $($field: $value),*
                }),
            )*
        ])
      }
  };
}

#[macro_export]
macro_rules! define_instructions_alt {
  ($version:path, $($instr:ident { $($field:ident: $value:expr),* }),* $(,)?) => {
      {
          use $version::{str_to_op, Instruction};
          use $version::*;

          Some(vec![
              $(
                  Instruction::$instr ($instr{
                      op: str_to_op(stringify!($instr)),
                      $($field: $value),*
                  }),
              )*
          ])
      }
  };
}
