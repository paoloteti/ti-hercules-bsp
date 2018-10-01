///
/// DCAN Controller Area Network module
///
/// The Controller Area Network is a high integrity serial communications
/// protocol for distributed real-time applications.
/// The DCAN module supports bit rates up to 1 Mbit/s and is compliant
/// to the CAN 2.0B protocol specification.
///

use vcell::VolatileCell;

#[repr(C)]
#[allow(non_snake_case)]
struct CanRegisters {
    CTL: VolatileCell<u32>,                 // 0x0000: Control
    ES: VolatileCell<u32>,                  // 0x0004: Error and Status
    EERC: VolatileCell<u32>,                // 0x0008: Error Counter
    BTR: VolatileCell<u32>,                 // 0x000C: Bit Timing
    INT: VolatileCell<u32>,                 // 0x0010: Interrupt
    TEST: VolatileCell<u32>,                // 0x0014: Test
    _reserved1: VolatileCell<u32>,          // 0x0018: Reserved
    PERR: VolatileCell<u32>,                // 0x001C: Parity/SECDED Error Code
    _reserved2: [VolatileCell<u32>; 24],    // 0x002C - 0x7C: Reserved
    ABOTR: VolatileCell<u32>,               // 0x0080: Auto Bus On Time
    TXRQX: VolatileCell<u32>,               // 0x0084: Transmission Request X
    TXRQx: [VolatileCell<u32>; 4],          // 0x0088-0x0094: Transmission Request
    NWDATX: VolatileCell<u32>,              // 0x0098: New Data X
    NWDATx: [VolatileCell<u32>; 4],         // 0x009C-0x00A8: New Data
    INTPNDX: VolatileCell<u32>,             // 0x00AC: Interrupt Pending X
    INTPNDx: [VolatileCell<u32>; 4],        // 0x00B0-0x00BC: Interrupt Pending
    MSGVALX: VolatileCell<u32>,             // 0x00C0: Message Valid X
    MSGVALx: [VolatileCell<u32>; 4],        // 0x00C4-0x00D0: Message Valid
    _reserved3: VolatileCell<u32>,          // 0x00D4: Reserved
    INTMUXx: [VolatileCell<u32>; 4],        // 0x00D8-0x00E4: Interrupt Multiplexer
    _reserved4: [VolatileCell<u32>; 6],     // 0x00E8: Reserved

#[cfg(target_endian = "little")]
    IF1NO: VolatileCell<u8>,                // 0x0100: IF1 Msg Number
#[cfg(target_endian = "little")]
    IF1STAT: VolatileCell<u8>,              // 0x0100: IF1 Status
#[cfg(target_endian = "little")]
    IF1CMD: VolatileCell<u8>,               // 0x0100: IF1 Command
#[cfg(target_endian = "little")]
    _reserved9: VolatileCell<u8>,           // 0x0100: IF1 Reserved

#[cfg(not(target_endian = "little"))]
    _reserved9: VolatileCell<u8>,           // 0x0100: IF1 Reserved
#[cfg(not(target_endian = "little"))]
    IF1CMD: VolatileCell<u8>,               // 0x0100: IF1 Command
#[cfg(not(target_endian = "little"))]
    IF1STAT: VolatileCell<u8>,              // 0x0100: IF1 Status
#[cfg(not(target_endian = "little"))]
    IF1NO: VolatileCell<u8>,                // 0x0100: IF1 Msg Number

