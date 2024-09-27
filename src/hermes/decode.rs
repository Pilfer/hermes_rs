use std::io;

#[allow(dead_code)]
pub(crate) fn decode_f64<R>(r: &mut R) -> f64
where
    R: ?Sized + io::Read,
{
    let mut buf = [0u8; 8];
    r.read_exact(&mut buf).expect("Could not decode f64");
    let mut result = 0u64;
    // for i in 0..8 {
    for (i, &b) in buf.iter().enumerate() {
        result |= (b as u64) << (8 * i);
    }

    f64::from_bits(result)
}

#[allow(dead_code)]
pub(crate) fn decode_u64<R>(r: &mut R) -> u64
where
    R: ?Sized + io::Read,
{
    let mut buf = [0u8; 8];
    let mut shift = 0;
    r.read_exact(&mut buf).expect("Could not decode u32");
    let mut result = 0u64;
    for b in buf {
        result |= (b as u64) << shift;
        shift += 8;
    }
    result
}

pub(crate) fn decode_u32<R>(r: &mut R) -> u32
where
    R: ?Sized + io::Read,
{
    let mut buf = [0u8; 4];
    let mut shift = 0;
    r.read_exact(&mut buf).expect("Could not decode u32");
    let mut result = 0u32;
    for b in buf {
        result |= (b as u32) << shift;
        shift += 8;
    }
    result
}

#[allow(dead_code)]
pub(crate) fn decode_i32<R>(r: &mut R) -> i32
where
    R: ?Sized + io::Read,
{
    let mut buf = [0u8; 4];
    let mut shift = 0;
    r.read_exact(&mut buf).expect("Could not decode u32");
    let mut result = 0i32;
    for b in buf {
        result |= (b as i32) << shift;
        shift += 8;
    }
    result
}

#[allow(dead_code)]
pub(crate) fn decode_u16<R>(r: &mut R) -> u16
where
    R: ?Sized + io::Read,
{
    let mut buf = [0u8; 2];
    let mut shift = 0;
    r.read_exact(&mut buf).expect("Could not decode u16");
    let mut result = 0u16;
    for b in buf {
        result |= (b as u16) << shift;
        shift += 8;
    }
    result
}

pub(crate) fn decode_u8<R>(r: &mut R) -> u8
where
    R: ?Sized + io::Read,
{
    let mut buf = [0u8; 1];
    r.read_exact(&mut buf).expect("Could not decode u8");

    buf[0]
}

#[allow(dead_code)]
pub(crate) fn decode_i8<R>(r: &mut R) -> i8
where
    R: ?Sized + io::Read,
{
    decode_u8(r) as i8
}

pub(crate) fn read_bitfield(bits: &[u8], start_bit: usize, num_bits: usize) -> u32 {
    let mut value: u32 = 0;
    let mut written_bits = 0;

    let mut bit_idx = start_bit;

    while written_bits < num_bits {
        let byte_idx = bit_idx / 8;
        let bits_in_current_byte = 8 - (bit_idx % 8);
        let bits_to_read = std::cmp::min(bits_in_current_byte, num_bits - written_bits);

        let mask = (1 << bits_to_read) - 1;
        let shift = bit_idx % 8;

        let byte_value = (bits[byte_idx] as u32) >> shift;
        value |= (byte_value & mask) << written_bits;

        written_bits += bits_to_read;
        bit_idx += bits_to_read;
    }

    value
}

// Make a function to print where the cursor is in the reader
pub(crate) fn _print_cursor<R>(r: &mut R) -> io::Result<u64>
where
    R: io::Read + io::Seek,
{
    #[allow(clippy::seek_from_current)]
    let pos = r.seek(io::SeekFrom::Current(0))?;
    println!("Cursor is at position: {:#X}", pos);
    Ok(pos)
}

/*
        let _current_pos = r.seek(io::SeekFrom::Current(0)).unwrap();
        let align = 4 - (_current_pos % 4);
        if align != 4 {
            r.seek(io::SeekFrom::Current(align as i64))
                .expect("unable to align to 4 bytes");
        }
*/

pub(crate) fn align_reader<R>(r: &mut R, num_bytes: u64) -> io::Result<u64>
where
    R: io::Read + io::Seek,
{
    let mut current_pos = r.seek(io::SeekFrom::Current(0))?;
    let align = num_bytes - (current_pos % num_bytes);
    if align != num_bytes {
        r.seek(io::SeekFrom::Current(align as i64))?;
    }
    current_pos = r.seek(io::SeekFrom::Current(0))?;
    Ok(current_pos)
}
