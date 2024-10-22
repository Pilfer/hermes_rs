use std::io;

use crate::hermes::decode::decode_u32;
use crate::hermes::encode::encode_u32;
use crate::hermes::Serializable;

#[derive(Debug, Clone)]
pub struct DebugInfo {
    pub header: DebugInfoHeader,
    pub string_table: Vec<DebugStringTable>,
    pub string_storage: Vec<u8>,
    pub file_regions: Vec<DebugFileRegion>,
    pub sources_data_storage: Vec<u8>,
    pub scope_desc_data_storage: Vec<u8>,
    pub textified_callee_storage: Vec<u8>, // Only present on >= HBC v91
    pub string_table_storage: Vec<u8>,     // Only present on >= HBC v91
}

impl Serializable for DebugInfo {
    type Version = u32;

    fn size(&self) -> usize {
        self.header.size()
            + self.string_table.iter().map(|x| x.size()).sum::<usize>()
            + self.string_storage.len()
            + self.file_regions.iter().map(|x| x.size()).sum::<usize>()
            // + self.offsets.iter().map(|x| x.size()).sum::<usize>()
            + self.sources_data_storage.len()
            + self.scope_desc_data_storage.len()
            + self.textified_callee_storage.len()
            + self.string_table_storage.len()
    }

    fn deserialize<R>(r: &mut R, version: u32) -> Self
    where
        R: io::Read + io::BufRead + io::Seek,
    {
        let header = DebugInfoHeader::deserialize(r, version);

        let string_table = (0..header.filename_count)
            .map(|_| DebugStringTable::deserialize(r, version))
            .collect();

        let string_storage = {
            let mut buf = vec![0; header.filename_storage_size as usize];
            r.read_exact(&mut buf).unwrap();
            buf
        };

        let file_regions = (0..header.file_region_count)
            .map(|_| DebugFileRegion::deserialize(r, version))
            .collect();

        // let offsets = (0..header.file_region_count)
        // .map(|_| DebugInfoOffsets::deserialize(r, version))
        // .collect();

        let sources_data_storage = {
            let mut buf = vec![0; (header.debug_data_size - 1) as usize];
            r.read_exact(&mut buf).unwrap();
            buf
        };

        let scope_desc_data_storage = if version >= 91 {
            let mut buf = vec![
                0;
                (header.textified_callee_offset - header.scope_desc_data_offset - 1)
                    as usize
            ];
            r.read_exact(&mut buf).unwrap();
            buf
        } else {
            let mut buf =
                vec![0; (header.debug_data_size - header.scope_desc_data_offset - 1) as usize];
            r.read_exact(&mut buf).unwrap();
            buf
        };

        let textified_callee_storage = if version >= 91 {
            let mut buf =
                vec![0; (header.string_table_offset - header.textified_callee_offset - 1) as usize];
            r.read_exact(&mut buf).unwrap();
            buf
        } else {
            vec![]
        };

        let string_table_storage = if version >= 91 {
            let read_length = if header.debug_data_size - header.string_table_offset > 0 {
                (header.debug_data_size - header.string_table_offset - 1) as usize
            } else {
                0
            };

            let mut buf = vec![0; read_length];
            r.read_exact(&mut buf).unwrap();
            buf
        } else {
            vec![]
        };

        DebugInfo {
            header,
            string_table,
            string_storage,
            file_regions,
            // offsets,
            sources_data_storage,
            scope_desc_data_storage,
            textified_callee_storage,
            string_table_storage,
        }
    }

    fn serialize<W>(&self, w: &mut W)
    where
        W: std::io::Write + std::io::Seek,
    {
        self.header.serialize(w);

        for entry in &self.string_table {
            entry.serialize(w);
        }
        w.write_all(&self.string_storage).unwrap();
        for entry in &self.file_regions {
            entry.serialize(w);
        }
        // for entry in &self.offsets {
        // entry.serialize(w);
        // }
        w.write_all(&self.sources_data_storage).unwrap();
        w.write_all(&self.scope_desc_data_storage).unwrap();
        w.write_all(&self.textified_callee_storage).unwrap();
        w.write_all(&self.string_table_storage).unwrap();
    }
}

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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct DebugStringTable {
    pub offset: u32,
    pub length: u32,
}

impl Serializable for DebugStringTable {
    type Version = u32;
    fn size(&self) -> usize {
        8
    }

    fn deserialize<R>(r: &mut R, _version: u32) -> Self
    where
        R: io::Read + io::BufRead + io::Seek,
    {
        let offset = decode_u32(r);
        let length = decode_u32(r);
        DebugStringTable { offset, length }
    }

    fn serialize<W>(&self, w: &mut W)
    where
        W: io::Write,
    {
        encode_u32(w, self.offset);
        encode_u32(w, self.length);
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct FunctionDebugInfoDeserializer {
    data: Vec<u8>,
    offset: u32,
    function_index: u32,
    current_line: u32,
    current_column: u32,
}
