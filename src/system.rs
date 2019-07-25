///!
///! System Control Module (SYS) and
///! Programmable Built-In Self-Test (PBIST) Module
///!
///! The PBIST architecture consists of a small coprocessor
///! with a dedicated instruction set targeted specifically
///! toward testing memories. This coprocessor executes test
///! routines stored in the PBIST ROMand runs them on multiple
///! on-chip memory instances. The on-chip memory configuration
///! information is also stored in the PBIST ROM. The testing is
///!  done in parallel for each of the CPU data RAMs, while it is
///! done sequentially for the rest of the memories.
///!
///! Reference:
///! TMS570LS Series Technical Reference Manual (TRM), code SPNU489B

use crate::config;
use crate::esm;
use crate::esm_ch::EsmError;
use vcell::VolatileCell;

const LPO_TRIM_ADDR: *const u32 = 0xF008_01B4 as *const u32;

/// Read LPO TRIM value from OTP memory
fn lpo_trim() -> u32 {
    unsafe { ::core::ptr::read_volatile(LPO_TRIM_ADDR) >> 16 }
}

/// Check if there is a valid LPO TRIM value in OTP memory
#[inline]
fn lpo_trim_available() -> bool {
    lpo_trim() != 0xFFFF
}

#[repr(C)]
struct SysRegister1 {
    syspc1: VolatileCell<u32>,
    syspc2: VolatileCell<u32>,
    syspc3: VolatileCell<u32>,
    syspc4: VolatileCell<u32>,
    syspc5: VolatileCell<u32>,
    syspc6: VolatileCell<u32>,
    syspc7: VolatileCell<u32>,
    syspc8: VolatileCell<u32>,
    syspc9: VolatileCell<u32>,
    sswpll1: VolatileCell<u32>,
    sswpll2: VolatileCell<u32>,
    sswpll3: VolatileCell<u32>,
    csdis: VolatileCell<u32>,
    csdisset: VolatileCell<u32>,
    csdisclr: VolatileCell<u32>,
    cddis: VolatileCell<u32>,
    cddisset: VolatileCell<u32>,
    cddisclr: VolatileCell<u32>,
    ghvsrc: VolatileCell<u32>,
    vclkasrc: VolatileCell<u32>,
    rclksrc: VolatileCell<u32>,
    csvstat: VolatileCell<u32>,
    mstgcr: VolatileCell<u32>,
    minitgcr: VolatileCell<u32>,
    msinena: VolatileCell<u32>,
    mstfail: VolatileCell<u32>,
    mstcgstat: VolatileCell<u32>,
    ministat: VolatileCell<u32>,
    pllctl1: VolatileCell<u32>,
    pllctl2: VolatileCell<u32>,
    syspc10: VolatileCell<u32>,
    dieid_l: VolatileCell<u32>,
    dieid_h: VolatileCell<u32>,
    vrctl: VolatileCell<u32>,
    lpomonctl: VolatileCell<u32>,
    clktest: VolatileCell<u32>,
    dftctrlreg1: VolatileCell<u32>,
    dftctrlreg2: VolatileCell<u32>,
    rsvd1: VolatileCell<u32>,
    rsvd2: VolatileCell<u32>,
    gpreg1: VolatileCell<u32>,
    btrmsel: VolatileCell<u32>,
    impfasts: VolatileCell<u32>,
    impftadd: VolatileCell<u32>,
    ssisr1: VolatileCell<u32>,
    ssisr2: VolatileCell<u32>,
    ssisr3: VolatileCell<u32>,
    ssisr4: VolatileCell<u32>,
    ramgcr: VolatileCell<u32>,
    bmmcr1: VolatileCell<u32>,
    bmmcr2: VolatileCell<u32>,
    cpurstcr: VolatileCell<u32>,
	clkcntl: VolatileCell<u32>,
    ecpcntl: VolatileCell<u32>,
    dspgcr: VolatileCell<u32>,
    devcr1: VolatileCell<u32>,
    sysecr: VolatileCell<u32>,
    syssrc: VolatileCell<u32>,
    systasr: VolatileCell<u32>,
    gblstat: VolatileCell<u32>,
    dev: VolatileCell<u32>,
    ssivec: VolatileCell<u32>,
    ssif: VolatileCell<u32>
}
const SYS1_BASE_ADDR: *const SysRegister1 = 0xFFFF_FF00 as *const SysRegister1;

