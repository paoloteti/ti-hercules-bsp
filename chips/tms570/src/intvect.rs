///
/// ARM Cortex-R Vectors Table
///

#[allow(dead_code)]
extern "C" {
    fn tms570_reset();
    pub fn prefetch_abort();
    pub fn phantom_interrupt();
    pub fn svc_handler();
    fn _dabort();
}

global_asm!(r#"
    .section .intvecs,"a",%progbits
    .extern tms570_reset
    .extern _dabort
    .extern prefetch_abort
    .extern phantom_interrupt
    .extern svc_handler
    .weak reset_entry

reset_entry:
    b   tms570_reset
undef_entry:
    b   undef_entry
    b   svc_handler
    b   prefetch_abort
    b   _dabort
    b   phantom_interrupt
    ldr pc,[pc,#-0x1b0]
    ldr pc,[pc,#-0x1b0]
"#);