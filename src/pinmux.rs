/// Map PINNMRx register to bit field
///
/// Reference: TMS570LS31x/21x 16/32-Bit RISC Flash Microcontroller
///            Chapter 4.6 Signal Multiplexing and Control
///

macro_rules! grp_option {
    ($pin: expr, $group: expr, $shift: expr, $option: expr) => {
        (($pin as isize) << 24)
            | (($group as isize) << 16)
            | (($shift as isize) << 8)
            | ($option as isize)
    };
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum PinMux {
    Pin38_SCIRX = grp_option!(38, 7, 16, 1),
    Pin39_SCITX = grp_option!(39, 8, 0, 1),

    Pin40_MIBSPI1NCS_2 = grp_option!(40, 8, 8, 0),
    Pin40_N2HET1_19 = grp_option!(40, 8, 8, 1),

    Pin55_GIOB2 = grp_option!(55, 9, 16, 2),
}

impl PinMux {
    pub fn pin(self) -> usize {
        (self as usize) >> 24
    }

    pub fn option(self) -> usize {
        (self as usize) & 0xff
    }

    pub fn shift(self) -> usize {
        ((self as usize) >> 8) & 0xff
    }

    pub fn group(self) -> usize {
        ((self as usize) >> 16) & 0xff
    }
}
