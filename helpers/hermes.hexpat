// Basic hermes file format pattern for ImHex
// This has a bunch of incorrect patterns due to constant testing against
// multiple versions of HBC, but should be a good starting point.

#include <std/io.pat>
#include <std/mem.pat>
import std.mem;


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


enum ProhibitInvoke: u8 {
    ProhibitCall = 0,
    ProhibitConstruct = 1,
    ProhibitNone = 2,
};

bitfield Flags {
    ProhibitInvoke prohibit_invoke: 2;
    strict_mode: 1;
    has_exception_handler: 1;
    has_debug_info: 1;
    overflowed: 1;
    unused: 2;
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
    //prohibit_invoke: 2;
    //strict_mode: 1;
    //has_exception_handler: 1;
    //has_debug_info: 1;
    //overflowed: 1;
    //unused: 2;
};



bitfield FuncBitfields {
    u32 offset;
    u32 paramCount;
    u32 byteSize;
    u32 funcName;
    u32 infoOffset;
    u32 frameSize;
    u32 envSize;
    u8 hrci;
    u8 hwci;
};

struct ExceptionHandlerInfo {
    u32 start, end, target;
};

struct DebugInfoOffsets {
    u32 src;
    u32 scope_desc;
    u32 callee;
};



fn isOverflowedOld(u8 flag) {
    return (flag & 0x01) != 0;
};

fn isOverflowed(u8 flag) {
    return ((flag >> 6) & 0x01) != 0;
};

fn hasDebugInfo(u8 flag) {
    return (flag & 0x02) != 0;
};

fn hasExceptionHandler(u8 flag) {
    return (flag & 0x04) != 0;
};

fn getStrictMode(u8 flag) {
    return (flag & 0x18) >> 3;
};

fn getProhibitInvoke(u8 flag) {
    return (flag & 0x60) >> 5;
};

u8 lmao out;
fn check_large() {
    SmallFuncBitfields fields @ $;
    u8 flag_byte @ $;

    lmao = flag_byte;
    
    return isOverflowed(flag_byte);
};

struct smallFuncHeader {
    SmallFuncBitfields fields;
    Flags flag;
    //if (!isOverflowed(flag)) {
    u8 bytecode[fields.byteSize] @ fields.offset; 
    //}
};


u32 fo out;

struct LargeFunctionHeader {
	FuncBitfields fields;
    Flags flag;

    if (flag.has_exception_handler == 1) {
        ExceptionHandlerInfo exceptions;
    }
    
    if (flag.has_debug_info == 1) {
        DebugInfoOffsets dio;
    }
    
    u8 bytecode[fields.byteSize] @ fields.offset;
};

u16 example_offset out;
u16 fh_offset out;

fn get_real_offset(u16 addr) {

    u16 fh_offset = addr;
    u16 ret @ fh_offset;
    return ret;
};

fn get_real_info_offset(u16 addr) {

    u16 fh_offset = addr;
    u16 ret @ fh_offset + 8;
    return ret;
};



fn getLargeOffset(u32 real_offset, u32 real_info_offset) {
    u32 value = (real_info_offset << 16) | (real_offset & 0xffff);

    return value;
};

//import std::io;




u32 _fh_offset_val out;
union FunctionHeader {
 
    if (check_large() == true) {
        example_offset = addressof(this);
        u32 real_offset = get_real_offset(example_offset);
        u32 real_info_offset = get_real_info_offset(example_offset);
        //pppp = real_info_offset;
        SmallFuncBitfields fields;
        Flags flag;
        //example_offset = addressof(flag);
        
        if (flag.overflowed == 1){
            std::print("flag: {}", flag);
            u32 off = getLargeOffset(real_offset, real_info_offset);
            _fh_offset_val = u32(off);
            //u8 off = (fields.infoOffset << 16 &0xff) | fields.offset;
            //u8 lol2 @ off;
            std::print("value : {}", off);
            LargeFunctionHeader fh @ off;
        }

        //LargeFunctionHeader fh @ 905724;
        
    } else {
        smallFuncHeader sfh;
    } 
};

bitfield StringKindEntry {
    count: 31;
	kind: 1;
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
u8 foo out;


// cherrypick which funcs you want to inspect
FunctionHeader funcs[1] @ 0x80;

// Example of a SFH that is overflowed, showing relationship to LargeFunctionHeader
FunctionHeader large[1] @ 7488;


/*
Other items:


StringKindEntry string_kind_entry_table[header.stringKindCount] @ foo;
u32 identifierHashes[header.identifierCount] @ $;
SmallStringTableEntry string_table_entries[header.stringCount] @ $;
OverflowStringTable overflow_string_table[header.overflowStringCount] @ $;
u8 stringStorage[header.stringStorageSize] @ $;
u8 arrayBuffer[header.arrayBufferSize] @ $;

// regexp
if (header.regExpStorageSize > 0) {
    u8 regexp[header.regExpStorageSize] @ $;
}

u8 objKeyBuffer[header.objKeyBufferSize] @ $;
u8 objValueBuffer[header.objValueBufferSize] @ $;


DebugInfo debug_info @ header.debugInfoOffset;

u8 sha1_footer[20] @ $;
*/