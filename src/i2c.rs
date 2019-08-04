use crate::config;
use vcell::VolatileCell;

#[repr(C)]
#[allow(non_snake_case)]
struct i2cRegisters {
    /// I2C Own Address register
    OAR: VolatileCell<u32>,
    /// I2C Interrupt Mask/Status register
    IMR: VolatileCell<u32>,
    /// I2C Interrupt Status register
    STR: VolatileCell<u32>,
    /// I2C Clock Divider Low register
    CKL: VolatileCell<u32>,
    /// I2C Clock Divider High register
    CKH: VolatileCell<u32>,
    /// I2C Data Count register
    CNT: VolatileCell<u32>,

    /// I2C Data Receive register
    #[cfg(target_endian = "little")]
    DRR: VolatileCell<u8>,

    /// I2C Data Receive register, Reserved
    #[cfg(target_endian = "little")]
    _reserved1: VolatileCell<u8>,

    /// I2C Data Receive register, Reserved
    #[cfg(target_endian = "little")]
    _reserved2: VolatileCell<u8>,

    /// I2C Data Receive register, Reserved
    #[cfg(target_endian = "little")]
    _reserved3: VolatileCell<u8>,

    /// I2C Data Receive register, Reserved
    #[cfg(target_endian = "big")]
    _reserved3: VolatileCell<u8>,

    /// I2C Data Receive register, Reserved
    #[cfg(target_endian = "big")]
    _reserved2: VolatileCell<u8>,

    /// I2C Data Receive register, Reserved
    #[cfg(target_endian = "big")]
    _reserved1: VolatileCell<u8>,

    /// I2C Data Receive register
    #[cfg(target_endian = "big")]
    DRR: VolatileCell<u8>,

    /// I2C Slave Address register
    SAR: VolatileCell<u32>,

    /// I2C Data Transmit register,
    #[cfg(target_endian = "little")]
    DXR: VolatileCell<u8>,

    /// I2C Data Transmit register, Reserved
    #[cfg(target_endian = "little")]
    _reserved4: VolatileCell<u8>,

    /// I2C Data Transmit register, Reserved
    #[cfg(target_endian = "little")]
    _reserved5: VolatileCell<u8>,

    /// I2C Data Transmit register, Reserved
    #[cfg(target_endian = "little")]
    _reserved6: VolatileCell<u8>,

    /// I2C Data Transmit register, Reserved
    #[cfg(target_endian = "big")]
    _reserved6: VolatileCell<u8>,

    /// I2C Data Transmit register, Reserved
    #[cfg(target_endian = "big")]
    _reserved5: VolatileCell<u8>,

    /// I2C Data Transmit register, Reserved
    #[cfg(target_endian = "big")]
    _reserved4: VolatileCell<u8>,

    /// I2C Data Transmit register
    #[cfg(target_endian = "big")]
    DXR: VolatileCell<u8>,

    /// I2C Mode register
    MDR: VolatileCell<u32>,
    /// I2C Interrupt Vector register
    IVR: VolatileCell<u32>,
    /// I2C Extended Mode register
    EMDR: VolatileCell<u32>,
    /// I2C Prescaler register
    PSC: VolatileCell<u32>,
    /// I2C Peripheral ID register 1
    PID11: VolatileCell<u32>,
    /// I2C Peripheral ID register 2
    PID12: VolatileCell<u32>,
    /// I2C DMA Control Register
    DMACR: VolatileCell<u32>,
    /// Reserved
    _reserved7: VolatileCell<u32>,
    /// Reserved
    _reserved8: VolatileCell<u32>,
    /// Pin Function Register
    PFNC: VolatileCell<u32>,
    /// Pin Direction Register
    DIR: VolatileCell<u32>,
    /// Pin Data In Register
    DIN: VolatileCell<u32>,
    /// Pin Data Out Register
    DOUT: VolatileCell<u32>,
    /// Pin Data Set Register
    SET: VolatileCell<u32>,
    /// Pin Data Clr Register
    CLR: VolatileCell<u32>,
    /// Pin Open Drain Output Enable Register
    PDR: VolatileCell<u32>,
    /// Pin Pullup/Pulldown Disable Register
    PDIS: VolatileCell<u32>,
    /// Pin Pullup/Pulldown Selection Register
    PSEL: VolatileCell<u32>,
    /// Pin Slew Rate Select Register
    PSRS: VolatileCell<u32>,
}
const I2C_BASE_ADDR: *const i2cRegisters = 0xFFF7_D400 as *const i2cRegisters;

