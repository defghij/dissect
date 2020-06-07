use crate::elf::types::*;
//use crate::elf::errors::*;
use crate::elf::enumerations::*;
use crate::utility_functions::arrays::*;
use std::fmt;
use std::marker;

//=======================
// traits for type enums =
//=========================
trait Constructor {}
trait Validate {
    fn is_constructable(bytes: &[u8]) -> bool;
    fn is_valid(&self) -> bool;
}
trait GetHeaderValues {
    fn get_e_type_value(&self) ->Half;
    fn get_e_machine_string(&self) -> String;
    fn get_e_machine_value(&self) -> Half;
    fn get_e_version_value(&self) -> Word;
    fn get_e_entry_value(&self) -> Addr;
    fn get_e_phoff_value(&self) -> Offset;
    fn get_e_shoff_value(&self) -> Offset;
    fn get_e_flags_value(&self) -> Word;
    fn get_e_ehsize_value(&self) -> Half;
    fn get_e_phentsize_value(&self) -> Half;
    fn get_e_phnum_value(&self) -> Half;
    fn get_e_shentsize_value(&self) -> Half;
    fn get_e_shnum_value(&self) -> Half;
    fn get_e_shstrndx_value(&self) -> Half;
}

/* Struct that contains all the Elf sub-structures. */

// TODO Make these generic over memory width?
// Overall ELF Structs
/*
#[allow(non_snake_case)]
pub struct ELF {
    ELF32: ELF32Binary,
    ELF64: ELF64Binary,
}
#[allow(non_snake_case)]
pub struct ELF32Binary {
    pub Header: Header<Elf32>,
    // TODO Implement rest of binary structure.
}
#[allow(non_snake_case)]
pub struct ELF64Binary {
    pub Header: Header<Elf64>,
    // TODO Implement rest of binary structure.
}

impl ELF32Binary {
    pub fn new(file: Vec<u8>) -> Result<ELF32Binary, String> {
        Ok(ELF32Binary {Header:
            Header::<Elf32>::new(&file[Header::<Elf32>::HEADER_START..Header::<Elf32>::HEADER_LEN])
        })
    }
}
impl ELF64Binary {
    pub fn new(bytes: &[u8]) -> ELF64Binary {
        ELF64Binary {Header:
            Header::<Elf64>::new(&bytes[Header::<Elf64>::HEADER_START..Header::<Elf64>::HEADER_LEN])
        }
    }
}
*/

//=========================
//  Header related structs =
//===========================
pub struct EIdent{
    pub magic_number: [u8; 4],      // Magic number
    pub ei_class:      u8,          // File class
    pub ei_data:       u8,          // Encoding of data in object file sections.
    pub ei_version:    u8,          // ELF header version number
    pub ei_osabi:      u8,          // Specifies the OS or ABI ELF extentions used in this file
    pub ei_abiversion: u8,          // Version of ABI to which the object is target
    pub ei_pad:       [u8; 6],      // Marks the start of padding
    pub ei_nident:     u8,          // Size of the e_ident array
}

/*  Header Field Types
        Ident  = EIdent,
        Half   = Half::Elf32Half || Half::Elf64Half,
        Word   = Word::Elf32Word || Word::Elf64Word,
        Addr   = Addr::Elf32Addr || Addr::Elf64Addr,
        Offset = Offset::Elf32Off || Offset::Elf64Off
*/
pub struct Header<Elf>{
    pub e_ident:     EIdent,    // Magic Number and other useful information
    pub e_type:      Half,       // Object file type
    pub e_machine:   Half,       // Architecture
    pub e_version:   Word,       // Object file version
    pub e_entry:     Addr,       // Entry point virtual address
    pub e_phoff:     Offset,     // Program header table offset
    pub e_shoff:     Offset,     // Section header table offset
    pub e_flags:     Word,       // Processor-specific flags
    pub e_ehsize:    Half,       // Elf Header size
    pub e_phentsize: Half,       // Program header table entry size
    pub e_phnum:     Half,       // Program header table entry count
    pub e_shentsize: Half,       // Section header table entry size
    pub e_shnum:     Half,       // Section header table entry count
    pub e_shstrndx:  Half,       // Section header string table index
    _marker: marker::PhantomData<Elf>,
}

