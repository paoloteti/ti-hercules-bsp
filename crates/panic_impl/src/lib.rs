#![no_std]
#![feature(core_intrinsics)]
#![feature(panic_implementation)]


#[cfg(not(test))]
#[panic_implementation]
pub extern fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::intrinsics::abort() }
}

