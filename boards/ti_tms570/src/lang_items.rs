/// Default panic handler
#[lang = "panic_fmt"]
#[cfg(not(test))]
#[no_mangle]
pub extern fn panic_fmt(_args: ::core::fmt::Arguments,
                        _file: &'static str,
                        _line: u32) -> ! {
    loop{}
}

/// Lang item required to make the normal `main` work in applications
/// Called by TMS570 crate
#[lang = "start"]
extern "C" fn start<T>(main: fn() -> T, _argc: isize, _argv: *const *const u8) -> isize
where
    T: Termination,
{
    main();

    0
}

#[lang = "termination"]
pub trait Termination {
    fn report(self) -> i32;
}

impl Termination for () {
    fn report(self) -> i32 {
        0
    }
}

