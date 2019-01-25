use crate::esm_ch::{EsmError, EsmGroup};
use vcell::VolatileCell;

#[repr(C)]
pub struct Esm {
    eepapr1: VolatileCell<u32>,
    depapr1: VolatileCell<u32>,
    iesr1: VolatileCell<u32>,
    iecr1: VolatileCell<u32>,
    ilsr1: VolatileCell<u32>,
    ilcr1: VolatileCell<u32>,
    sr1: [VolatileCell<u32>; 3],
    epsr: VolatileCell<u32>,
    ioffhr: VolatileCell<u32>,
    iofflr: VolatileCell<u32>,
    ltcr: VolatileCell<u32>,
    ltcpr: VolatileCell<u32>,
    ekr: VolatileCell<u32>,
    ssr2: VolatileCell<u32>,
    iepsr4: VolatileCell<u32>,
    iepcr4: VolatileCell<u32>,
    iesr4: VolatileCell<u32>,
    iecr4: VolatileCell<u32>,
    ilsr4: VolatileCell<u32>,
    ilcr4: VolatileCell<u32>,
    sr4: [VolatileCell<u32>; 3],
}

const ESM_BASE_ADDR: *const Esm = 0xFFFF_F500 as *const Esm;

impl Esm {
    pub unsafe fn new() -> &'static Esm {
        &*ESM_BASE_ADDR
    }

    /// Init and reset the ESM driver.
    pub fn reset(&self, preload: u16) {
        // disable error pin channels and interrupts
        self.depapr1.set(0xFFFF_FFFF);
        self.iepcr4.set(0xFFFF_FFFF);
        self.iecr1.set(0xFFFF_FFFF);
        self.iecr4.set(0xFFFF_FFFF);

        self.clear_all_errors();
        if self.error_pin_active() {
            self.error_reset();
        } else {
            self.normal_operation();
        }
        self.set_preload(preload);
    }

    pub fn set_preload(&self, preload: u16) {
        self.ltcpr.set(u32::from(preload - 1))
    }

    pub fn error_reset(&self) {
        self.ekr.set(0x5)
    }

    pub fn normal_operation(&self) {
        self.ekr.set(0x0)
    }

    pub fn error_pin_active(&self) -> bool {
        self.epsr.get() == 0x0
    }

    pub fn high_level_interrupt(&self) -> u32 {
        self.ioffhr.get() - 1
    }

    pub fn low_level_interrupt(&self) -> u32 {
        self.iofflr.get() - 1
    }

    /// Up to 128 error channels are supported, divided into 3 different groups:
    /// – 64 Group1 (low severity) channels with configurable interrupt
    ///   generation and configurable ERROR pin behavior
    /// – 32 Group2 (high severity) channels with predefined interrupt
    ///   generation and predefined ERROR pin behavior
    /// – 32 Group3 (high severity) channels with no interrupt generation
    ///   and predefined ERROR pin behavior. These channels have no interrupt
    ///   response as they are reserved for CPU based diagnostics which generate
    ///   aborts directly to the CPU.
    pub fn error_is_set(&self, err: EsmError) -> bool {
        let ch = err.ch();
        let group = err.group();
        if ch < 31 {
            (self.sr1[group].get() >> ch) & 0x1 == 0x1
        } else {
            (self.sr4[group].get() >> (ch - 32)) & 0x1 == 0x1
        }
    }

    pub fn clear_error(&self, err: EsmError) {
        let ch = err.ch();
        let group = err.group();
        if ch < 31 {
            self.sr1[group].set(0x1 << ch);
        } else {
            self.sr4[group].set(0x1 << (ch - 32));
        }
    }

    pub fn clear_all_errors(&self) {
        self.sr1[0].set(0xFFFF_FFFF);
        self.sr1[1].set(0xFFFF_FFFF);
        self.sr1[2].set(0xFFFF_FFFF);
        self.ssr2.set(0xFFFF_FFFF);
        self.sr4[0].set(0xFFFF_FFFF)
    }

    pub fn shadow_stat_clear(&self, group: EsmGroup) {
        self.ssr2.set(0x1 << (group as u32));
    }

    pub fn disable_interrupt(&self, err: EsmError) {
        self.iecr4.set(0x1 << (err.ch() - 31));
        self.iecr1.set(0x1 << err.ch());
    }

    pub fn enable_interrupt(&self, err: EsmError) {
        self.iesr4.set(0x1 << (err.ch() - 31));
        self.iesr1.set(0x1 << err.ch());
    }

    pub fn disable_error(&self, err: EsmError) {
        self.iepcr4.set(0x1 << (err.ch() - 31));
        self.depapr1.set(0x1 << err.ch());
    }

    pub fn enable_error(&self, err: EsmError) {
        self.iepsr4.set(0x1 << (err.ch() - 31));
        self.eepapr1.set(0x1 << err.ch());
    }
}
