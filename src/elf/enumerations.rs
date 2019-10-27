
/* Fields in the e_ident array.  The EI_* macros are indices into the
   array.  The macros under each EI_* macro are the values the byte
   may have.  */
pub enum ELFMAG_Bit {
    EI_MAG0	= 0,		/* File identification byte 0 index */
    EI_MAG1	= 1,		/* File identification byte 1 index */
    EI_MAG2	= 2,		/* File identification byte 2 index */
    EI_MAG3	= 3,		/* File identification byte 3 index */
}

pub enum ELFMAG_Byte {
    ELFMAG0 = 0x7f,  	/* Magic number byte 0 */
    ELFMAG1	= 0x45,		/* Magic number byte 1 */
    ELFMAG2	= 0x4C,		/* Magic number byte 2 */
    ELFMAG3	= 0x46,		/* Magic number byte 3 */
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