#[repr(C)]
struct SysRegister2 {
    pllctl3: VolatileCell<u32>,
    rsvd1: VolatileCell<u32>,
    stcclkdiv: VolatileCell<u32>,
    rsvd2: [VolatileCell<u32>; 6],
    ecpcntrl0: VolatileCell<u32>,
    rsvd3: [VolatileCell<u32>; 5],
    clc2cntl: VolatileCell<u32>,
    vclkacon1: VolatileCell<u32>,
    rsvd4: [VolatileCell<u32>; 11],
    clkslip: VolatileCell<u32>,
    rsvd5: [VolatileCell<u32>; 30],
    efc_ctlen: VolatileCell<u32>,
    dieidl_reg0: VolatileCell<u32>,
    dieidh_reg1: VolatileCell<u32>,
    dieidl_reg2: VolatileCell<u32>,
    diedih_reg3: VolatileCell<u32>
}
const SYS2_BASE_ADDR: *const SysRegister2 = 0xFFFF_E100 as *const SysRegister2;

#[allow(dead_code)]
#[allow(non_snake_case)]
struct PbistRegisters {
    /// RAM Configuration
    RAMT: VolatileCell<u32>,
    /// Datalogger
    DLR: VolatileCell<u32>,
    _reserved1: [VolatileCell<u32>;6],
    /// PBIST Activate
    PACT: VolatileCell<u32>,
    /// PBIST ID
    PBISTID: VolatileCell<u32>,
    /// Override
    OVER: VolatileCell<u32>,
    _reserved2: VolatileCell<u32>,
    /// Fail Status Fail 0
    FSRF0: VolatileCell<u32>,
    _reserved5: VolatileCell<u32>,
    /// Fail Status Count 0
    FSRC0: VolatileCell<u32>,
    /// Fail Status Count 1
    FSRC1: VolatileCell<u32>,
    /// Fail Status Address 0
    FSRA0: VolatileCell<u32>,
    /// Fail Status Address 1
    FSRA1: VolatileCell<u32>,
    /// Fail Status Data 0
    FSRDL0: VolatileCell<u32>,
    _reserved3: VolatileCell<u32>,
    /// Fail Status Data 1
    FSRDL1: VolatileCell<u32>,
    _reserved4: [VolatileCell<u32>;3],
    /// ROM Mask
    ROM: VolatileCell<u32>,
    /// Algorithm Mask
    ALGO: VolatileCell<u32>,
    /// RAM Info Mask Lower
    RINFOL: VolatileCell<u32>,
    /// RAM Info Mask Upper
    RINFOU: VolatileCell<u32>,
}
const PBIST_BASE_ADDR: *const PbistRegisters = 0xFFFF_E560 as *const PbistRegisters;


/// MCU available Spleep Modes
#[derive(Clone, Copy)]
pub enum SleepMode {
    /// Use Doze mode
    Doze = 0x000F_3F02,
    /// Use Snooze mode
    Snooze = 0x000F_3F03,
    /// Sleep mode: all pheriferal sleeping
    Sleep = 0x000F_FFFF,
}

/// Enumerates MCU's available clock domains
#[derive(Clone, Copy)]
pub enum ClockDomains {
    /// AVCLK1 domain
    AvClk1 = 4,
    /// AVCLK2 domain
    AvClk2 = 5,
    /// VCLK3 domain
    VClk3 = 8,
    /// VCLK4 domain
    VClk4 = 9,
    /// VCLK3 domain
    AvClk3 = 10,
    /// AVCLK4 domain
    AvClk4 = 11,
}

/// Enumerates available clock sources
#[derive(Clone, Copy)]
pub enum SysClockSources {
    /// Oscillator clock Source
    Osc = 0,
    /// Pll1 clock Source
    Pll1 = 1,
    /// External clock Source
    External1 = 3,
    /// Low power oscillator low clock Source
    Lpolow = 4,
    /// Low power oscillator high clock Source
    LpiHigh = 5,
    /// Pll2 clock Source
    Pll2 = 6,
    /// External 2 clock Source
    External2 = 7,
    /// Synchronous VCLK1 clock Source
    Vclk = 9,
}

#[derive(Clone, Copy)]
pub enum Ram {
    Internal =  0x1,
    Dma =       0x1 << 1,
    Vim =       0x1 << 2,
    Het1 =      0x1 << 3,
    Htu1 =      0x1 << 4,
    Can1 =      0x1 << 5,
    Can2 =      0x1 << 6,
    MibSPI1 =   0x1 << 7,
    Adc1 =      0x1 << 8,
    Can3 =      0x1 << 10,
    MibSPI3 =   0x1 << 11,
    MibSPI5 =   0x1 << 12,
    FlexRayTU = 0x1 << 13,
    Adc2 =      0x1 << 14,
    Het2 =      0x1 << 15,
    Htu2 =      0x1 << 16,
}

