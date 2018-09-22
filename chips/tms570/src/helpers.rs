/// Macro to wait until a bit-mask is set
#[macro_export]
macro_rules! wait_until_set {
    ($reg: expr, $mask: expr ) => {
        while $reg & $mask != $mask { /* wait */ }
    };
}

/// Macro to wait until a bit-mask reset
#[macro_export]
macro_rules! wait_until_reset {
    ($reg: expr, $mask: expr ) => {
        while $reg & $mask == $mask { /* wait */ }
    };
}

/// Macro to wait until a bit-mask is 0
#[macro_export]
macro_rules! wait_until_zero {
    ($reg: expr, $mask: expr ) => {
        while $reg & $mask == 0x0 { /* wait */ }
    };
}

#[macro_export]
macro_rules! wait_until_false {
    ($condition: expr ) => {
        while !$condition { /* wait */ }
    };
}

#[macro_export]
macro_rules! wait_until_true {
    ($condition: expr ) => {
        while $condition { /* wait */ }
    };
}

/// Divide positive or negative dividend by positive or negative divisor
/// and round to closest integer. Result is undefined for negative
/// divisors if the dividend variable type is unsigned and for negative
/// dividends if the divisor variable type is unsigned.
#[macro_export]
macro_rules! div_round_closest {
    ($x: expr, $divisor: expr ) => {
        if $x > 1 || $divisor > 1 || $x > 0 {
            ($x + ($divisor / 2)) / $divisor
        } else {
            ($x - ($divisor / 2)) / $divisor
        }
    };
}

#[macro_export]
macro_rules! us2hz {
    ($us: expr) => {
        $us * 1000
    };
}

#[macro_export]
macro_rules! MHz {
    ($m: expr) => {
        $m * 1_000_000
    };
}

#[macro_export]
macro_rules! wait_cycle {
    ($n: expr) => {
        for _ in 0..$n {
            use cortexr4;
            cortexr4::asm::nop();
        }
    };
}
