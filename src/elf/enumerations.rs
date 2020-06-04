use crate::elf::structures::*;
use crate::elf::types::*;


/* Fields in the e_ident array.  The EI_* macros are indices into the
   array.  The macros under each EI_* macro are the values the byte
   may have.  */
#[allow(non_camel_case_types)]
pub enum ELFMAG_Bit {
    EI_MAG0	= 0,		/* File identification byte 0 index */
    EI_MAG1	= 1,		/* File identification byte 1 index */
    EI_MAG2	= 2,		/* File identification byte 2 index */
    EI_MAG3	= 3,		/* File identification byte 3 index */
}
#[allow(non_camel_case_types)]
pub enum ELFMAG_Byte {
    ELFMAG0 = 0x7f,  	/* Magic number byte 0 */
    ELFMAG1	= 0x45,		/* Magic number byte 1 */
    ELFMAG2	= 0x4C,		/* Magic number byte 2 */
    ELFMAG3	= 0x46,		/* Magic number byte 3 */
}

pub enum Elf {
    Elf32,
    Elf64,
}

pub trait ELF {}
pub trait Selectable<T32,T64> {
    fn is_elf32_type(&self) -> bool;
    fn is_elf64_type(&self) -> bool;
    fn get_elf32_value(&self) -> Option<T32>;
    fn get_elf64_value(&self) -> Option<T64>;
}

//=================
// Structure types =
//===================
pub enum Elf32Types {
    Elf32Half,
    Elf32Word,
    Elf32Addr,
    Elf32Off,
} impl ELF for Elf32Types{}

pub enum Elf64Types {
    Elf64Half,
    Elf64Word,
    Elf64Addr,
    Elf64Off
} impl ELF for Elf64Types{}

//=======================
// Structure Field Types =
//=========================
pub enum ElfTypes {
    Half,
    Word,
    Addr,
    Offset,
} impl ELF for ElfTypes{}

#[derive(Debug, Copy, Clone)]
pub enum Half {
    Elf32(Elf32Half),
    Elf64(Elf64Half)
}
impl Selectable<Elf32Half, Elf64Half> for Half {
    fn is_elf32_type(&self) -> bool{
        match self {
            Half::Elf32(_) => true,
            _ => false,
        }
    }
    fn is_elf64_type(&self) -> bool{
        match self {
            Half::Elf64(_) => true,
            _ => false,
        }
    }
    fn get_elf32_value(&self) -> Option<Elf32Half>{
        match self {
            Half::Elf32(v) => Some(v.to_owned()),
            _ => None,
        }
    }
    fn get_elf64_value(&self) -> Option<Elf64Half>{
        match self {
            Half::Elf64(v) => Some(v.to_owned()),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Word {
    Elf32(Elf32Word),
    Elf64(Elf64Word)
}
impl Selectable<Elf32Word, Elf64Word> for Word {
    fn is_elf32_type(&self) -> bool{
        match self {
            Word::Elf32(_) => true,
            _ => false,
        }
    }
    fn is_elf64_type(&self) -> bool{
        match self {
            Word::Elf64(_) => true,
            _ => false,
        }
    }
    fn get_elf32_value(&self) -> Option<Elf32Word>{
        match self {
            Word::Elf32(v) => Some(v.to_owned()),
            _ => None,
        }
    }
    fn get_elf64_value(&self) -> Option<Elf64Word>{
        match self {
            Word::Elf64(v) => Some(v.to_owned()),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Addr {
    Elf32(Elf32Addr),
    Elf64(Elf64Addr),
}
impl Selectable<Elf32Addr, Elf64Addr> for Addr {
    fn is_elf32_type(&self) -> bool{
        match self {
            Addr::Elf32(_) => true,
            _ => false,
        }
    }
    fn is_elf64_type(&self) -> bool{
        match self {
            Addr::Elf64(_) => true,
            _ => false,
        }
    }
    fn get_elf32_value(&self) -> Option<Elf32Addr>{
        match self {
            Addr::Elf32(v) => Some(v.to_owned()),
            _ => None,
        }
    }
    fn get_elf64_value(&self) -> Option<Elf64Addr>{
        match self {
            Addr::Elf64(v) => Some(v.to_owned()),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Offset {
    Elf32(Elf32Off),
    Elf64(Elf64Off),
}
impl Selectable<Elf32Off, Elf64Off> for Offset {
    fn is_elf32_type(&self) -> bool{
        match self {
            Offset::Elf32(_) => true,
            _ => false,
        }
    }
    fn is_elf64_type(&self) -> bool{
        match self {
            Offset::Elf64(_) => true,
            _ => false,
        }
    }
    fn get_elf32_value(&self) -> Option<Elf32Off>{
        match self {
            Offset::Elf32(v) => Some(v.to_owned()),
            _ => None,
        }
    }
    fn get_elf64_value(&self) -> Option<Elf64Off>{
        match self {
            Offset::Elf64(v) => Some(v.to_owned()),
            _ => None,
        }
    }
}




/*
#define EI_CLASS	4		/* File class byte index */
#define ELFCLASSNONE	0		/* Invalid class */
#define ELFCLASS32	1		/* 32-bit objects */
#define ELFCLASS64	2		/* 64-bit objects */
#define ELFCLASSNUM	3

#define EI_DATA		5		/* Data encoding byte index */
#define ELFDATANONE	0		/* Invalid data encoding */
#define ELFDATA2LSB	1		/* 2's complement, little endian */
#define ELFDATA2MSB	2		/* 2's complement, big endian */
#define ELFDATANUM	3

#define EI_VERSION	6		/* File version byte index */
					/* Value must be EV_CURRENT */

#define EI_OSABI	7		/* OS ABI identification */
#define ELFOSABI_NONE		0	/* UNIX System V ABI */
#define ELFOSABI_SYSV		0	/* Alias.  */
#define ELFOSABI_HPUX		1	/* HP-UX */
#define ELFOSABI_NETBSD		2	/* NetBSD.  */
#define ELFOSABI_GNU		3	/* Object uses GNU ELF extensions.  */
#define ELFOSABI_LINUX		ELFOSABI_GNU /* Compatibility alias.  */
#define ELFOSABI_SOLARIS	6	/* Sun Solaris.  */
#define ELFOSABI_AIX		7	/* IBM AIX.  */
#define ELFOSABI_IRIX		8	/* SGI Irix.  */
#define ELFOSABI_FREEBSD	9	/* FreeBSD.  */
#define ELFOSABI_TRU64		10	/* Compaq TRU64 UNIX.  */
#define ELFOSABI_MODESTO	11	/* Novell Modesto.  */
#define ELFOSABI_OPENBSD	12	/* OpenBSD.  */
#define ELFOSABI_ARM_AEABI	64	/* ARM EABI */
#define ELFOSABI_ARM		97	/* ARM */
#define ELFOSABI_STANDALONE	255	/* Standalone (embedded) application */

#define EI_ABIVERSION	8		/* ABI version */

#define EI_PAD		9		/* Byte index of padding bytes */
*/
