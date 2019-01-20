use vcell::VolatileCell;

#[repr(C)]
#[allow(non_snake_case)]
pub struct Flash {
    FRDCNTL: VolatileCell<u32>,
    _reserved1: VolatileCell<u32>,
    FEDACCTRL1: VolatileCell<u32>,
    FEDACCTRL2: VolatileCell<u32>,
    FCORERRCNT: VolatileCell<u32>,
    FCORERRADD: VolatileCell<u32>,
    FCORERRPOS: VolatileCell<u32>,
    FEDACSTATUS: VolatileCell<u32>,
    FUNCERRADD: VolatileCell<u32>,
    FEDACSDIS: VolatileCell<u32>,
    FPRIMADDTAG: VolatileCell<u32>,
    FREDUADDTAG: VolatileCell<u32>,
    FBPROT: VolatileCell<u32>,
    FBSE: VolatileCell<u32>,
    FBBUSY: VolatileCell<u32>,
    FBAC: VolatileCell<u32>,
    FBFALLBACK: VolatileCell<u32>,
    FBPRDY: VolatileCell<u32>,
    FPAC1: VolatileCell<u32>,
    FPAC2: VolatileCell<u32>,
    FMAC: VolatileCell<u32>,
    FMSTAT: VolatileCell<u32>,
    FEMUDMSW: VolatileCell<u32>,
    FEMUDLSW: VolatileCell<u32>,
    FEMUECC: VolatileCell<u32>,
    FLOCK: VolatileCell<u32>,
    FEMUADDR: VolatileCell<u32>,
    FDIAGCTRL: VolatileCell<u32>,
    FRAWDATAH: VolatileCell<u32>,
    FRAWDATAL: VolatileCell<u32>,
    FRAWECC: VolatileCell<u32>,
    FPAROVR: VolatileCell<u32>,
    _reserved2: [VolatileCell<u32>; 16],
    FEDACSDIS2: VolatileCell<u32>,
    _reserved3: [VolatileCell<u32>; 15],
    _reserved4: [VolatileCell<u32>; 13],
    _reserved5: [VolatileCell<u32>; 85],
    FSMWRENA: VolatileCell<u32>,
    _reserved6: [VolatileCell<u32>; 6],
    FSMSECTOR: VolatileCell<u32>,
    _reserved7: [VolatileCell<u32>; 4],
    EEPROMCONFIG: VolatileCell<u32>,
    _reserved8: [VolatileCell<u32>; 19],
    EECTRL1: VolatileCell<u32>,
    EECTRL2: VolatileCell<u32>,
    EECORRERRCNT: VolatileCell<u32>,
    EECORRERRADD: VolatileCell<u32>,
    EECORRERRPOS: VolatileCell<u32>,
    EESTATUS: VolatileCell<u32>,
    EEUNCERRADD: VolatileCell<u32>,
}
const FLASH_BASE_ADDR: *const Flash = 0xFFF8_7000 as *const Flash;

pub enum FlashWPowerModes {
    /// flash bank power mode sleep
    Sleep = 0x0,
    /// flash bank power mode standby
    Standby = 0x1,
    /// flash bank power mode active
    Active = 0x3,
}

impl Flash {
    pub unsafe fn new() -> &'static Flash {
        &*FLASH_BASE_ADDR
    }

    /// Unlock FSM registers for writing
    fn unlock_fsm(&self) {
        self.FSMWRENA.set(0x5);
    }

    /// Lock FSM registers
    fn lock_fsm(&self) {
        self.FSMWRENA.set(0xA);
    }

    /// Setup flash read mode, address wait states and data wait states
    ///
    /// # Arguments
    ///
    /// * `ws` The random read wait state bits indicate how many wait states
    /// are added to a flash read access. In Pipeline mode there is always one wait.
    ///
    /// * `address_ws` Address Setup Wait State is enabled. Address is latched one
    /// cycle before decoding to determine pipeline hit or miss. Address Setup Wait State
    /// is only available in pipeline mode
    pub fn setup(&self, power: FlashWPowerModes, ws: u8, address_ws: bool, pipeline: bool) {
        let aswsten = (address_ws as u32) << 4;
        let rwait = u32::from(ws & 0xF) << 8;
        let enpipe = pipeline as u32;

        self.FRDCNTL.set(rwait | aswsten | enpipe);

        // Setup flash access wait states for bank 7
        self.unlock_fsm();
        self.EEPROMCONFIG.set(0x0000_0002 | (0x3 << 16));
        self.lock_fsm();

        // Setup flash bank power modes
        let mode = power as u32;
        self.FBFALLBACK.set((mode << 14) | // BANK 7
                            (mode << 2)  | // BANK 1
                            mode);         // BANK 0
    }
}
