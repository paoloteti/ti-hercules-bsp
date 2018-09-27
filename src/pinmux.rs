
/// Map PINNMRx register to bit field
///
/// Reference: TMS570LS31x/21x 16/32-Bit RISC Flash Microcontroller
///            Chapter 4.6 Signal Multiplexing and Control
///

macro_rules! grp_bit {
    ($pin: expr, $group: expr, $bit: expr) => {
        (($pin as isize) << 24) | (($group as isize) << 16) | ($bit as isize)
    }
}


#[derive(Copy,Clone)]
#[allow(non_camel_case_types)]
pub enum PinMux {
    // PIN 1
    Pin1_GIOB3 	        = grp_bit!(1, 0, 0),
    // PIN 2
    Pin2_GIOA0 	        = grp_bit!(2, 0, 8),
    // PIN 3
    Pin3_MIBSPI3NCS3 	= grp_bit!(3, 0, 16),
    Pin3_I2C_SCL 	  	= grp_bit!(3, 0, 17),
    Pin3_N2HET1_29   	= grp_bit!(3, 0, 18),
    Pin3_NTZ1 	  	    = grp_bit!(3, 0, 19),
    // PIN 4
    Pin4_MIBSPI3NCS_2 	= grp_bit!(4, 0, 24),
    Pin4_I2C_SDA 		= grp_bit!(4, 0, 25),
    Pin4_HET1_27 		= grp_bit!(4, 0, 26),
    Pin4_NTZ2 	        = grp_bit!(4, 0, 27),
    // PIN 5
    Pin5_GIOA1         = grp_bit!(5, 1, 0),
    // PIN 6
    Pin6_N2HET1_11     = grp_bit!(6, 1, 8),
    Pin6_MIBSPI3NCS4   = grp_bit!(6, 1, 9),
    Pin6_N2HET2_18     = grp_bit!(6, 1, 10),
    Pin6_EPWM1SYNCO    = grp_bit!(6, 1, 13),
    // PIN 9
    Pin9_GIOA2         = grp_bit!(9, 2, 8),
    Pin9_N2HET2_0      = grp_bit!(9, 2, 3),
    Pin9_EQEP21        = grp_bit!(9, 2, 4),

    Pin38_SCIRX        = grp_bit!(38, 7, 17),
    Pin39_SCITX        = grp_bit!(39, 8, 1),
}

impl PinMux {
    pub fn pin(self) -> usize {
        (self as usize) >> 24
    }

    pub fn bit(self) -> usize {
        (self as usize) & 0xff
    }

    pub fn group(self) -> usize {
        ((self as usize) >> 16) & 0xff
    }
}