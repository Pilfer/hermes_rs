
#[allow(dead_code)]
pub(crate) fn encode_u8<W>(w: &mut W, value: u8) 
where
  W: std::io::Write,
{
  w.write_all(&value.to_le_bytes()).expect("Could not encode u8");
}


#[allow(dead_code)]
pub(crate) fn encode_i8<W>(w: &mut W, value: i8) 
where W: std::io::Write, {
  w.write_all(&value.to_le_bytes()).expect("Could not encode i8");
}

#[allow(dead_code)]
pub(crate) fn encode_u16<W>(w: &mut W, value: u16) 
where
  W: std::io::Write,
{
  w.write_all(&value.to_le_bytes()).expect("Could not encode u16");
}

#[allow(dead_code)]
pub(crate) fn encode_i16<W>(w: &mut W, value: i16) 
where
  W: std::io::Write,
{
  w.write_all(&value.to_le_bytes()).expect("Could not encode i16");
}

#[allow(dead_code)]
pub(crate) fn encode_u32<W>(w: &mut W, value: u32) 
where
  W: std::io::Write,
{
  w.write_all(&value.to_le_bytes()).expect("Could not encode u32");
}

pub(crate) fn encode_i32<W>(w: &mut W, value: i32) 
where
  W: std::io::Write,
{
  w.write_all(&value.to_le_bytes()).expect("Could not encode i32");
}

#[allow(dead_code)]
pub(crate) fn encode_f64<W>(w: &mut W, value: f64) 
where
  W: std::io::Write,
{
  w.write_all(&value.to_le_bytes()).expect("Could not encode f64");
}

#[allow(dead_code)]
pub(crate) fn encode_u64<W>(w: &mut W, value: u64) 
where
  W: std::io::Write,
{
  w.write_all(&value.to_le_bytes()).expect("Could not encode u64");
}

#[allow(dead_code)]
pub(crate) fn write_bitfield<W>(w: &mut W, value: u32, bits: u32) 
where
  W: std::io::Write,
{
  let mut shift = 0;
  for _ in 0..bits {
    let mask = 1 << shift;
    let bit = (value & mask) >> shift;
    w.write_all(&[bit as u8]).expect("Could not write bitfield");
    shift += 1;
  }
}

