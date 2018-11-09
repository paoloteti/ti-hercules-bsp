////
use vcell::VolatileCell;

#[repr(C)]
#[allow(non_snake_case)]
struct HetRegisters {
    /// Global control register
    GCR: VolatileCell<u32>,
    /// Prescale factor register
    PFR: VolatileCell<u32>,
    /// Current address register
    ADDR: VolatileCell<u32>,
    /// Interrupt offset register 1
    OFF1: VolatileCell<u32>,
    /// Interrupt offset register 2
    OFF2: VolatileCell<u32>,
    /// Interrupt enable set register
    INTENAS: VolatileCell<u32>,
    /// Interrupt enable clear register
    INTENAC: VolatileCell<u32>,
    /// Exception control register 1
    EXC1: VolatileCell<u32>,
    /// Exception control register 2
    EXC2: VolatileCell<u32>,
    /// Interrupt priority register
    PRY: VolatileCell<u32>,
    /// Interrupt flag register
    FLG: VolatileCell<u32>,
    /// AND share control register
    AND: VolatileCell<u32>,
    /// Reserved
    _reserved1: VolatileCell<u32>,
    /// High resolution share register
    HRSH: VolatileCell<u32>,
    /// XOR share register
    XOR: VolatileCell<u32>,
    /// Request enable set register
    REQENS: VolatileCell<u32>,
    /// Request enable clear register
    REQENC: VolatileCell<u32>,
    /// Request destination select register
    REQDS: VolatileCell<u32>,
    /// Reserved
    _reserved2: VolatileCell<u32>,
    /// Direction register
    DIR: VolatileCell<u32>,
    /// Data input register
    DIN: VolatileCell<u32>,
    /// Data output register
    DOUT: VolatileCell<u32>,
    /// Data output set register
    DSET: VolatileCell<u32>,
    /// Data output clear register
    DCLR: VolatileCell<u32>,
    /// Open drain register
    PDR: VolatileCell<u32>,
    /// Pull disable register
    PULDIS: VolatileCell<u32>,
    /// Pull select register
    PSL: VolatileCell<u32>,
    /// Reserved
    _reserved3: VolatileCell<u32>,
    /// Reserved
    _reserved4: VolatileCell<u32>,
    /// Parity control register
    PCR: VolatileCell<u32>,
    /// Parity address register
    PAR: VolatileCell<u32>,
    /// Parity pin select register
    PPR: VolatileCell<u32>,
    /// Suppression filter preload register
    SFPRLD: VolatileCell<u32>,
    /// Suppression filter enable register
    SFENA: VolatileCell<u32>,
    /// Reserved
    _reserved5: VolatileCell<u32>,
    /// Loop back pair select register
    LBPSEL: VolatileCell<u32>,
    /// Loop back pair direction register
    LBPDIR: VolatileCell<u32>,
    /// Pin disable register
    PINDIS: VolatileCell<u32>,
}

#[allow(dead_code)]
struct HetInstruction {
    program: VolatileCell<u32>,
    control: VolatileCell<u32>,
    data: VolatileCell<u32>,
    _reserved: VolatileCell<u32>,
}

#[repr(C)]
struct HetRam {
    instruction: [HetInstruction; 160]
}

const HET1_REG_ADDR: *const HetRegisters = 0xFFF7_B800 as *const HetRegisters;
const HET2_REG_ADDR: *const HetRegisters = 0xFFF7_B900 as *const HetRegisters;
const HET_REG_ADDR: [*const HetRegisters; 2] = [HET1_REG_ADDR, HET2_REG_ADDR];

const HET1_RAM_ADDR: *const HetRam = 0xFF46_0000 as *const HetRam;
const HET2_RAM_ADDR: *const HetRam = 0xFF44_0000 as *const HetRam;
const HET_RAM_ADDR: [*const HetRam; 2] = [HET1_RAM_ADDR, HET2_RAM_ADDR];


#[derive(Copy, Clone)]
pub enum HetID {
    One = 0,
    Two = 1,
}

#[allow(dead_code)]
pub struct Het {
    pub id: HetID,
    regs: &'static HetRegisters,
    ram: &'static HetRam,
}

impl Het {
    pub fn new(id: HetID) -> Het {
        Het {
            id: id,
            regs: unsafe { &*HET_REG_ADDR[id as usize] },
            ram: unsafe { &*HET_RAM_ADDR[id as usize] },
        }
    }

    pub fn reset(&self) {
        self.regs.GCR.set(self.regs.GCR.get() | 0x0002_0000);
    }
}