pub enum EcpClockSource {
    TiedLow     = 0x0,
    HCLK        = 0x1,
    External    = 0x2,
    TiedHigh    = 0x3,
}

enum EclkMode {
    Gpio        = 0x0,
    Functional  = 0x1,
}

pub struct Sys {
    sys1: &'static SysRegister1,
    sys2: &'static SysRegister2,
    pbist: &'static PbistRegisters,
}

impl Sys {
    pub fn new() -> Sys {
        Sys {
            sys1: unsafe { &*SYS1_BASE_ADDR },
            sys2: unsafe { &*SYS2_BASE_ADDR },
            pbist: unsafe { &*PBIST_BASE_ADDR },
        }
    }

    pub fn enable_pheripherals(&self, enable:bool) {
        let cntl = self.sys1.clkcntl.get();
        if enable {
            self.sys1.clkcntl.set(cntl | 0x0000_0100);
        } else {
            self.sys1.clkcntl.set(cntl & 0xFFFF_FEFF);
        }
    }

    /// Disable PLL1 and PLL2
    pub fn disable_pll(&self) {
        let plls = 0x1 << (SysClockSources::Pll1 as u32) |
                   0x1 << (SysClockSources::Pll2 as u32);
        self.sys1.csdisset.set(plls);
        wait_until_set!(self.sys1.csdis.get(), plls)
    }

    /// Enable PLL1 and PLL2
    pub fn enable_pll(&self) {
        let plls = 0x1 << (SysClockSources::Pll1 as u32) |
                   0x1 << (SysClockSources::Pll2 as u32);
        self.sys1.csdisclr.set(plls)
    }

    /// Clear Global Status Register
    pub fn clear_global_status(&self) {
        self.sys1.gblstat.set(0x301);
    }

    // Setup all PLLs
    pub fn setup_pll(&self) {
        self.disable_pll();
        self.clear_global_status();

        // Setup pll control register 1:
        // - Setup reset on oscillator slip
        // - Setup bypass on pll slip
        // - setup Pll output clock divider to max before Lock
        // - Setup reset on oscillator fail
        // - Setup reference clock divider
        // - Setup Pll multiplier
        let pll_ctl1 = 0x0000_0000
                    | 0x2000_0000
                    | (0x1F << 24)
                    | 0x0000_0000
                    | ((6 - 1) << 16)
                    | (0x7700);
        self.sys1.pllctl1.set(pll_ctl1);

        // Setup pll control register 2:
        // - Setup spreading rate
        // - Setup bandwidth adjustment
        // - Setup internal Pll output divider
        // - Setup spreading amount
        let pll_ctl2 = (255 << 22)
                    | (7 << 12)
                    | ((2 - 1) << 9)
                    | 61;
        self.sys1.pllctl2.set(pll_ctl2);

        // Setup pll2 control register:
        // - setup Pll output clock divider to max before Lock
        // - Setup reference clock divider
        // - Setup internal Pll output divider
        // - Setup Pll multiplier
        let pll_ctl3 = ((2 - 1) << 29)
                        | (0x1F << 24)
                        | ((6 - 1)<< 16)
                        | (0x7700);
        self.sys2.pllctl3.set(pll_ctl3);

        self.enable_pll()
    }

    pub fn wait_pll_lock(&self) {
        loop {
            let csvstat = self.sys1.csvstat.get();
            let csdis = self.sys1.csdis.get();
            let mask = (csdis ^ 0xff) & 0xff;
            if csvstat & mask == mask {
                break;
            }
        }
    }

    pub fn set_pll_divider(&self, div1:u8, div3:u8) {
        let pll1 = self.sys1.pllctl1.get();
        let pll2 = self.sys2.pllctl3.get();
        let p1 = (pll1 & 0xE0FF_FFFF) | (u32::from(div1) << 24);
        let p2 = (pll2 & 0xE0FF_FFFF) | (u32::from(div3) << 24);

        self.sys1.pllctl1.set(p1);
        self.sys2.pllctl3.set(p2);
        self.sys1.pllctl2.set(self.sys1.pllctl2.get() | 0x00000000); //TODO rivedere
    }

