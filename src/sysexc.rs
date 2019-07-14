/// System Exception Module

use vcell::VolatileCell;

#[repr(C)]
pub struct SysException {
    exc: VolatileCell<u32>,
}

/// System Reset root-causes
pub enum Reset {
    /// Power-on reset condition
    PowerOn     = 0x8000,
    /// Reset caused due to oscillator failure
    OscFailure  = 0x4000,
    /// Windowed watchdog violation or
    /// ICEPICK Reset (loading code / reset through a debugger)
    WdIcePick   = 0x2000,
    /// Reset caused due to CPU reset.
    /// CPU reset can be caused by CPU self-test completion, or
    /// by toggling the "CPU RESET" bit of the CPU Reset Control Register.
    Cpu         = 0x0020,
    /// Reset caused due to software reset.
    Sw          = 0x0010,
    /// Reset caused by external device.
    External    = 0x0008,
}

const SYS_EXC_ADDR: *const SysException = 0xFFFF_FFE4 as *const SysException;

impl SysException {
    pub unsafe fn new() -> &'static SysException {
        &*SYS_EXC_ADDR
    }

    pub fn power_on(&self) -> bool {
        self.exc.get() & (Reset::PowerOn as u32) != 0
    }

    pub fn osc_failure(&self) -> bool {
        self.exc.get() & (Reset::OscFailure as u32) != 0
    }

    pub fn wd_iecpick_reset(&self) -> bool {
        self.exc.get() & (Reset::WdIcePick as u32) != 0
    }

    pub fn sw_reset(&self) -> bool {
        self.exc.get() & (Reset::Sw as u32) != 0
    }

    pub fn cpu_reset(&self) -> bool {
        self.exc.get() & (Reset::Cpu as u32) != 0
    }

    pub fn external(&self) -> bool {
        self.exc.get() & (Reset::External as u32) != 0
    }

    pub fn clear(&self, flag:Reset) {
        self.exc.set(flag as u32)
    }

    pub fn clear_all(&self) {
        self.exc.set(0xffff)
    }
}
