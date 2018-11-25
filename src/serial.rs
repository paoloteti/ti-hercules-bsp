pub mod event {
    pub type SciEvent = u32;
    /// Framing error
    pub const FE_INT: SciEvent = 0x0400_0000;
    /// Overrun error
    pub const OE_INT: SciEvent = 0x0200_0000;
    /// Parity error
    pub const PE_INT: SciEvent = 0x0100_0000;
    /// Receive buffer ready
    pub const RX_INT: SciEvent = 0x0000_0200;
    /// Transmit buffer ready
    pub const TX_INT: SciEvent = 0x0000_0100;
    /// Wakeup
    pub const WAKE_INT: SciEvent = 0x0000_0002;
    /// Break detect
    pub const BREAK_INT: SciEvent = 0x0000_0001;
}

pub enum StopBits {
    /// Two stop bits
    Two = 0x1 << 4,
    /// One stop bits
    One = 0x0,
}

pub enum DataBits {
    /// The character is 1 bit long.
    One = 0,
    /// The character is 2 bit long.
    Two = 1,
    /// The character is 3 bit long.
    Three = 2,
    /// The character is 4 bit long.
    For = 3,
    /// The character is 5 bit long.
    Five = 4,
    /// The character is 6 bit long.
    Six = 5,
    /// The character is 7 bit long.
    Seven = 6,
    /// The character is 8 bit long.
    Eight = 7,
}

pub enum Parity {
    /// Parity Odd
    Odd = 0b0100,
    /// Parity Even
    Even = 0b1100,
    /// Parity None
    None = 0b0000,
}

pub trait SerialLine {
    fn new(id: u32, databits: DataBits, stop: StopBits, parity: Parity) -> Self;
    fn id(&self) -> u32;
    fn set_baudrate(&mut self, baudrate: u32) -> &mut Self;
    fn rx_enable(&mut self, enable: bool) -> &mut Self;
    fn tx_enable(&mut self, enable: bool) -> &mut Self;
    fn baudrate(&self) -> u32;
    fn open(&self);
    fn close(&self);
    fn put(&self, b: u8);
    fn get(&self) -> u8;
    fn getc_try(&self) -> Option<u8>;
    fn write(&self, buffer: &[u8]);
    fn read(&self, buffer: &mut [u8]);
    fn error(&self) -> u32;
    fn interrupt(&self, ev: event::SciEvent);
}
