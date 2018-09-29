/// VCLK frequency [MHz]
pub const VCLK: u32 = MHz!(80);

/// LPO value to use if not available in OTP memory
pub const LPO: u32 = 0xFF;

/// RTI1 Clock Frequency [MHz]
pub const RTICLK1: u32 = MHz!(80);

/// LPC preload as used by the ESM driver
pub const LPC_PRELOAD: u32 = 16384;
