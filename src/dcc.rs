///
/// Dual-Clock Comparator (DCC) Module
///
use vcell;

#[repr(C)]
pub struct Dcc {
    /// Control
    gctrl: VolatileCell<u32>,
    /// Revision Id
    rev: VolatileCell<u32>,
    /// Counter0 Seed
    cnt0seed: VolatileCell<u32>,
    /// Valid0 Seed
    valid0seed: VolatileCell<u32>,
    /// Counter1 Seed
    cnt1seed: VolatileCell<u32>,
    /// Status
    stat: VolatileCell<u32>,
    /// Counter0 Value
    cnt0: VolatileCell<u32>,
    /// Valid0 Value
    valid0: VolatileCell<u32>,
    /// Counter1 Value
    cnt1: VolatileCell<u32>,
    /// Counter1 Clock Source Selection
    cnt1clrsrc: VolatileCell<u32>,
    /// Counter0 Clock Source Selection
    cnt0clrsrc: VolatileCell<u32>,
}
const DCC1_BASE_ADDR: *const Dcc = 0xFFFF_EC00 as *const Dcc;
const DCC2_BASE_ADDR: *const Dcc = 0xFFFF_F400 as *const Dcc;

#[derive(Copy, Clone)]
enum DccId {
    One = 0,
    Two = 1,
}

#[derive(Copy, Clone)]
enum DccClockSource {
    CNT0_HF_LPO = 0x5,
    CNT0_TCK = 0xA,
    CNT0_OSCIN = 0xF,
    CNT1_PLL1 = 0x0,
    CNT1_PLL2 = 0x1,
    CNT1_LF_LPO = 0x2,
    CNT1_HF_LPO = 0x3,
    CNT1_EXTCLKIN1 = 0x5,
    CNT1_EXTCLKIN2 = 0x6,
    CNT1_VCLK = 0x8,
    CNT1_N2HET1_31 = 0xA,
}

impl Dcc {
    pub fn new(id: DccId) -> &'static Dcc {
        match id {
            DccId::One => unsafe { &*DCC1_BASE_ADDR },
            DccId::Two => unsafe { &*DCC2_BASE_ADDR },
        }
    }

    /// Enable/Disable DCC module. Module will automatically start/stop counting.
    pub fn enable(&self, enable: bool) {
        if enable {
            self.gctrl.set(self.gctrl.get() & 0xFFFF_FFFA);
        } else {
            self.gctrl.set(self.gctrl.get() & 0xFFFF_FFF5);
        }
    }

    /// Set clock source 0. Must be enabled using clock1_source_enable()
    pub fn clock0_source(&self, clk_src: DccClockSource) {
        self.cnt0clrsrc.set(clk_src as u32)
    }

    /// Set clock source 1 but not enable it.
    /// Must be enabled in the next using clock1_source_enable();
    pub fn clock1_source(&self, clk_src: DccClockSource) {
        self.cnt1clrsrc.set(clk_src as u32)
    }

    /// Set and enable both clock sources. Automatically enable source clock 1.
    pub fn select_source(&self, clk0_src: DccClockSource, clk1_src: DccClockSource) {
        self.cnt0clrsrc.set(clk0_src as u32);
        self.cnt1clrsrc.set(clk1_src as u32);
        clock1_source_enable(true)
    }

    /// Disable/disable clock 1 source. Clock source 0 is always enabled and can't be disabled
    pub fn clock1_source_enable(&self, enable: bool) {
        const ENABLE_KEY: u32 = 0xA;
        if enable {
            self.cnt1clrsrc
                .set(self.cnt0clrsrc.get() | (ENABLE_KEY << 12))
        } else {
            self.cnt1clrsrc
                .set(self.cnt0clrsrc.get() | (!ENABLE_KEY << 12))
        }
    }

    /// Set DCC Clock source 0 counter seed value
    pub fn counter0_seed(&self, cnt0: u32) {
        self.cnt0seed.set(cnt0);
    }

    /// Set DCC Clock Source 0 Counter tolerance value
    pub fn tollerance(&self, valid0: u32) {
        self.valid0.set(valid0);
    }

    /// Set DCC Clock Source 1 Counter tolerance value
    pub fn counter1_seed(&self, cnt1: u32) {
        self.cnt1seed.set(cnt1);
    }

    pub fn error(&self) -> bool {
        self.stat.get() & 0x1 != 0
    }
}
