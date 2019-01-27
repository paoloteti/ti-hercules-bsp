///! The general-purpose input/output (GIO) module provides
///! the TMS570 familyof devices with input/output (I/O) capability.
///! The I/O pins are bidirectional and bit-programmable.
///! The GIO module also supports external interrupt capability.
use vcell::VolatileCell;

#[repr(C)]
struct GioRegisters {
    /// Global Control
    gcr0: VolatileCell<u32>,
    /// Reserved
    _reserved: VolatileCell<u32>,
    /// Interrupt Detect
    intdet: VolatileCell<u32>,
    /// Interrupt Polarity
    pol: VolatileCell<u32>,
    /// Interrupt Enable Set
    enaset: VolatileCell<u32>,
    /// Interrupt Enable Clear
    enaclr: VolatileCell<u32>,
    /// Interrupt Priority Set
    lvlset: VolatileCell<u32>,
    /// Interrupt Priority Clear
    lvlclr: VolatileCell<u32>,
    /// Interrupt Flag
    flg: VolatileCell<u32>,
    /// Interrupt Offset A
    off1: VolatileCell<u32>,
    /// Interrupt Offset B
    off2: VolatileCell<u32>,
    /// Emulation 1
    emu1: VolatileCell<u32>,
    /// Emulation 2
    emu2: VolatileCell<u32>,
}
const GIO_BASE_ADDR: *const GioRegisters = 0xFFF7_BC00 as *const GioRegisters;

#[repr(C)]
struct GioPortsRegisters {
    /// Data Direction
    dir: VolatileCell<u32>,
    /// Data Input
    din: VolatileCell<u32>,
    /// Data Output
    dout: VolatileCell<u32>,
    /// Data Output Set
    dset: VolatileCell<u32>,
    /// Data Output Clear
    dclr: VolatileCell<u32>,
    /// Open Drain
    pdr: VolatileCell<u32>,
    /// Pullup Disable
    puldis: VolatileCell<u32>,
    /// Pull Up/Down Selection
    psl: VolatileCell<u32>,
}
const GIO_PORTA_ADDR: *const GioPortsRegisters = 0xFFF7_BC34 as *const GioPortsRegisters;
const GIO_PORTB_ADDR: *const GioPortsRegisters = 0xFFF7_BC54 as *const GioPortsRegisters;
const MIBSPI_PORT1_ADDR: *const GioPortsRegisters = 0xFFF7_F418 as *const GioPortsRegisters;
const MIBSPI_PORT3_ADDR: *const GioPortsRegisters = 0xFFF7_F818 as *const GioPortsRegisters;
const MIBSPI_PORT5_ADDR: *const GioPortsRegisters = 0xFFF7_FC18 as *const GioPortsRegisters;
const LIN_PORT_ADDR: *const GioPortsRegisters = 0xFFF7_E440 as *const GioPortsRegisters;
const SCI_PORT_ADDR: *const GioPortsRegisters = 0xFFF7_E540 as *const GioPortsRegisters;
const HET_PORT1_ADDR: *const GioPortsRegisters = 0xFFF7_B84C as *const GioPortsRegisters;
const HET_PORT2_ADDR: *const GioPortsRegisters = 0xFFF7_B94C as *const GioPortsRegisters;
const I2C_PORT_ADDR: *const GioPortsRegisters = 0xFFF7_D44C as *const GioPortsRegisters;

