/// Work Around for Errata CORTEX-R4#66:
///
/// Errata Description:
///     Register Corruption During A Load-Multiple Instruction At
///     an Exception Vector
/// Workaround:
///     Disable out-of-order completion for divide instructions
///     (bit 7) in Auxiliary Control register
#[inline]
pub unsafe fn errata66() {
    asm!("
        mrc p15, #0, r0, c1, c0, #1
        orr r0, r0, #0x80
        mcr p15, #0, r0, c1, c0, #1
    "::: "memory" : "volatile");
}

/// Work Around for Errata CORTEX-R4#57:
///
/// Errata Description:
///     Conditional VMRS APSR_Nzcv, FPSCR May
///     Evaluate With Incorrect Flags
/// Workaround:
///     Disable out-of-order single-precision floating point
///     multiply-accumulate instruction completion [BIT 16 (Set DOOFMACS)]
#[inline]
pub unsafe fn errata57() {
    asm!("
        mrc p15, #0, r0, c15, c0, #0
        orr r0, r0, #0x10000
        mcr p15, #0, r0, c15, c0, #0
    "::: "memory" : "volatile");
}