    IF1MSK: VolatileCell<u32>,              // 0x0104: IF1 Mask
    IF1ARB: VolatileCell<u32>,              // 0x0108: IF1 Arbitration
    IF1MCTL: VolatileCell<u32>,             // 0x010C: IF1 Message Control
    IF1DATx: [VolatileCell<u8>; 8],         // 0x0110-0x0114: IF1 Data A and B
    _reserved5: [VolatileCell<u32>; 2],     // 0x0118: Reserved

#[cfg(target_endian = "little")]
    IF2NO: VolatileCell<u8>,                // 0x0120: IF2 Msg No
#[cfg(target_endian = "little")]
    IF2STAT: VolatileCell<u8>,              // 0x0120: IF2 Status
#[cfg(target_endian = "little")]
    IF2CMD: VolatileCell<u8>,               // 0x0120: IF2 Command
#[cfg(target_endian = "little")]
    _reserved10: VolatileCell<u8>,          // 0x0120: IF2 Reserved

#[cfg(not(target_endian = "little"))]
    _reserved10: VolatileCell<u8>,          // 0x0120: IF2 Reserved
#[cfg(not(target_endian = "little"))]
    IF2CMD: VolatileCell<u8>,               // 0x0120: IF2 Command
#[cfg(not(target_endian = "little"))]
    IF2STAT: VolatileCell<u8>,              // 0x0120: IF2 Status
#[cfg(not(target_endian = "little"))]
    IF2NO: VolatileCell<u8>,                // 0x0120: IF2 Msg Number

    IF2MSK: VolatileCell<u32>,              // 0x0124: IF2 Mask
    IF2ARB: VolatileCell<u32>,              // 0x0128: IF2 Arbitration
    IF2MCTL: VolatileCell<u32>,             // 0x012C: IF2 Message Control
    IF2DATx: [VolatileCell<u8>; 8],         // 0x0130-0x0134: IF2 Data A and B
    _reserved6: [VolatileCell<u32>; 2],     // 0x0138: Reserved
    IF3OBS: VolatileCell<u32>,              // 0x0140: IF3 Observation
    IF3MSK: VolatileCell<u32>,              // 0x0144: IF3 Mask
    IF3ARB: VolatileCell<u32>,              // 0x0148: IF3 Arbitration
    IF3MCTL: VolatileCell<u32>,             // 0x014C: IF3 Message Control
    IF3DATx: [VolatileCell<u8>; 8],         // 0x0150-0x0154: IF3 Data A and B
    _reserved7: [VolatileCell<u32>; 2],     // 0x0158: Reserved
    IF3UEy: [VolatileCell<u32>; 4],         // 0x0160-0x016C: IF3 Update Enable
    _reserved8: [VolatileCell<u32>; 28],    // 0x0170: Reserved
    TIOC: VolatileCell<u32>,                // 0x01E0: TX IO Control
    RIOC: VolatileCell<u32>,                // 0x01E4: RX IO Control
}

const CAN1_BASE_ADDR: *const CanRegisters = 0xFFF7_DC00 as *const CanRegisters;
const CAN2_BASE_ADDR: *const CanRegisters = 0xFFF7_DE00 as *const CanRegisters;
const CAN3_BASE_ADDR: *const CanRegisters = 0xFFF7_E000 as *const CanRegisters;

const CAN_BASE_ADDR: [*const CanRegisters; 3] = [
    CAN1_BASE_ADDR, CAN2_BASE_ADDR, CAN3_BASE_ADDR
];

const CAN1_RAM_ADDR: *const u32 = 0xFF1E_0000 as *const u32;
const CAN2_RAM_ADDR: *const u32 = 0xFF1C_0000 as *const u32;
const CAN3_RAM_ADDR: *const u32 = 0xFF1A_0000 as *const u32;
const CAN_RAM_ADDR: [*const u32; 3] = [
    CAN1_RAM_ADDR, CAN2_RAM_ADDR, CAN3_RAM_ADDR
];

const CAN1_PARRAM_ADDR: *const u32 = 0xFF1E_0010 as *const u32;
const CAN2_PARRAM_ADDR: *const u32 = 0xFF1C_0010 as *const u32;
const CAN3_PARRAM_ADDR: *const u32 = 0xFF1A_0010 as *const u32;
const CAN_PARRAM_ADDR: [*const u32; 3] = [
    CAN1_PARRAM_ADDR, CAN2_PARRAM_ADDR, CAN3_PARRAM_ADDR
];


const DIR_WRITE:u8 = 0x1 << 7;
const DIR_READ:u8  = 0x0 << 7;
const NEWDAT:u8    = 0x1;
const TXREQ:u8     = 0x0;
const BUSY:u8      = 0x1 << 7;
const MSG_LOST:u32 = 0x1 << 14; /* IFxMCTL: message lost */
const DATA_A:u8    = 0x1;
const DATA_B:u8    = 0x1 << 1;

