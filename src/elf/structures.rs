use crate::elf::types::*;
use crate::elf::enumerations::*;
use crate::utility_functions::arrays::*;


/* Struct that contains all the Elf sub-structures. */

pub struct EIdentStruct{
  pub magic_number: [u8; 4],      // Magic number
  pub ei_class:      u8,          // File class
  pub ei_data:       u8,          // Encoding of data in object file sections. 
  pub ei_version:    u8,          // ELF header version number
  pub ei_osabi:      u8,          // Specifies the OS or ABI ELF extentions used in this file
  pub ei_abiversion: u8,          // Version of ABI to which the object is target
  pub ei_pad:       [u8; 6],      // Marks the start of padding
  pub ei_nident:     u8,          // Size of the e_ident array
}
impl EIdentStruct {
    pub fn new(bytes : &[u8]) -> EIdentStruct {

        let m_array = [ bytes[0], bytes[1], bytes[2], bytes[3] ];

        // Build up ei_pad array field
        let mut ei_pad_array: [u8; 6] = Default::default();
        ei_pad_array.copy_from_slice(&bytes[9 .. 15]);

        let e_ident = EIdentStruct { magic_number:  m_array,
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
    pub fn fmt_cli_out(&self) -> Result<String, &str>{
        let mut string_out = String::new();
        string_out.push_str( &(format!("\tbinary:      {}\n", self.get_type())));
        string_out.push_str( &(format!("\tclass:       {}\n", self.get_ei_class())));
        string_out.push_str( &(format!("\tencoding:    {}\n", self.get_ei_encoding())));
        string_out.push_str( &(format!("\tversion:     {}\n", self.get_ei_version())));
        string_out.push_str( &(format!("\tOS ABI:      {}\n", self.get_ei_osabi())));
        string_out.push_str( &(format!("\tABI version: {}\n", self.get_ei_abiversion())));
        return Ok(string_out);
    }
    //Private helpers for fmt_cli_out
    fn get_type(&self) -> String{
        let mut type_string = String::new();
        for byte in 0..4 {                                     // Get magic header
            type_string.push_str(&(format!("{}",self.magic_number[byte] as char)).to_string());
        }
        return type_string;
    }
    fn get_ei_class(&self) -> String {
        match self.ei_class as u16 {                    
            ELFCLASS32 => return String::from("32bit"),
            ELFCLASS64 => return String::from("64bit"),
            _ => return String::from("Invalid EI Class"),
        }
    }
    fn get_ei_encoding(&self) -> String {
        match self.ei_data {
            1 => return String::from("little endian"),
            2 => return String::from("big endian"),
            _ => return String::from("Failed to find valid Data Format entry."),
        };
    }
    fn get_ei_version(&self) -> String {
        match self.ei_version{
            1 => return String::from("Current Version"),
            _ => return String::from("Invalid Version"),
        }
    }
    fn get_ei_osabi(&self) -> String {
        match self.ei_osabi {
            0  => return String::from("Unix System V"),
            1  => return String::from("HP-UX"),
            2  => return String::from("NetBSD"),
            3  => return String::from("Linux"),
            6  => return String::from("Solaris"),
            7  => return String::from("IBM AIX"),
            8  => return String::from("IRIX"),
            9  => return String::from("FreeBSD"),
            10 => return String::from("TRU64 Unix"),
            11 => return String::from("Novel Modest"),
            12 => return String::from("OpenBSD"),
            64...255 => return String::from("Architecture Specific"),
            _ => return String::from("Failed to find valid OS/ABI entry."),
        } 
    }
    fn get_ei_abiversion(&self) -> String {
        match self.ei_abiversion{
            0 => return String::from("Unspecified"),
            _ => return String::from("Other"),
        }
    }
}

type ELF32 = ELFHeader32;
pub struct ELFHeader32{
  pub e_ident:     EIdentStruct,    // Magic Number and other useful information
  pub e_type:      Elf32Half,       // Object file type
  pub e_machine:   Elf32Half,       // Architecture
  pub e_version:   Elf32Word,       // Object file version
  pub e_entry:     Elf32Addr,       // Entry point virtual address
  pub e_phoff:     Elf32Off,        // Program header table offset          
  pub e_shoff:     Elf32Off,        // Section header table offset              
  pub e_flags:     Elf32Word,       // Processor-specific flags
  pub e_ehsize:    Elf32Half,       // Elf Header size
  pub e_phentsize: Elf32Half,       // Program header table entry size
  pub e_phnum:     Elf32Half,       // Program header table entry count
  pub e_shentsize: Elf32Half,       // Section header table entry size
  pub e_shnum:     Elf32Half,       // Section header table entry count              
  pub e_shstrndx:  Elf32Half        // Section header string table index
}
impl ELFHeader32{
    pub fn new(bytes: &[u8], e_ident: EIdentStruct) -> ELFHeader32 {
        let e_header = ELFHeader32 {e_ident:     e_ident,
                                    e_type:      vec_slice_to_u16_array(&bytes[0 .. 2]).unwrap(),
                                    e_machine:   vec_slice_to_u16_array(&bytes[2 .. 4]).unwrap(),
                                    e_version:   vec_slice_to_u32_array(&bytes[4 .. 8]).unwrap(),
                                    e_entry:     vec_slice_to_u32_array(&bytes[8 .. 12]).unwrap(),
                                    e_phoff:     vec_slice_to_u32_array(&bytes[12 .. 16]).unwrap(),
                                    e_shoff:     vec_slice_to_u32_array(&bytes[16 .. 20]).unwrap(),
                                    e_flags:     vec_slice_to_u32_array(&bytes[20 .. 24]).unwrap(),
                                    e_ehsize:    vec_slice_to_u16_array(&bytes[24 .. 26]).unwrap(),
                                    e_phentsize: vec_slice_to_u16_array(&bytes[26 .. 28]).unwrap(),
                                    e_phnum:     vec_slice_to_u16_array(&bytes[28 .. 30]).unwrap(),
                                    e_shentsize: vec_slice_to_u16_array(&bytes[30 .. 32]).unwrap(),
                                    e_shnum:     vec_slice_to_u16_array(&bytes[32 .. 34]).unwrap(),
                                    e_shstrndx:  vec_slice_to_u16_array(&bytes[34 .. 36]).unwrap(),
                                    };
        return e_header;
    }
    //pub fn validate(&self) -> bool {
        // Fill out this function to valiate the header struct. 

    //}
    pub fn fmt_cli_out(&self) -> String {   
        let mut string_out = String::new();
        /**
         * To do the emachine portion just write a python script to 
         * read in the file and look for he emachine values then
         * reformat them to the appropriate rust code.
         */

        string_out.push_str( &self.e_ident.fmt_cli_out().unwrap());
        string_out.push_str( &(format!("\teident size: {:X}\n", self.e_ident.ei_nident)));
        string_out.push_str( &(format!("\ttype:      {:X}\n",   self.e_type)));
        string_out.push_str( &(format!("\te_machine: {}\n", self.get_e_machine())));
        string_out.push_str( &(format!("\te_version: {:X}\n",   self.e_version.swap_bytes())));
        string_out.push_str( &(format!("\te_entry:   {:X}\n",   self.e_entry.swap_bytes())));
        string_out.push_str( &(format!("\te_phoff:   {}\n",     self.e_phoff.swap_bytes())));
        string_out.push_str( &(format!("\te_shstridx {}\n",     self.e_shstrndx.swap_bytes())));
        return string_out;
    }
    fn get_e_machine(&self) -> String{
        match self.e_machine.swap_bytes(){
            0 => return String::from("No Machine"),
            1 => return String::from("AT&T WE 32100"),
            2 => return String::from("SPARC"),
            3 => return String::from("Intel 80386"),
            _ => return String::from("To be implemented"),
        }
    }
}

type ELF64 = ELFHeader64;
pub struct ELFHeader64{
  pub e_ident:    EIdentStruct,     // Magic Number and other useful information
  pub e_type:      Elf64Half,       // Object file type
  pub e_machine:   Elf64Half,       // Architecture
  pub e_version:   Elf64Word,       // Object file version
  pub e_entry:     Elf64Addr,       // Entry point virtual address
  pub e_phoff:     Elf64Off,        // Program header table offset          
  pub e_shoff:     Elf64Off,        // Section header table offset              
  pub e_flags:     Elf64Word,       // Processor-specific flags
  pub e_ehsize:    Elf64Half,       // Elf Header size
  pub e_phentsize: Elf64Half,       // Program header table entry size
  pub e_phnum:     Elf64Half,       // Program header table entry count
  pub e_shentsize: Elf64Half,       // Section header table entry size
  pub e_shnum:     Elf64Half,       // Section header table entry count              
  pub e_shstrndx:  Elf64Half        // Section header string table index
}
impl ELFHeader64 {
    pub fn new(bytes: &[u8], e_ident: EIdentStruct) -> ELFHeader64 {
        
        let e_header = ELFHeader64 {e_ident:     e_ident,
                                    e_type:      vec_slice_to_u16_array(&bytes[0 .. 2]).unwrap(),
                                    e_machine:   vec_slice_to_u16_array(&bytes[2 .. 4]).unwrap(),
                                    e_version:   vec_slice_to_u32_array(&bytes[4 .. 8]).unwrap(),
                                    e_entry:     vec_slice_to_u64_array(&bytes[8 .. 16]).unwrap(),
                                    e_phoff:     vec_slice_to_u64_array(&bytes[16 .. 24]).unwrap(),
                                    e_shoff:     vec_slice_to_u64_array(&bytes[24 .. 32]).unwrap(),
                                    e_flags:     vec_slice_to_u32_array(&bytes[32 .. 36]).unwrap(),
                                    e_ehsize:    vec_slice_to_u16_array(&bytes[36 .. 38]).unwrap(),
                                    e_phentsize: vec_slice_to_u16_array(&bytes[38 .. 40]).unwrap(),
                                    e_phnum:     vec_slice_to_u16_array(&bytes[40 .. 42]).unwrap(),
                                    e_shentsize: vec_slice_to_u16_array(&bytes[42 .. 44]).unwrap(),
                                    e_shnum:     vec_slice_to_u16_array(&bytes[44 .. 46]).unwrap(),
                                    e_shstrndx:  vec_slice_to_u16_array(&bytes[46 .. 48]).unwrap(),
                                    };                                           
        return e_header;
    }
}























