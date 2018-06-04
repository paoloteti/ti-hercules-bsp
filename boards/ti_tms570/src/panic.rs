use core::intrinsics;
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_implementation]
#[no_mangle]
pub extern fn panic(_info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}

