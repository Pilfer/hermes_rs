use std::ops::Add;

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Reg8(pub u8);
impl Default for Reg8 {
    fn default() -> Self {
        Reg8(0)
    }
}
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
impl Into<u32> for Reg8 {
    fn into(self) -> u32 {
        self.0 as u32
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Reg32(pub u32);
impl Default for Reg32 {
    fn default() -> Self {
        Reg32(0)
    }
}
impl From<u32> for Reg32 {
    fn from(value: u32) -> Self {
        Reg32(value)
    }
}
impl From<Reg32> for u32 {
    fn from(value: Reg32) -> Self {
        value.0
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UInt8(pub u8);
impl Default for UInt8 {
    fn default() -> Self {
        UInt8(0)
    }
}
impl From<u8> for UInt8 {
    fn from(value: u8) -> Self {
        UInt8(value)
    }
}
impl From<UInt8> for u8 {
    fn from(value: UInt8) -> Self {
        value.0
    }
}
impl Into<u32> for UInt8 {
    fn into(self) -> u32 {
        self.0 as u32
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UInt16(pub u16);
impl Default for UInt16 {
    fn default() -> Self {
        UInt16(0)
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
impl Into<u32> for UInt16 {
    fn into(self) -> u32 {
        self.0 as u32
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UInt32(pub u32);
impl Default for UInt32 {
    fn default() -> Self {
        UInt32(0)
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

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Addr8(pub i8);
impl Default for Addr8 {
    fn default() -> Self {
        Addr8(0)
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
impl Into<u32> for Addr8 {
    fn into(self) -> u32 {
        self.0 as u32
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Addr32(pub i32);
impl Default for Addr32 {
    fn default() -> Self {
        Addr32(0)
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
impl Into<u32> for Addr32 {
    fn into(self) -> u32 {
        self.0 as u32
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Imm32(pub i32);
impl Default for Imm32 {
    fn default() -> Self {
        Imm32(0)
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
impl Into<u32> for Imm32 {
    fn into(self) -> u32 {
        self.0 as u32
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Double(pub f64);
impl Default for Double {
    fn default() -> Self {
        Double(0.0)
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
impl Into<u32> for Double {
    fn into(self) -> u32 {
        self.0 as u32
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StringIDUInt8(pub u8);
impl Default for StringIDUInt8 {
    fn default() -> Self {
        StringIDUInt8(0)
    }
}
impl From<u8> for StringIDUInt8 {
    fn from(value: u8) -> Self {
        StringIDUInt8(value)
    }
}
impl From<StringIDUInt8> for u8 {
    fn from(value: StringIDUInt8) -> Self {
        value.0
    }
}
impl Into<u32> for StringIDUInt8 {
    fn into(self) -> u32 {
        self.0 as u32
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StringIDUInt16(pub u16);
impl Default for StringIDUInt16 {
    fn default() -> Self {
        StringIDUInt16(0)
    }
}
impl From<u16> for StringIDUInt16 {
    fn from(value: u16) -> Self {
        StringIDUInt16(value)
    }
}
impl From<StringIDUInt16> for u16 {
    fn from(value: StringIDUInt16) -> Self {
        value.0
    }
}
impl Into<u32> for StringIDUInt16 {
    fn into(self) -> u32 {
        self.0 as u32
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StringIDUInt32(pub u32);
impl Default for StringIDUInt32 {
    fn default() -> Self {
        StringIDUInt32(0)
    }
}
impl From<u32> for StringIDUInt32 {
    fn from(value: u32) -> Self {
        StringIDUInt32(value)
    }
}
impl From<StringIDUInt32> for u32 {
    fn from(value: StringIDUInt32) -> Self {
        value.0
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FunctionIDUInt8(pub u8);
impl Default for FunctionIDUInt8 {
    fn default() -> Self {
        FunctionIDUInt8(0)
    }
}
impl From<u8> for FunctionIDUInt8 {
    fn from(value: u8) -> Self {
        FunctionIDUInt8(value)
    }
}
impl From<FunctionIDUInt8> for u8 {
    fn from(value: FunctionIDUInt8) -> Self {
        value.0
    }
}
impl Into<u32> for FunctionIDUInt8 {
    fn into(self) -> u32 {
        self.0 as u32
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FunctionIDUInt16(pub u16);
impl Default for FunctionIDUInt16 {
    fn default() -> Self {
        FunctionIDUInt16(0)
    }
}
impl From<u16> for FunctionIDUInt16 {
    fn from(value: u16) -> Self {
        FunctionIDUInt16(value)
    }
}
impl From<FunctionIDUInt16> for u16 {
    fn from(value: FunctionIDUInt16) -> Self {
        value.0
    }
}
impl Into<u32> for FunctionIDUInt16 {
    fn into(self) -> u32 {
        self.0 as u32
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FunctionIDUInt32(pub u32);
impl Default for FunctionIDUInt32 {
    fn default() -> Self {
        FunctionIDUInt32(0)
    }
}
impl From<u32> for FunctionIDUInt32 {
    fn from(value: u32) -> Self {
        FunctionIDUInt32(value)
    }
}
impl From<FunctionIDUInt32> for u32 {
    fn from(value: FunctionIDUInt32) -> Self {
        value.0
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BigIntIDUInt16(pub u16);
impl Default for BigIntIDUInt16 {
    fn default() -> Self {
        BigIntIDUInt16(0)
    }
}
impl From<u16> for BigIntIDUInt16 {
    fn from(value: u16) -> Self {
        BigIntIDUInt16(value)
    }
}
impl From<BigIntIDUInt16> for u16 {
    fn from(value: BigIntIDUInt16) -> Self {
        value.0
    }
}
impl Into<u32> for BigIntIDUInt16 {
    fn into(self) -> u32 {
        self.0 as u32
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BigIntIDUInt32(pub u32);
impl Default for BigIntIDUInt32 {
    fn default() -> Self {
        BigIntIDUInt32(0)
    }
}
impl From<u32> for BigIntIDUInt32 {
    fn from(value: u32) -> Self {
        BigIntIDUInt32(value)
    }
}
impl From<BigIntIDUInt32> for u32 {
    fn from(value: BigIntIDUInt32) -> Self {
        value.0
    }
}

impl Add<u16> for StringIDUInt16 {
    type Output = Self;

    fn add(self, other: u16) -> Self {
        StringIDUInt16::from(u16::from(self) + other)
    }
}

impl Add<u32> for StringIDUInt32 {
    type Output = Self;

    fn add(self, other: u32) -> Self {
        StringIDUInt32::from(u32::from(self) + other)
    }
}

impl Add<u8> for StringIDUInt8 {
    type Output = Self;

    fn add(self, other: u8) -> Self {
        StringIDUInt8::from(u8::from(self) + other)
    }
}

impl Add<u16> for FunctionIDUInt16 {
    type Output = Self;

    fn add(self, other: u16) -> Self {
        FunctionIDUInt16::from(u16::from(self) + other)
    }
}

impl Add<u32> for FunctionIDUInt32 {
    type Output = Self;

    fn add(self, other: u32) -> Self {
        FunctionIDUInt32::from(u32::from(self) + other)
    }
}

macro_rules! impl_display {
  ($($t:ty),*) => {
      $(
          impl std::fmt::Display for $t {
              fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                  write!(f, "{}", self.0)
              }
          }
      )*
  };
}

macro_rules! impl_from_for_usize {
  ($($t:ty),*) => {
      $(
          impl From<$t> for usize {
              fn from(value: $t) -> Self {
                  value.0 as usize
              }
          }
      )*
  };
}

impl_display!(
    Reg8,
    Reg32,
    UInt8,
    UInt16,
    UInt32,
    Addr8,
    Addr32,
    Imm32,
    Double,
    StringIDUInt8,
    StringIDUInt16,
    StringIDUInt32,
    FunctionIDUInt8,
    FunctionIDUInt16,
    FunctionIDUInt32,
    BigIntIDUInt16,
    BigIntIDUInt32
);

impl_from_for_usize!(
    Reg8,
    Reg32,
    UInt8,
    UInt16,
    UInt32,
    Addr8,
    Addr32,
    Imm32,
    Double,
    StringIDUInt8,
    StringIDUInt16,
    StringIDUInt32,
    FunctionIDUInt8,
    FunctionIDUInt16,
    FunctionIDUInt32,
    BigIntIDUInt16,
    BigIntIDUInt32
);
