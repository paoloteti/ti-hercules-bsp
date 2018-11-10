use pinmux;
///
/// I/O Multiplexing and Control Module (IOMM)
///
use vcell::VolatileCell;

#[repr(C)]
pub struct Iomm {
    /// Revision
    revision: VolatileCell<u32>,
    /// Reserved
    _reserved1: [VolatileCell<u32>; 7],
    /// Device Endianness
    endian: VolatileCell<u32>,
    /// Reserved
    _reserved2: [VolatileCell<u32>; 5],
    /// Secret Key 0
    kicker0: VolatileCell<u32>,
    /// Secret Key 1
    kicker1: VolatileCell<u32>,
    /// Reserved
    _reserved3: [VolatileCell<u32>; 40],
    /// Error Raw Status / Set
    err_raw_status: VolatileCell<u32>,
    /// Error Enabled Status / Clear
    err_enabled_status: VolatileCell<u32>,
    /// Error Signaling Enable
    err_enable: VolatileCell<u32>,
    /// Error Signaling Enable Clear
    err_enable_clr: VolatileCell<u32>,
    /// Reserved
    _reserved4: VolatileCell<u32>,
    /// Fault Address
    fault_address: VolatileCell<u32>,
    /// Fault Status
    fault_status: VolatileCell<u32>,
    // Fault Clear
    fault_clear: VolatileCell<u32>,
    /// Reserved
    _reserved5: [VolatileCell<u32>; 4],
    /// Pin Multiplexing Control
    pinnmr: [VolatileCell<u32>; 47],
}
const IOMM_BASE_ADDR: *const Iomm = 0xFFFF_EA00 as *const Iomm;

impl Iomm {
    pub unsafe fn new() -> &'static Iomm {
        &*IOMM_BASE_ADDR
    }

    pub fn revision(&self) -> u32 {
        self.revision.get()
    }

    /// Set a 64bits key to enable/disable pin muxing
    /// Key value is not trivial to avoid accidental pin muxing at
    /// runtime. For the same reason function is not public.
    fn kicker(&self, open: bool) {
        if open {
            self.kicker0.set(0x83E7_0B13);
            self.kicker1.set(0x95A4_F1E0);
        } else {
            self.kicker0.set(0x0);
            self.kicker1.set(0x0);
        }
    }

    fn configure(&self, pin: pinmux::PinMux) {
        let mut muxreg = self.pinnmr[pin.group()].get();
        let bit = 0x1 << (pin.shift() + pin.option());
        muxreg = (muxreg & !(0xFF << pin.shift())) | bit;
        self.pinnmr[pin.group()].set(muxreg);
    }

    pub fn setup_pins(&self, pins: &[pinmux::PinMux]) {
        self.kicker(true);
        for p in pins.iter() {
            self.configure(*p);
        }
        self.kicker(false);
    }
}
