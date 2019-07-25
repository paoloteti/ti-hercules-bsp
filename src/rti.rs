///
/// RTI Control Module
///
use core::mem;
use crate::config::RTICLK1;
use vcell::VolatileCell;
use crate::dwd::{DWD, WdViolation};

#[repr(C)]
#[allow(non_snake_case)]
struct Counter {
    FRCx: VolatileCell<u32>,        // 0x10,0x30: Free Running Counter x
    UCx: VolatileCell<u32>,         // 0x14,0x34: Up Counter x
    CPUCx: VolatileCell<u32>,       // 0x18,0x38: Compare Up Counter x
    _rsvd1: VolatileCell<u32>,      // 0x1C,0x3C: Reserved
    CAFRCx: VolatileCell<u32>,      // 0x20,0x40: Capture Free Running Counter x
    CAUCx: VolatileCell<u32>,       // 0x24,0x44: Capture Up Counter x
    _rsvd2: [VolatileCell<u32>; 2], // 0x28,0x48: Reserved
}

#[repr(C)]
#[allow(non_snake_case)]
struct Comparator {
    COMPx: VolatileCell<u32>,      // 0x50,0x58,0x60,0x68: Compare x
    UDCPx: VolatileCell<u32>,      // 0x54,0x5C,0x64,0x6C: Update Compare x
}

#[repr(C)]
#[allow(non_snake_case)]
struct RtiRegisters {
    GCTRL: VolatileCell<u32>,          // 0x00: Global Control
    TBCTRL: VolatileCell<u32>,         // 0x04: Timebase Control
    CAPCTRL: VolatileCell<u32>,        // 0x08: Capture Control
    COMPCTRL: VolatileCell<u32>,       // 0x0C: Compare Control
    Cnt: [Counter; 2],
    Cmp: [Comparator; 4],
    TBLCOMP: VolatileCell<u32>,        // 0x70: External Clock Timebase Low Compare
    TBHCOMP: VolatileCell<u32>,        // 0x74: External Clock Timebase High Compare
    _rsvd3: [VolatileCell<u32>; 2],    // 0x78: Reserved
    SETINTENA: VolatileCell<u32>,      // 0x80: Set/Status Interrupt
    CLEARINTENA: VolatileCell<u32>,    // 0x84: Clear/Status Interrupt
    INTFLAG: VolatileCell<u32>,        // 0x88: Interrupt Flag
    _rsvd4: VolatileCell<u32>,         // 0x8C: Reserved
    DWDCTRL: VolatileCell<u32>,        // 0x90: Digital Watchdog Control
    DWDPRLD: VolatileCell<u32>,        // 0x94: Digital Watchdog Preload
    WDSTATUS: VolatileCell<u32>,       // 0x98: Watchdog Status
    WDKEY: VolatileCell<u32>,          // 0x9C: Watchdog Key
    DWDCNTR: VolatileCell<u32>,        // 0xA0: Digital Watchdog Down Counter
    WWDRXNCTRL: VolatileCell<u32>,     // 0xA4: Digital Windowed Watchdog Reaction Control
    WWDSIZECTRL: VolatileCell<u32>,    // 0xA8: Digital Windowed Watchdog Window Size Control
    INTCLRENABLE: VolatileCell<u32>,   // 0xAC: RTI Compare Interrupt Clear Enable
    COMP0CLR: VolatileCell<u32>,       // 0xB0: RTI Compare 0 Clear
    COMP1CLR: VolatileCell<u32>,       // 0xB4: RTI Compare 1 Clear
    COMP2CLR: VolatileCell<u32>,       // 0xB8: RTI Compare 2 Clear
    COMP3CLR: VolatileCell<u32>,       // 0xBC: RTI Compare 3 Clear
}
const RTI_BASE_ADDR: *const RtiRegisters = 0xFFFF_FC00 as *const RtiRegisters;



#[derive(Clone, Copy)]
pub struct ChipWatchDog {
    regs: &'static RtiRegisters,
}


impl DWD for ChipWatchDog {
    fn new() -> ChipWatchDog {
        ChipWatchDog { regs: unsafe { &*RTI_BASE_ADDR } }
    }

    fn start(&self, expire:u32) {
        self.status_clear();
        self.expire(expire).unwrap();
        self.counter_enable();
    }

    /// Reset Digital Watchdog
    fn reset(&self) {
        self.regs.WDKEY.set(0xE51A);
        self.regs.WDKEY.set(0xA35C);
    }

    /// Generate system reset using DWD
    fn sys_reset(&self) {
        self.regs.WDKEY.set(0xE51A);
        self.regs.WDKEY.set(0x2345);
    }

    fn status(&self) -> WdViolation {
        let stat = self.regs.WDSTATUS.get();
        let violation: WdViolation = unsafe {
            mem::transmute(stat as i8)
        };
        violation
    }

    fn time_violation(&self) -> bool {
        self.status() != WdViolation::KeySeqViolation &&
        self.status() != WdViolation::NoTimeViolation
    }

    fn status_clear(&self) {
        self.regs.WDSTATUS.set(0xFF);
    }

    fn expire(&self, expire:u32) -> Result<(), u32> {
        // texp = (DWDPRLD + 1) x 2^13 / RTICLK1
        // where: DWDPRLD = 0...4095
        let dwd_preload = ((expire * RTICLK1) / (1 << 13)) - 1;
        if dwd_preload < 4095 {
            self.regs.DWDPRLD.set(dwd_preload);
            return Ok(())
        }
        Err(dwd_preload)
    }

    /// Enable the DWD counter
    fn counter_enable(&self) {
        self.regs.DWDCTRL.set(0xA985_59DA);
    }

    fn count_down(&self) -> u32 {
        self.regs.DWDCNTR.get()
    }

}