pub struct Gio {
    regs: &'static GioRegisters,
    ports: [&'static GioPortsRegisters; 10],
}

#[derive(Clone, Copy, PartialEq)]
pub enum GioDirection {
    Output = 0x0,
    Input = 0x1,
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum GioPorts {
    /// Gio Port A
    A = 0,
    /// Gio Port B
    B = 1,
    /// Gio Port 1 from MIBSPI
    MibSpiPort1 = 2,
    /// Gio Port 3 from MIBSPI
    MibSpiPort3 = 3,
    /// Gio Port 5 from MIBSPI
    MibSpiPort5 = 4,
    /// Gio Port from LIN
    LinPort = 5,
    /// Gio Port from SCI
    SciPort = 6,
    /// Gio Port 1 from HET
    HetPort1 = 7,
    /// Gio Port 2 from HET
    HetPort2 = 8,
    /// Gio Port 1 from I2C
    I2cPort1 = 9,
}

#[derive(Clone, Copy)]
pub enum Edge {
    /// Falling edge (high to low)
    Falling,
    /// Rising edge (low to high)
    Rising,
}

#[derive(Clone, Copy)]
pub enum Pull {
    /// Pull DOWN
    Down = 0x0,
    /// Pull UP
    Up = 0x1,
}

impl Gio {
    pub fn new() -> Gio {
        let gio = Gio {
            regs: unsafe { &*GIO_BASE_ADDR },
            ports: unsafe {
                [
                    &*GIO_PORTA_ADDR,
                    &*GIO_PORTB_ADDR,
                    &*MIBSPI_PORT1_ADDR,
                    &*MIBSPI_PORT3_ADDR,
                    &*MIBSPI_PORT5_ADDR,
                    &*LIN_PORT_ADDR,
                    &*SCI_PORT_ADDR,
                    &*HET_PORT1_ADDR,
                    &*HET_PORT2_ADDR,
                    &*I2C_PORT_ADDR,
                ]
            },
        };
        // Reset it (if not already out of reset)
        if gio.regs.gcr0.get() == 0x0 {
            gio.reset();
        }
        gio
    }

    /// Force GIO module reset.
    /// Struct constructor put GIO module out of reset, so usually
    /// there is no need to explicit reset the controller.
    pub fn reset(&self) {
        self.regs.gcr0.set(0x1);
        self.regs.enaclr.set(0xFF);
        self.regs.lvlclr.set(0xFF);
    }

    /// Configure GIO direction.
    pub fn direction(&self, port: GioPorts, n: usize, dir: GioDirection) {
        let d = self.ports[port as usize].dir.get();
        match dir {
            GioDirection::Input => self.ports[port as usize].dir.set(d & !(0x1 << n)),
            GioDirection::Output => self.ports[port as usize].dir.set(d | 0x1 << n),
        }
    }

    pub fn set(&self, port: GioPorts, n: usize, on: bool) {
        if on {
            self.ports[port as usize].dset.set(0x1 << n);
        } else {
            self.ports[port as usize].dclr.set(0x1 << n);
        }
    }

    pub fn toogle(&self, port: GioPorts, n: usize) {
        self.set(port, n, !self.get(port, n))
    }

    pub fn set_all(&self, port: GioPorts, v: u32) {
        self.ports[port as usize].dout.set(v)
    }

    pub fn get(&self, port: GioPorts, n: usize) -> bool {
        (self.ports[port as usize].din.get() >> n) & 0x1 != 0x1
    }

    pub fn get_all(&self, port: GioPorts) -> u32 {
        self.ports[port as usize].din.get()
    }

    /// Enable/Disable pull up/down functionality.
    /// Function has effect only when GIO pin is an input pin
    pub fn pull_enable(&self, port: GioPorts, n: usize, enable: bool) {
        self.ports[port as usize].puldis.set((enable as u32) << n);
    }

    /// Configure pin a in pull up or pull down functionality.
    /// No need to explicity invoke pull_enable(true).
    pub fn pull(&self, port: GioPorts, n: usize, p: Pull) {
        self.ports[port as usize].psl.set((p as u32) << n);
        self.pull_enable(port, n, true);
    }

    pub fn open_drain(&self, port: GioPorts, n: usize, od: bool) {
        let pdr = self.ports[port as usize].pdr.get();
        self.ports[port as usize].pdr.set(pdr | (od as u32) << n);
    }

    /// Controls the polarity — rising edge (low to high)
    /// or falling edge (high to low)
    /// To ensure recognition of the signal as an edge, the signal must
    /// maintain the new level for at least one VCLK cycle
    pub fn edge(&self, port: GioPorts, n: usize, e: Edge) {
        let mask = ((port as u32) * 8) + (n as u32);
        match e {
            Edge::Falling => self.regs.pol.set(self.regs.pol.get() & !mask),
            Edge::Rising => self.regs.pol.set(self.regs.pol.get() | mask),
        }
    }

    pub fn interrupt(&self, port: GioPorts, pin: usize, enable: bool) {
        let mask = (enable as u32) << (((port as u32) * 8) + (pin as u32));
        if enable {
            self.regs.enaset.set(mask);
        } else {
            self.regs.flg.set(mask); // clear pending, if any
            self.regs.enaclr.set(mask);
        }
    }
}
