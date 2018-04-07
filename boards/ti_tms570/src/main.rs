#![no_std]
#![feature(lang_items)]
#![feature(compiler_builtins_lib)]
#![feature(naked_functions)]

extern crate compiler_builtins;
extern crate tms570;

use tms570::serial::{SerialLine, Parity, StopBits, DataBits};
use tms570::scilin::SciChipset;
use tms570::gio::{Gio, GioPorts, GioDirection};
use tms570::iomm::Iomm;
use tms570::pinmux::PinMux;

pub mod lang_items;
pub mod handlers;

fn main() {
    // Muxing pins is inherently unsafe
    unsafe {
        let pmux = Iomm::new();
        pmux.setup_pins(&[PinMux::Pin38_SCIRX, PinMux::Pin39_SCITX]);
    }

    let ioport = Gio::new();
    let mut uart:SciChipset = SerialLine::new(0, DataBits::Eight,
                                              StopBits::One,
                                              Parity::None);
    uart.rx_enable(true)
        .tx_enable(true)
        .set_baudrate(115_200);
    uart.open();

    ioport.direction(GioPorts::A, 7, GioDirection::Input);

    loop {
        let button = ioport.get(GioPorts::A, 7);
        if button {
            uart.write(b"Hello World!\n");
        }
    }
}
