use config;
///
/// SCI/LIN Device Driver
///
use core::cell::Cell;
use serial::{event, DataBits, Parity, SerialLine, StopBits};
use vcell::VolatileCell;

#[repr(C)]
#[allow(non_snake_case)]
struct LinRegisters {
    GCR0: VolatileCell<u32>,        // 0x00: Global control 0
    GCR1: VolatileCell<u32>,        // 0x04: Global control 1
    GCR2: VolatileCell<u32>,        // 0x08: Global control 2
    SETINT: VolatileCell<u32>,      // 0x0C: Set interrupt enable
    CLEARINT: VolatileCell<u32>,    // 0x10: Clear interrupt enable
    SETINTLVL: VolatileCell<u32>,   // 0x14: Set interrupt level
    CLEARINTLVL: VolatileCell<u32>, // 0x18: Set interrupt level
    FLR: VolatileCell<u32>,         // 0x1C: interrupt flag
    INTVECT0: VolatileCell<u32>,    // 0x20: interrupt vector Offset 0
    INTVECT1: VolatileCell<u32>,    // 0x24: interrupt vector Offset 1
    FORMAT: VolatileCell<u32>,      // 0x28: Format Control
    BRS: VolatileCell<u32>,         // 0x2C: Baud rate selection
    ED: VolatileCell<u32>,          // 0x30: Emulation
    RD: VolatileCell<u32>,          // 0x34: Receive data
    TD: VolatileCell<u32>,          // 0x38: Transmit data
    PIO0: VolatileCell<u32>,        // 0x3C: Pin function
    PIO1: VolatileCell<u32>,        // 0x40: Pin direction
    PIO2: VolatileCell<u32>,        // 0x44: Pin data in
    PIO3: VolatileCell<u32>,        // 0x48: Pin data out
    PIO4: VolatileCell<u32>,        // 0x4C: Pin data set
    PIO5: VolatileCell<u32>,        // 0x50: Pin data clr
    PIO6: VolatileCell<u32>,        // 0x54: Pin open drain output enable
    PIO7: VolatileCell<u32>,        // 0x58: Pin pullup/pulldown disable
    PIO8: VolatileCell<u32>,        // 0x5C: Pin pullup/pulldown selection
    COMP: VolatileCell<u32>,        // 0x60: Compare
    RDx: [VolatileCell<u32>; 8],    // 0x64-0x0068: RX buffer
    MASK: VolatileCell<u32>,        // 0x6C: Mask
    ID: VolatileCell<u32>,          // 0x70: Identification
    TDx: [VolatileCell<u32>; 8],    // 0x74-0x0078: TX buffer
    MBRSR: VolatileCell<u32>,       // 0x7C: Maximum baud rate selection
    _rsvd1: [VolatileCell<u32>; 4], // 0x80 - 0x8C: Reserved
    IODFTCTRL: VolatileCell<u32>,   // 0x90: IODFT loopback
}
#[allow(dead_code)]
const LIN_BASE_ADDR: *const LinRegisters = 0xFFF7_E400 as *const LinRegisters;

#[repr(C)]
#[allow(non_snake_case)]
struct SciRegisters {
    GCR0: VolatileCell<u32>, // 0x00 Global Control 0
    GCR1: VolatileCell<u32>, // 0x04 Global Control 1
    GCR2: VolatileCell<u32>, // 0x08 Global Control 2
    // Note:GCR2 is Applicable only to LIN & SCI Compatibility
    // Mode,Reserved for standalone SCI
    SETINT: VolatileCell<u32>,       // 0x0C Set Interrupt Enable
    CLEARINT: VolatileCell<u32>,     // 0x10 Clear Interrupt Enable
    SETINTLVL: VolatileCell<u32>,    // 0x14 Set Interrupt Level
    CLEARINTLVL: VolatileCell<u32>,  // 0x18 Set Interrupt Level
    FLR: VolatileCell<u32>,          // 0x1C Interrupt Flag
    INTVECT0: VolatileCell<u32>,     // 0x20 Interrupt Vector Offset 0
    INTVECT1: VolatileCell<u32>,     // 0x24 Interrupt Vector Offset 1
    FORMAT: VolatileCell<u32>,       // 0x28 Format Control
    BRS: VolatileCell<u32>,          // 0x2C Baud Rate Selection
    ED: VolatileCell<u32>,           // 0x30 Emulation
    RD: VolatileCell<u32>,           // 0x34 Receive Data Buffer
    TD: VolatileCell<u32>,           // 0x38 Transmit Data Buffer
    PIO0: VolatileCell<u32>,         // 0x3C Pin Function
    PIO1: VolatileCell<u32>,         // 0x40 Pin Direction
    PIO2: VolatileCell<u32>,         // 0x44 Pin Data In
    PIO3: VolatileCell<u32>,         // 0x48 Pin Data Out
    PIO4: VolatileCell<u32>,         // 0x4C Pin Data Set
    PIO5: VolatileCell<u32>,         // 0x50 Pin Data Clr
    PIO6: VolatileCell<u32>,         // 0x54: Pin Open Drain Output Enable
    PIO7: VolatileCell<u32>,         // 0x58: Pin Pullup/Pulldown Disable
    PIO8: VolatileCell<u32>,         // 0x5C: Pin Pullup/Pulldown Selection
    _rsdv2: [VolatileCell<u32>; 12], // 0x60: Reserved
    IODFTCTRL: VolatileCell<u32>,    // 0x90: I/O Error Enable
}
//const SCI_BASE_ADDR: *const SciRegisters = 0xFFF7_E500 as *const SciRegisters;
const SCI_BASE_ADDR: *const SciRegisters = 0xFFF7_E400 as *const SciRegisters;

