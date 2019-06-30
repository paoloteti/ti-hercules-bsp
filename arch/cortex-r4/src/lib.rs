//! Shared implementations for ARM Cortex-R4 MCUs.

#![crate_name = "cortexr4"]
#![crate_type = "rlib"]
#![feature(link_llvm_intrinsics)]
#![no_std]

pub mod asm;
pub mod mpu;

