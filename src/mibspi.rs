///
use vcell::VolatileCell;

#[repr(C)]
#[allow(non_snake_case)]
struct MibSpiRegisters {
    GCR0: VolatileCell<u32>,            // Global Control 0
    GCR1: VolatileCell<u32>,            // Global Control 1
    INT0: VolatileCell<u32>,            // Interrupt Register
    LVL: VolatileCell<u32>,             // Interrupt Level
    FLG: VolatileCell<u32>,             // Interrupt flags
    PC0: VolatileCell<u32>,             // Function Pin Enable
    PC1: VolatileCell<u32>,             // Pin Direction
    PC2: VolatileCell<u32>,             // Pin Input Latch
    PC3: VolatileCell<u32>,             // Pin Output Latch
    PC4: VolatileCell<u32>,             // Output Pin Set
    PC5: VolatileCell<u32>,             // Output Pin Clr
    PC6: VolatileCell<u32>,             // Open Drain Output Enable
    PC7: VolatileCell<u32>,             // Pullup/Pulldown Disable
    PC8: VolatileCell<u32>,             // Pullup/Pulldown Selection
    DAT0: VolatileCell<u32>,            // Transmit Data
    DAT1: VolatileCell<u32>,            // Transmit Data with Format and Chip Select
    BUF: VolatileCell<u32>,             // Receive Buffer
    EMU: VolatileCell<u32>,             // Emulation Receive Buffer
    DELAY: VolatileCell<u32>,           // Delays
    DEF: VolatileCell<u32>,             // Default Chip Select
    FMT0: VolatileCell<u32>,            // Data Format 0
    FMT1: VolatileCell<u32>,            // Data Format 1
    FMT2: VolatileCell<u32>,            // Data Format 2
    FMT3: VolatileCell<u32>,            // Data Format 3
    INTVECT0: VolatileCell<u32>,        // Interrupt Vector 0
    INTVECT1: VolatileCell<u32>,        // Interrupt Vector 1
    SRSEL: VolatileCell<u32>,           // Slew Rate Select
    PMCTRL: VolatileCell<u32>,          // Parallel Mode Control
    MIBSPIE: VolatileCell<u32>,         // Multi-buffer Mode Enable
    TGITENST: VolatileCell<u32>,        // TG Interrupt Enable Set
    TGITENCR: VolatileCell<u32>,        // TG Interrupt Enable Clear
    TGITLVST: VolatileCell<u32>,        // Transfer Group Interrupt Level Set
    TGITLVCR: VolatileCell<u32>,        // Transfer Group Interrupt Level Clear
    TGINTFLG: VolatileCell<u32>,        // Transfer Group Interrupt Flag
    _reserved1: [VolatileCell<u32>; 2], // Reserved
    TICKCNT: VolatileCell<u32>,         // Tick Counter
    LTGPEND: VolatileCell<u32>,         // Last TG End Pointer
    TGCTRL: [VolatileCell<u32>; 16],    // Transfer Group Control
    DMACTRL: [VolatileCell<u32>; 8],    // DMA Control
    DMACOUNT: [VolatileCell<u32>; 8],   // DMA Count
    DMACNTLEN: VolatileCell<u32>,       // DMA Control length
    _reserved2: VolatileCell<u32>,      // Reserved
    UERRCTRL: VolatileCell<u32>,        // Multi-buffer RAM Uncorrectable Parity Error Control
    UERRSTAT: VolatileCell<u32>,        // Multi-buffer RAM Uncorrectable Parity Error Status
    UERRADDRRX: VolatileCell<u32>,      // RXRAM Uncorrectable Parity Error Address
    UERRADDRTX: VolatileCell<u32>,      // TXRAM Uncorrectable Parity Error Address
    RXOVRN_BUF_ADDR: VolatileCell<u32>, // RXRAM Overrun Buffer Address
    IOLPKTSTCR: VolatileCell<u32>,      // IO loopback
    EXT_PRESCALE1: VolatileCell<u32>,
    EXT_PRESCALE2: VolatileCell<u32>,
}

const MIBSPI1_ADDR: *const MibSpiRegisters = 0xFFF7_F400 as *const MibSpiRegisters;
const MIBSPI3_ADDR: *const MibSpiRegisters = 0xFFF7_F800 as *const MibSpiRegisters;
const MIBSPI5_ADDR: *const MibSpiRegisters = 0xFFF7_FC00 as *const MibSpiRegisters;
const MIBSPI_ADDR: [*const MibSpiRegisters; 3] = [MIBSPI1_ADDR, MIBSPI3_ADDR, MIBSPI5_ADDR];

#[cfg(target_endian = "little")]
#[allow(dead_code)]
struct MibSpiBufferTx {
    data: u16,    // tx buffer data
    control: u16, // tx buffer control
}

#[cfg(target_endian = "big")]
#[allow(dead_code)]
struct MibSpiBufferTx {
    control: u16, // tx buffer control
    data: u16,    // tx buffer data
}

#[cfg(target_endian = "little")]
#[allow(dead_code)]
struct MibSpiBufferRx {
    data: u16,  // rx buffer data
    flags: u16, // rx buffer flags
}

#[cfg(target_endian = "big")]
#[allow(dead_code)]
struct MibSpiBufferRx {
    data: u16,  // rx buffer data
    flags: u16, // rx buffer flags
}

#[allow(dead_code)]
struct MibspiRam {
    tx: [MibSpiBufferTx; 128],
    rx: [MibSpiBufferRx; 128],
}
const MIBSPI1_RAM_ADDR: *const MibspiRam = 0xFF0E_0000 as *const MibspiRam;
const MIBSPI3_RAM_ADDR: *const MibspiRam = 0xFF0C_0000 as *const MibspiRam;
const MIBSPI5_RAM_ADDR: *const MibspiRam = 0xFF0A_0000 as *const MibspiRam;
const MIBSPI_RAM_ADDR: [*const MibspiRam; 3] =
    [MIBSPI1_RAM_ADDR, MIBSPI3_RAM_ADDR, MIBSPI5_RAM_ADDR];

#[derive(Copy, Clone)]
pub enum MibSpiID {
    One = 0,
    Three = 1,
    Five = 2,
}

#[allow(dead_code)]
pub struct MibSpi {
    pub id: MibSpiID,
    regs: &'static MibSpiRegisters,
    ram: &'static MibspiRam,
}

impl MibSpi {
    pub fn new(id: MibSpiID, master: bool) -> MibSpi {
        let mibspi = MibSpi {
            id: id,
            regs: unsafe { &*MIBSPI_ADDR[id as usize] },
            ram: unsafe { &*MIBSPI_RAM_ADDR[id as usize] },
        };
        mibspi.init(master);
        mibspi
    }

    pub fn init(&self, master: bool) {
        self.regs.GCR0.set(0x0);
        self.regs.GCR0.set(0x1);
        let internal_clock = 0x1 << 1;

        let gcr1 = internal_clock | (master as u32);
        self.regs.GCR1.set(self.regs.GCR1.get() | gcr1);

        // startup the module
        self.regs.GCR1.set(self.regs.GCR1.get() | 0x0100_0000);
    }

    /// SPIENA pin high-impedance enable. When active, the SPIENA pin
    /// is forced to high-impedance when not driving a low signal.
    /// If inactive, then the pin will output both a high and a low signal.
    pub fn highz(&self, enable: bool) {
        let enablehighz = (enable as u32) << 24;
        let int0 = self.regs.INT0.get() & !enablehighz;
        self.regs.INT0.set(int0 | enablehighz);
    }
}
