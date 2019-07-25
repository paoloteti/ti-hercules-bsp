use crate::config::RTICLK1;
use crate::dwd::{WdViolation, DWD};
///
/// RTI Control Module
///
use core::mem;
use vcell::VolatileCell;

#[repr(C)]
#[allow(non_snake_case)]
struct Counter {
    /// Free Running Counter
    FRCx: VolatileCell<u32>,
    /// Up Counter x
    UCx: VolatileCell<u32>,
    /// Compare Up Counter x
    CPUCx: VolatileCell<u32>,
    /// Reserved
    _rsvd1: VolatileCell<u32>,
    /// Capture Free Running Counter x
    CAFRCx: VolatileCell<u32>,
    /// Capture Up Counter x
    CAUCx: VolatileCell<u32>,
    /// Reserved
    _rsvd2: [VolatileCell<u32>; 2],
}

#[repr(C)]
#[allow(non_snake_case)]
struct Comparator {
    /// Compare x
    COMPx: VolatileCell<u32>,
    /// Update Compare x
    UDCPx: VolatileCell<u32>,
}

#[repr(C)]
#[allow(non_snake_case)]
struct RtiRegisters {
    /// Global Control
    GCTRL: VolatileCell<u32>,
    /// Timebase Control
    TBCTRL: VolatileCell<u32>,
    /// Capture Control
    CAPCTRL: VolatileCell<u32>,
    /// Compare Control
    COMPCTRL: VolatileCell<u32>,
    Cnt: [Counter; 2],
    Cmp: [Comparator; 4],
    /// External Clock Timebase Low Compare
    TBLCOMP: VolatileCell<u32>,
    /// External Clock Timebase High Compare
    TBHCOMP: VolatileCell<u32>,
    /// Reserved
    _rsvd3: [VolatileCell<u32>; 2],
    /// Set/Status Interrupt
    SETINTENA: VolatileCell<u32>,
    /// Clear/Status Interrupt
    CLEARINTENA: VolatileCell<u32>,
    /// Interrupt Flag
    INTFLAG: VolatileCell<u32>,
    /// Reserved
    _rsvd4: VolatileCell<u32>,
    /// Digital Watchdog Control
    DWDCTRL: VolatileCell<u32>,
    /// Digital Watchdog Preload
    DWDPRLD: VolatileCell<u32>,
    /// Watchdog Status
    WDSTATUS: VolatileCell<u32>,
    /// Watchdog Key
    WDKEY: VolatileCell<u32>,
    /// Digital Watchdog Down Counter
    DWDCNTR: VolatileCell<u32>,
    /// Digital Windowed Watchdog Reaction Control
    WWDRXNCTRL: VolatileCell<u32>,
    /// Digital Windowed Watchdog Window Size Control
    WWDSIZECTRL: VolatileCell<u32>,
    /// RTI Compare Interrupt Clear Enable
    INTCLRENABLE: VolatileCell<u32>,
    /// RTI Compare 0 Clear
    COMP0CLR: VolatileCell<u32>,
    /// RTI Compare 1 Clear
    COMP1CLR: VolatileCell<u32>,
    /// RTI Compare 2 Clear
    COMP2CLR: VolatileCell<u32>,
    /// RTI Compare 3 Clear
    COMP3CLR: VolatileCell<u32>,
}
const RTI_BASE_ADDR: *const RtiRegisters = 0xFFFF_FC00 as *const RtiRegisters;

#[derive(Clone, Copy)]
pub struct ChipWatchDog {
    regs: &'static RtiRegisters,
}

impl DWD for ChipWatchDog {
    fn new() -> ChipWatchDog {
        ChipWatchDog {
            regs: unsafe { &*RTI_BASE_ADDR },
        }
    }

    fn start(&self, expire: u32) {
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
        let violation: WdViolation = unsafe { mem::transmute(stat as i8) };
        violation
    }

    fn time_violation(&self) -> bool {
        self.status() != WdViolation::KeySeqViolation
            && self.status() != WdViolation::NoTimeViolation
    }

    fn status_clear(&self) {
        self.regs.WDSTATUS.set(0xFF);
    }

    fn expire(&self, expire: u32) -> Result<(), u32> {
        // texp = (DWDPRLD + 1) x 2^13 / RTICLK1
        // where: DWDPRLD = 0...4095
        let dwd_preload = ((expire * RTICLK1) / (1 << 13)) - 1;
        if dwd_preload < 4095 {
            self.regs.DWDPRLD.set(dwd_preload);
            return Ok(());
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
