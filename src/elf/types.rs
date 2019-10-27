/**
 * File that holds typedefs defined in elf.h
 */


/* types for a 16-bit quantity. */
pub type Elf32Half = u16;
pub type Elf64Half = u16;

/* Types for signed and unsigend 32-bit quantities. */
pub type Elf32Word  = u32;
pub type Elf32Sword = i32;
pub type Elf64Word  = u32; 
pub type Elf64Sword = i32;

/* Types for signed and unsigned 64-bit quantities. */
pub type Elf32Xword  = u64;
pub type Elf32Sxword = i64;
pub type Elf64Xword  = u64;
pub type Elf64Sxword = i64;

/* Type of adddresses. */
pub type Elf32Addr = u32;
pub type Elf64Addr = u64;

/* Type of file offsets. */
pub type Elf32Off = u32;
pub type Elf64Off = u64;

/* Type for section indices, which are 16-bit quantitiies. */
pub type Elf32Section = u16;
pub type Elf64Section = u16;

/* Type for version symbol information. */
pub type Elf32Versym = Elf32Half; 
pub type Elf64versym = Elf64Half;

pub const EI_CLASS:      u8 = 4;
pub const EI_DATA:       u8 = 5;
pub const EI_VERSION:    u8 = 6;
pub const EI_OSABI:      u8 = 7;
pub const EI_ABIVERSION: u8 = 8;
pub const EI_PAD:        u8 = 9;
