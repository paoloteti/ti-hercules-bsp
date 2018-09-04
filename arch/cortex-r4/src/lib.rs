//! Shared implementations for ARM Cortex-R4 MCUs.

#![crate_name = "cortexr4"]
#![crate_type = "rlib"]
#![feature(asm,naked_functions)]
#![no_std]

pub mod asm;
pub mod mpu;
pub mod pmu;
pub mod silicon;

