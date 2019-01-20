use crate::dma_ctrl::DmaControlPacket;
///
/// Direct Memory Access Controller (DMA) Module
///
/// Available features as listed in
///     "Chapter 20 - TMS570LS Series Technical Reference Manual (TRM), code SPNU489B)"
/// are:
/// - One master port - PortB (64 bits wide) that interfaces microcontrollers
///   Memory System.
/// - FIFO buffer(4 entries deep and each 64bit wide)
/// - Channel control information is stored in RAM protected by parity
/// - 16 channels with individual enable
/// - Channel chaining capability
/// - 32 peripheral DMA requests
/// - Hardware and Software DMA requests
/// - 8, 16, 32 or 64-bit transactions supported
/// - Multiple addressing modes for source/destination (fixed, increment, offset)
/// - Auto-initiation
/// - Power-management mode
/// - Memory Protection for the adress range DMA can access with four configurable
///   memory regions
use vcell::VolatileCell;

/// Number of supported DMA Regions
const DMA_REGION_NUM: usize = 4;

#[repr(C)]
struct DmaMpr {
    /// Protection Region Start Address
    start_add: VolatileCell<u32>,
    /// Protection Region End Address
    end_add: VolatileCell<u32>,
}

#[repr(C)]
#[allow(non_snake_case)]
struct DmaRegisters {
    /// Global Control
    GCTRL: VolatileCell<u32>,
    /// Channel Pending
    PEND: VolatileCell<u32>,
    /// Fall Back
    FBREG: VolatileCell<u32>,
    /// Status
    DMASTAT: VolatileCell<u32>,
    /// Reserved
    _reserved1: VolatileCell<u32>,
    /// HW Channel Enable Set
    HWCHENAS: VolatileCell<u32>,
    /// Reserved
    _reserved2: VolatileCell<u32>,
    /// HW Channel Enable Reset
    HWCHENAR: VolatileCell<u32>,
    /// Reserved
    _reserved3: VolatileCell<u32>,
    /// SW Channel Enable Set
    SWCHENAS: VolatileCell<u32>,
    /// Reserved
    _reserved4: VolatileCell<u32>,
    /// SW Channel Enable Reset
    SWCHENAR: VolatileCell<u32>,
    /// Reserved
    _reserved5: VolatileCell<u32>,
    /// Channel Priority Set
    CHPRIOS: VolatileCell<u32>,
    /// Reserved
    _reserved6: VolatileCell<u32>,
    /// Channel Priority Reset
    CHPRIOR: VolatileCell<u32>,
    /// Reserved
    _reserved7: VolatileCell<u32>,
    /// Global Channel Interrupt Enable Set
    GCHIENAS: VolatileCell<u32>,
    /// Reserved
    _reserved8: VolatileCell<u32>,
    /// Global Channel Interrupt Enable Reset
    GCHIENAR: VolatileCell<u32>,
    /// Reserved
    _reserved9: VolatileCell<u32>,
    /// DMA Request Assignment
    DREQASI: [VolatileCell<u32>; 8],
    /// Reserved
    _reserved10: [VolatileCell<u32>; 8],
    /// Port Assignment
    PAR: [VolatileCell<u32>; 4],
    /// Reserved
    _reserved11: [VolatileCell<u32>; 4],
    /// FTC Interrupt Mapping
    FTCMAP: VolatileCell<u32>,
    /// Reserved
    _reserved12: VolatileCell<u32>,
    /// LFS Interrupt Mapping
    LFSMAP: VolatileCell<u32>,
    /// Reserved
    _reserved13: VolatileCell<u32>,
    /// HBC Interrupt Mapping
    HBCMAP: VolatileCell<u32>,
    /// Reserved
    _reserved14: VolatileCell<u32>,
    /// BTC Interrupt Mapping
    BTCMAP: VolatileCell<u32>,
    /// Reserved
    _reserved15: VolatileCell<u32>,
    /// BER Interrupt Mapping
    BERMAP: VolatileCell<u32>,
    /// Reserved
    _reserved16: VolatileCell<u32>,
    /// FTC Interrupt Enable Set
    FTCINTENAS: VolatileCell<u32>,
    /// Reserved
    _reserved17: VolatileCell<u32>,
    /// FTC Interrupt Enable Reset
    FTCINTENAR: VolatileCell<u32>,
    /// Reserved
    _reserved18: VolatileCell<u32>,
    /// LFS Interrupt Enable Set
    LFSINTENAS: VolatileCell<u32>,
    /// Reserved
    _reserved19: VolatileCell<u32>,
    /// LFS Interrupt Enable Reset
    LFSINTENAR: VolatileCell<u32>,
    /// Reserved
    _reserved20: VolatileCell<u32>,
    /// HBC Interrupt Enable Set
    HBCINTENAS: VolatileCell<u32>,
    /// Reserved
    _reserved21: VolatileCell<u32>,
    /// HBC Interrupt Enable Reset
    HBCINTENAR: VolatileCell<u32>,
    /// Reserved
    _reserved22: VolatileCell<u32>,
    /// BTC Interrupt Enable Set
    BTCINTENAS: VolatileCell<u32>,
    /// Reserved
    _reserved23: VolatileCell<u32>,
    /// BTC Interrupt Enable Reset
    BTCINTENAR: VolatileCell<u32>,
    /// Reserved
    _reserved24: VolatileCell<u32>,
    /// Global Interrupt Flag
    GINTFLAG: VolatileCell<u32>,
    /// Reserved
    _reserved25: VolatileCell<u32>,
    /// FTC Interrupt Flag
    FTCFLAG: VolatileCell<u32>,
    /// Reserved
    _reserved26: VolatileCell<u32>,
    /// LFS Interrupt Flag
    LFSFLAG: VolatileCell<u32>,
    /// Reserved
    _reserved27: VolatileCell<u32>,
    /// HBC Interrupt Flag
    HBCFLAG: VolatileCell<u32>,
    /// Reserved
    _reserved28: VolatileCell<u32>,
    /// BTC Interrupt Flag
    BTCFLAG: VolatileCell<u32>,
    /// Reserved
    _reserved29: VolatileCell<u32>,
    /// BER Interrupt Flag
    BERFLAG: VolatileCell<u32>,
    /// Reserved
    _reserved30: VolatileCell<u32>,
    /// FTCA Interrupt Channel Offset
    FTCAOFFSET: VolatileCell<u32>,
    /// LFSA Interrupt Channel Offset
    LFSAOFFSET: VolatileCell<u32>,
    /// HBCA Interrupt Channel Offset
    HBCAOFFSET: VolatileCell<u32>,
    /// BTCA Interrupt Channel Offset
    BTCAOFFSET: VolatileCell<u32>,
    /// BERA Interrupt Channel Offset
    BERAOFFSET: VolatileCell<u32>,
    /// FTCB Interrupt Channel Offset
    FTCBOFFSET: VolatileCell<u32>,
    /// LFSB Interrupt Channel Offset
    LFSBOFFSET: VolatileCell<u32>,
    //  HBCB Interrupt Channel Offset
    HBCBOFFSET: VolatileCell<u32>,
    // 0x016C: BTCB Interrupt Channel Offset
    BTCBOFFSET: VolatileCell<u32>,
    // 0x0170: BERB Interrupt Channel Offset
    BERBOFFSET: VolatileCell<u32>,
    /// Reserved
    _reserved31: VolatileCell<u32>,
    /// Port Control
    PTCRL: VolatileCell<u32>,
    /// RAM Test Control
    RTCTRL: VolatileCell<u32>,
    ///  Debug Control
    DCTRL: VolatileCell<u32>,
    /// Watch Point
    WPR: VolatileCell<u32>,
    /// Watch Mask
    WMR: VolatileCell<u32>,
    PAACSADDR: VolatileCell<u32>,
    PAACDADDR: VolatileCell<u32>,
    PAACTC: VolatileCell<u32>,
    /// Port B Active Channel Source Address
    PBACSADDR: VolatileCell<u32>,
    /// Port B Active Channel Destination Address
    PBACDADDR: VolatileCell<u32>,
    /// Port B Active Channel Transfer Count
    PBACTC: VolatileCell<u32>,
    /// Reserved
    _reserved32: VolatileCell<u32>,
    /// Parity Control
    DMAPCR: VolatileCell<u32>,
    /// DMA Parity Error Address
    DMAPAR: VolatileCell<u32>,
    /// DMA Memory Protection Control
    DMAMPCTRL: VolatileCell<u32>,
    /// DMA Memory Protection Status
    DMAMPST: VolatileCell<u32>,
    /// DMA Memory Protection Address range
    DMAMPR: [DmaMpr; DMA_REGION_NUM],
}
const DMA_BASE_ADDR: *const DmaRegisters = 0xFFFF_F000 as *const DmaRegisters;

