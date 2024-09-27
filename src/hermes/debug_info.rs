use std::io;

use crate::hermes::decode::decode_u32;
use crate::hermes::encode::encode_u32;
use crate::hermes::Serializable;

#[derive(Debug, Clone)]
pub struct DebugInfoOffsets {
    pub src: u32,
    pub scope_desc: u32,
    pub callee: u32,
}

impl Serializable for DebugInfoOffsets {
    type Version = u32;
    fn size(&self) -> usize {
        12
    }

    fn deserialize<R>(r: &mut R, _version: u32) -> Self
    where
        R: io::Read + io::BufRead + io::Seek,
    {
        DebugInfoOffsets {
            src: decode_u32(r),
            scope_desc: decode_u32(r),
            callee: decode_u32(r),
        }
    }

    fn serialize<W>(&self, _w: &mut W)
    where
        W: io::Write,
    {
        // encode_u32(w, self.string_data_off);
    }
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

impl Serializable for DebugInfoHeader {
    type Version = u32;
    fn size(&self) -> usize {
        28
    }

    fn deserialize<R>(r: &mut R, _version: u32) -> Self
    where
        R: io::Read + io::BufRead + io::Seek,
    {
        DebugInfoHeader {
            filename_count: decode_u32(r),
            filename_storage_size: decode_u32(r),
            file_region_count: decode_u32(r),
            scope_desc_data_offset: decode_u32(r),
            textified_callee_offset: decode_u32(r),
            string_table_offset: decode_u32(r),
            debug_data_size: decode_u32(r),
        }
    }

    fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write,
    {
        // encode_u32(w, self.string_data_off);
        encode_u32(w, self.filename_count);
        encode_u32(w, self.filename_storage_size);
        encode_u32(w, self.file_region_count);
        encode_u32(w, self.scope_desc_data_offset);
        encode_u32(w, self.textified_callee_offset);
        encode_u32(w, self.string_table_offset);
        encode_u32(w, self.debug_data_size);
    }
}

#[derive(Debug)]
pub struct DebugFileRegion {
    pub from_address: u32,
    pub filename_id: u32,
    pub source_mapping_url_id: u32,
}

impl Serializable for DebugFileRegion {
    type Version = u32;
    fn size(&self) -> usize {
        12
    }

    fn deserialize<R>(r: &mut R, _version: u32) -> Self
    where
        R: io::Read + io::BufRead + io::Seek,
    {
        DebugFileRegion {
            from_address: decode_u32(r),
            filename_id: decode_u32(r),
            source_mapping_url_id: decode_u32(r),
        }
    }

    fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write,
    {
        // encode_u32(w, self.string_data_off);
        encode_u32(w, self.from_address);
        encode_u32(w, self.filename_id);
        encode_u32(w, self.source_mapping_url_id);
    }
}
