///
/// Tightly-Coupled RAM (TCRAM) module
///
use vcell::VolatileCell;

#[repr(C)]
pub struct Tcram {
    ramctrl: VolatileCell<u32>,
    ramthreshold: VolatileCell<u32>,
    ramoccur: VolatileCell<u32>,
    ramintctrl: VolatileCell<u32>,
    ramerrstatus: VolatileCell<u32>,
    ramerraddr: VolatileCell<u32>,
    _reserved1: VolatileCell<u32>,
    ramuerraddr: VolatileCell<u32>,
    _reserved2: [VolatileCell<u32>; 4],
    ramtest: VolatileCell<u32>,
    _reserved3: VolatileCell<u32>,
    ramaddrdecvect: VolatileCell<u32>,
    ramperaddr: VolatileCell<u32>,
}
const TCRAM1_BASE_ADDR: *const Tcram = 0xFFFF_F800 as *const Tcram;
const TCRAM2_BASE_ADDR: *const Tcram = 0xFFFF_F900 as *const Tcram;

#[derive(Clone, Copy)]
pub enum TcRamID {
    One,
    Two,
}

impl Tcram {
    pub unsafe fn new(id: TcRamID) -> &'static Tcram {
        match id {
            TcRamID::One => &*TCRAM1_BASE_ADDR,
            TcRamID::Two => &*TCRAM2_BASE_ADDR,
        }
    }

    /// Test if ECC memory writes are enabled.
    pub fn ecc_write_enabled(&self) -> bool {
        self.ramctrl.get() & 0x80 != 0
    }

    pub fn clear_error(&self) {
        self.ramerrstatus.set(0x20)
    }
}