#[repr(C)]
#[allow(non_snake_case)]
struct Pcp {
    ISADDR: VolatileCell<u32>,
    IDADDR: VolatileCell<u32>,
    ITCOUNT: VolatileCell<u32>,
    _reserved1: VolatileCell<u32>,
    CHCTRL: VolatileCell<u32>,
    EIOFF: VolatileCell<u32>,
    FIOFF: VolatileCell<u32>,
    _reserved2: VolatileCell<u32>,
}

#[repr(C)]
#[allow(non_snake_case)]
struct Wcp {
    CSADDR: VolatileCell<u32>,
    CDADDR: VolatileCell<u32>,
    CTCOUNT: VolatileCell<u32>,
    _reserved: VolatileCell<u32>,
}

const DMA_CH_MAX: usize = 32;
#[repr(C)]
struct DmaRam {
    pcp: [Pcp; DMA_CH_MAX],
    _reserved: [VolatileCell<u32>; 256],
    _wcp: [Wcp; DMA_CH_MAX],
}
const DMA_RAM_BASE_ADDR: *const DmaRam = 0xFFF8_0000 as *const DmaRam;

// Global Control Register (GCTRL) bits masks
const DMA_BUSY: u32 = 0x1 << 14;
const DMA_EN: u32 = 0x1 << 16;
const DMA_RESET: u32 = 0x0;

