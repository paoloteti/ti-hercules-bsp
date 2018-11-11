///
/// Analog To Digital Converter (ADC) Module
///
/// The microcontroller includes two 12-bit ADC modules.
/// The main features of each of the ADC modules are:
/// - Selectable 10-bit or 12-bit resolution
/// - Successive-approximation-register architecture
/// - Three conversion groups – Group1, Group2 and Event Group
/// - All three conversion groups can be configured to be
///   hardware-triggered; group1 and group2 can also be triggered by software
/// - Conversion results are stored in a 64-word memory (SRAM)
///   These 64 words are divided between the three conversion groups and are
///   configurable by software
///   Accesses to the conversion result RAM are protected by parity
/// - Flexible options for generating DMA requests for transferring conversion
///   results
/// - Multichannel conversions performed in ascending order, one channel at a time
/// - Single or continuous conversion modes
/// - Embedded self-test logic for input channel failure mode (open / short)
///   detection
/// - Embedded calibration logic for offset error correction
/// - Enhanced Power-down mode
/// - External event pin (ADEVT) to trigger conversions
///   ADEVT is also programmable as general-purpose I/O
/// - Eight hardware events to trigger conversions
use core::cmp::min;
use vcell::VolatileCell;

#[repr(C)]
#[allow(non_snake_case)]
struct GxBUF {
    BUF0: VolatileCell<u32>,
    BUF1: VolatileCell<u32>,
    BUF2: VolatileCell<u32>,
    BUF3: VolatileCell<u32>,
    BUF4: VolatileCell<u32>,
    BUF5: VolatileCell<u32>,
    BUF6: VolatileCell<u32>,
    BUF7: VolatileCell<u32>,
}