const TX_ENABLE: u32 = 0x1 << 25; // enable transmit
const RX_ENABLE: u32 = 0x1 << 24; // enable receive
const INT_CLOCK: u32 = 0x1 << 5; // internal clock
const ASYNC_TIMING: u32 = 0x1 << 1; // asynchronous timing mode

const PIO0_TX_FUNC_LIN: u32 = 0x1 << 2; // LINTX as SCI/LIN transmit pin
const PIO0_RX_FUNC_LIN: u32 = 0x1 << 1; // LINRX as SCI/LIN receive pin

const SCIGCR1_SWNRST: u32 = 0x1 << 7;

pub struct SciChipset {
    id: Cell<u32>,
    baudrate: Cell<u32>,
    regs: &'static SciRegisters,
}

impl SciRegisters {
    unsafe fn new() -> &'static SciRegisters {
        &*SCI_BASE_ADDR
    }

    pub fn reset(&self) {
        self.GCR0.set(0x0); // module is in reset
        self.GCR0.set(0x1); // is out of reset
    }

    pub fn sw_reset(&self, rst: bool) {
        if rst {
            self.GCR1.set(self.GCR1.get() | SCIGCR1_SWNRST)
        } else {
            self.GCR1.set(self.GCR1.get() & !SCIGCR1_SWNRST)
        }
    }

    pub fn tx_enable(&self, enable: bool) {
        let ctrl = self.GCR1.get();
        if enable {
            self.GCR1.set(ctrl | TX_ENABLE)
        } else {
            self.GCR1.set(ctrl & !TX_ENABLE)
        }
    }

    pub fn rx_enable(&self, enable: bool) {
        let ctrl = self.GCR1.get();
        if enable {
            self.GCR1.set(ctrl | RX_ENABLE)
        } else {
            self.GCR1.set(ctrl & !RX_ENABLE)
        }
    }

    pub fn clear_interrupts(&self) {
        self.CLEARINT.set(0xFFFF_FFFF);
        self.CLEARINTLVL.set(0xFFFF_FFFF);
    }

    pub fn getc(&self) -> u8 {
        wait_until_zero!(self.FLR.get(), event::RX_INT);
        self.RD.get() as u8
    }

    pub fn getc_try(&self) -> Option<u8> {
        if self.FLR.get() & event::RX_INT == 0 {
            return None;
        }
        Some(self.RD.get() as u8)
    }

    pub fn putc(&self, b: u8) {
        wait_until_zero!(self.FLR.get(), event::TX_INT);
        self.TD.set(u32::from(b));
    }

    pub fn write(&self, buffer: &[u8]) {
        for b in buffer.iter() {
            self.putc(*b);
        }
    }

    pub fn read(&self, buffer: &mut [u8]) {
        for b in buffer.iter_mut() {
            *b = self.getc();
        }
    }

    pub fn flags(&self) -> u32 {
        let mask = event::FE_INT | event::OE_INT | event::PE_INT | event::BREAK_INT;
        let err = self.FLR.get() | mask;
        self.FLR.set(mask); // reset errors
        err
    }
}

impl SerialLine for SciChipset {
    /// `databits` Number of bit per char
    /// `stop` Number of stop bits
    /// `parity` Parity Odd, Even or None (disabled)
    fn new(id: u32, databits: DataBits, stop: StopBits, parity: Parity) -> SciChipset {
        let ser_line = SciChipset {
            id: Cell::new(id),
            baudrate: Cell::new(0),
            regs: unsafe { SciRegisters::new() },
        };

        ser_line.regs.reset();
        ser_line.regs.clear_interrupts();

        let ctrl = INT_CLOCK | ASYNC_TIMING | (stop as u32) | (parity as u32);
        ser_line.regs.GCR1.set(ctrl);

        ser_line.regs.FORMAT.set(databits as u32);
        ser_line.regs.PIO0.set(PIO0_TX_FUNC_LIN | PIO0_RX_FUNC_LIN);
        ser_line
    }

    fn id(&self) -> u32 {
        self.id.get()
    }

    fn open(&self) {
        self.regs.sw_reset(true);
    }

    fn close(&self) {
        self.regs.sw_reset(false);
    }

    #[inline]
    fn baudrate(&self) -> u32 {
        self.baudrate.get()
    }

    fn set_baudrate(&mut self, baudrate: u32) -> &mut SciChipset {
        if baudrate > 0 {
            let is_async = self.regs.GCR1.get() & ASYNC_TIMING != 0;
            let f = if is_async { 16 } else { 1 };
            let br = udiv_round_closest!(config::VCLK, f * baudrate) - 1;
            self.regs.BRS.set(br);
            self.baudrate.set(baudrate);
        }
        self
    }

    #[inline]
    fn write(&self, buffer: &[u8]) {
        self.regs.write(buffer)
    }

    #[inline]
    fn read(&self, buffer: &mut [u8]) {
        self.regs.read(buffer)
    }

    #[inline]
    fn put(&self, b: u8) {
        self.regs.putc(b)
    }

    #[inline]
    fn get(&self) -> u8 {
        self.regs.getc()
    }

    #[inline]
    fn getc_try(&self) -> Option<u8> {
        self.regs.getc_try()
    }

    #[inline]
    fn error(&self) -> u32 {
        self.regs.flags()
    }

    fn interrupt(&self, ev: event::SciEvent) {
        self.regs.SETINT.set(ev & !event::TX_INT)
    }

    fn rx_enable(&mut self, enable: bool) -> &mut SciChipset {
        self.regs.rx_enable(enable);
        self
    }

    fn tx_enable(&mut self, enable: bool) -> &mut SciChipset {
        self.regs.tx_enable(enable);
        self
    }
}