// Implementation details for elf header related structs
impl EIdent {
    pub const EIdent_Len: usize = 16;
    pub fn new(bytes : &[u8]) -> EIdent {

        let m_array = [ bytes[0], bytes[1], bytes[2], bytes[3] ];

        // Build up ei_pad array field
        let mut ei_pad_array: [u8; 6] = Default::default();
        ei_pad_array.copy_from_slice(&bytes[9 .. 15]);

        let e_ident = EIdent { magic_number:  m_array,
                               ei_class:      bytes[4],
                               ei_data:       bytes[5],
                               ei_version:    bytes[6],
                               ei_osabi:      bytes[7],
                               ei_abiversion: bytes[8],
                               ei_pad:        ei_pad_array,
                               ei_nident:     bytes[15],
                              };
        return e_ident;
    }
    pub fn is_elf(&self) -> bool{
        match self.magic_number {
            [0x7F,0x45, 0x4C, 0x46] => return true,
            _                       => return false,
        }
    }
    pub fn is_32bit(&self) -> bool {
        self.ei_class == 1
    }
    pub fn is_64bit(&self) -> bool {
        self.ei_class == 0
    }
    //Private helpers for fmt::Display to convert values to meaningful strings.
    fn get_type_string(&self) -> String{
        let mut type_string = String::new();
        for byte in 0..4 {                                     // Get magic header
            type_string.push_str(&(format!("{}",self.magic_number[byte] as char)).to_string());
        }
        return type_string;
    }
    fn get_ei_class_string(&self) -> String {
        match self.ei_class as u16 {
            ELFCLASS32 => return String::from("32bit"),
            ELFCLASS64 => return String::from("64bit"),
            _ => return String::from("Invalid EI Class"),
        }
    }
    fn get_ei_encoding_string(&self) -> String {
        match self.ei_data {
            1 => return String::from("little endian"),
            2 => return String::from("big endian"),
            _ => return String::from("Failed to find valid Data Format entry."),
        };
    }
    fn get_ei_version_string(&self) -> String {
        match self.ei_version{
            1 => return String::from("Current ELF Version"),
            _ => return String::from("Invalid Version"),
        }
    }
    fn get_ei_osabi_string(&self) -> String {
        match self.ei_osabi {
            0  => String::from("Unix System V"),
            1  => String::from("HP-UX"),
            2  => String::from("NetBSD"),
            3  => String::from("Linux"),
            6  => String::from("Solaris"),
            7  => String::from("IBM AIX"),
            8  => String::from("IRIX"),
            9  => String::from("FreeBSD"),
            10 => String::from("TRU64 Unix"),
            11 => String::from("Novel Modest"),
            12 => String::from("OpenBSD"),
            64..=255 => String::from("Architecture Specific"),
            _ => String::from("Invalid OS ABI value"),
        }
    }
    fn get_ei_abiversion_string(&self) -> String {
        match self.ei_abiversion {
            0 => return String::from("Unspecified"),
            _ => return String::from("Other"),
        }
    }
}
impl Validate for EIdent{
    fn is_valid(&self) -> bool {
        self.is_elf()
    }
    fn is_constructable(bytes: &[u8]) -> bool {
        bytes.len() == EIdent::EIdent_Len
    }
}
impl fmt::Display for EIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output_string: String = String::new();
        output_string.push_str(&format!("\tMagic Numbers:                     {}\n",
            self.get_type_string()));
        output_string.push_str(&format!("\tClass:                             {}\n",
            self.get_ei_class_string()));
        output_string.push_str(&format!("\tEncoding:                          {}\n",
            self.get_ei_encoding_string()));
        output_string.push_str(&format!("\tVersion:                           {}\n",
            self.ei_version));
        output_string.push_str(&format!("\tOS/ABI:                            {}\n",
            self.get_ei_osabi_string()));
        output_string.push_str(&format!("\tABI Version:                       {}\n",
            self.get_ei_abiversion_string()));

        write!(f,"{}", output_string)
    }
}

impl Header<Elf32> {
    pub const HEADER_START: usize = 0;
    pub const HEADER_END:   usize = 52;
    pub const HEADER_LEN:   usize = (Header::<Elf32>::HEADER_END - Header::<Elf32>::HEADER_START);