    /// Setup GCLK, HCLK and VCLK clock source for normal operation,
    /// power down mode and after wakeup
    pub fn setup_clock_source(&self, gclk:SysClockSources, hclk:SysClockSources,
                              vclk:SysClockSources) {


        self.sys1.ghvsrc.set(((gclk as u32) << 24) |
                             ((hclk as u32) << 16) |
                              (vclk as u32));
        self.sys1.rclksrc.set( (0x1 << 24)
                              | ((SysClockSources::Vclk as u32) << 16)
                              | (0x1 << 8)
                              | (SysClockSources::Vclk as u32));
        self.sys1.vclkasrc.set(((SysClockSources::Vclk as u32) << 8) |
                               (SysClockSources::Vclk as u32));

        self.sys2.vclkacon1.set(((SysClockSources::Vclk as u32) << 16) |
                                 (SysClockSources::Vclk as u32));

    }

    /// Get device infos
    pub fn device_info(&self) -> u32 {
        self.sys1.dev.get()
    }

    /// Configure the LPO such that HF LPO is as
    /// close to 10MHz as possible.
    /// Use LPO from OTP memory if available.
    pub fn trim_lpo(&self) {
        let lpo = if lpo_trim_available() {
            lpo_trim()
        } else {
            config::LPO
        };
        self.sys1.lpomonctl.set((0x1 << 24) | lpo);
    }

    pub fn power_down(&self, mode:SleepMode) {
        let v = mode as u32;
        // Disable clock sources
        self.sys1.csdisset.set(v & 0xFF);
        // Disable clock domains
        self.sys1.cddis.set((v >> 8) & 0xFFF);
        //TODO
        //support::wfi()
    }

    pub fn activate_peripherals(&self, act:bool) {
        if act {
            self.sys1.clkcntl.set(self.sys1.clkcntl.get() & 0xFFFF_FEFF)
        } else {
            self.sys1.clkcntl.set(self.sys1.clkcntl.get() | 0x0000_0100)
        }
    }

    pub fn peripherals_clock_divider(&self, vclk1:u8, vclk2:u8, vclk3:u8, vclk4:u8) {
        let div1 = u32::from(vclk1) << 24;
        let div2 = u32::from(vclk2) << 16;
        let div3 = u32::from(vclk3) << 8;
        let div4 = u32::from(vclk4);

        // Note: VCLK and VCLK2 clock ratio restrictions.
        // VCLK2 must always be greater than or equal to VCLK.
        // In addition, the VCLK and VCLK2 clock ratios must not
        // be changed simultaneously.
        self.sys1.clkcntl.set((self.sys1.clkcntl.get() & 0xF0FF_FFFF) | div1);
        self.sys1.clkcntl.set((self.sys1.clkcntl.get() & 0xFFF0_FFFF) | div2);

        self.sys2.clc2cntl.set((self.sys2.clc2cntl.get() & 0xFFFF_F0F0) | div3 | div4);
    }

    fn memory_self_controller(&self, enable:bool) {
        // PBIST ROM clock frequency = HCLK frequency /2, so
        // ROM_DIV = ROM clock source is HCLK divided by 2.
        // PBIST will reset for 32 VBUS cycles.
        if enable {
            self.sys1.mstgcr.set(0x0000_0100 | 0xA);
            wait_cycle!(32);
        } else {
            self.sys1.mstgcr.set(0x0000_0100 | 0x5);
        }
    }

    /// Enable/Disable Memory Hardware init
    #[inline(always)]
    fn memory_controller_enable(&self, enable: bool) {
        if enable {
            self.sys1.minitgcr.set(0xA)
        } else {
            self.sys1.minitgcr.set(0x5)
        }
    }

    #[inline(always)]
    pub fn init_memory(&self, ram:Ram) {
        self.memory_controller_enable(true);
        self.sys1.msinena.set(ram as u32);
        // Wait until Memory Hardware Initialization complete
        wait_until_set!(self.sys1.mstcgstat.get(), 0x0000_0100);
        self.memory_controller_enable(false);
    }

    /// Disable all clock domains (convenient function)
    pub fn clock_domain_enable_all(&self) {
        self.sys1.cddis.set(0x0);
    }

    /// Enable or disable a specific clock domain
    pub fn clock_domain_setup(&self, disable:bool, domain:ClockDomains) {
        if disable {
            self.sys1.cddisclr.set(0x1 << (domain as u32));
        } else {
            self.sys1.cddisset.set(0x1 << (domain as u32));
        }
    }


