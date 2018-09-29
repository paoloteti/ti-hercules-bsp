///
/// I/O Multiplexing and Control Module (IOMM)
///
use vcell::VolatileCell;
use pinmux;

#[repr(C)]
pub struct Iomm {
    revision: VolatileCell<u32>,      	    // Revision
    _reserved1: [VolatileCell<u32>; 7],
    endian: VolatileCell<u32>,         	    // Device Endianness
    _reserved2: [VolatileCell<u32>; 5],
    kicker0: VolatileCell<u32>,		        // Secret Key 0
    kicker1: VolatileCell<u32>,		        // Secret Key 1
    _reserved3: [VolatileCell<u32>; 40],
    err_raw_status: VolatileCell<u32>, 	    // Error Raw Status / Set
    err_enabled_status: VolatileCell<u32>,  // Error Enabled Status / Clear
    err_enable: VolatileCell<u32>,    	    // Error Signaling Enable
    err_enable_clr: VolatileCell<u32>, 	    // Error Signaling Enable Clear
    _reserved4: VolatileCell<u32>,
    fault_address: VolatileCell<u32>, 	    // Fault Address
    fault_status: VolatileCell<u32>,  	    // Fault Status
    fault_clear: VolatileCell<u32>,   	    // Fault Clear
    _reserved5: [VolatileCell<u32>; 4],
    pinnmr: [VolatileCell<u32>; 47],        // Pin Multiplexing Control
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
    fn kicker(&self, open:bool) {
	    if open {
	        self.kicker0.set(0x83E7_0B13);
	        self.kicker1.set(0x95A4_F1E0);
	    } else {
            self.kicker0.set(0x0);
	        self.kicker1.set(0x0);
	    }
    }

    fn configure(&self, pin: pinmux::PinMux, active: bool) {
        if active {
            self.pinnmr[pin.group()].set(0x1 << pin.bit());
	    } else {
            let muxreg = self.pinnmr[pin.group()].get();
            self.pinnmr[pin.group()].set(muxreg & !(0x1 << pin.bit()));
	    }
    }

    /// Setup connection between pin and specified peripherals/function
    /// Leaves rest of multiplexer setup intact.
    pub fn setup_pin_function(&self, pin:pinmux::PinMux, active:bool) {
        self.kicker(true);
        self.configure(pin, active);
	    self.kicker(false)
    }

    pub fn setup_pins(&self, pins: &[pinmux::PinMux]) {
        self.kicker(true);
        for p in pins.iter() {
            self.configure(*p, true);
        }
        self.kicker(false);
    }

    pub fn pin_set_function(&self, pin: pinmux::PinMux) {
	    self.setup_pin_function(pin, true);
    }

    pub fn pin_clear_function(&self, pin: pinmux::PinMux)  {
	    self.setup_pin_function(pin, false);
    }
}