    pub fn new(bytes: &[u8]) -> Header<Elf32> {
        let e_header: Header<Elf32> = Header {
                e_ident:     EIdent::new(&bytes[0 .. 16]),
                e_type:      Half::Elf32(  vec_slice_to_u16(&bytes[16 .. 18]).unwrap()),
                e_machine:   Half::Elf32(  vec_slice_to_u16(&bytes[18 .. 20]).unwrap()),
                e_version:   Word::Elf32(  vec_slice_to_u32(&bytes[20 .. 24]).unwrap()),
                e_entry:     Addr::Elf32(  vec_slice_to_u32(&bytes[24 .. 28]).unwrap()),
                e_phoff:     Offset::Elf32(vec_slice_to_u32(&bytes[28 .. 32]).unwrap()),
                e_shoff:     Offset::Elf32(vec_slice_to_u32(&bytes[32 .. 36]).unwrap()),
                e_flags:     Word::Elf32(  vec_slice_to_u32(&bytes[36 .. 40]).unwrap()),
                e_ehsize:    Half::Elf32(  vec_slice_to_u16(&bytes[40 .. 42]).unwrap()),
                e_phentsize: Half::Elf32(  vec_slice_to_u16(&bytes[42 .. 44]).unwrap()),
                e_phnum:     Half::Elf32(  vec_slice_to_u16(&bytes[44 .. 46]).unwrap()),
                e_shentsize: Half::Elf32(  vec_slice_to_u16(&bytes[46 .. 48]).unwrap()),
                e_shnum:     Half::Elf32(  vec_slice_to_u16(&bytes[48 .. 50]).unwrap()),
                e_shstrndx:  Half::Elf32(  vec_slice_to_u16(&bytes[50 .. 52]).unwrap()),
                _marker: marker::PhantomData,
                };
        e_header
    }
    pub fn new_partial(bytes: &[u8], e_ident_st: EIdent) -> Header<Elf32> {
        let e_header: Header<Elf32> = Header {
                e_ident:     e_ident_st,
                e_type:      Half::Elf32(  vec_slice_to_u16(&bytes[0 .. 2]).unwrap()),
                e_machine:   Half::Elf32(  vec_slice_to_u16(&bytes[2 .. 4]).unwrap()),
                e_version:   Word::Elf32(  vec_slice_to_u32(&bytes[4 .. 8]).unwrap()),
                e_entry:     Addr::Elf32(  vec_slice_to_u32(&bytes[8 .. 12]).unwrap()),
                e_phoff:     Offset::Elf32(vec_slice_to_u32(&bytes[12 .. 16]).unwrap()),
                e_shoff:     Offset::Elf32(vec_slice_to_u32(&bytes[16 .. 20]).unwrap()),
                e_flags:     Word::Elf32(  vec_slice_to_u32(&bytes[20 .. 24]).unwrap()),
                e_ehsize:    Half::Elf32(  vec_slice_to_u16(&bytes[24 .. 26]).unwrap()),
                e_phentsize: Half::Elf32(  vec_slice_to_u16(&bytes[26 .. 28]).unwrap()),
                e_phnum:     Half::Elf32(  vec_slice_to_u16(&bytes[28 .. 30]).unwrap()),
                e_shentsize: Half::Elf32(  vec_slice_to_u16(&bytes[30 .. 32]).unwrap()),
                e_shnum:     Half::Elf32(  vec_slice_to_u16(&bytes[32 .. 34]).unwrap()),
                e_shstrndx:  Half::Elf32(  vec_slice_to_u16(&bytes[34 .. 36]).unwrap()),
                _marker: marker::PhantomData,
                };
        e_header
    }
}
impl Validate for Header<Elf32>{
    //TODO Better way to validate Header structure?
    fn is_valid(&self) -> bool {
        self.e_ident.is_valid()
    }
    fn is_constructable(bytes: &[u8]) -> bool {
        bytes.len() == Header::<Elf32>::HEADER_LEN
    }
}
impl GetHeaderValues for Header<Elf32> {
    fn get_e_type_value(&self) -> Half {
        self.e_type
    }
    fn get_e_machine_string(&self) -> String{
        let e_machine = match self.e_machine {
            Half::Elf32(e_machine) => e_machine,
            Half::Elf64(e_machine) => e_machine,
        };
        match e_machine.swap_bytes(){
            0 => return String::from("No Machine"),
            1 => return String::from("AT&T WE 32100"),
            2 => return String::from("SPARC"),
            3 => return String::from("Intel 80386"),
            _ => return String::from("To be implemented"),
        }
    }
    fn get_e_machine_value(&self) -> Half {
        self.e_machine
    }
    fn get_e_version_value(&self) -> Word {
        self.e_version
    }
    fn get_e_entry_value(&self) -> Addr {
        self.e_entry
    }
    fn get_e_phoff_value(&self) -> Offset {
        self.e_phoff
    }
    fn get_e_shoff_value(&self) -> Offset {
        self.e_shoff
    }
    fn get_e_flags_value(&self) -> Word {
        self.e_flags
    }
    fn get_e_ehsize_value(&self) -> Half {
        self.e_ehsize
    }
    fn get_e_phentsize_value(&self) -> Half {
        self.e_phentsize
    }
    fn get_e_phnum_value(&self) -> Half {
        self.e_phnum
    }
    fn get_e_shentsize_value(&self) -> Half {
        self.e_shentsize
    }
    fn get_e_shnum_value(&self) -> Half {
        self.e_shentsize
    }
    fn get_e_shstrndx_value(&self) -> Half {
        self.e_shstrndx
    }
}
impl fmt::Display for Header<Elf32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output_string: String = String::new();
        output_string.push_str(&format!("{}",&self.e_ident));
        output_string.push_str(&format!("\tType:                              {:X}\n",
            self.get_e_type_value()
                .get_elf32_value()
                .unwrap()
                .swap_bytes()
        ));
        output_string.push_str(&format!("\tMachine:                           {}\n",
            self.get_e_machine_string()));
        output_string.push_str(&format!("\tVersion:                           {:X}\n",
            self.get_e_version_value()
                .get_elf32_value()
                .unwrap()
                .swap_bytes()
            ));
        output_string.push_str(&format!("\tEntry point address:               0x{:X}\n",
            self.get_e_entry_value()
                .get_elf32_value()
                .unwrap()
                .swap_bytes()
            ));
        output_string.push_str(&format!("\tStart of program headers:          {}\n",
            self.get_e_phoff_value()
                .get_elf32_value()
                .unwrap()
                .swap_bytes()
            ));
        output_string.push_str(&format!("\tSection header string table index: {}\n",
            self.get_e_shstrndx_value()
                .get_elf32_value()
                .unwrap()
                .swap_bytes()
            ));

        write!(f, "{}", output_string.to_string())
    }
}

