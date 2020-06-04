
/* File that holds typedefs defined in elf.h
 * and some closely related custom defined
 * types (defined toward top)
 */
 use crate::elf::enumerations::*;

pub type Elf32   = Elf32Types;
pub type Elf64   = Elf64Types;

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

/* Constants for EIdent fields */
pub const ELFMAG:              &str = "177ELF"; //
pub const SELFMAG:             u16 = 4; //
pub const EI_CLASS:            u16 = 4; // class byte index
pub const ELFCLASSNONE:        u16 = 0; // class
pub const ELFCLASS32:          u16 = 1; // objects
pub const ELFCLASS64:          u16 = 2; // objects
pub const ELFCLASSNUM:         u16 = 3; //
pub const EI_DATA:             u16 = 5; // encoding byte index
pub const ELFDATANONE:         u16 = 0; // data encoding
pub const ELFDATA2LSB:         u16 = 1; // complement, little endian
pub const ELFDATA2MSB:         u16 = 2; // complement, big endian
pub const ELFDATANUM:          u16 = 3; //
pub const EI_VERSION:          u16 = 6; // version byte index
pub const EI_OSABI:            u16 = 7; // ABI identification
pub const ELFOSABI_NONE:       u16 = 0; // System V ABI
pub const ELFOSABI_SYSV:       u16 = 0; //
pub const ELFOSABI_HPUX:       u16 = 1; //
pub const ELFOSABI_NETBSD:     u16 = 2; //
pub const ELFOSABI_GNU:        u16 = 3; // uses GNU ELF extensions.
pub const ELFOSABI_LINUX:      u16 = ELFOSABI_GNU; // alias.
pub const ELFOSABI_SOLARIS:    u16 = 6; // Solaris.
pub const ELFOSABI_AIX:        u16 = 7; // AIX.
pub const ELFOSABI_IRIX:       u16 = 8; // Irix.
pub const ELFOSABI_FREEBSD:    u16 = 9; //
pub const ELFOSABI_TRU64:      u16 = 10; // TRU64 UNIX.
pub const ELFOSABI_MODESTO:    u16 = 11; // Modesto.
pub const ELFOSABI_OPENBSD:    u16 = 12; //
pub const ELFOSABI_ARM_AEABI:  u16 = 64; // EABI
pub const ELFOSABI_ARM:        u16 = 97; //
pub const ELFOSABI_STANDALONE: u16 = 255; // (embedded) application
pub const EI_ABIVERSION:       u16 = 8; // version
pub const EI_PAD:              u16 = 9; // index of padding bytes

/* Legal values for e_type (object file type).  */
pub const ET_NONE:   u16 = 0; // file type
pub const ET_REL:    u16 = 1; // file
pub const ET_EXEC:   u16 = 2; // file
pub const ET_DYN:    u16 = 3; // object file
pub const ET_CORE:   u16 = 4; // file
pub const ET_NUM:    u16 = 5; // of defined types
pub const ET_LOOS:   u16 = 0xfe00; // range start
pub const ET_HIOS:   u16 = 0xfeff; // range end
pub const ET_LOPROC: u16 = 0xff00; // range start
pub const ET_HIPROC: u16 = 0xffff; // range end

