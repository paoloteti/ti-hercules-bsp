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

/// Macro to wait until not equal
#[macro_export]
macro_rules! wait_until_neq {
    ($reg: expr, $v: expr ) => {
        while $reg != $v { /* wait */ }
    };
}

/// Macro to wait until not equal
#[macro_export]
macro_rules! wait_until_eq {
    ($reg: expr, $v: expr ) => {
        while $reg == $v { /* wait */ }
    };
}

/// Divide positive by positive divisor and round to closest
/// unsigned integer.
#[macro_export]
macro_rules! udiv_round_closest {
    ($x: expr, $divisor: expr ) => {
        ($x + ($divisor / 2)) / $divisor
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
