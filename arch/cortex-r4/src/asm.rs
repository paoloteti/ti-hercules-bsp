use core::ops::FnOnce;

#[inline(always)]
/// NOP instruction
pub fn nop() {
    unsafe {
        asm!("nop" :::: "volatile");
    }
}

#[inline(always)]
/// WFI (Wait For Interrupt) makes the processor suspend
/// execution (Clock is stopped) until one of the following
/// events take place:
///    An IRQ interrupt
///    An FIQ interrupt
///    A Debug Entry request made to the processor.
pub unsafe fn wfi() {
    asm!("wfi" :::: "volatile");
}

#[inline(always)]
/// Global interrupts disable
pub unsafe fn interrupts_disable() {
    asm!("cpsid i" :::: "volatile");
}

#[inline(always)]
/// Global interrupts disable
pub unsafe fn interrupts_enable() {
    asm!("cpsie i" :::: "volatile");
}

pub unsafe fn atomic<F, R>(f: F) -> R
    where F: FnOnce() -> R
{
    interrupts_disable();
    let res = f();
    interrupts_enable();
    return res;
}
