// Basic hermes file format pattern for ImHex

u8 version out;

struct DebugHeader {
    u32 filename_count;
    u32 filename_storage_size;
    u32 file_region_count;
    u32 scope_desc_data_offset;
    u32 textified_callee_offset;
    u32 string_table_offset;
    u32 debug_data_size;
};

struct DebugStringEntry {
    u32 offset, length;
};

struct DebugFileRegion {
    u32 from_address,filename_id, source_mapping_url_id;
};

struct DebugSourceEntry {
    u8 func_idx;
    u8 line;
    u8 column;
};

struct DebugInfo {
    DebugHeader header;
    DebugStringEntry debug_string_table[header.filename_count]; // good
    char debug_string_storage[header.filename_storage_size]; // good
    
    DebugFileRegion file_regions[header.file_region_count]; // probably good? maybe not.
    
    if (version >= 91) {
        u8 source_data_size = header.scope_desc_data_offset;
        u8 scope_desc_data_size = header.textified_callee_offset - header.scope_desc_data_offset;
        u8 textified_data_size = header.string_table_offset - header.textified_callee_offset;
        u8 string_table_size = header.debug_data_size - header.string_table_offset;
        
        char source_data_storage[source_data_size];
        char scope_desc_data_storage[scope_desc_data_size];
        char textified_callee_storage[textified_data_size];
        char string_storage[string_table_size];
    } else {
        char source_data_storage[(header.debug_data_size)];
    }
};


bitfield Flags {
    strict_mode: 2;
    has_exception_handler: 1;
    has_debug_info: 1;
    overflowed: 1;
};

bitfield SmallFuncBitfields {
	offset: 25;
	paramCount: 7;
	byteSize: 15;
	funcName: 17;
	infoOffset: 25;
	frameSize: 7;
	envSize: 8;
	hrci:8;
	hwci: 8;
};

bitfield FuncBitfields {
	offset: 3;
    paramCount: 32;
    byteSize: 32;
    funcName: 32;
    infoOffset: 32;
    frameSize: 32;
    envSize: 8;
    hrci:8;
    hwci: 8;
    prohibit_invoke: 2;
};

struct DebugInfoOffsets {
    u32 src;
    u32 scope_desc;
    u32 callee;
};

bool overflowed out;
fn check_large() {
    SmallFuncBitfields fields;
    u8 flag;

    return flag << 10 | 0x4;
};

struct smallFuncHeader {
    SmallFuncBitfields fields;
    Flags flag;
    DebugInfoOffsets @ fields.infoOffset;
    u8 bytecode[fields.byteSize] @ fields.offset; 
};


bitfield StringKindEntry {
	kind: 31;
	count: 1;
};

bitfield SmallStringTableEntry {
	bool is_utf_16: 1;
	offset: 23;
	length: 8;
};

struct OverflowStringTable {
	u32 offset;
	u32 length;
};

struct HermesHeader {
	u64 magic;
	u32 version;
	u8 sh[20];
	u32 filelength;
	u32 globalCodeIndex;
	u32 fnCount;
	u32 stringKindCount;
	u32 identifierCount;
	u32 stringCount;
	u32 overflowStringCount;
	u32 stringStorageSize;
	u32 bigIntCount;
	u32 bigIntStorageSize;
	u32 regExpCount;
	u32 regExpStorageSize;
	u32 arrayBufferSize;
	u32 objKeyBufferSize;
	u32 objValueBufferSize;
	u32 segmentID;
	u32 cjsModuleCount;
	u32 functionSourceCount;
	u32 debugInfoOffset;
	u8 _padding[20];
	
};

HermesHeader header @ $;
version = header.version;

smallFuncHeader funcs[header.fnCount] @ $;
StringKindEntry string_kind_entry_table[header.stringKindCount] @ $;
u32 identifierHashes[header.identifierCount] @ $;
SmallStringTableEntry string_table_entries[header.stringCount] @ $;
OverflowStringTable overflow_string_table[header.overflowStringCount] @ $;
u8 stringStorage[header.stringStorageSize] @ $;
u8 arrayBuffer[header.arrayBufferSize] @ $;
u8 objKeyBuffer[header.objKeyBufferSize] @ $;
u8 objValueBuffer[header.objValueBufferSize] @ $;

DebugInfo debug_info @ header.debugInfoOffset;
u8 sha1_footer[20] @ $;