#[repr(C)]
#[allow(non_snake_case)]
struct AdcRegisters {
    RSTCR: VolatileCell<u32>,              // 0x0000: Reset control
    OPMODECR: VolatileCell<u32>,           // 0x0004: Operating mode control
    CLOCKCR: VolatileCell<u32>,            // 0x0008: Clock control
    CALCR: VolatileCell<u32>,              // 0x000C: Calibration control
    GxMODECR: [VolatileCell<u32>; 3],      // 0x0010,0x0014,0x0018: Group 0-2 mode control
    EVSRC: VolatileCell<u32>,              // 0x001C: Group 0 trigger source control
    G1SRC: VolatileCell<u32>,              // 0x0020: Group 1 trigger source control
    G2SRC: VolatileCell<u32>,              // 0x0024: Group 2 trigger source control
    GxINTENA: [VolatileCell<u32>; 3],      // 0x0028,0x002C,0x0030: Group 0-2 interrupt enable
    GxINTFLG: [VolatileCell<u32>; 3],      // 0x0034,0x0038,0x003C: Group 0-2 interrupt flag
    GxINTCR: [VolatileCell<u32>; 3],       // 0x0040-0x0048: Group 0-2 interrupt threshold
    EVDMACR: VolatileCell<u32>,            // 0x004C: Group 0 DMA control
    G1DMACR: VolatileCell<u32>,            // 0x0050: Group 1 DMA control
    G2DMACR: VolatileCell<u32>,            // 0x0054: Group 2 DMA control
    BNDCR: VolatileCell<u32>,              // 0x0058: Buffer boundary control
    BNDEND: VolatileCell<u32>,             // 0x005C: Buffer boundary end
    EVSAMP: VolatileCell<u32>,             // 0x0060: Group 0 sample window
    G1SAMP: VolatileCell<u32>,             // 0x0064: Group 1 sample window
    G2SAMP: VolatileCell<u32>,             // 0x0068: Group 2 sample window
    EVSR: VolatileCell<u32>,               // 0x006C: Group 0 status
    G1SR: VolatileCell<u32>,               // 0x0070: Group 1 status
    G2SR: VolatileCell<u32>,               // 0x0074: Group 2 status
    GxSEL: [VolatileCell<u32>; 3],         // 0x0078-0x007C: Group 0-2 channel select
    CALR: VolatileCell<u32>,               // 0x0084: Calibration
    SMSTATE: VolatileCell<u32>,            // 0x0088: State machine state
    LASTCONV: VolatileCell<u32>,           // 0x008C: Last conversion
    buff: [GxBUF; 3],                      // 0x0090 ..0x00EC: Group x-y result buffer
    EVEMUBUFFER: VolatileCell<u32>,        // 0x00F0: Group 0 emulation result buffer
    G1EMUBUFFER: VolatileCell<u32>,        // 0x00F4: Group 1 emulation result buffer
    G2EMUBUFFER: VolatileCell<u32>,        // 0x00F8: Group 2 emulation result buffer
    EVTDIR: VolatileCell<u32>,             // 0x00FC: Event pin direction
    EVTOUT: VolatileCell<u32>,             // 0x0100: Event pin digital output
    EVTIN: VolatileCell<u32>,              // 0x0104: Event pin digital input
    EVTSET: VolatileCell<u32>,             // 0x0108: Event pin set
    EVTCLR: VolatileCell<u32>,             // 0x010C: Event pin clear
    EVTPDR: VolatileCell<u32>,             // 0x0110: Event pin open drain
    EVTDIS: VolatileCell<u32>,             // 0x0114: Event pin pull disable
    EVTPSEL: VolatileCell<u32>,            // 0x0118: Event pin pull select
    EVSAMPDISEN: VolatileCell<u32>,        // 0x011C: Group 0 sample discharge
    G1SAMPDISEN: VolatileCell<u32>,        // 0x0120: Group 1 sample discharge
    G2SAMPDISEN: VolatileCell<u32>,        // 0x0124: Group 2 sample discharge
    MAGINTCR1: VolatileCell<u32>,          // 0x0128: Magnitude interrupt control 1
    MAGINT1MASK: VolatileCell<u32>,        // 0x012C: Magnitude interrupt mask 1
    MAGINTCR2: VolatileCell<u32>,          // 0x0130: Magnitude interrupt control 2
    MAGINT2MASK: VolatileCell<u32>,        // 0x0134: Magnitude interrupt mask 2
    MAGINTCR3: VolatileCell<u32>,          // 0x0138: Magnitude interrupt control 3
    MAGINT3MASK: VolatileCell<u32>,        // 0x013C: Magnitude interrupt mask 3
    _reserved1: VolatileCell<u32>,         // 0x0140: Reserved
    _reserved2: VolatileCell<u32>,         // 0x0144: Reserved
    _reserved3: VolatileCell<u32>,         // 0x0148: Reserved
    _reserved4: VolatileCell<u32>,         // 0x014C: Reserved
    _reserved5: VolatileCell<u32>,         // 0x0150: Reserved
    _reserved6: VolatileCell<u32>,         // 0x0154: Reserved
    MAGTHRINTENASET: VolatileCell<u32>,    // 0x0158: Magnitude interrupt set
    MAGTHRINTENACLR: VolatileCell<u32>,    // 0x015C: Magnitude interrupt clear
    MAGTHRINTFLG: VolatileCell<u32>,       // 0x0160: Magnitude interrupt flag
    MAGTHRINTOFFSET: VolatileCell<u32>,    // 0x0164: Magnitude interrupt offset
    GxFIFORESETCR: [VolatileCell<u32>; 3], // 0x0168,0x016C,0x0170: Group 0-2 fifo reset
    EVRAMADDR: VolatileCell<u32>,          // 0x0174: Group 0 RAM pointer
    G1RAMADDR: VolatileCell<u32>,          // 0x0178: Group 1 RAM pointer
    G2RAMADDR: VolatileCell<u32>,          // 0x017C: Group 2 RAM pointer
    PARCR: VolatileCell<u32>,              // 0x0180: Parity control
    PARADDR: VolatileCell<u32>,            // 0x0184: Parity error address
    PWRUPDLYCTRL: VolatileCell<u32>,       // 0x0188: Power-Up delay control
    _reserved7: VolatileCell<u32>,         // 0x018C: Reserved
    ADEVCHNSELMODECTRL: VolatileCell<u32>, // 0x0190: Event Group Channel Selection Mode Control
    ADG1CHNSELMODECTRL: VolatileCell<u32>, // 0x0194: Group1 Channel Selection Mode Control
    ADG2CHNSELMODECTRL: VolatileCell<u32>, // 0x0198: Group2 Channel Selection Mode Control
    ADEVCURRCOUNT: VolatileCell<u32>,      // 0x019C: Event Group Current Count
    ADEVMAXCOUNT: VolatileCell<u32>,       // 0x01A0: Event Group Max Count
    ADG1CURRCOUNT: VolatileCell<u32>,      // 0x01A4: Group1 Current Count
    ADG1MAXCOUNT: VolatileCell<u32>,       // 0x01A8: Group1 Max Count
    ADG2CURRCOUNT: VolatileCell<u32>,      // 0x01AC: Group2 Current Count
    ADG2MAXCOUNT: VolatileCell<u32>,       // 0x01B0: Group2 Max Count
}
const ADC1_BASE_ADDR: *const AdcRegisters = 0xFFF7_C000 as *const AdcRegisters;
const ADC2_BASE_ADDR: *const AdcRegisters = 0xFFF7_C200 as *const AdcRegisters;

const ADC_BASE_ADDR: [*const AdcRegisters; 2] = [ADC1_BASE_ADDR, ADC2_BASE_ADDR];

