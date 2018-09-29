///
/// PCR Driver
///

use vcell::VolatileCell;

#[repr(C)]
#[allow(non_snake_case)]
pub struct Pcr {
    PMPROTSET0: VolatileCell<u32>,
    PMPROTSET1: VolatileCell<u32>,
    _reserved1: [VolatileCell<u32>; 2],
    PMPROTCLR0: VolatileCell<u32>,
    PMPROTCLR1: VolatileCell<u32>,
    _reserved2: [VolatileCell<u32>; 2],
    PPROTSET0: VolatileCell<u32>,
    PPROTSET1: VolatileCell<u32>,
    PPROTSET2: VolatileCell<u32>,
    PPROTSET3: VolatileCell<u32>,
    _reserved3: [VolatileCell<u32>; 4],
    PPROTCLR0: VolatileCell<u32>,
    PPROTCLR1: VolatileCell<u32>,
    PPROTCLR2: VolatileCell<u32>,
    PPROTCLR3: VolatileCell<u32>,
    _reserved4: [VolatileCell<u32>; 4],
    PCSPWRDWNSET0: VolatileCell<u32>,
    PCSPWRDWNSET1: VolatileCell<u32>,
    _reserved5: [VolatileCell<u32>; 2],
    PCSPWRDWNCLR0: VolatileCell<u32>,
    PCSPWRDWNCLR1: VolatileCell<u32>,
    _reserved6: [VolatileCell<u32>; 2],
    PSPWRDWNSET0: VolatileCell<u32>,
    PSPWRDWNSET1: VolatileCell<u32>,
    PSPWRDWNSET2: VolatileCell<u32>,
    PSPWRDWNSET3: VolatileCell<u32>,
    _reserved7: [VolatileCell<u32>; 4],
    PSPWRDWNCLR0: VolatileCell<u32>,
    PSPWRDWNCLR1: VolatileCell<u32>,
    PSPWRDWNCLR2: VolatileCell<u32>,
    PSPWRDWNCLR3: VolatileCell<u32>,
}
const PCR_BASE_ADDR: *const Pcr = 0xFFFF_E000 as *const Pcr;

impl Pcr {
    pub unsafe fn new() -> &'static Pcr {
        &*PCR_BASE_ADDR
    }

    pub fn enable_all(&self) {
        self.PSPWRDWNCLR0.set(0xFFFF_FFFF);
        self.PSPWRDWNCLR1.set(0xFFFF_FFFF);
        self.PSPWRDWNCLR2.set(0xFFFF_FFFF);
        self.PSPWRDWNCLR3.set(0xFFFF_FFFF);
    }
}