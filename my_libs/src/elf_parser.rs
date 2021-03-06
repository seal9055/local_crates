use byteorder::{LittleEndian, ReadBytesExt};
use std::{mem};

pub const ELFMAGIC:     u32 = 0x464c457f;
pub const ARCH64:       u8  = 0x2;
pub const LITTLEENDIAN: u8  = 0x1;
pub const TYPEEXEC:     u16 = 0x2;
pub const LOADSEGMENT:  u32 = 0x1;
pub const RISCV:        u16 = 0xf3;

#[derive(Debug, Copy, Clone)]
pub struct Header {
	pub magic:             u32,
	pub bitsize:           u8,
	pub endian:            u8,
	pub ident_abi_version: u8,
	pub target_platform:   u8,
	pub abi_version:       u8,
	pub padding:           [u8; 7],
	pub o_type:            u16,
	pub machine:           u16,
	pub version:           u32,
	pub entry_addr:        usize,
	pub phoff:             usize, // Program Header Offset
	pub shoff:             usize, // Section Header Offset
	pub flags:             u32,
	pub ehsize:            u16,
	pub phentsize:         u16,
	pub phnum:             u16, // Number of Program Headers
	pub shentsize:         u16,
	pub shnum:             u16,
	pub shstrndx:          u16,
}

impl Header {
    pub fn new(mut binary: &[u8]) -> Option<Self> {
        if binary.len() < mem::size_of::<Header>() { return None; }
        Some(Header {
            magic            : binary.read_u32::<LittleEndian>().unwrap(),
            bitsize          : binary.read_u8::<>().unwrap(),
            endian           : binary.read_u8::<>().unwrap(),
            ident_abi_version: binary.read_u8::<>().unwrap(),
            target_platform  : binary.read_u8::<>().unwrap(),
            abi_version      : binary.read_u8::<>().unwrap(),
            padding          : [0u8;7].map(|_| binary.read_u8::<>().unwrap()),
            o_type           : binary.read_u16::<LittleEndian>().unwrap(),
            machine          : binary.read_u16::<LittleEndian>().unwrap(),
            version          : binary.read_u32::<LittleEndian>().unwrap(),
            entry_addr       : binary.read_u64::<LittleEndian>().unwrap() as usize,
            phoff            : binary.read_u64::<LittleEndian>().unwrap() as usize,
            shoff            : binary.read_u64::<LittleEndian>().unwrap() as usize,
            flags            : binary.read_u32::<LittleEndian>().unwrap(),
            ehsize           : binary.read_u16::<LittleEndian>().unwrap(),
            phentsize        : binary.read_u16::<LittleEndian>().unwrap(),
            phnum            : binary.read_u16::<LittleEndian>().unwrap(),
            shentsize        : binary.read_u16::<LittleEndian>().unwrap(),
            shnum            : binary.read_u16::<LittleEndian>().unwrap(),
            shstrndx         : binary.read_u16::<LittleEndian>().unwrap(),
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ProgramHeader {
	pub seg_type: u32,
	pub flags:    u32,
	pub offset:   usize,
	pub vaddr:    usize,
	pub paddr:    usize,
	pub filesz:   usize,
	pub memsz:    usize,
	pub align:    usize,
}

impl ProgramHeader {
    pub fn new(mut binary: &[u8]) -> Option<Self> {
        if binary.len() < mem::size_of::<ProgramHeader>() { return None; }
        Some(ProgramHeader {
            seg_type: binary.read_u32::<LittleEndian>().unwrap(),
            flags   : binary.read_u32::<LittleEndian>().unwrap(),
            offset  : binary.read_u64::<LittleEndian>().unwrap() as usize,
            vaddr   : binary.read_u64::<LittleEndian>().unwrap() as usize,
            paddr   : binary.read_u64::<LittleEndian>().unwrap() as usize,
            filesz  : binary.read_u64::<LittleEndian>().unwrap() as usize,
            memsz   : binary.read_u64::<LittleEndian>().unwrap() as usize,
            align   : binary.read_u64::<LittleEndian>().unwrap() as usize,
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SectionHeader {
    pub s_name:      u32,
    pub s_type:      u32,
    pub s_flags:     usize,
    pub s_addr:      usize,
    pub s_offset:    usize,
    pub s_size:      usize,
    pub s_link:      u32,
    pub s_info:      u32,
    pub s_addralign: usize,
    pub s_entsize:   usize,
}

impl SectionHeader {
    pub fn new(mut binary: &[u8]) -> Option<Self> {
        if binary.len() < mem::size_of::<SectionHeader>() { return None; }
        Some(SectionHeader {
            s_name:      binary.read_u32::<LittleEndian>().unwrap(),
            s_type:      binary.read_u32::<LittleEndian>().unwrap(),
            s_flags:     binary.read_u64::<LittleEndian>().unwrap() as usize,
            s_addr:      binary.read_u64::<LittleEndian>().unwrap() as usize,
            s_offset:    binary.read_u64::<LittleEndian>().unwrap() as usize,
            s_size:      binary.read_u64::<LittleEndian>().unwrap() as usize,
            s_link:      binary.read_u32::<LittleEndian>().unwrap(),
            s_info:      binary.read_u32::<LittleEndian>().unwrap(),
            s_addralign: binary.read_u64::<LittleEndian>().unwrap() as usize,
            s_entsize:   binary.read_u64::<LittleEndian>().unwrap() as usize,
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SymbolTable {
    pub sym_name:  u32,
    pub sym_info:  u8,
    pub sym_other: u8,
    pub sym_shndx: u16,
    pub sym_value: usize,
    pub sym_size:  usize,
}

impl SymbolTable {
    pub fn new(mut binary: &[u8]) -> Option<Self> {
        if binary.len() < mem::size_of::<SymbolTable>() { return None; }
        Some(SymbolTable {
            sym_name:  binary.read_u32::<LittleEndian>().unwrap(),
            sym_info:  binary.read_u8::<>().unwrap(),
            sym_other: binary.read_u8::<>().unwrap(),
            sym_shndx: binary.read_u16::<LittleEndian>().unwrap(),
            sym_value: binary.read_u64::<LittleEndian>().unwrap() as usize,
            sym_size:  binary.read_u64::<LittleEndian>().unwrap() as usize,
        })
    }
}