const ADC1_RAM_ADDR: *const u32 = 0xFF3E_0000 as *const u32;
const ADC2_RAM_ADDR: *const u32 = 0xFF3A_0000 as *const u32;
const ADC_RAM_ADDR: [*const u32; 2] = [ADC1_RAM_ADDR, ADC2_RAM_ADDR];

const ADC1_PRAM_ADDR: *const u32 = 0xFF3E_1000 as *const u32;
const ADC2_PRAM_ADDR: *const u32 = 0xFF3A_1000 as *const u32;
const ADC_PRAM_ADDR: [*const u32; 2] = [ADC1_PRAM_ADDR, ADC2_PRAM_ADDR];

const ADC1_LUT_ADDR: *const u32 = 0xFF3E_2000 as *const u32;
const ADC2_LUT_ADDR: *const u32 = 0xFF3A_2000 as *const u32;
const ADC_LUT_ADDR: [*const u32; 2] = [ADC1_LUT_ADDR, ADC2_LUT_ADDR];

pub enum ConvEvent {
    /// Interrupt is not generated (event disabled)
    None = 0x0,
    /// Interrupt is generated if threshold counter reaches zero.
    Threshold = 0x1,
    /// A memory overrun occurs when the ADC tries to write a
    /// new conversion result to the Group results memory which is already full
    ConversionOverrun = 0x1 << 1,
    /// A Group conversion end interrupt is generated when conversion
    /// of all the channels selected for conversion in the Group is done.
    ConversionEnd = 0x1 << 3,
}

#[derive(Copy, Clone)]
pub enum AdcID {
    One = 0,
    Two = 1,
}

#[derive(Copy, Clone)]
pub enum AdcGroup {
    /// ADC event group
    Event = 0,
    /// ADC group 1
    One = 1,
    /// ADC group 2
    Two = 2,
}

#[derive(Copy, Clone)]
pub enum AdcCoreResolution {
    Bit10 = 0,
    Bit12 = 1,
}

#[derive(Copy, Clone)]
pub enum ReadDataFormat {
    /// 12 bit data
    Bit12 = 0x0000,
    /// 10 bit data
    Bit10 = 0x0100,
    /// 8 bit data
    Bit8 = 0x0200,
}

#[derive(Copy, Clone, Default)]
pub struct AdcSample {
    pub valid: bool,
    pub value: u32,
    pub ch: u32,
}

#[allow(dead_code)]
pub struct Adc {
    pub id: AdcID,
    fifo_size: u8,
    format: ReadDataFormat,
    regs: &'static AdcRegisters,
    ram: *const u32,
    pram: *const u32,
    lut: *const u32,
}

impl Adc {
    pub fn new(id: AdcID, fifo_size: u8, res: AdcCoreResolution) -> Adc {
        let adc = Adc {
            id: id,
            fifo_size: fifo_size,
            format: ReadDataFormat::Bit12,
            regs: unsafe { &*ADC_BASE_ADDR[id as usize] },
            ram: unsafe { &*ADC_RAM_ADDR[id as usize] },
            pram: unsafe { &*ADC_PRAM_ADDR[id as usize] },
            lut: unsafe { &*ADC_LUT_ADDR[id as usize] },
        };
        adc.init(res);
        adc
    }

    fn init(&self, core_res: AdcCoreResolution) {
        // Reset ADC
        self.regs.RSTCR.set(0x1);
        self.regs.RSTCR.set(0x0);
        let res = (core_res as u32) << 31;
        self.regs.OPMODECR.set(self.regs.OPMODECR.get() | res);
        self.regs.CLOCKCR.set(0x7);
        self.regs.BNDCR.set((8 << 16) | 16);
        self.regs.BNDEND.set(self.regs.BNDEND.get() & 0xFFFF0002);

        //TODO: move outside
        self.regs.G1SAMP.set(1);
        self.regs.G2SAMP.set(1);
    }


    pub fn group_resolution(&mut self, grp: AdcGroup, dformat: ReadDataFormat) {
        self.format = dformat;
        // Always add channel id in conversion result
        self.regs.GxMODECR[grp as usize].set((dformat as u32) | 0x20);
    }

    pub fn activate(&self) {
        self.regs.OPMODECR.set(self.regs.OPMODECR.get() | 0x0000_0001);
        // Wait for buffer initialization complete
        wait_until_not_zero!(self.regs.BNDEND.get(), 0xFFFF_0000);
	}

    pub fn done(&self, group: AdcGroup) -> bool {
        self.regs.GxINTFLG[group as usize].get() & ConvEvent::ConversionEnd as u32 != 0
    }

