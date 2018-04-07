
/// TRIPLE_READ_SLOW_READ  for PBIST and STC ROM
pub const TRIPLEREADSLOW:u32     = 0x00000001;
/// TRIPLE_READ_SLOW_READ  for PBIST and STC ROM
pub const TRIPLEREADFAST:u32     = 0x00000002;
/// March13 N Algo for 2 Port mem
pub const MARCH13N_DP:u32        = 0x00000004;
/// March13 N Algo for 1 Port mem
pub const MARCH13N_SP:u32        = 0x00000008;
/// Down1a Algor forces the switching fo all data bits & most addr
/// bits on consecutive read cycles
pub const DOWN1A_DP:u32          = 0x00000010;
/// Down1a Algor forces the switching fo all data bits & most addr
/// bits on consecutive read cycles
pub const DOWN1A_SP:u32          = 0x00000020;
/// Map Column algo (to identify bit line senstivities) for 2 Port memory
pub const MAPCOLUMN_DP:u32       = 0x00000040;
/// Map Column algo (to identify bit line senstivities) for 1 Port memory
pub const MAPCOLUMN_SP:u32       = 0x00000080;
/// Pre-Charge algo to exercise pre-charge capability for 2 port memory
pub const PRECHARGE_DP:u32       = 0x00000100;
/// Pre-Charge algo to exercise pre-charge capability for 1 port memory
pub const PRECHARGE_SP:u32       = 0x00000200;
/// Global column decode logic algo for 2 Port memories
pub const DTXN2A_DP:u32          = 0x00000400;
/// Global column decode logic algo for 1 Port memories
pub const DTXN2A_SP:u32          = 0x00000800;
/// pmos oper algo for 2 port memories
pub const PMOSOPEN_DP:u32        = 0x00001000;
/// pmos oper algo for 1 port memories
pub const PMOSOPEN_SP:u32        = 0x00002000;
/// pmos open slice1 for 2port memories
pub const PPMOSOPENSLICE1_DP:u32 = 0x00004000;
/// pmos open slice2 for 2port memories
pub const PPMOSOPENSLICE2_DP:u32 = 0x00008000;
/// flip10 algo for 2 port memories
pub const FLIP10_DP:u32          = 0x00010000;
/// flip10  algo for 1 port memories
pub const FLIP10_SP:u32          = 0x00020000;
/// iddq  algo for 2 port memories
pub const IDDQ_DP:u32            = 0x00040000;
/// iddq  algo for 1 port memories
pub const IDDQ_SP:u32            = 0x00080000;
/// retention  algo for 2 port memories
pub const RETENTION_DP:u32       = 0x00100000;
/// retention  algo for 1 port memories
pub const RETENTION_SP:u32       = 0x00200000;
/// iddq2 algo for 2 port memories
pub const IDDQ2_DP:u32           = 0x00400000;
/// iddq2 algo for 1 port memories
pub const IDDQ2_SP:u32           = 0x00800000;
/// retention2  algo for 2 port memories
pub const RETENTION2_DP:u32      = 0x01000000;
/// retention2  algo for 1 port memories
pub const RETENTION2_SP:u32      = 0x02000000;
/// iddqwstripe  algo for 2 port memories
pub const IDDQROWSTRIPE_DP:u32   = 0x04000000;
/// iddqwstripe  algo for 1 port memories
pub const IDDQROWSTRIPE_SP:u32   = 0x08000000;
/// iddqwstripe2  algo for 2 port memories
pub const IDDQROWSTRIPE2_DP:u32  = 0x10000000;
/// iddqwstripe2  algo for 1 port memories
pub const IDDQROWSTRIPE2_SP:u32  = 0x20000000;


pub const PBIST_ROM:u32         = 0x1 << (1 - 1);
pub const STC_ROM:u32           = 0x1 << (2 - 1);
pub const DCAN1:u32             = 0x1 << (3 - 1);
pub const DCAN2:u32             = 0x1 << (4 - 1);
pub const DCAN3:u32             = 0x1 << (5 - 1);
pub const ESRAM1:u32            = 0x1 << (6 - 1);
pub const MIBSPI1:u32           = 0x1 << (7 - 1);
pub const MIBSPI3:u32           = 0x1 << (8 - 1);
pub const MIBSPI5:u32           = 0x1 << (9 - 1);
pub const VIM:u32               = 0x1 << (10 - 1);
pub const MIBADC1:u32           = 0x1 << (11 - 1);
pub const DMA:u32               = 0x1 << (12 - 1);
pub const N2HET1:u32            = 0x1 << (13 - 1);
pub const HETTU1:u32            = 0x1 << (14 - 1);
pub const RTP:u32               = 0x1 << (15 - 1);
pub const FLEXRAY_SINGLE:u32    = 0x1 << (16 - 1);
pub const FLEXRAY_DUAL:u32      = 0x1 << (17 - 1);
pub const MIBADC2:u32           = 0x1 << (18 - 1);
pub const N2HET2:u32            = 0x1 << (19 - 1);
pub const HETTU2:u32            = 0x1 << (20 - 1);
pub const ESRAM5:u32            = 0x1 << (21 - 1);
pub const ESRAM6:u32            = 0x1 << (22 - 1);
pub const ETHERNET_SINGLE:u32   = 0x1 << (23 - 1);
pub const ETHERNET_DUAL1:u32    = 0x1 << (24 - 1);
pub const ETHERNET_DUAL2:u32    = 0x1 << (25 - 1);
pub const ESRAM8:u32            = 0x1 << (28 - 1);