#[allow(dead_code)]
mod flags {
    pub const FD_FORMAT: u32 = 0x0008; // Free Data Format
    pub const START_BYTE: u32 = 0x0010;

    pub const RESET_IN: u32 = 0x0000;
    pub const RESET_OUT: u32 = 0x0020;

    pub const DLOOPBACK: u32 = 0x0040;
    pub const REPEATMODE: u32 = 0x0080; // In Master Mode only
    pub const AMODE_10BIT: u32 = 0x0100;
    pub const AMODE_7BIT: u32 = 0x0000;

    pub const TRANSMITTER: u32 = 0x0200;
    pub const RECEIVER: u32 = 0x0000;

    pub const MASTER: u32 = 0x0400;
    pub const SLAVE: u32 = 0x0000;
    pub const STOP_COND: u32 = 0x0800; // In Master Mode only
    pub const START_COND: u32 = 0x2000; // In Master Mode only
    pub const FREE_RUN: u32 = 0x4000;
    pub const NACK_MODE: u32 = 0x8000;

    pub const AL_INT: u32 = 0x0001; // arbitration lost
    pub const NACK_INT: u32 = 0x0002; // no acknowledgment
    pub const ARDY_INT: u32 = 0x0004; // access ready
    pub const RX_INT: u32 = 0x0008; // receive data ready
    pub const TX_INT: u32 = 0x0010; // transmit data ready
    pub const SCD_INT: u32 = 0x0020; // stop condition detect
    pub const AAS_INT: u32 = 0x0040; // address as slave

    pub const AL: u32 = 0x0001; // arbitration lost
    pub const NACK: u32 = 0x0002; // no acknowledgement
    pub const ARDY: u32 = 0x0004; // access ready
    pub const RX: u32 = 0x0008; // receive data ready
    pub const TX: u32 = 0x0010; // transmit data ready
    pub const SCD: u32 = 0x0020; // stop condition detect
    pub const AD0: u32 = 0x0100; // address Zero Status
    pub const AAS: u32 = 0x0200; // address as slave
    pub const XSMT: u32 = 0x0400; // Transmit shift empty not
    pub const RXFULL: u32 = 0x0800; // receive full
    pub const BUSBUSY: u32 = 0x1000; // bus busy
    pub const NACKSNT: u32 = 0x2000; // No Ack Sent
    pub const SDIR: u32 = 0x4000; // Slave Direction
}

#[derive(Clone, Copy)]
pub enum I2cBitCount {
    Use2Bit = 0x2,
    Use3Bit = 0x3,
    Use4Bit = 0x4,
    Use5Bit = 0x5,
    Use6Bit = 0x6,
    Use7Bit = 0x7,
    Use8Bit = 0x0,
}

pub struct I2C {
    regs: &'static i2cRegisters,
}

impl I2C {
    pub fn new(master_address: u8, slave_address: u8, baud: u32) -> I2C {
        let i2c = I2C {
            regs: unsafe { &*I2C_BASE_ADDR },
        };
        i2c.regs.MDR.set(0x0); // reset
        let mdr = i2c.regs.MDR.get();

        i2c.set_master_address(master_address);
        i2c.set_slave_address(slave_address);
        // Backward Compatibility mode always disabled
        i2c.regs.EMDR.set(0x0);
        i2c.set_baudrate(baud);
        i2c.regs.MDR.set(mdr | flags::RESET_OUT);
        i2c
    }

    fn set_mode(&self, mode: u32) {
        self.regs.MDR.set(self.regs.MDR.get() | mode);
    }

