use core::ptr;
///
/// Cyclic Redundancy Check Controller Module (MCRC)
///
/// MCRC Controller is a module which is used to perform CRC
/// (CyclicRedundancy Check) to verify the integrity of memory system.
/// A signature representing the contents of the memory is obtained when the
/// contents of the memory are read into MCRC Controller.
/// The responsibility of MCRC controller is tocalculate the signature for a
/// set of data and then compare the calculated signature value against a
/// pre-determined good signature value. MCRC controller provides up to four
/// channels to perform CRC calculation on multiple memories in parallel and can
/// be used on any memory system. Channel 1 can also be put into data trace
/// mode. In data trace mode, MCRC controller compresses each data being read
/// through CPU read data bus.
use vcell::VolatileCell;

#[repr(C)]
pub struct HwCrc {
    /// Global Control 0
    ctrl0: VolatileCell<u32>,
    /// Reserved
    _reserved1: VolatileCell<u32>,
    /// Global Control 1
    ctrl1: VolatileCell<u32>,
    /// Reserved
    _reserved2: VolatileCell<u32>,
    /// Global Control 2
    ctrl2: VolatileCell<u32>,
    /// Reserved
    _reserved3: VolatileCell<u32>,
    /// Interrupt Enable Set
    ints: VolatileCell<u32>,
    /// Reserved
    _reserved4: VolatileCell<u32>,
    /// Interrupt Enable Reset
    intr: VolatileCell<u32>,
    /// Reserved
    _reserved5: VolatileCell<u32>,
    /// Interrupt Status
    status: VolatileCell<u32>,
    /// Reserved
    _reserved6: VolatileCell<u32>,
    /// Interrupt Offset
    int_offset_reg: VolatileCell<u32>,
    /// Reserved
    _reserved7: VolatileCell<u32>,
    /// CRC Busy
    busy: VolatileCell<u32>,
    /// Reserved
    _reserved8: VolatileCell<u32>,
    /// Pattern Counter Preload 1
    pcount_reg1: VolatileCell<u32>,
    /// Sector Counter Preload 1
    scount_reg1: VolatileCell<u32>,
    /// Current Sector 1
    cursec_reg1: VolatileCell<u32>,
    /// Channel 1 Watchdog Timeout Preload A
    wdto_pld1: VolatileCell<u32>,
    /// Channel 1 Block Complete Timeout Preload B
    bcto_pld1: VolatileCell<u32>,
    /// Reserved
    _reserved9: [VolatileCell<u32>; 3],
    /// Channel 1 PSA signature low
    psa_sigregl1: VolatileCell<u32>,
    /// Channel 1 PSA signature high
    psa_sigregh1: VolatileCell<u32>,
    /// Channel 1 CRC value low
    regl1: VolatileCell<u32>,
    /// Channel 1 CRC value high
    regh1: VolatileCell<u32>,
    /// Channel 1 PSA sector signature low
    psa_secsigregl1: VolatileCell<u32>,
    /// Channel 1 PSA sector signature high
    psa_secsigregh1: VolatileCell<u32>,
    /// Channel 1 Raw Data Low
    raw_dataregl1: VolatileCell<u32>,
    /// Channel 1 Raw Data High
    raw_dataregh1: VolatileCell<u32>,
    /// CRC Pattern Counter Preload 2
    pcount_reg2: VolatileCell<u32>,
    /// Sector Counter Preload 2
    scount_reg2: VolatileCell<u32>,
    /// Current Sector 2
    cursec_reg2: VolatileCell<u32>,
    /// Channel 2 Watchdog Timeout Preload A
    wdto_pld2: VolatileCell<u32>,
    /// Channel 2 Block Complete Timeout Preload B
    bcto_pld2: VolatileCell<u32>,
    /// Reserved
    _reserved10: [VolatileCell<u32>; 3],
    /// Channel 2 PSA signature low
    psa_sigregl2: VolatileCell<u32>,
    /// Channel 2 PSA signature high
    psa_sigregh2: VolatileCell<u32>,
    /// Channel 2 CRC value low
    regl2: VolatileCell<u32>,
    /// Channel 2 CRC value high
    regh2: VolatileCell<u32>,
    /// Channel 2 PSA sector signature low
    psa_secsigregl2: VolatileCell<u32>,
    /// Channel 2 PSA sector signature high
    psa_secsigregh2: VolatileCell<u32>,
    /// Channel 2 Raw Data Low
    raw_dataregl2: VolatileCell<u32>,
    /// Channel 2 Raw Data High
    raw_dataregh2: VolatileCell<u32>,
}
const HWCRC_BASE_ADDR: *const HwCrc = 0xFE00_0000 as *const HwCrc;

