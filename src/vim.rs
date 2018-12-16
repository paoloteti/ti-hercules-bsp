use crate::esm::Esm;
use crate::esm_ch::EsmError;
use vcell::VolatileCell;

pub const VIM_CHANNELS: usize = 128;
const VIM_CH_GROUPS: usize = 32;

#[repr(C)]
pub struct VimRegisters {
    parflg: VolatileCell<u32>,            // Parity Flag
    parctl: VolatileCell<u32>,            // Parity Control
    adderr: VolatileCell<u32>,            // Address Parity Error
    fbparerr: VolatileCell<u32>,          // Fall-Back Address Parity Error
    irq_index: VolatileCell<u32>,         // 0x00
    fiq_index: VolatileCell<u32>,         // 0x04
    _reserved1: VolatileCell<u32>,        // 0x08
    _reserved2: VolatileCell<u32>,        // 0x0C
    firgpr: [VolatileCell<u32>; 4],       // 0x10-0x1C
    intreq: [VolatileCell<u32>; 4],       // 0x20-0x2C
    req_maskset: [VolatileCell<u32>; 4],  // 0x30-0x3C
    req_maskclr: [VolatileCell<u32>; 4],  // 0x40-0x4C
    wake_maskset: [VolatileCell<u32>; 4], // 0x50-0x5C
    wake_maskclr: [VolatileCell<u32>; 4], // 0x60-0x6C
    irq_vecreg: VolatileCell<u32>,        // 0x70
    fiq_vecreg: VolatileCell<u32>,        // 0x74
    capevt: VolatileCell<u32>,            // 0x78
    _reserved3: VolatileCell<u32>,        // 0x7C
    chan_ctrl: [VolatileCell<u32>; 32],   // 0x80-0x0FC
}
const VIM_BASE_ADDR: *const VimRegisters = 0xFFFF_FDEC as *const VimRegisters;

pub struct VimTable {
    isr: [VolatileCell<u32>; VIM_CHANNELS],
}
/// VIM RAM base address
const VIM_RAM_BASE_ADDR: *const VimTable = 0xFFF8_2000 as *const VimTable;

pub struct VimParityRam {
    parity: [VolatileCell<u32>; VIM_CHANNELS],
}
/// VIM Parity RAM base address
const VIM_PRAM_BASE_ADDR: *const VimParityRam = 0xFFF8_2400 as *const VimParityRam;

const TEST_ENABLE: u32 = 0x10;

fn vim_dummy_isr() {}

// .vim.table shall be at 0xFFF8_2000
#[used]
#[link_section = ".vim_table"]
static INTERRUPTS: [fn(); VIM_CHANNELS] = [vim_dummy_isr; VIM_CHANNELS];

#[derive(Copy, Clone)]
pub enum VimType {
    SysInterrupt,
    FirInterrupt,
}

pub struct Vim {
    regs: &'static VimRegisters,
    table: &'static VimTable,
    pram: &'static VimParityRam,
}

#[allow(dead_code)]
impl Vim {

    pub unsafe fn new() -> Vim {
        Vim {
            regs: &*VIM_BASE_ADDR,
            table: &*VIM_RAM_BASE_ADDR,
            pram: &*VIM_PRAM_BASE_ADDR,
        }
    }

    pub fn fiq_id(&self) -> usize {
        self.regs.fiq_index.get() as usize
    }

    pub fn irq_id(&self) -> usize {
        self.regs.irq_index.get() as usize
    }

    pub fn isr_set(&self, ch: usize, isr: fn()) {
        if ch > 2 && ch < VIM_CHANNELS {
            self.table.isr[ch].set(isr as u32);
        }
    }

    pub fn parity_fallback_handler(&self, fb: fn()) {
        self.regs.fbparerr.set(fb as u32)
    }

    pub fn parity_enable(&self, enable: bool) {
        if enable {
            self.regs.parflg.set(0xA)
        } else {
            self.regs.parflg.set(0x5)
        }
    }

    /// clear VIM RAM parity error flag in VIM
    pub fn parity_flag_clear(&self) {
        self.regs.parflg.set(0x1);
    }

    pub fn parity_check(&self) -> bool {
        let mut error = false;
        let parctl = self.regs.parctl.get();
        // Enable parity checking and parity test mode
        self.regs.parctl.set(TEST_ENABLE);

        // Flip a bit for fault injection
        self.pram.parity[0].set(self.pram.parity[0].get() ^ 0x1);

        self.parity_enable(true);

        // cause parity error reading back data
        let _dummy = self.pram.parity[0].get();

        // check if ESM group1 channel 15 is flagged
        let esm = unsafe { Esm::new() };
        if !esm.error_is_set(EsmError::VimParity) {
            error = false;
        } else {
            self.parity_flag_clear();
            esm.clear_error(EsmError::VimParity);
            self.pram.parity[0].set(self.pram.parity[0].get() ^ 0x1);
        }

        self.regs.parctl.set(parctl);
        error
    }

    pub fn set_type(&self, ch: usize, int_type: VimType) {
        if ch < VIM_CHANNELS {
            let grp = ch / VIM_CH_GROUPS;
            let id = 0x1 << (ch % VIM_CH_GROUPS);
            let stat = self.regs.firgpr[grp].get();
            match int_type {
                VimType::SysInterrupt => self.regs.firgpr[grp].set(stat & !id),
                VimType::FirInterrupt => self.regs.firgpr[grp].set(stat | id),
            }
        }
    }

    pub fn interrupt_enable(&self, ch: usize, enable: bool) {
        if ch < VIM_CHANNELS {
            let grp = ch / VIM_CH_GROUPS;
            let id = ch % VIM_CH_GROUPS;
            if enable {
                self.regs.req_maskset[grp].set(0x1 << id);
            } else {
                self.regs.req_maskclr[grp].set(0x1 << id);
            }
        }
    }

    #[inline]
    pub fn clear_esm_interrupt(&self) {
        self.regs.intreq[0].set(0x1)
    }

    pub unsafe fn default_parity_fallback_handler(&self) {
        let error_address = self.regs.adderr.get();
        let _error_ch = ((error_address & 0x1FF) >> 2) as usize;

        // FIXME(pteti) Fix error address using a backup table
        // self.table.isr[error_ch].set(self.BACKUP_ISR_TABLE[error_ch] as u32);
        self.parity_flag_clear();
        let v = if self.fiq_id() != 0 { self.fiq_id() - 1 } else { self.irq_id() - 1 };

        if v == 0 {
            // ESM interrupt (high-priority) We can't disable.
            // So clear it and clear related error.
            self.clear_esm_interrupt();
            let esm = Esm::new();
            let vec = esm.high_level_interrupt();
            let error = EsmError::from(vec as u8);
            esm.clear_error(error);
        } else {
            // FIQ/IRQ: disable and enable again.
            self.interrupt_enable(v, false);
            self.interrupt_enable(v, true);
        }
    }
}
