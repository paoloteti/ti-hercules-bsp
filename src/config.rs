/// VCLK frequency [MHz]
pub const VCLK: u32 = MHz!(80);

/// LPO value to use if not available in OTP memory (Low-frequency trim value)
///
/// Admitted values (forced to 1Fh if out of range):
///
/// | 0h  20.67  | 1h  25.76  | 2h  30.84  | 3h  35.90  | 4h  40.93  |
/// | 5h  45.95  | 6h  50.97  | 7h  55.91  | 8h  60.86  | 9h  65.78  |
/// | Ah  70.75  | Bh  75.63  | Ch  80.61  | Dh  85.39  | Eh  90.23  |
/// | Fh  95.11  | 10h 100.0  | 11h 104.84 | 12h 109.51 | 13h 114.31 |
/// | 14h 119.01 | 15h 123.75 | 16h 128.62 | 17h 133.31 | 18h 138.03 |
/// | 19h 142.75 | 1Ah 147.32 | 1Bh 152.02 | 1Ch 156.63 | 1Dh 161.38 |
/// | 1Eh 165.90 | 1Fh 170.42 |
pub const LPO: u8 = 0x10;

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