#[derive(Clone, Copy, PartialEq)]
pub enum ChannelMode {
    /// Data Capture mode. In this mode, the PSA Signature does
    /// not compress data when it is written.
    DataCapute = 0x0,
    Auto = 0x1,
    /// After DMA does the transfer, CPU is invoked by CC interrupt
    /// to do signature verification
    Semi = 0x2,
    Full = 0x3,
}

#[derive(Clone, Copy, PartialEq)]
pub enum CrcChannel {
    /// CRC Channel 1
    CH1 = 0x0,
    /// CRC Channel 2
    CH2 = 0x1,
}

#[derive(Clone, Copy, PartialEq)]
pub enum CrcInterrupt {
    /// Compression complete interrupt is generated in Semi-CPU mode only. When the data pattern
    /// counter reaches zero, the compression complete flag is set and the interrupt is generated.
    CompressionComplete = 0x01,
    /// CRC fail interrupt is generated in AUTO mode only. When the signature verification fails, the
    /// CRC fail flag is set, CPU should take action to address the fail condition and clear the CRC
    /// fail flag after it resolves the CRC mismatch.
    CrcFail = 0x02,
    /// Overrun Interrupt is generated in either AUTO or Semi-CPU mode. During AUTO mode, if a CRC fail
    /// is detected then the current sector number is recorded in the current sector register.
    /// If CRC fail status bit is not cleared and current sector register is not read by the host CPU before
    /// another CRC fail is detected for another sector then an overrun interrupt is generated.
    /// During Semi-CPU mode, when the data pattern counter finishes counting, it generates a compression
    /// complete interrupt. At the same time the signature is copied into the PSA Sector Signature Register.
    /// If the host CPU does not readthe signature from PSA Sector Signature Register before it is updated
    /// again with a new signature value then an overrun interrupt is generated.
    Overun = 0x04,
    /// Underrun interrupt only occurs in AUTO mode. The interrupt is generated when the CRC Value Register
    /// is not updated with the corresponding signature when the data pattern counter finishes counting.
    /// During AUTO mode, MCRC Controller generates DMA request to update CRC Value Register in
    /// synchronization to the corresponding sector of the memory. Signature verfification is also performed
    /// if underrun condition is detected.
    /// A CRC fail interrupt is generated at the same time as the underrun interrupt.
    Underrun = 0x08,
    /// To ensure that the memory system is examined within a pre-defined time frame and no loss of
    /// incoming data there is a 24 bit timeout counter per CRC channel.
    /// See TMS570LSxx Reference Manual par. 22.3.10.5
    Timeout = 0x10,
}

