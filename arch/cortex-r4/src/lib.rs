//! Shared implementations for ARM Cortex-R4 MCUs.

#![crate_name = "cortexr4"]
#![crate_type = "rlib"]
#![feature(asm,const_fn,naked_functions)]
#![no_std]

pub mod mpu;
pub mod asm;
pub mod pmu;

/// Work Around for Errata CORTEX-R4#66:
///
/// Errata Description:
///     Register Corruption During A Load-Multiple Instruction At
///     an Exception Vector
/// Workaround:
///     Disable out-of-order completion for divide instructions
///     (bit 7) in Auxiliary Control register
pub unsafe fn errata66() {
    asm!("
        push {r0}
        mrc p15, #0, r0, c1, c0, #1
        orr r0, r0, #0x80
        mcr p15, #0, r0, c1, c0, #1
        pop {r0}
        bx lr
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
pub unsafe fn errata57() {
    asm!("
        push {r0}
        mrc p15, #0, r0, c15, c0, #0
        orr r0, r0, #0x10000
        mcr p15, #0, r0, c15, c0, #0
        pop {r0}
        bx lr
    "::: "memory" : "volatile");
}
