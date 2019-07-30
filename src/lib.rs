//! Peripheral implementations for the TMS570x MCUs.
//!

#![crate_name = "tms570"]
#![crate_type = "rlib"]
#![feature(asm)]
#![feature(global_asm)]
#![feature(naked_functions)]
#![no_std]

#[macro_use]
mod helpers;

pub mod adc;
pub mod can;
pub mod ccm;
pub mod config;
pub mod dma;
pub mod dma_ctrl;
pub mod dwd;
pub mod efuse;
pub mod esm;
pub mod esm_ch;
pub mod flash;
pub mod gio;
pub mod het;
pub mod hwcrc;
pub mod intvect;
pub mod iomm;
pub mod mibspi;
pub mod pbist;
pub mod pcr;
pub mod pinmux;
pub mod rti;
pub mod scilin;
pub mod serial;
pub mod startup;
pub mod stc;
pub mod sysexc;
pub mod system;
pub mod tcram;
pub mod vim;

extern "C" {
    static mut _heap_start: u32;
    static mut _heapsize: u32;
}

#[no_mangle]
#[naked]
pub unsafe extern "C" fn reset() -> ! {
    startup::tms570_startup()
}

pub fn heap_start() -> *mut u32 {
    unsafe { &mut _heap_start }
}

pub fn heap_size() -> *mut u32 {
    unsafe { &mut _heapsize }
}
