
#[derive(Copy,Clone,PartialEq)]
pub enum WdViolation {
    NoTimeViolation          = 0x00,
    KeySeqViolation          = 0x04,
    StartTimeWindowViolation = 0x08,
    EndTimeWindowViolation   = 0x10,
    TimeWindowViolation      = 0x20,
}

pub trait DWD {
    fn new() -> Self;
    fn start(&self, expire:u32);
    fn reset(&self);
    fn sys_reset(&self);
    fn status(&self) -> WdViolation;
    fn time_violation(&self) -> bool;
    fn status_clear(&self);
    fn expire(&self, expire:u32) -> Result<(), u32>;
    fn counter_enable(&self);
    fn count_down(&self) -> u32;
}