#[derive(Clone, Copy)]
pub enum DmaDebug {
    IgnoreSuspend = 0x0 << 8,
    FinishBlock = 0x1 << 8,
    FinishFrame = 0x2 << 8,
    ImmediateStop = 0x3 << 8,
}

#[derive(Clone, Copy)]
pub enum DmaPriority {
    /// Priority queue Low
    Low = 0,
    /// Priority queue High
    High = 1,
}

#[derive(Clone, Copy)]
pub enum DmaPermission {
    /// Read and Write permissions
    Full = 0,
    /// Read only permission
    Read = 1,
    /// Write only permission
    Write = 2,
    /// All permissions are denied
    Denied = 3,
}

#[derive(Clone, Copy)]
pub enum DmaInterrupt {
    /// Frame transfer complete
    FTC = 0,
    /// Last frame transfer started
    LFS = 1,
    /// First half of block complete
    HBC = 2,
    /// Block transfer complete
    BTC = 3,
}

#[derive(Clone, Copy)]
pub enum DmaRegion {
    Region0 = 0,
    Region1 = 1,
    Region2 = 2,
    Region3 = 3,
}

pub struct Dma {
    regs: &'static DmaRegisters,
    ram: &'static DmaRam,
}

impl Dma {
    pub fn new() -> Dma {
        Dma {
            regs: unsafe { &*DMA_BASE_ADDR },
            ram: unsafe { &*DMA_RAM_BASE_ADDR },
        }
    }

    pub fn debug(&self, mode: DmaDebug) {
        self.regs.GCTRL.set(self.regs.GCTRL.get() | (mode as u32))
    }

    pub fn enable(&self, on: bool) {
        if on {
            self.regs.GCTRL.set(DMA_EN | DMA_RESET);
            self.regs
                .GCTRL
                .set(self.regs.GCTRL.get() | DmaDebug::ImmediateStop as u32);
        } else {
            // be sure to complete tranfer
            wait_until_set!(self.regs.GCTRL.get(), DMA_BUSY);
            self.regs.GCTRL.set(0x0);
        }
    }

    pub fn assign(&self, ch: u32, line: u32) {
        let reg_id = ch >> 2;
        let offset = (0x3 - (ch - (reg_id << 2))) << 3;
        // link 'channel' to 'line'
        let dreqasi_reg = &self.regs.DREQASI[reg_id as usize];
        dreqasi_reg.set(dreqasi_reg.get() & !(0xff << offset));
        dreqasi_reg.set(dreqasi_reg.get() | line);
    }

    /// Assigns Low/High priority to a given channel
    pub fn set_priority(&self, ch: u32, prio: DmaPriority) {
        match prio {
            DmaPriority::Low => self.regs.CHPRIOR.set(0x1 << ch),
            DmaPriority::High => self.regs.CHPRIOS.set(0x1 << ch),
        }
    }

