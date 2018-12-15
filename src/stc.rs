use cortexr4::asm::{nop, wfi};
use system;
use vcell::VolatileCell;

#[repr(C)]
pub struct Stc {
    /// Control Register 0
    stcgcr0: VolatileCell<u32>,
    /// Control Register 1
    stcgcr1: VolatileCell<u32>,
    /// Self-Test Run Timeout Counter Preload
    stctpr: VolatileCell<u32>,
    /// Self-Test Current ROM Address
    stccaddr: VolatileCell<u32>,
    /// Self-Test Current Interval Count
    stccicr: VolatileCell<u32>,
    /// Self-Test Global Status
    stcgstat: VolatileCell<u32>,
    /// Self-Test Fail Status
    stcfstat: VolatileCell<u32>,
    /// CPU1 Current MISR block 3
    cpu1_curmisr3: VolatileCell<u32>,
    /// CPU1 Current MISR block 2
    cpu1_curmisr2: VolatileCell<u32>,
    /// CPU1 Current MISR block 1
    cpu1_curmisr1: VolatileCell<u32>,
    /// CPU1 Current MISR block 0
    cpu1_curmisr0: VolatileCell<u32>,
    /// CPU2 Current MISR block 3
    cpu2_curmisr3: VolatileCell<u32>,
    /// CPU2 Current MISR block 2
    cpu2_curmisr2: VolatileCell<u32>,
    /// CPU2 Current MISR block 1
    cpu2_curmisr1: VolatileCell<u32>,
    /// CPU2 Current MISR block 0
    cpu2_curmisr0: VolatileCell<u32>,
    /// Signature Compare Self-Check
    stcscscr: VolatileCell<u32>,
}

const STC_BASE_ADDR: *const Stc = 0xFFFF_E600 as *const Stc;

/// Insert stuck-at-fault inside CPU so that STC signature compare will fail
const SCSCR_FAULT_INS: u32 = 0x1 << 4;
/// Key to enable self-test
const SCSCR_SELF_CHECK_KEY: u32 = 0xA;

impl Stc {
    pub unsafe fn new() -> &'static Stc {
        &*STC_BASE_ADDR
    }

    /// Perform STC module self check
    pub unsafe fn self_test(&self, intervals: u16, restart: bool) {
        // STC clock = normal mode CPU clock frequency/2 = 180MHz/2
        let sys2 = system::Sys::new();
        sys2.set_stc_clock_divider(0x0100_0000);
        self.setup_intervals(intervals, restart);

        // Enable comparator self-check and stuck-at-0 fault insertion in CPU
        self.stcscscr.set(SCSCR_FAULT_INS | SCSCR_SELF_CHECK_KEY);
        // Maximum time-out period
        self.stctpr.set(0xFFFF_FFFF);
        self.activate();
    }

    /// Perform CPU self test using STC module
    pub unsafe fn cpu_self_test(&self, intervals: u16, timeout: u32, restart: bool) {
        // Run specified no of test intervals starting from 0
        self.setup_intervals(intervals, restart);
        self.stctpr.set(timeout);
        self.activate();
    }

    fn setup_intervals(&self, intervals: u16, restart: bool) {
        self.stcgcr0.set(u32::from(intervals) << 16);
        if restart {
            self.stcgcr0.set(self.stcgcr0.get() | 0x1);
        }
    }

    unsafe fn activate(&self) {
        // wait for 16 VBUS clock cycles at least, based on HCLK to VCLK ratio
        wait_cycle!(16);
        // Enable self-test
        self.stcgcr1.set(SCSCR_SELF_CHECK_KEY);
        // Idle the CPU so that the self-test can start
        wfi();
        nop();
        nop();
        nop();
        nop();
    }
}