impl Header<Elf64> {
    pub const HEADER_START: usize = 0;
    pub const HEADER_END:   usize = 64;
    pub const HEADER_LEN:   usize = (Header::<Elf32>::HEADER_END - Header::<Elf32>::HEADER_START);

    pub fn new(bytes: &[u8]) -> Header<Elf64> {
        let e_header = Header {e_ident:      EIdent::new(&bytes[0 .. 16]),
                                e_type:      Half::Elf64(  vec_slice_to_u16(&bytes[16 .. 18]).unwrap()),
                                e_machine:   Half::Elf64(  vec_slice_to_u16(&bytes[18 .. 20]).unwrap()),
                                e_version:   Word::Elf64(  vec_slice_to_u32(&bytes[20 .. 24]).unwrap()),
                                e_entry:     Addr::Elf64(  vec_slice_to_u64(&bytes[24 .. 32]).unwrap()),
                                e_phoff:     Offset::Elf64(vec_slice_to_u64(&bytes[32 .. 40]).unwrap()),
                                e_shoff:     Offset::Elf64(vec_slice_to_u64(&bytes[40 .. 48]).unwrap()),
                                e_flags:     Word::Elf64(  vec_slice_to_u32(&bytes[48 .. 52]).unwrap()),
                                e_ehsize:    Half::Elf64(  vec_slice_to_u16(&bytes[52 .. 54]).unwrap()),
                                e_phentsize: Half::Elf64(  vec_slice_to_u16(&bytes[54 .. 56]).unwrap()),
                                e_phnum:     Half::Elf64(  vec_slice_to_u16(&bytes[56 .. 58]).unwrap()),
                                e_shentsize: Half::Elf64(  vec_slice_to_u16(&bytes[58 .. 60]).unwrap()),
                                e_shnum:     Half::Elf64(  vec_slice_to_u16(&bytes[60 .. 62]).unwrap()),
                                e_shstrndx:  Half::Elf64(  vec_slice_to_u16(&bytes[62 .. 64]).unwrap()),
                                _marker: marker::PhantomData // 48
                                };
        e_header
    }
    pub fn new_partial(bytes: &[u8], e_ident_st: EIdent) -> Header<Elf64> {
        let e_header = Header {e_ident:     e_ident_st,
                                e_type:      Half::Elf64(  vec_slice_to_u16(&bytes[0 .. 2]).unwrap()),
                                e_machine:   Half::Elf64(  vec_slice_to_u16(&bytes[2 .. 4]).unwrap()),
                                e_version:   Word::Elf64(  vec_slice_to_u32(&bytes[4 .. 8]).unwrap()),
                                e_entry:     Addr::Elf64(  vec_slice_to_u64(&bytes[8 .. 16]).unwrap()),
                                e_phoff:     Offset::Elf64(vec_slice_to_u64(&bytes[16 .. 24]).unwrap()),
                                e_shoff:     Offset::Elf64(vec_slice_to_u64(&bytes[24 .. 32]).unwrap()),
                                e_flags:     Word::Elf64(  vec_slice_to_u32(&bytes[32 .. 36]).unwrap()),
                                e_ehsize:    Half::Elf64(  vec_slice_to_u16(&bytes[36 .. 38]).unwrap()),
                                e_phentsize: Half::Elf64(  vec_slice_to_u16(&bytes[38 .. 40]).unwrap()),
                                e_phnum:     Half::Elf64(  vec_slice_to_u16(&bytes[40 .. 42]).unwrap()),
                                e_shentsize: Half::Elf64(  vec_slice_to_u16(&bytes[42 .. 44]).unwrap()),
                                e_shnum:     Half::Elf64(  vec_slice_to_u16(&bytes[44 .. 46]).unwrap()),
                                e_shstrndx:  Half::Elf64(  vec_slice_to_u16(&bytes[46 .. 48]).unwrap()),
                                _marker: marker::PhantomData
                                };
        e_header

    }
}
impl Validate for Header<Elf64>{
    //TODO Better way to validate Header structure?
    fn is_valid(&self) -> bool {
        self.e_ident.is_valid()
    }
    fn is_constructable(bytes: &[u8]) -> bool {
        bytes.len() == Header::<Elf64>::HEADER_LEN
    }
}
impl GetHeaderValues for Header<Elf64> {
    fn get_e_type_value(&self) -> Half {
        self.e_type
    }
    fn get_e_machine_string(&self) -> String{
        let e_machine = match self.e_machine {
            Half::Elf32(e_machine) => e_machine,
            Half::Elf64(e_machine) => e_machine,
        };
        match e_machine.swap_bytes(){
            0 => return String::from("No Machine"),
            1 => return String::from("AT&T WE 32100"),
            2 => return String::from("SPARC"),
            3 => return String::from("Intel 80386"),
            _ => return String::from("To be implemented"),
        }
    }
    fn get_e_machine_value(&self) -> Half {
        self.e_machine
    }
    fn get_e_version_value(&self) -> Word {
        self.e_version
    }
    fn get_e_entry_value(&self) -> Addr {
        self.e_entry
    }
    fn get_e_phoff_value(&self) -> Offset {
        self.e_phoff
    }
    fn get_e_shoff_value(&self) -> Offset {
        self.e_shoff
    }
    fn get_e_flags_value(&self) -> Word {
        self.e_flags
    }
    fn get_e_ehsize_value(&self) -> Half {
        self.e_ehsize
    }
    fn get_e_phentsize_value(&self) -> Half {
        self.e_phentsize
    }
    fn get_e_phnum_value(&self) -> Half {
        self.e_phnum
    }
    fn get_e_shentsize_value(&self) -> Half {
        self.e_shentsize
    }
    fn get_e_shnum_value(&self) -> Half {
        self.e_shentsize
    }
    fn get_e_shstrndx_value(&self) -> Half {
        self.e_shstrndx
    }
}
impl fmt::Display for Header<Elf64> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output_string: String = String::new();
        output_string.push_str(&format!("{}",&self.e_ident));
        output_string.push_str(&format!("\tType:                              {:X}\n",
            self.get_e_type_value()
                .get_elf64_value()
                .unwrap()
                .swap_bytes()
        ));
        output_string.push_str(&format!("\tMachine:                           {}\n",
            self.get_e_machine_string()));
        output_string.push_str(&format!("\tVersion:                           {:X}\n",
            self.get_e_version_value()
                .get_elf64_value()
                .unwrap()
                .swap_bytes()
            ));
        output_string.push_str(&format!("\tEntry point address:               0x{:X}\n",
            self.get_e_entry_value()
                .get_elf64_value()
                .unwrap()
                .swap_bytes()
            ));
        output_string.push_str(&format!("\tStart of program headers:          {}\n",
            self.get_e_phoff_value()
                .get_elf64_value()
                .unwrap()
                .swap_bytes()
            ));
        output_string.push_str(&format!("\tSection header string table index: {}\n",
            self.get_e_shstrndx_value()
                .get_elf64_value()
                .unwrap()
                .swap_bytes()
            ));

        write!(f, "{}", output_string.to_string())
    }
}