    // Enable/Disable the selected interrupt for the selected channel
    pub fn interrupt(&self, ch: u32, int_type: DmaInterrupt, enable: bool) {
        if enable {
            self.regs.GCHIENAS.set(0x1 << ch);
            match int_type {
                DmaInterrupt::FTC => self.regs.FTCINTENAS.set(0x1 << ch),
                DmaInterrupt::LFS => self.regs.LFSINTENAS.set(0x1 << ch),
                DmaInterrupt::HBC => self.regs.HBCINTENAS.set(0x1 << ch),
                DmaInterrupt::BTC => self.regs.BTCINTENAS.set(0x1 << ch),
            }
        } else {
            self.regs.GCHIENAR.set(0x1 << ch);
            match int_type {
                DmaInterrupt::FTC => self.regs.FTCINTENAR.set(0x1 << ch),
                DmaInterrupt::LFS => self.regs.LFSINTENAR.set(0x1 << ch),
                DmaInterrupt::HBC => self.regs.HBCINTENAR.set(0x1 << ch),
                DmaInterrupt::BTC => self.regs.BTCINTENAR.set(0x1 << ch),
            }
        }
    }

    pub fn parity_enable(&self, enable: bool) {
        if enable {
            self.regs.DMAPCR.set(0xA)
        } else {
            self.regs.DMAPCR.set(0x5)
        }
    }

    /// Setup start and end address of a given DMA region
    pub fn region_range(&self, region: DmaRegion, start: u32, end: u32) {
        self.regs.DMAMPR[region as usize].start_add.set(start);
        self.regs.DMAMPR[region as usize].end_add.set(end)
    }

    /// Setup region permisson and interrupt
    pub fn region_enable(&self, region: DmaRegion, perm: DmaPermission, int_enable: bool) {
        let reg_bit = (region as u32) * 8;
        self.regs
            .DMAMPCTRL
            .set(self.regs.DMAMPCTRL.get() & !(0xFF << reg_bit));
        self.regs
            .DMAMPCTRL
            .set(self.regs.DMAMPCTRL.get() | (0x1 << reg_bit));
        // set permission
        self.regs
            .DMAMPCTRL
            .set(self.regs.DMAMPCTRL.get() | ((perm as u32) << (reg_bit + 1)));
        if int_enable {
            self.regs
                .DMAMPCTRL
                .set(self.regs.DMAMPCTRL.get() | (0x1 << (reg_bit + 3)));
        }
    }

    /// Disable a given region
    pub fn region_disable(&self, region: DmaRegion) {
        let mask = !(0x1 << ((region as u32) * 8));
        self.regs.DMAMPCTRL.set(self.regs.DMAMPCTRL.get() & mask)
    }

    /// Enables the DMA channel for hardware or software triggering
    pub fn channel_enable(&self, ch: u32, hw_trigger: bool) {
        if hw_trigger {
            self.regs.HWCHENAS.set(0x1 << ch)
        } else {
            self.regs.SWCHENAS.set(0x1 << ch)
        }
    }

    pub fn control_packet(&self, ch: usize, ctrl_pkt: DmaControlPacket) {
        self.ram.pcp[ch].ISADDR.set(ctrl_pkt.source);
        self.ram.pcp[ch].IDADDR.set(ctrl_pkt.destination);
        self.ram.pcp[ch]
            .ITCOUNT
            .set((ctrl_pkt.frame_count << 16) | ctrl_pkt.frame_count);

        let chctrl = ((ctrl_pkt.read_size as u32) << 14)
            | ((ctrl_pkt.write_size as u32) << 12)
            | ((ctrl_pkt.trigger_type as u32) << 8)
            | ((ctrl_pkt.addressing_mode_src as u32) << 3)
            | ((ctrl_pkt.addressing_mode_dst as u32) << 2)
            | (ctrl_pkt.auto_init as u32);

        self.ram.pcp[ch].CHCTRL.set((chctrl << 16) | chctrl);
        self.ram.pcp[ch].EIOFF.set(
            ((ctrl_pkt.element_dest_offset as u32) << 16) | (ctrl_pkt.element_source_offset as u32),
        );

        self.ram.pcp[ch].EIOFF.set(
            ((ctrl_pkt.frame_dest_offset as u32) << 16) | (ctrl_pkt.frame_source_offset as u32),
        );

        let reg_id = ch >> 3;
        let bit = (0x7 - (ch - (reg_id << 3))) << 2;

        self.regs.PAR[reg_id].set(self.regs.PAR[reg_id].get() & !(0xff << bit));
        self.regs.PAR[reg_id].set(self.regs.PAR[reg_id].get() | ((ctrl_pkt.port as u32) << bit));
    }
}
