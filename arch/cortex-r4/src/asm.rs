extern "C" {
    #[link_name = "llvm.arm.hint"]
    fn hint(a: i32);
}

/// NOP instruction
#[inline(always)]
pub fn nop() {
    unsafe { hint(0) };
}

/// WFI (Wait For Interrupt) makes the processor suspend
/// execution (Clock is stopped) until one of the following
/// events take place:
///    An IRQ interrupt
///    An FIQ interrupt
///    A Debug Entry request made to the processor.
#[inline(always)]
pub unsafe fn wfi() {
    hint(2);
}

