use vcell::VolatileCell;
use esm_ch::{EsmError, EsmGroup};


#[allow(dead_code)]
pub struct Esm {
    eepapr1: VolatileCell<u32>,    // 0x0000
    depapr1: VolatileCell<u32>,    // 0x0004
    iesr1: VolatileCell<u32>,      // 0x0008
    iecr1: VolatileCell<u32>,      // 0x000C
    ilsr1: VolatileCell<u32>,      // 0x0010
    ilcr1: VolatileCell<u32>,      // 0x0014
    sr1: [VolatileCell<u32>; 3],   // 0x0018, 0x001C, 0x0020
    epsr: VolatileCell<u32>,       // 0x0024
    ioffhr: VolatileCell<u32>,     // 0x0028
    iofflr: VolatileCell<u32>,     // 0x002C
    ltcr: VolatileCell<u32>,       // 0x0030
    ltcpr: VolatileCell<u32>,      // 0x0034
    ekr: VolatileCell<u32>,        // 0x0038
    ssr2: VolatileCell<u32>,       // 0x003C
    iepsr4: VolatileCell<u32>,     // 0x0040
    iepcr4: VolatileCell<u32>,     // 0x0044
    iesr4: VolatileCell<u32>,      // 0x0048
    iecr4: VolatileCell<u32>,      // 0x004C
    ilsr4: VolatileCell<u32>,      // 0x0050
    ilcr4: VolatileCell<u32>,      // 0x0054
    sr4: [VolatileCell<u32>; 3],   // 0x0058, 0x005C, 0x0060
}

const ESM_BASE_ADDR: *const Esm = 0xFFFF_F500 as *const Esm;

impl Esm {
    pub unsafe fn new() -> &'static Esm {
        &*ESM_BASE_ADDR
    }

    pub fn error_reset(&self) {
        self.ekr.set(0x5)
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
    pub fn error_is_set(&self, err:EsmError) -> bool {
        let ch = err.ch();
        let group = err.group();
        if ch < 31 {
            (self.sr1[group].get() >> ch) & 0x1 == 0x1
        } else {
            (self.sr4[group].get() >> (ch - 32)) & 0x1 == 0x1
        }
    }

    pub fn clear_error(&self, err:EsmError) {
        let ch = err.ch();
        let group = err.group();
        if ch < 31 {
            self.sr1[group].set(0x1 << ch);
        } else {
            self.sr4[group].set(0x1 << (ch - 32));
        }
    }

    pub fn shadow_stat_clear(&self, group:EsmGroup) {
        self.ssr2.set(0x1 << (group as u32));
    }
}
