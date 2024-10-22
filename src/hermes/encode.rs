#[allow(dead_code)]
pub(crate) fn encode_u8<W>(w: &mut W, value: u8)
where
    W: std::io::Write,
{
    w.write_all(&value.to_le_bytes())
        .expect("Could not encode u8");
}

#[allow(dead_code)]
pub(crate) fn encode_i8<W>(w: &mut W, value: i8)
where
    W: std::io::Write,
{
    w.write_all(&value.to_le_bytes())
        .expect("Could not encode i8");
}

#[allow(dead_code)]
pub(crate) fn encode_u16<W>(w: &mut W, value: u16)
where
    W: std::io::Write,
{
    w.write_all(&value.to_le_bytes())
        .expect("Could not encode u16");
}

#[allow(dead_code)]
pub(crate) fn encode_i16<W>(w: &mut W, value: i16)
where
    W: std::io::Write,
{
    w.write_all(&value.to_le_bytes())
        .expect("Could not encode i16");
}

#[allow(dead_code)]
pub(crate) fn encode_u32<W>(w: &mut W, value: u32)
where
    W: std::io::Write,
{
    w.write_all(&value.to_le_bytes())
        .expect("Could not encode u32");
}

pub(crate) fn encode_i32<W>(w: &mut W, value: i32)
where
    W: std::io::Write,
{
    w.write_all(&value.to_le_bytes())
        .expect("Could not encode i32");
}

#[allow(dead_code)]
pub(crate) fn encode_f64<W>(w: &mut W, value: f64)
where
    W: std::io::Write,
{
    w.write_all(&value.to_le_bytes())
        .expect("Could not encode f64");
}

#[allow(dead_code)]
pub(crate) fn encode_u64<W>(w: &mut W, value: u64)
where
    W: std::io::Write,
{
    w.write_all(&value.to_le_bytes())
        .expect("Could not encode u64");
}

#[allow(dead_code)]
pub(crate) fn write_bitfield(bits: &mut [u8], start_bit: usize, num_bits: usize, value: u32) {
    let mut written_bits = 0;
    let mut bit_idx = start_bit;

    while written_bits < num_bits {
        let byte_idx = bit_idx / 8;
        let bits_in_current_byte = 8 - (bit_idx % 8);
        let bits_to_write = std::cmp::min(bits_in_current_byte, num_bits - written_bits);

        if bits_to_write == 0 {
            break;
        }

        // Ensure bits_to_write is within a valid range
        if bits_to_write >= 32 {
            panic!("bits_to_write is too large: {}", bits_to_write);
        }

        let mask = ((1u32 << bits_to_write) - 1) << (bit_idx % 8);
        let byte_value = ((value >> written_bits) & ((1u32 << bits_to_write) - 1)) << (bit_idx % 8);

        bits[byte_idx] = (bits[byte_idx] & !(mask as u8)) | (byte_value as u8);

        written_bits += bits_to_write;
        bit_idx += bits_to_write;
    }
}

#[allow(dead_code)]
pub(crate) fn encode_sleb128<W>(w: &mut W, value: i64)
where
    W: std::io::Write,
{
    let mut val = value;
    loop {
        let byte = (val & 0x7F) as u8;
        val >>= 7;
        if (val == 0 && byte & 0x40 == 0) || (val == -1 && byte & 0x40 != 0) {
            encode_u8(w, byte);
            break;
        } else {
            encode_u8(w, byte | 0x80);
        }
    }
}

#[allow(dead_code)]
pub fn align_writer<W>(w: &mut W, alignment: usize)
where
    W: std::io::Write + std::io::Seek,
{
    let padding = (alignment
        - (w.seek(std::io::SeekFrom::Current(0)).unwrap() as usize) % alignment)
        % alignment;
    for _ in 0..padding {
        encode_u8(w, 0);
    }
}