const CAN_FRAME_SIZE:usize = 8;

#[cfg(target_endian = "big")]
const REMAP_BIG_ENDIAN: [usize; CAN_FRAME_SIZE] = [ 3, 2, 1, 0, 7, 6, 5, 4 ];

#[derive(Copy,Clone)]
pub enum CanID {
    One     = 0,
    Two     = 1,
    Three   = 2,
}

pub enum CanReturn {
    Success { ret: usize },
    NoError,
    DataLost,
    InvalidMsgBox,
    WrongBufferSize,
    PendingMessages,
}

macro_rules! valid_mbox {
    ($mbox: expr) => { $mbox >= 0x1 && $mbox <= 0x80 }
}

#[allow(dead_code)]
pub struct DCan {
    pub id: CanID,
    regs: &'static CanRegisters,
    ram: *const u32,
    pram: *const u32,
}

impl DCan  {
    pub fn new(id: CanID) -> DCan {
        DCan {
            id: id,
            regs: unsafe { &*CAN_BASE_ADDR[id as usize] },
            ram: unsafe { &*CAN_RAM_ADDR[id as usize] },
            pram: unsafe { &*CAN_PARRAM_ADDR[id as usize] },
        }
    }

    pub fn error(&self) -> u32 {
        self.regs.ES.get()
    }

    #[inline(always)]
    fn raw_set(&self, i: usize, b: u8) {
        if cfg!(target_endian = "big") {
            self.regs.IF1DATx[REMAP_BIG_ENDIAN[i]].set(b);
        } else {
            self.regs.IF1DATx[i].set(b);
        }
    }

    #[inline(always)]
    fn raw_get(&self, i: usize) -> u8 {
        if cfg!(target_endian = "big") {
            self.regs.IF2DATx[REMAP_BIG_ENDIAN[i]].get()
        } else {
            self.regs.IF2DATx[i].get()
        }
    }

    pub fn send(&self, mbox:u8, data:&[u8]) -> CanReturn {
        if !valid_mbox!(mbox) {
            return CanReturn::InvalidMsgBox;
        }
        if data.len() > CAN_FRAME_SIZE {
            return CanReturn::WrongBufferSize;
        }

        let regid = (mbox >> 5) as usize;
        if self.regs.TXRQx[regid].get() & (0x1 << mbox) == 0 {
            wait_until_set!(self.regs.IF1STAT.get(), BUSY);
            if !data.is_empty() {
                self.regs.IF1CMD.set(DIR_WRITE | TXREQ | DATA_A | DATA_B);
                for i in 0..data.len() {
                    self.raw_set(i, data[i]);
                }
            } else { // Remote Frame
                self.regs.IF1CMD.set(DIR_WRITE | TXREQ);
            }
            self.regs.IF1NO.set(mbox);
            return CanReturn::Success{ret: data.len()};
        }
        CanReturn::PendingMessages
    }

    pub fn receive(&self, mbox:u8, data:&mut[u8]) -> CanReturn {
        let mut size = 0;

        if !valid_mbox!(mbox) {
            return CanReturn::InvalidMsgBox;
        }
        // check if data have been lost
        if self.regs.IF2MCTL.get() & MSG_LOST != 0 {
            return CanReturn::DataLost;
        }

        let regid = (mbox >> 5) as usize;
        if self.regs.NWDATx[regid].get() & (0x1 << mbox) == 0 {
            wait_until_set!(self.regs.IF2STAT.get(), BUSY);
            self.regs.IF1CMD.set(DIR_READ | NEWDAT | DATA_A | DATA_B);
            self.regs.IF2NO.set(mbox);
            wait_until_set!(self.regs.IF2STAT.get(), BUSY);
            // Get real frame size
            size = (self.regs.IF2MCTL.get() & 0xF) as usize;
            if size > CAN_FRAME_SIZE {
                size = CAN_FRAME_SIZE;
            }
            if size > data.len() {
                return CanReturn::WrongBufferSize;
            }
            for i in 0..size {
                data[i] = self.raw_get(i);
            }
        }
        CanReturn::Success{ret: size}
    }
}
