
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
pub(crate) fn write_bitfield(bits: &mut [u8], start_bit: usize, num_bits: usize, value: u32) {
  let mut written_bits = 0;
  let mut bit_idx = start_bit;

  while written_bits < num_bits {
      let byte_idx = bit_idx / 8;
      let bits_in_current_byte = 8 - (bit_idx % 8);
      let bits_to_write = std::cmp::min(bits_in_current_byte, num_bits - written_bits);

      let mask = ((1 << bits_to_write) - 1) << (bit_idx % 8);
      let byte_value = ((value >> written_bits) & ((1 << bits_to_write) - 1)) << (bit_idx % 8);

      bits[byte_idx] = (bits[byte_idx] & !mask) | byte_value as u8;

      written_bits += bits_to_write;
      bit_idx += bits_to_write;
  }
}
