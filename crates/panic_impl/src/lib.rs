#![no_std]
#![feature(core_intrinsics)]
#![feature(panic_handler)]

#[cfg(not(test))]
#[panic_handler]
pub extern "C" fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::intrinsics::abort() }
}
