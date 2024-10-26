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
pub mod regexp_table;
pub mod string_kind;
pub mod string_table;

use std::io;

pub use function_header::{
    FunctionHeaderFlag, FunctionHeaderFlagProhibitions, SmallFunctionHeader,
};
pub use header::{HermesHeader, HermesStructReader};
pub use hermes_file::HermesFile;
pub use string_kind::{StringKind, StringKindEntry};
pub use string_table::{OverflowStringTableEntry, SmallStringTableEntry};

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
        op,
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

    #[allow(unused_mut, unused_assignments)]
    fn get_address_field(&self) -> u32 {
      let mut val = 0;
      $(
        val = match stringify!($arg) {
          "Addr8" | "Addr32" => self.$field as u32,
          _ => 0
        };
      )*
      val
    }

    #[allow(unused_mut)]
    fn display<R>(&self, _hermes: &hermes::hermes_file::HermesFile<R>) -> String
    where R: io::Read + io::BufRead + io::Seek {

      let mut display_string = format!("{} ", op_to_str(self.op));
      $(
          display_string = match stringify!($arg) {
            "StringIDUInt8" | "StringIDUInt16" | "StringIDUInt32" => format!("{} \"{}\"", display_string, _hermes.get_string_from_storage_by_index(self.$field as usize)),
            "FunctionIDUInt8" | "FunctionIDUInt16" | "FunctionIDUInt32" => {

              let target_function = _hermes.function_headers[self.$field as usize].func_name();
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
              let bigint_addr = self.$field as usize;
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
};
}

#[macro_export]
macro_rules! build_instructions {
($(($opcode:expr, $instruction:ident, $($operand:ident : $type:ident),*)),*) => {
  use std::io;
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
    #[cfg(feature = "v96")]
    V96(v96::Instruction),
}

impl Instruction {
    pub fn display<R>(&self, _hermes: &HermesFile<R>) -> String
    where
        R: io::Read + io::BufRead + io::Seek,
    {
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
            #[cfg(feature = "v96")]
            Instruction::V96(instruction) => instruction.display(_hermes),
        }
    }

    pub fn is_jmp(&self) -> bool {
        match self {
            #[cfg(feature = "v89")]
            Instruction::V89(instruction) => instruction.is_jmp(),
            #[cfg(feature = "v90")]
            Instruction::V90(instruction) => instruction.is_jmp(),
            #[cfg(feature = "v93")]
            Instruction::V93(instruction) => instruction.is_jmp(),
            #[cfg(feature = "v94")]
            Instruction::V94(instruction) => instruction.is_jmp(),
            #[cfg(feature = "v95")]
            Instruction::V95(instruction) => instruction.is_jmp(),
            #[cfg(feature = "v96")]
            Instruction::V96(instruction) => instruction.is_jmp(),
        }
    }

    pub fn get_address_field(&self) -> u32 {
        match self {
            #[cfg(feature = "v89")]
            Instruction::V89(instruction) => instruction.get_address_field(),
            #[cfg(feature = "v90")]
            Instruction::V90(instruction) => instruction.get_address_field(),
            #[cfg(feature = "v93")]
            Instruction::V93(instruction) => instruction.get_address_field(),
            #[cfg(feature = "v94")]
            Instruction::V94(instruction) => instruction.get_address_field(),
            #[cfg(feature = "v95")]
            Instruction::V95(instruction) => instruction.get_address_field(),
            #[cfg(feature = "v96")]
            Instruction::V96(instruction) => instruction.get_address_field(),
        }
    }

    pub fn size(&self) -> usize {
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
            #[cfg(feature = "v96")]
            Instruction::V96(instruction) => instruction.size(),
        }
    }

    pub fn serialize<W>(&self, w: &mut W)
    where
        W: std::io::Write,
    {
        match self {
            #[cfg(feature = "v89")]
            Instruction::V89(instruction) => instruction.serialize(w),
            #[cfg(feature = "v90")]
            Instruction::V90(instruction) => instruction.serialize(w),
            #[cfg(feature = "v93")]
            Instruction::V93(instruction) => instruction.serialize(w),
            #[cfg(feature = "v94")]
            Instruction::V94(instruction) => instruction.serialize(w),
            #[cfg(feature = "v95")]
            Instruction::V95(instruction) => instruction.serialize(w),
            #[cfg(feature = "v96")]
            Instruction::V96(instruction) => instruction.serialize(w),
        }
    }

    pub fn deserialize<R>(r: &mut R, op: u8) -> Self
    where
        R: io::Read + io::BufRead + io::Seek,
    {
        match op {
            #[cfg(feature = "v89")]
            89 => Instruction::V89(v89::Instruction::deserialize(r, op)),
            #[cfg(feature = "v90")]
            90 => Instruction::V90(v90::Instruction::deserialize(r, op)),
            #[cfg(feature = "v93")]
            93 => Instruction::V93(v93::Instruction::deserialize(r, op)),
            #[cfg(feature = "v94")]
            94 => Instruction::V94(v94::Instruction::deserialize(r, op)),
            #[cfg(feature = "v95")]
            95 => Instruction::V95(v95::Instruction::deserialize(r, op)),
            #[cfg(feature = "v96")]
            96 => Instruction::V96(v96::Instruction::deserialize(r, op)),
            _ => Instruction::V96(v96::Instruction::deserialize(r, op)),
        }
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
          use $version::{ $(
              $instr,
          )* };

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
