use esm::Esm;
use esm_ch::{EsmError, EsmGroup};
use vcell::VolatileCell;
use vim::Vim;

pub struct Ccm {
    ccmsr: VolatileCell<u32>,   // 0x00 CCM-R4F StatusRegister
    ccmkeyr: VolatileCell<u32>, // 0x04 CCM-R4F Key Register
}
const CCMR4F_BASE_ADDR: *const Ccm = 0xFFFF_F600 as *const Ccm;

/// Self-test Error
const STE: u32 = 0x1;
/// Self-test Error Type
const STET: u32 = 0x1 << 1;
/// Self-test Complete
const STC: u32 = 0x1 << 8;
/// Compare Error
const CMPE: u32 = 0x1 << 16;

pub enum SelfTestError {
    /// Self-test not failed
    None,
    /// Self-test failed during Compare Match Test.
    CompareMatch,
    /// Self-test failed during Compare Mismatch Test
    CompareMismatch,
}

/// CCMR4F Modes
pub enum CcmMode {
    Lockstep = 0x0,
    SelfTest = 0x6,
    ErrorForcing = 0x9,
    SelfTestErrorForcing = 0xF,
}

impl Ccm {
    pub unsafe fn new() -> &'static Ccm {
        &*CCMR4F_BASE_ADDR
    }

    pub fn set_mode(&self, mode: CcmMode) {
        self.ccmkeyr.set(mode as u32)
    }

    pub fn self_test_error(&self) -> bool {
        self.ccmsr.get() & STE != 0
    }

    pub fn self_test_error_type(&self) -> SelfTestError {
        if self.self_test_error() {
            if (self.ccmsr.get() & STET) != STET {
                return SelfTestError::CompareMismatch;
            } else {
                return SelfTestError::CompareMatch;
            }
        }
        SelfTestError::None
    }

    /// Check if a compare error occurs
    /// 'false' if CPU signals are identical.
    /// 'true' if CPU signal compare mismatch.
    pub fn compare_error(&self) -> bool {
        self.ccmsr.get() & CMPE != 0
    }

    /// Clear any CPU signal compare mismatch error
    pub fn clear_error(&self) {
        self.ccmsr.set(CMPE)
    }

    pub unsafe fn self_test(&self) -> bool {
        self.set_mode(CcmMode::SelfTest);
        // Wait for CCM self-test to complete
        wait_until_set!(self.ccmsr.get(), STC);
        if self.self_test_error() {
            return false;
        }
        let esm = Esm::new();
        // Check CCM-R4 self-test error flag by itself (without compare error)
        if esm.error_is_set(EsmError::CCMR4SelfTest) {
            return false;
        }
        self.set_mode(CcmMode::ErrorForcing);
        wait_until_neq!(self.ccmkeyr.get(), 0);

        // check if compare error flag is set
        if esm.error_is_set(EsmError::CCMR4LockStep) {
            return false;
        }

        // Check FIQIVEC to ESM High Interrupt flag is set
        if Vim::new().fiq_id() == 1 {
            return false;
        }

        // clear ESM dual-CPU lock-step error
        esm.clear_error(EsmError::CCMR4LockStep);
        // clear ESM group2 shadow status flag
        esm.shadow_stat_clear(EsmGroup::Two);
        // clear ESM CCM-R4 self-test error
        esm.clear_error(EsmError::CCMR4SelfTest);
        // nERROR pin will become inactive once the LTC counter expires
        esm.error_reset();

        self.set_mode(CcmMode::SelfTestErrorForcing);
        wait_until_neq!(self.ccmkeyr.get(), 0);
        if esm.error_is_set(EsmError::CCMR4SelfTest) {
            return false;
        }
        esm.clear_error(EsmError::CCMR4SelfTest);
        true
    }
}
