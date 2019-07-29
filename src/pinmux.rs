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
    // Pin 1
    GIOB3         = grp_option!(1, 0, 0, 0),
    // Pin 2
    GIOA0         = grp_option!(2, 0, 8, 0),
    // Pin 3
    MIBSPI3NCS_3  = grp_option!(3, 0, 16, 0),
    I2C_SCL       = grp_option!(3, 0, 16, 1),
    H2HET1_29     = grp_option!(3, 0, 16, 2),
    zTZ1          = grp_option!(3, 0, 16, 3),
    // Pin 4
    MIBSPI3NCS_2  = grp_option!(4, 0, 24, 0),
    I2C_SDA       = grp_option!(4, 0, 24, 1),
    H2HET1_27     = grp_option!(4, 0, 24, 2),
    zTZ2          = grp_option!(4, 0, 24, 3),
    // Pin 5
    GIOA1         = grp_option!(5, 1, 0, 0),
    // Pin 6
    H2NET1_11     = grp_option!(6, 1, 8, 0),
    MIBSPI3NCS_4  = grp_option!(6, 1, 8, 1),
    H2NET2_18     = grp_option!(6, 1, 8, 2),
    // Pin 9
    GIOA2         = grp_option!(9, 2, 0, 0),
    H2HET2_0      = grp_option!(9, 2, 0, 3),
    EQEP2I        = grp_option!(9, 2, 0, 4),
    // Pin 14
    GIOA5         = grp_option!(14, 2, 24, 0),
    EXTCLKIN1     = grp_option!(14, 2, 24, 1),
    EPWM1A        = grp_option!(14, 2, 24, 2),
    // Pin 15
    H2HET1_22     = grp_option!(15, 3, 8, 0),
    // Pin 16
    GIOA6         = grp_option!(16, 3, 16, 0),
    N2HET2_4      = grp_option!(16, 3, 16, 1),
    EPWM1B        = grp_option!(16, 3, 16, 2),
    // Pin 22
    GIOA7         = grp_option!(22, 4, 0, 0),
    N2HET2_6      = grp_option!(22, 4, 0, 1),
    EPWM2A        = grp_option!(22, 4, 0, 2),
    // Pin 24
    H2HET1_3      = grp_option!(24, 4, 24, 0),
    SPI4NCS_0     = grp_option!(24, 4, 24, 1),
    // Pin 37
    MIBSPI3NCS_1 = grp_option!(37, 7, 8, 0),
    N2HET1_25    = grp_option!(37, 7, 8, 1),
    // Pin 38
    H2NET1_6     = grp_option!(38, 7, 16, 0),
    SCIRX        = grp_option!(38, 7, 16, 1),
    EPWM5A       = grp_option!(38, 7, 16, 2),
    // Pin 39
    H2NET1_13    = grp_option!(39, 8, 0, 0),
    SCITX        = grp_option!(39, 8, 0, 1),
    EPWM5B       = grp_option!(39, 8, 0, 2),
    // Pin 40
    MIBSPI1NCS_2 = grp_option!(40, 8, 8, 0),
    N2HET1_19    = grp_option!(40, 8, 8, 1),
    // Pin 55
    MIBSPI3NCS_0 = grp_option!(55, 9, 16, 0),
    AD2EVT       = grp_option!(55, 9, 16, 1),
    GIOB2        = grp_option!(55, 9, 16, 2),
    EQEP1I       = grp_option!(55, 9, 16, 3),
    // Pin 91
    N2NET1_28    = grp_option!(91, 11, 24, 0),
    MIBSPI1NCS_5 = grp_option!(91, 11, 24, 1),
    // Pin 92
    N2NET1_26    = grp_option!(92, 12, 0, 0),
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