/* Legal values for e_machine (architecture).  */
pub const EM_NONE:          u16 = 0;   // No machine
pub const EM_M32:           u16 = 1;   // WE 32100
pub const EM_SPARC:         u16 = 2;   // SPARC
pub const EM_386:           u16 = 3;   // 80386
pub const EM_68K:           u16 = 4;   // m68k family
pub const EM_88K:           u16 = 5;   // m88k family
pub const EM_IAMCU:         u16 = 6;   // MCU
pub const EM_860:           u16 = 7;   // 80860
pub const EM_MIPS:          u16 = 8;   // R3000 big-endian
pub const EM_S370:          u16 = 9;   // System/370
pub const EM_MIPS_RS3_LE:   u16 = 10;  // R3000 little-endian
pub const EM_PARISC:        u16 = 15;  //
pub const EM_VPP500:        u16 = 17;  // VPP500
pub const EM_SPARC32PLUS:   u16 = 18;  // "v8plus"
pub const EM_960:           u16 = 19;  // 80960
pub const EM_PPC:           u16 = 20;  //
pub const EM_PPC64:         u16 = 21;  // 64-bit
pub const EM_S390:          u16 = 22;  // S390
pub const EM_SPU:           u16 = 23;  // SPU/SPC
pub const EM_V800:          u16 = 36;  // V800 series
pub const EM_FR20:          u16 = 37;  // FR20
pub const EM_RH32:          u16 = 38;  // RH-32
pub const EM_RCE:           u16 = 39;  // RCE
pub const EM_ARM:           u16 = 40;  //
pub const EM_FAKE_ALPHA:    u16 = 41;  // Alpha
pub const EM_SH:            u16 = 42;  // SH
pub const EM_SPARCV9:       u16 = 43;  // v9 64-bit
pub const EM_TRICORE:       u16 = 44;  // Tricore
pub const EM_ARC:           u16 = 45;  // RISC Core
pub const EM_H8_300:        u16 = 46;  // H8/300
pub const EM_H8_300H:       u16 = 47;  // H8/300H
pub const EM_H8S:           u16 = 48;  // H8S
pub const EM_H8_500:        u16 = 49;  // H8/500
pub const EM_IA_64:         u16 = 50;  // Merced
pub const EM_MIPS_X:        u16 = 51;  // MIPS-X
pub const EM_COLDFIRE:      u16 = 52;  // Coldfire
pub const EM_68HC12:        u16 = 53;  // M68HC12
pub const EM_MMA:           u16 = 54;  // MMA Multimedia Accelerator
pub const EM_PCP:           u16 = 55;  // PCP
pub const EM_NCPU:          u16 = 56;  // nCPU embeeded RISC
pub const EM_NDR1:          u16 = 57;  // NDR1 microprocessor
pub const EM_STARCORE:      u16 = 58;  // Start*Core processor
pub const EM_ME16:          u16 = 59;  // ME16 processor
pub const EM_ST100:         u16 = 60;  // ST100 processor
pub const EM_TINYJ:         u16 = 61;  // Logic Corp. Tinyj emb.fam
pub const EM_X86_64:        u16 = 62;  // x86-64 architecture
pub const EM_PDSP:          u16 = 63;  // DSP Processor
pub const EM_PDP10:         u16 = 64;  // PDP-10
pub const EM_PDP11:         u16 = 65;  // PDP-11
pub const EM_FX66:          u16 = 66;  // FX66 microcontroller
pub const EM_ST9PLUS:       u16 = 67;  // ST9+ 8/16 mc
pub const EM_ST7:           u16 = 68;  // ST7 8 bit mc
pub const EM_68HC16:        u16 = 69;  // MC68HC16 microcontroller
pub const EM_68HC11:        u16 = 70;  // MC68HC11 microcontroller
pub const EM_68HC08:        u16 = 71;  // MC68HC08 microcontroller
pub const EM_68HC05:        u16 = 72;  // MC68HC05 microcontroller
pub const EM_SVX:           u16 = 73;  // Graphics SVx
pub const EM_ST19:          u16 = 74;  // ST19 8 bit mc
pub const EM_VAX:           u16 = 75;  // VAX
pub const EM_CRIS:          u16 = 76;  // Communications 32-bit emb.proc
pub const EM_JAVELIN:       u16 = 77;  // Technologies 32-bit emb.proc
pub const EM_FIREPATH:      u16 = 78;  // 14 64-bit DSP Processor
pub const EM_ZSP:           u16 = 79;  // Logic 16-bit DSP Processor
pub const EM_MMIX:          u16 = 80;  // Knuth's educational 64-bit proc
pub const EM_HUANY:         u16 = 81;  // University machine-independent object files
pub const EM_PRISM:         u16 = 82;  // Prism
pub const EM_AVR:           u16 = 83;  // AVR 8-bit microcontroller
pub const EM_FR30:          u16 = 84;  // FR30
pub const EM_D10V:          u16 = 85;  // D10V
pub const EM_D30V:          u16 = 86;  // D30V
pub const EM_V850:          u16 = 87;  // v850
pub const EM_M32R:          u16 = 88;  // M32R
pub const EM_MN10300:       u16 = 89;  // MN10300
pub const EM_MN10200:       u16 = 90;  // MN10200
pub const EM_PJ:            u16 = 91;  //
pub const EM_OPENRISC:      u16 = 92;  // 32-bit embedded processor
pub const EM_ARC_COMPACT:   u16 = 93;  // International ARCompact
pub const EM_XTENSA:        u16 = 94;  // Xtensa Architecture
pub const EM_VIDEOCORE:     u16 = 95;  // VideoCore
pub const EM_TMM_GPP:       u16 = 96;  // Multimedia General Purpose Proc
pub const EM_NS32K:         u16 = 97;  // Semi. 32000
pub const EM_TPC:           u16 = 98;  // Network TPC
pub const EM_SNP1K:         u16 = 99;  // SNP 1000
pub const EM_ST200:         u16 = 100; // ST200
pub const EM_IP2K:          u16 = 101; // IP2xxx
pub const EM_MAX:           u16 = 102; // processor
pub const EM_CR:            u16 = 103; // Semi. CompactRISC
pub const EM_F2MC16:        u16 = 104; // F2MC16
pub const EM_MSP430:        u16 = 105; // Instruments msp430
pub const EM_BLACKFIN:      u16 = 106; // Devices Blackfin DSP
pub const EM_SE_C33:        u16 = 107; // Epson S1C33 family
pub const EM_SEP:           u16 = 108; // embedded microprocessor
pub const EM_ARCA:          u16 = 109; // RISC
pub const EM_UNICORE:       u16 = 110; // & MPRC Peking Uni. mc series
pub const EM_EXCESS:        u16 = 111; // configurable cpu
pub const EM_DXP:           u16 = 112; // Semi. Deep Execution Processor
pub const EM_ALTERA_NIOS2:  u16 = 113; // Nios II
pub const EM_CRX:           u16 = 114; // Semi. CompactRISC CRX
pub const EM_XGATE:         u16 = 115; // XGATE
pub const EM_C166:          u16 = 116; // C16x/XC16x
pub const EM_M16C:          u16 = 117; // M16C
pub const EM_DSPIC30F:      u16 = 118; // Technology dsPIC30F
pub const EM_CE:            u16 = 119; // Communication Engine RISC
pub const EM_M32C:          u16 = 120; // M32C
pub const EM_TSK3000:       u16 = 131; // TSK3000
pub const EM_RS08:          u16 = 132; // RS08
pub const EM_SHARC:         u16 = 133; // Devices SHARC family
pub const EM_ECOG2:         u16 = 134; // Technology eCOG2
pub const EM_SCORE7:        u16 = 135; // S+core7 RISC
pub const EM_DSP24:         u16 = 136; // Japan Radio (NJR) 24-bit DSP
pub const EM_VIDEOCORE3:    u16 = 137; // VideoCore III
pub const EM_LATTICEMICO32: u16 = 138; // for Lattice FPGA
pub const EM_SE_C17:        u16 = 139; // Epson C17
pub const EM_TI_C6000:      u16 = 140; // Instruments TMS320C6000 DSP
pub const EM_TI_C2000:      u16 = 141; // Instruments TMS320C2000 DSP
pub const EM_TI_C5500:      u16 = 142; // Instruments TMS320C55x DSP
pub const EM_TI_ARP32:      u16 = 143; // Instruments App. Specific RISC
pub const EM_TI_PRU:        u16 = 144; // Instruments Prog. Realtime Unit
pub const EM_MMDSP_PLUS:    u16 = 160; // 64bit VLIW DSP
pub const EM_CYPRESS_M8C:   u16 = 161; // M8C
pub const EM_R32C:          u16 = 162; // R32C
pub const EM_TRIMEDIA:      u16 = 163; // Semi. TriMedia
pub const EM_QDSP6:         u16 = 164; // DSP6
pub const EM_8051:          u16 = 165; // 8051 and variants
pub const EM_STXP7X:        u16 = 166; // STxP7x
pub const EM_NDS32:         u16 = 167; // Tech. compact code emb. RISC
pub const EM_ECOG1X:        u16 = 168; // Technology eCOG1X
pub const EM_MAXQ30:        u16 = 169; // Semi. MAXQ30 mc
pub const EM_XIMO16:        u16 = 170; // Japan Radio (NJR) 16-bit DSP
pub const EM_MANIK:         u16 = 171; // Reconfigurable RISC
pub const EM_CRAYNV2:       u16 = 172; // NV2 vector architecture
pub const EM_RX:            u16 = 173; // RX
pub const EM_METAG:         u16 = 174; // Tech. META
pub const EM_MCST_ELBRUS:   u16 = 175; // Elbrus
pub const EM_ECOG16:        u16 = 176; // Technology eCOG16
pub const EM_CR16:          u16 = 177; // Semi. CompactRISC CR16
pub const EM_ETPU:          u16 = 178; // Extended Time Processing Unit
pub const EM_SLE9X:         u16 = 179; // Tech. SLE9X
pub const EM_L10M:          u16 = 180; // L10M
pub const EM_K10M:          u16 = 181; // K10M
pub const EM_AARCH64:       u16 = 183; // AARCH64
pub const EM_AVR32:         u16 = 185; // 32-bit microprocessor
pub const EM_STM8:          u16 = 186; // STM8
pub const EM_TILE64:        u16 = 187; // TILE64
pub const EM_TILEPRO:       u16 = 188; // TILEPro
pub const EM_MICROBLAZE:    u16 = 189; // MicroBlaze
pub const EM_CUDA:          u16 = 190; // CUDA
pub const EM_TILEGX:        u16 = 191; // TILE-Gx
pub const EM_CLOUDSHIELD:   u16 = 192; //
pub const EM_COREA_1ST:     u16 = 193; // Core-A 1st gen.
pub const EM_COREA_2ND:     u16 = 194; // Core-A 2nd gen.
pub const EM_ARC_COMPACT2:  u16 = 195; // ARCompact V2
pub const EM_OPEN8:         u16 = 196; // RISC
pub const EM_RL78:          u16 = 197; // RL78
pub const EM_VIDEOCORE5:    u16 = 198; // VideoCore V
pub const EM_78KOR:         u16 = 199; // 78KOR
pub const EM_56800EX:       u16 = 200; // 56800EX DSC
pub const EM_BA1:           u16 = 201; // BA1
pub const EM_BA2:           u16 = 202; // BA2
pub const EM_XCORE:         u16 = 203; // xCORE
pub const EM_MCHP_PIC:      u16 = 204; // 8-bit PIC(r)
pub const EM_KM32:          u16 = 210; // KM32
pub const EM_KMX32:         u16 = 211; // KMX32
pub const EM_EMX16:         u16 = 212; // KMX16
pub const EM_EMX8:          u16 = 213; // KMX8
pub const EM_KVARC:         u16 = 214; // KVARC
pub const EM_CDP:           u16 = 215; // CDP
pub const EM_COGE:          u16 = 216; // Smart Memory Processor
pub const EM_COOL:          u16 = 217; // CoolEngine
pub const EM_NORC:          u16 = 218; // Optimized RISC
pub const EM_CSR_KALIMBA:   u16 = 219; // Kalimba
pub const EM_Z80:           u16 = 220; // Z80
pub const EM_VISIUM:        u16 = 221; // and Data Services VISIUMcore
pub const EM_FT32:          u16 = 222; // Chip FT32
pub const EM_MOXIE:         u16 = 223; // processor
pub const EM_AMDGPU:        u16 = 224; // GPU
pub const EM_RISCV:         u16 = 243; //
pub const EM_BPF:           u16 = 247; // BPF -- in-kernel virtual machine
pub const EM_NUM:           u16 = 248; //