    fn reset_mode(&self, mode: u32) {
        self.regs.MDR.set(self.regs.MDR.get() & !mode);
    }

    #[inline]
    pub fn free_run(&self) {
        self.set_mode(flags::FREE_RUN)
    }

    #[inline]
    pub fn trasmitter(&self) {
        self.set_mode(flags::TRANSMITTER)
    }

    pub fn receiver(&self) {
        self.reset_mode(flags::TRANSMITTER);
        self.set_mode(flags::RECEIVER)
    }

    // Set operation mode to MASTER
    #[inline]
    pub fn master(&self) {
        self.reset_mode(flags::SLAVE);
        self.set_mode(flags::MASTER)
    }

    // Set operation mode as SLAVE
    #[inline]
    pub fn slave(&self) {
        self.reset_mode(flags::MASTER);
        self.set_mode(flags::SLAVE)
    }

    #[inline]
    pub fn start(&self) {
        self.set_mode(flags::START_COND)
    }

    #[inline]
    pub fn stop(&self) {
        self.set_mode(flags::STOP_COND)
    }

    pub fn set_master_address(&self, master_addr: u8) {
        self.regs.OAR.set(master_addr as u32);
    }

    pub fn set_slave_address(&self, slave_addr: u8) {
        self.regs.OAR.set(slave_addr as u32);
    }

    fn set_baudrate(&self, baud: u32) {
        let prescale = (config::VCLK / MHz!(8)) - 1;

        let d = if prescale >= 2 {
            5
        } else if prescale != 0 {
            6
        } else {
            7
        };

        if baud > 0 {
            let div = 2 * baud * 1_000 * (prescale + 1);
            let ck = (config::VCLK / div) - d;
            self.regs.PSC.set(prescale);
            self.regs.CKH.set(ck);
            self.regs.CKL.set(ck);
        }
    }

    pub fn nack_mode(&self) {
        self.set_mode(flags::NACK)
    }

    pub fn error(&self) -> bool {
        let err = (self.regs.STR.get() & (flags::AL_INT | flags::NACK_INT)) != 0x0;
        self.regs.STR.set(flags::AL_INT | flags::NACK_INT);
        err
    }

    pub fn count(&self, count: u32) {
        self.regs.CNT.set(count);
    }

    pub fn send_byte(&self, b: u8) {
        wait_until_zero!(self.regs.STR.get(), flags::TX_INT);
        self.regs.DXR.set(b);
    }

    pub fn receive_byte(&self) -> u8 {
        wait_until_zero!(self.regs.STR.get(), flags::RX_INT);
        self.regs.DRR.get()
    }

    pub fn receive(&self, buff: &mut [u8]) {
        let int_pending = (self.regs.IMR.get() & flags::RX_INT) != 0;
        if int_pending {
            self.regs.STR.set(flags::AL_INT | flags::NACK_INT);
        // TODO complete
        } else {
            for b in buff {
                *b = self.receive_byte();
            }
        }
    }

    pub fn data_ready(&self) -> bool {
        self.regs.STR.get() & flags::RX_INT != 0x0
    }

    pub fn trasmission_ready(&self) -> bool {
        self.regs.STR.get() & flags::TX_INT != 0x0
    }

    // Clear the stop condition detect (SCD) flag
    pub fn clear_stop_condition(&self) {
        self.regs.STR.set(flags::SCD_INT);
    }

    pub fn busy(&self) -> bool {
        (self.regs.STR.get() & flags::BUSBUSY) == flags::BUSBUSY
    }

    pub fn stop_detected(&self) -> bool {
        self.regs.STR.get() & flags::SCD_INT != 0x0
    }

    pub fn master_ready(&self) -> bool {
        (self.regs.MDR.get() & flags::MASTER) == 0x0
    }

    pub fn disable_interrupt(&mut self) -> &mut I2C {
        self.regs.IMR.set(0x0);
        self
    }

    pub fn disable_dma(&mut self) -> &mut I2C {
        self.regs.DMACR.set(0x0);
        self
    }
}
