use cortexr4::asm::{wfi,nop};
use system;
use vcell::VolatileCell;

#[allow(dead_code)]
pub struct Stc {
    stcgcr0: VolatileCell<u32>,		    // 0x00: STC Control 0
    stcgcr1: VolatileCell<u32>,		    // 0x04: STC Control 1
    stctpr: VolatileCell<u32>,		    // 0x08: STC Self-Test Run Timeout Counter Preload
    stccaddr: VolatileCell<u32>,	    // 0x0C: STC Self-Test Current ROM Address
    stccicr: VolatileCell<u32>,		    // 0x10: STC Self-Test Current Interval Count
    stcgstat: VolatileCell<u32>,        // 0x14: STC Self-Test Global Status
	stcfstat: VolatileCell<u32>,		// 0x18: STC Self-Test Fail Status
    cpu1_curmisr3: VolatileCell<u32>,	// 0x1C: STC CPU1 Current MISR
    cpu1_curmisr2: VolatileCell<u32>,	// 0x20: STC CPU1 Current MISR
    cpu1_curmisr1: VolatileCell<u32>,	// 0x24: STC CPU1 Current MISR
    cpu1_curmisr0: VolatileCell<u32>,	// 0x28: STC CPU1 Current MISR
    cpu2_curmisr3: VolatileCell<u32>,	// 0x2C: STC CPU2 Current MISR
    cpu2_curmisr2: VolatileCell<u32>,	// 0x30: STC CPU2 Current MISR
    cpu2_curmisr1: VolatileCell<u32>,	// 0x34: STC CPU2 Current MISR
    cpu2_curmisr0: VolatileCell<u32>,	// 0x38: STC CPU2 Current MISR
    stcscscr: VolatileCell<u32>,		// 0x3C: STC Signature Compare Self-Check
}

const STC_BASE_ADDR: *const Stc = 0xFFFF_E600 as *const Stc;


/// Insert stuck-at-fault inside CPU so that STC signature compare will fail
pub const STCSCSCR_FAULT_INS: u32 = 0x1 << 4;
/// Key to enable self-test
pub const STCSCSCR_SELF_CHECK_KEY: u32 = 0xA;

impl Stc {
    pub unsafe fn new() -> &'static Stc {
        &*STC_BASE_ADDR
    }

    /// Perform STC module self check
    pub fn self_test(&self, intervals:u16, restart:bool) {
        // STC clock = normal mode CPU clock frequency/2 = 180MHz/2
        let sys2 = system::Sys::new();
        sys2.set_stc_clock_divider(0x0100_0000);
        self.setup_intervals(intervals, restart);

        // Enable comparator self-check and stuck-at-0 fault insertion in CPU
        self.stcscscr.set(STCSCSCR_FAULT_INS | STCSCSCR_SELF_CHECK_KEY);
        // Maximum time-out period
        self.stctpr.set(0xFFFF_FFFF);
        self.activate();
    }

    /// Perform CPU self test using STC module
    pub fn cpu_self_test(&self, intervals:u16, timeout:u32, restart:bool) {
        // Run specified no of test intervals starting from 0
        self.setup_intervals(intervals, restart);
        self.stctpr.set(timeout);
        self.activate();
    }

    fn setup_intervals(&self, intervals:u16, restart:bool) {
        self.stcgcr0.set((intervals as u32) << 16);
        if restart {
            self.stcgcr0.set(self.stcgcr0.get() | 0x1);
        }
    }

    fn activate(&self) {
        // wait for 16 VBUS clock cycles at least, based on HCLK to VCLK ratio
        wait_cycle!(16);
        self.stcgcr1.set(STCSCSCR_SELF_CHECK_KEY); // Enable self-test
        /* Idle the CPU so that the self-test can start */
        unsafe {
            wfi();
            nop();
            nop();
            nop();
            nop();
        }
    }
}