impl HwCrc {
    pub unsafe fn new() -> &'static HwCrc {
        &*HWCRC_BASE_ADDR
    }

    pub fn ch_reset(&self, ch: CrcChannel, reset: bool) {
        if reset {
            self.ctrl0.set(self.ctrl0.get() | (0x1 << (ch as u32)));
        } else {
            self.ctrl0.set(self.ctrl0.get() & !(0x1 << (ch as u32)));
        }
    }

    pub fn power_down(&self, pdwn: bool) {
        self.ctrl1.set(pdwn as u32);
    }

    /// wd_timeout: timeout (max 2^23 [us]) within which the DMA must
    ///             transfer the next block of data patterns.
    /// blk_timeout: timeout (max 2^23 [us]) within which the CRC for
    ///             an entire block needs to complete before a timeout interrupt is generated.
    pub fn configure(
        &self,
        ch: CrcChannel,
        pcount: u32,
        scount: u32,
        wd_timeout: u32,
        blk_timeout: u32,
    ) {
        match ch {
            CrcChannel::CH1 => {
                self.pcount_reg1.set(pcount);
                self.scount_reg1.set(scount);
                self.wdto_pld1.set(us2hz!(wd_timeout));
                self.bcto_pld1.set(us2hz!(blk_timeout));
            }
            CrcChannel::CH2 => {
                self.pcount_reg2.set(pcount);
                self.scount_reg2.set(scount);
                self.wdto_pld2.set(us2hz!(wd_timeout));
                self.bcto_pld2.set(us2hz!(blk_timeout));
            }
        }
    }

    /// During AUTO or Semi-CPU mode, the busy flag is set when the first data pattern
    /// of the block is compressed and remains set until the the last data pattern of the block
    /// is compressed. The flag is cleared when the last data pattern of the block is compressed
    pub fn is_busy(&self, ch: CrcChannel) -> bool {
        (self.busy.get() & (ch as u32)) != 0
    }

    pub fn set_ch_mode(&self, ch: CrcChannel, mode: ChannelMode) {
        let mask = (mode as u32) << ((ch as u32) * 8);
        self.ctrl2.set(self.ctrl2.get() | mask);
    }

    /// When set, the channel is put into data trace mode. The channel snoops
    /// on the CPU Peripheral Bus Master, Flash, System RAM buses for any
    /// read transaction. Any read data on these buses is compressed by the
    /// PSA Signature . When suspend is on, the PSA Signature
    /// does not compress any read data on these buse
    pub fn data_trace(&self, enable: bool) {
        if enable {
            self.ctrl2.set(self.ctrl2.get() | (0x1 << 4));
        } else {
            self.ctrl2.set(self.ctrl2.get() & !(0x1 << 4));
        }
    }

    /// Current known good signature value stored for a given channel
    pub fn digest(&self, ch: CrcChannel) -> u64 {
        match ch {
            CrcChannel::CH1 => {
                (u64::from(self.psa_sigregl1.get()) << 32) | u64::from(self.psa_sigregh1.get())
            }
            CrcChannel::CH2 => {
                (u64::from(self.psa_sigregl2.get()) << 32) | u64::from(self.psa_sigregh2.get())
            }
        }
    }

    /// Generate CRC signature. (FULL CPU mode of operation only)
    pub unsafe fn generate(&self, ch: CrcChannel, mode: ChannelMode, data: *const u32, len: usize) {
        if mode == ChannelMode::Full {
            match ch {
                CrcChannel::CH1 => {
                    let dst = self.psa_sigregl1.get() as *mut u32;
                    ptr::copy(data, dst, len);
                }
                CrcChannel::CH2 => {
                    let dst = self.psa_sigregl2.get() as *mut u32;
                    ptr::copy(data, dst, len);
                }
            }
        }
    }

    pub fn notification(&self, ch: CrcChannel, flag: CrcInterrupt, enable: bool) {
        let mask = (flag as u32) * (ch as u32);
        if enable {
            self.ints.set(mask);
        } else {
            self.intr.set(mask);
        }
    }

    /// In AUTO mode, return the current sector number of which the signature
    /// verification fails. The sector counter is a free running up counter.
    /// When a sector fails, the erroneous sector number is logged into current
    /// sector ID and the CRC fail interrupt is generated
    /// The sector ID is frozen until it is read and the CRC fail status bit
    /// is cleared by CPU. While it is frozen, it does not capture
    /// another erroneoussector number. When this condition happens, an overrun
    /// interrupt is generated instead. Once the register is read and the CRC fail
    /// interrupt flag is cleared it can capture new erroneous sector number.
    pub fn failed_sector(&self, ch: CrcChannel) -> u32 {
        match ch {
            CrcChannel::CH1 => self.cursec_reg1.get(),
            CrcChannel::CH2 => self.cursec_reg2.get(),
        }
    }

    /// Get the highest priority pending interrupt vector address.
    /// This will automatically clear the respective interrupt flag
    pub fn interrupt(&self) -> u32 {
        self.int_offset_reg.get()
    }
}
