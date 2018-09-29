
use vcell;

#[repr(C)]
pub struct Dcc {
    gctrl: VolatileCell<u32>,      	    // 0x00: Control
    rev: VolatileCell<u32>,      		// 0x04: Revision Id
    cnt0seed: VolatileCell<u32>,    	// 0x08: Counter0 Seed
    valid0seed: VolatileCell<u32>,      // 0x0C: Valid0 Seed
    cnt1seed: VolatileCell<u32>,		// 0x10: Counter1 Seed
    stat: VolatileCell<u32>, 			// 0x14: Status
    cnt0: VolatileCell<u32>,    		// 0x18: Counter0 Value  	
    valid0: VolatileCell<u32>,    	    // 0x1C: Valid0 Value
    cnt1: VolatileCell<u32>,      	    // 0x20: Counter1 Value
    cnt1clrsrc: VolatileCell<u32>,   	// 0x24: Counter1 Clock Source Selection
    cnt0clrsrc: VolatileCell<u32>,   	// 0x28: Counter0 Clock Source Selection
}
const DCC_BASE_ADDR: *const Dcc = 0xFFFF_EC00 as *const Dcc;

enum DccClockSource {
    DCC1_CNT0_HF_LPO    = 0x5,
    DCC1_CNT0_TCK       = 0xA,
    DCC1_CNT0_OSCIN     = 0xF,
    DCC1_CNT1_PLL1      = 0x0,
    DCC1_CNT1_PLL2      = 0x1,
    DCC1_CNT1_LF_LPO    = 0x2,
    DCC1_CNT1_HF_LPO    = 0x3,
    DCC1_CNT1_EXTCLKIN1 = 0x5,
    DCC1_CNT1_EXTCLKIN2 = 0x6,
    DCC1_CNT1_VCLK      = 0x8,
    DCC1_CNT1_N2HET1_31 = 0xA,
};

impl Dcc {
    pub unsafe fn new() -> &'static Dcc {
        &*DCC_BASE_ADDR
    }

    pub check_frequency(&self, clk_src:DccClockSource) -> bool {
        self.stat.get() & 0x1 != 0
    }
}
