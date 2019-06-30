///! ESM Channel Assignments for TMS5700714
///!
///! Texas Instruments TMS5700714APGEQQ1 datasheet
///! SPNS226C – JUNE 2013 – REVISED NOVEMBER 2014
///!
///! Table 6-31. ESM Channel Assignments

macro_rules! map_ch_group {
    ($ch: expr, $group: expr) => {
        (($group as isize) << 24) | ($ch as isize)
    };
}

pub enum EsmGroup {
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
}

#[derive(Copy, Clone)]
pub enum EsmError {
    /// MibADC2 - RAM parity error
    MibADC2Ram = map_ch_group!(EsmGroup::One, 1),
    /// DMA - MPU configuration violation
    DmaMpu = map_ch_group!(EsmGroup::One, 2),
    /// DMA - control packet RAM parity error
    DmaParity = map_ch_group!(EsmGroup::One, 3),
    /// DMA - error on DMA read access, imprecise error
    DmaErrorImprecise = map_ch_group!(EsmGroup::One, 5),
    /// FMC - correctable ECC error: bus1 and bus2 interfaces
    /// (does not include accesses to Bank 7)
    FmcError = map_ch_group!(EsmGroup::One, 6),
    /// N2HET1 - RAM parity error
    N2HET1Parity = map_ch_group!(EsmGroup::One, 7),
    /// HET TU1/HET TU2 - dual-control packet RAM parity error
    HETTUxParity = map_ch_group!(EsmGroup::One, 8),
    /// HET TU1/HET TU2 - MPU configuration violation
    HETTUxMpuErr = map_ch_group!(EsmGroup::One, 9),
    /// PLL1 - Slip
    Pll1Splip = map_ch_group!(EsmGroup::One, 10),
    /// Clock Monitor - oscillator fail
    OscFail = map_ch_group!(EsmGroup::One, 11),
    /// DMA - error on DMA write access, imprecise error
    DmaErrorPrecise = map_ch_group!(EsmGroup::One, 13),
    /// VIM RAM - parity error
    VimParity = map_ch_group!(EsmGroup::One, 15),
    /// MibSPI1 - RAM parity error
    MibSPI1Parity = map_ch_group!(EsmGroup::One, 17),
    /// MibSPI3 - RAM parity error
    MibSPI3Parity = map_ch_group!(EsmGroup::One, 18),
    /// MibADC1 - RAM parity error
    MibADC1Parity = map_ch_group!(EsmGroup::One, 19),
    /// DCAN1 - RAM parity error
    DCAN1Parity = map_ch_group!(EsmGroup::One, 21),
    /// DCAN3 - RAM parity error
    DCAN3Parity = map_ch_group!(EsmGroup::One, 22),
    /// DCAN2 - RAM parity error
    DCAN2Parity = map_ch_group!(EsmGroup::One, 23),
    /// MibSPI5 - RAM parity error
    MibSPI5Parity = map_ch_group!(EsmGroup::One, 25),
    /// RAM even bank (B0TCM) - correctable ECC error
    RamEvenCorrectableECC = map_ch_group!(EsmGroup::One, 26),
    /// CPU - self-test failed
    CPUSelfTest = map_ch_group!(EsmGroup::One, 27),
    /// RAM odd bank (B1TCM) - correctable ECC error
    RamOddCorrectableECC = map_ch_group!(EsmGroup::One, 28),
    /// DCC1 - error
    DCC1 = map_ch_group!(EsmGroup::One, 30),
    /// CCM-R4 - self-test failed
    CCMR4SelfTest = map_ch_group!(EsmGroup::One, 31),
    /// N2HET2 - RAM parity error
    N2HET2Parity = map_ch_group!(EsmGroup::One, 34),
    /// FMC - correctable ECC error (Bank 7 access)
    FMCCorrectableEcc = map_ch_group!(EsmGroup::One, 35),
    /// FMC - uncorrectable ECC error (Bank 7 access)
    FMCUnCorrectableEcc = map_ch_group!(EsmGroup::One, 36),
    /// IOMM - Access to unimplemented location in IOMM frame, or
    /// write access detected in unprivileged mode
    IOMMAccess = map_ch_group!(EsmGroup::One, 37),
    /// Power domain controller compare error
    PowerDomainCompare = map_ch_group!(EsmGroup::One, 38),
    /// ?ower domain controller self-test error
    PowerDomainSelfTest = map_ch_group!(EsmGroup::One, 39),
    /// eFuse Controller Error – this error signal is generated
    /// when any bit in the eFuse controller error status register is set.
    /// The application can choose to generate an interrupt whenever
    /// this bit is set to service any eFuse controller error conditions.
    EFuseError = map_ch_group!(EsmGroup::One, 40),
    /// eFuse Controller - Self Test Error. This error signal is generated
    /// only when a self test on the eFuse controller generates an error condition.
    /// When an ECC self test error is detected, EsmGroup 1 channel 40 error signal
    /// will also be set.
    EFuseSelfTestError = map_ch_group!(EsmGroup::One, 41),
    /// DCC2 - error
    DCC2Error = map_ch_group!(EsmGroup::One, 62),
    /// CCMR4 - dual-CPU lock-step error
    CCMR4LockStep = map_ch_group!(EsmGroup::Two, 2),
    /// FMC - uncorrectable address parity error on accesses to main flash
    FMCUncorrectableParity = map_ch_group!(EsmGroup::Two, 4),
    /// RAM even bank (B0TCM) - uncorrectable redundant address decode error
    RamEvenUnCorrectableDecode = map_ch_group!(EsmGroup::Two, 6),
    /// RAM odd bank (B1TCM) - uncorrectable redundant address decode error
    RamOddUnCorrectableDecode = map_ch_group!(EsmGroup::Two, 8),
    /// RAM even bank (B0TCM) - address bus parity error
    RamEvenParity = map_ch_group!(EsmGroup::Two, 10),
    /// RAM odd bank (B1TCM) - address bus parity error
    RamOddParity = map_ch_group!(EsmGroup::Two, 12),
    /// TCM - ECC live lock detect
    TCMLock = map_ch_group!(EsmGroup::Two, 16),
    /// Windowed Watchdog (WWD) violation
    WWD = map_ch_group!(EsmGroup::Two, 24),
    /// eFuse Farm - autoload error
    EfuseAutoload = map_ch_group!(EsmGroup::Three, 1),
    /// RAM even bank (B0TCM) - ECC uncorrectable error
    RamEvenUnCorrectableECC = map_ch_group!(EsmGroup::Three, 3),
    /// RAM odd bank (B1TCM) - ECC uncorrectable error
    RamOddUnCorrectableECC = map_ch_group!(EsmGroup::Three, 5),
    /// FMC - uncorrectable ECC error: bus1 and bus2 interfaces
    /// (does not include address parity error and errors on accesses to Bank 7)
    FMCUncorrectableECC = map_ch_group!(EsmGroup::Three, 7),
}

impl EsmError {
    pub fn ch(self) -> usize {
        (self as usize) >> 24
    }

    pub fn group(self) -> usize {
        (self as usize) & 0xFF
    }
}

impl From<u8> for EsmError {
    fn from(v: u8) -> Self {
        let e = match v {
            0  ..= 31 => map_ch_group!(EsmGroup::One, v),
            32 ..= 63 => map_ch_group!(EsmGroup::Two, v - 32),
            64 ..= 95 => map_ch_group!(EsmGroup::Three, v - 64),
            96 ..= 127 => map_ch_group!(EsmGroup::Four, v - 96),
            _ => unreachable!(),
        };
        unsafe { core::mem::transmute(e) }
    }
}
