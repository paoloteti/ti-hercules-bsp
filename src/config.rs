/// VCLK frequency [MHz]
pub const VCLK: u32 = MHz!(80);

/// LPO value to use if not available in OTP memory
pub const LPO: u32 = 0xFF;

/// RTI1 Clock Frequency [MHz]
pub const RTICLK1: u32 = MHz!(80);

/// LPC preload as used by the ESM driver
pub const LPC_PRELOAD: u32 = 16384;

pub mod flash {
/// Flash access wait states for bank 7 (EEPROM emulation)
pub const EEPROM_WAITSTATE: u32 = 0x3;
/// EEPROM Auto-ssuspend Enable
pub const EEPROM_AUTOSUSP_EN: bool = false;
/// EEPROM Auto-suspend Startup Grace Period
pub const EEPROM_AUTOSTART_GRACE: u32 = 0x2;
}
