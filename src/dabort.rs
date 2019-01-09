use esm::Esm;
use esm_ch::EsmError;
use flash::Flash;
use tcram::{TcRamID, Tcram};

unsafe fn stop_cpu() {
    cortexr4::asm::interrupts_disable();
    loop {}
}

#[no_mangle]
pub unsafe fn data_abort() {
    let esm = Esm::new();

    if esm.error_is_set(EsmError::RamEvenUnCorrectableECC) {
        let tcram1 = Tcram::new(TcRamID::One);
        if tcram1.ecc_write_enabled() {
            // Real RAM error
            stop_cpu();
        }
        tcram1.clear_error();
        esm.clear_error(EsmError::RamEvenUnCorrectableECC);
        esm.error_reset();
    }

    if esm.error_is_set(EsmError::RamOddUnCorrectableECC) {
        let tcram2 = Tcram::new(TcRamID::Two);
        if tcram2.ecc_write_enabled() {
            // Real RAM error
            stop_cpu();
        }
        tcram2.clear_error();
        esm.clear_error(EsmError::RamOddUnCorrectableECC);
        esm.error_reset();
    }

    // Flash access error
    if esm.error_is_set(EsmError::FMCUncorrectableECC) {
        let flash = Flash::new();
        //   if flash.diag_active() {
        // Real RAM error
        stop_cpu();
        // }
        esm.clear_error(EsmError::FMCUncorrectableECC);
        esm.error_reset();
    }
    //custom_dabort()

    // The abort was caused intentionally.
    // To avoid to cause it again, just branch to the instruction
    // after the one that caused this abort
    asm!("subs    pc, lr, #4");
}