    pub fn eclk_functional_mode(&self, divider:u16, oscin:bool) {
        self.sys1.syspc1.set(EclkMode::Functional as u32);
        self.sys1.syspc2.set(0x1);
        self.sys1.syspc4.set(0x0);
        self.sys1.syspc7.set(0x0);
        self.sys1.syspc8.set(0x0);
        self.sys1.syspc9.set(0x1);

        // Note: Suspend mode (ECPCOS) is entered while performing
        // certain JTAG debugging operations, so force ECPCOS at 0 here.
        self.sys1.ecpcntl.set(((oscin as u32) << 24) | u32::from(divider));
    }

    pub fn eclk_gpio_setup(&self) {
        self.sys1.syspc1.set(EclkMode::Gpio as u32);
        //TODO
    }


    pub fn set_stc_clock_divider(&self, divider:u32) {
        self.sys2.stcclkdiv.set(divider)
    }

    /// Get Wafer and lot number
    pub fn die_id(&self) -> (u32, u32) {
        let wafer = self.sys2.dieidl_reg0.get() & 0x003F_FFFF;
        let lotnum = (self.sys2.dieidl_reg0.get() & 0xFFC0_0000 >> 22) |
                      self.sys2.dieidh_reg1.get() & 0x0000_3FFF << 10;
        (wafer, lotnum)
    }

    // Based on "PBIST Sequence" from TRM manual
    #[inline(always)]
    pub fn pbist_self_test(&self) {
        // Disable PBIST and ROM clocks
        self.pbist.PACT.set(0x0);
        self.memory_self_controller(false);
        self.memory_controller_enable(false);
        self.sys1.mstcgstat.set(0x1); // Clear PBIST Done
        self.sys1.msinena.set(Ram::Internal as u32);
        self.memory_self_controller(true);
        // Enable PBIST and ROM clocks
        self.pbist.PACT.set(0x3);
        // Let CPU to take control of PBIST */
        self.pbist.DLR.set(0x10);
        //FIXME(pteti): add always fail algorithm
    }

    /// Activate PBIST test on a given memory group using a given algorithm
    ///
    /// # Arguments
    /// - `algo`. PBIST algorithm mask (may select more than one).
    /// - `memories`. Memories group mask (may select more than one)
    ///
    /// # Safety
    /// - No all algorithms are supported for all memories groups
    /// - No all algorithms can be executed in parallel with others on
    ///   some memory group (es. ROM test can't be done in parallel)
    ///
    /// Refer to TMS570 Series Technical Reference Manual for details.
    #[inline(always)]
    pub fn pbist_run(&self, algo: u32, memories: u32) {
        self.memory_self_controller(false);
        self.memory_controller_enable(false);
        // Enable PBIST controller
        self.sys1.msinena.set(Ram::Internal as u32);
        self.memory_self_controller(true);
        // Enable PBIST clocks and ROM clock
        self.pbist.PACT.set(0x3);
        // Configure selected Algorithm and RAM groups
        self.pbist.ALGO.set(algo);
        self.pbist.RINFOL.set(memories);
        self.pbist.RINFOU.set(0x0);
        self.pbist.OVER.set(0x0);
        // load algorithm from ROM
        self.pbist.ROM.set(0x3);
        // run test
        self.pbist.DLR.set(0x14);
    }

    pub fn pbist_completed(&self) -> bool {
        self.sys1.mstcgstat.get() & 0x1 != 0
    }

    pub fn pbist_fail(&self) -> bool {
        self.pbist.FSRF0.get() != 0
    }

    pub fn pbist_stop(&self) {
        self.pbist.PACT.set(0x0);
        self.memory_self_controller(false);
    }

    /// Checks clock supervisor failure detection logic
    pub unsafe fn clock_supervisor_test(&self) -> bool {
        self.sys1.clktest.set(self.sys1.clktest.get() | 0x0300_0000);
        let ghvsrc = self.sys1.ghvsrc.get();
        self.sys1.ghvsrc.set(0x0505_0005);
        // disable oscillator so it fail
        self.sys1.csdisset.set(0x1);
        wait_until_zero!(self.sys1.gblstat.get(), 0x1);
        let esm = esm::Esm::new();
        if !esm.error_is_set(EsmError::OscFail) {
            return false;
        } else {
            // Disable test mode and restore original settings
            esm.clear_error(EsmError::OscFail);
            self.sys1.clktest.set(self.sys1.clktest.get() & !0x0300_0000);
            self.sys1.csdisclr.set(0x1);
            wait_until_zero!(self.sys1.csvstat.get(), 0x3);
            self.sys1.gblstat.set(0x301);  // clear any pending flag
            self.sys1.ghvsrc.set(ghvsrc);
        }
        true
    }
}
