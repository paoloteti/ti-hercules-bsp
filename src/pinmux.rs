///! Map PINNMRx register to bit field
///!
///! Reference: TMS570LS31x/21x 16/32-Bit RISC Flash Microcontroller
///!            Chapter 4.6 Signal Multiplexing and Control

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
#[rustfmt::skip]
pub enum PinMux {
    Pin1_GIOB3         = grp_option!(1, 0, 0, 0),
    Pin2_GIOA0         = grp_option!(2, 0, 8, 0),

    Pin3_MIBSPI3NCS_3  = grp_option!(3, 0, 16, 0),
    Pin3_I2C_SCL       = grp_option!(3, 0, 16, 1),
    Pin3_H2HET1_29     = grp_option!(3, 0, 16, 2),
    Pin3_zTZ1          = grp_option!(3, 0, 16, 3),

    Pin4_MIBSPI3NCS_2  = grp_option!(4, 0, 24, 0),
    Pin4_I2C_SDA       = grp_option!(4, 0, 24, 1),
    Pin4_H2HET1_27     = grp_option!(4, 0, 24, 2),
    Pin4_zTZ2          = grp_option!(4, 0, 24, 3),

    Pin5_GIOA1         = grp_option!(5, 1, 0, 0),

    Pin6_H2NET1_11     = grp_option!(6, 1, 8, 0),
    Pin6_MIBSPI3NCS_4  = grp_option!(6, 1, 8, 1),
    Pin6_H2NET2_18     = grp_option!(6, 1, 8, 2),

    Pin9_GIOA2         = grp_option!(9, 2, 0, 0),

    Pin38_H2NET1_6     = grp_option!(38, 7, 16, 0),
    Pin38_SCIRX        = grp_option!(38, 7, 16, 1),
    Pin38_EPWM5A       = grp_option!(38, 7, 16, 2),

    Pin39_H2NET1_13    = grp_option!(39, 8, 0, 0),
    Pin39_SCITX        = grp_option!(39, 8, 0, 1),
    Pin39_EPWM5B       = grp_option!(39, 8, 0, 2),

    Pin40_MIBSPI1NCS_2 = grp_option!(40, 8, 8, 0),
    Pin40_N2HET1_19    = grp_option!(40, 8, 8, 1),

    Pin55_GIOB2        = grp_option!(55, 9, 16, 2),
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