    pub fn fifo_full(&self, group: AdcGroup) -> bool {
        let is_full = ConvEvent::Threshold as u32 | ConvEvent::ConversionOverrun as u32;
        self.regs.GxINTFLG[group as usize].get() & is_full != 0
    }

    /// Resets ADC FiFo
    pub fn fifo_clean(&self, group: AdcGroup) {
        self.regs.GxFIFORESETCR[group as usize].set(0x1)
    }

    /// Unpack a raw sampling register into an AdcSample
    /// value based on configured ADC resolution
    fn unpack(&self, raw: u32, sample: &mut AdcSample) {
        match self.format {
            ReadDataFormat::Bit12 => {
                sample.valid = (raw >> 31) == 0x0;
                sample.value = raw & 0xfff;
                sample.ch = (raw >> 16) & 0x1f;
            },
            ReadDataFormat::Bit10 => {
                sample.valid = (raw >> 16) == 0x0;
                sample.value = raw & 0x3ff;
                sample.ch = (raw >> 10) & 0x1f;
            },
            ReadDataFormat::Bit8 => {
                sample.valid = (raw >> 16) == 0x0;
                sample.value = raw & 0xff;
                sample.ch = (raw >> 10) & 0x1f;
            },
        }
    }

    /// Get a list of samples from a given ADC group.
    /// No more than min(slice length, ADC fifo length) raw samples are
    /// copied into the slice.
    /// Return number of valid sample inside the slice.
    pub fn get(&self, group: AdcGroup, samples: &mut [AdcSample]) -> usize {
        let avail = self.regs.GxINTCR[group as usize].get();
        let count = if avail >= 256 {
            self.fifo_size
        } else {
            self.fifo_size - (avail as u8)
        };

        let size = min(samples.len(), count as usize);
        for i in 0..size {
            let raw = self.regs.buff[group as usize].BUF0.get();
            self.unpack(raw, &mut samples[i]);
        }
        size
    }

    /// Starts a conversion of the ADC hardware group
    pub fn start(&self, group: AdcGroup, ch: usize) {
        self.regs.GxINTCR[group as usize].set(self.fifo_size as u32);
        self.regs.GxSEL[group as usize].set(0x1 << ch);
    }

    /// Starts a conversion more than a channel of a given ADC hardware group
    pub fn start_parallel(&self, group: AdcGroup, mask: u32) {
        self.regs.GxINTCR[group as usize].set(self.fifo_size as u32);
        self.regs.GxSEL[group as usize].set(mask);
    }

    pub fn start_all(&self, group: AdcGroup) {
        self.start_parallel(group, 0xFFFF_FFFF);
    }

    /// Resets the FiFo read and write pointers
    pub fn reset_fifo(&self, group: AdcGroup) {
        self.regs.GxFIFORESETCR[group as usize].set(0x0);
    }

    /// Stops a conversion of the ADC hardware group
    pub fn stop(&self, group: AdcGroup) {
        self.regs.GxSEL[group as usize].set(0x0);
    }

    /// Stops conversion on all ADC hardware groups
    pub fn stop_all(&self) {
        self.stop(AdcGroup::Event);
        self.stop(AdcGroup::One);
        self.stop(AdcGroup::Two);
    }

    /// Computes offset error using Calibration mode
    /// ADC resolution is forced, temporarily, to be 12-bits.
    /// Offset error is the difference of the average value
    /// of 4 samples and (2^12 - 1)
    pub fn calibrate(&self) -> i16 {
        let mut sum = 0;
        let old_mode = self.regs.OPMODECR.get();
        self.regs.OPMODECR.set(AdcCoreResolution::Bit12 as u32);

        self.stop_all();

        for test in 0..3 {
            // Disable Calibration
            self.regs.CALCR.set(0x0);
            // Select test (change Bride-En, HiLo at each iteration)
            self.regs.CALCR.set((test << (test + 8)) as u32);
            // Enable Calibration
            self.regs.CALCR.set(self.regs.CALCR.get() | 0x1);
            // Start calibration conversion
            self.regs.CALCR.set(self.regs.CALCR.get() | 0x0001_0000);
            wait_until_set!(self.regs.CALCR.get(), 0x0001_0000);

            sum += self.regs.CALR.get();
        }
        self.regs.CALCR.set(0x0);

        // calculate error and write it back to CALR register as
        // a two's complement value
        let error = !(sum / 4).wrapping_sub(0x7FF);
        self.regs.CALR.set(error.wrapping_sub(1));

        self.regs.OPMODECR.set(old_mode);
        error as i16
    }

    /// Interrupt Enable on a given event.
    /// 'group' on witch even must be enabled
    /// 'event' kind of envent to be triggered
    pub fn event_notification(&self, group: AdcGroup, event: ConvEvent) {
        self.regs.GxINTENA[group as usize].set(event as u32)
    }
}
