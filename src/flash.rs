use crate::config;
use vcell::VolatileCell;

/// Read LPO TRIM value from OTP memory
pub fn lpo_trim_value() -> u8 {
    const LPO_TRIM_ADDR: *const u32 = 0xF008_01B4 as *const u32;
    unsafe { (::core::ptr::read_volatile(LPO_TRIM_ADDR) >> 16) as u8 }
}

/// Check if there is a valid LPO TRIM value in OTP memory
#[inline]
pub fn lpo_trim_available() -> bool {
    lpo_trim_value() != 0xFF
}

/// Retrive device part number as ASCII digit
pub fn dev_part_number_symb(pn: &mut [u8; 32]) {
    const LPO_PN_SYMBOLIZATION: *const u8 = 0xF008_01E0 as *const u8;
    let mut reg = LPO_PN_SYMBOLIZATION;

    for p in pn.iter_mut() {
        unsafe {
            *p = ::core::ptr::read_volatile(reg);
            reg = reg.offset(1);
        }
    }
}

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

pub enum DiagModes {
    Disabled = 0,
    /// Diagnostic ECC Data Correction test
    ECCDataCorrectionTest = 1,
    /// Diagnostic ECC Syndrome Reporting test
    ECCSyndromeReportingTest = 2,
    /// ECC Malfunction Test (same data)
    ECCMalfunctionTestSameData = 3,
    /// ECC Malfunction Test (inverted data)
    ECCMalfunctionTestInvertedData = 4,
    /// Address Tag Register Test
    AdressTagRegisterTest = 5,
    /// ECC Data Correction Diagnostic Test
    ECCDataCorrectionDiagnosticTest = 7,
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

    /// Setup diagnostic mode/algorithm
    pub fn diag_mode(&self, mode: DiagModes) {
        self.FDIAGCTRL.set(self.FDIAGCTRL.get() | (mode as u32))
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
        self.EEPROMCONFIG.set(
            config::flash::EEPROM_AUTOSTART_GRACE
                | ((config::flash::EEPROM_AUTOSUSP_EN as u32) << 8)
                | (config::flash::EEPROM_WAITSTATE << 16),
        );
        self.lock_fsm();

        // Setup flash bank power modes
        let mode = power as u32;
        self.FBFALLBACK.set(
            (mode << 14) | // BANK 7
                            (mode << 2)  | // BANK 1
                            mode,
        ); // BANK 0
    }
}
