#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub enum ArrayTypes {
    EmptyValueSized { value: u32 },
    NullValue {},
    TrueValue { value: bool },
    FalseValue { value: bool },
    NumberValue { value: u64 },
    ShortStringValue { value: u16 },
    LongStringValue { value: u32 },
    ByteStringValue { value: u8 },
    IntegerValue { value: u32 },
}
