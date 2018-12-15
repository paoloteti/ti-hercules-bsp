
use vcell::VolatileCell;

#[repr(C)]
#[allow(non_snake_case)]
pub struct Flash {
    FRDCNTL: VolatileCell<u32>,       // 0x0000
    _reserved1: VolatileCell<u32>,    // 0x0004
    FEDACCTRL1: VolatileCell<u32>,    // 0x0008
    FEDACCTRL2: VolatileCell<u32>,    // 0x000C
    FCORERRCNT: VolatileCell<u32>,    // 0x0010
    FCORERRADD: VolatileCell<u32>,    // 0x0014
    FCORERRPOS: VolatileCell<u32>,    // 0x0018
    FEDACSTATUS: VolatileCell<u32>,   // 0x001C
    FUNCERRADD: VolatileCell<u32>,    // 0x0020
    FEDACSDIS: VolatileCell<u32>,     // 0x0024
    FPRIMADDTAG: VolatileCell<u32>,   // 0x0028
    FREDUADDTAG: VolatileCell<u32>,   // 0x002C
    FBPROT: VolatileCell<u32>,        // 0x0030
    FBSE: VolatileCell<u32>,          // 0x0034
    FBBUSY: VolatileCell<u32>,        // 0x0038
    FBAC: VolatileCell<u32>,          // 0x003C
    FBFALLBACK: VolatileCell<u32>,    // 0x0040
    FBPRDY: VolatileCell<u32>,        // 0x0044
    FPAC1: VolatileCell<u32>,         // 0x0048
    FPAC2: VolatileCell<u32>,         // 0x004C
    FMAC: VolatileCell<u32>,          // 0x0050
    FMSTAT: VolatileCell<u32>,        // 0x0054
    FEMUDMSW: VolatileCell<u32>,      // 0x0058
    FEMUDLSW: VolatileCell<u32>,      // 0x005C
    FEMUECC: VolatileCell<u32>,       // 0x0060
    FLOCK: VolatileCell<u32>,         // 0x0064
    FEMUADDR: VolatileCell<u32>,      // 0x0068
    FDIAGCTRL: VolatileCell<u32>,     // 0x006C
    FRAWDATAH: VolatileCell<u32>,     // 0x0070
    FRAWDATAL: VolatileCell<u32>,     // 0x0074
    FRAWECC: VolatileCell<u32>,       // 0x0078
    FPAROVR: VolatileCell<u32>,       // 0x007C
     _reserved2: [VolatileCell<u32>;16],  // 0x009C
    FEDACSDIS2: VolatileCell<u32>,    // 0x00C0
    _reserved3: [VolatileCell<u32>;15],  // 0x00C4
    _reserved4: [VolatileCell<u32>;13],  // 0x0100
    _reserved5: [VolatileCell<u32>;85],  // 0x0134
    FSMWRENA: VolatileCell<u32>,      // 0x0288
    _reserved6: [VolatileCell<u32>;6],   // 0x028C
    FSMSECTOR: VolatileCell<u32>,     // 0x02A4
    _reserved7: [VolatileCell<u32>;4],   // 0x02A8
    EEPROMCONFIG: VolatileCell<u32>,  // 0x02B8
    _reserved8: [VolatileCell<u32>;19],  // 0x02BC
    EECTRL1: VolatileCell<u32>,       // 0x0308
    EECTRL2: VolatileCell<u32>,       // 0x030C
    EECORRERRCNT: VolatileCell<u32>,  // 0x0310
    EECORRERRADD: VolatileCell<u32>,  // 0x0314
    EECORRERRPOS: VolatileCell<u32>,  // 0x0318
    EESTATUS: VolatileCell<u32>,      // 0x031C
    EEUNCERRADD: VolatileCell<u32>,   // 0x0320
}
const FLASH_BASE_ADDR: *const Flash = 0xFFF8_7000 as *const Flash;

pub enum FlashWPowerModes {
    /// flash bank power mode sleep
    Sleep   = 0x0,
    /// flash bank power mode standby
    Standby = 0x1,
    /// flash bank power mode active
    Active  = 0x3,
}


impl Flash {
    pub unsafe fn new() -> &'static Flash {
        &*FLASH_BASE_ADDR
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
        let rwait = u32::from(ws & 0xF)  << 8;
        let enpipe = pipeline as u32;

        self.FRDCNTL.set(rwait | aswsten | enpipe);

        // Setup flash access wait states for bank 7
        self.FSMWRENA.set(0x5);
        self.EEPROMCONFIG.set(0x0000_0002 | (0x3 << 16));
        self.FSMWRENA.set(0xA);

        // Setup flash bank power modes
        let mode = power as u32;
        self.FBFALLBACK.set((mode << 14) | // BANK 7
                            (mode << 2)  | // BANK 1
                            mode);         // BANK 0
    }
}
