#![no_std]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(naked_functions)]
#![feature(alloc)]
#![feature(allocator_api)]

extern crate panic_impl;
extern crate tms570;
extern crate alloc;
extern crate linked_list_allocator;
use tms570::serial::{SerialLine, Parity, StopBits, DataBits};
use tms570::scilin::SciChipset;
use tms570::gio::{Gio, GioPorts, GioDirection};
use tms570::iomm::Iomm;
use tms570::pinmux::PinMux;
use linked_list_allocator::LockedHeap;
use alloc::vec::Vec;

pub mod lang_items;
pub mod handlers;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn heap_init() {
    unsafe {
        let start = tms570::heap_start() as usize;
        let size = tms570::heap_size() as usize;
        ALLOCATOR.lock().init(start, size);
    }
}

fn main() {
    unsafe {
        // Muxing pins is inherently unsafe
        let pmux = Iomm::new();
        pmux.setup_pins(&[PinMux::Pin38_SCIRX, PinMux::Pin39_SCITX]);
    }

    heap_init();

    let ioport = Gio::new();
    let mut uart:SciChipset = SerialLine::new(0, DataBits::Eight,
                                              StopBits::One,
                                              Parity::None);
    uart.rx_enable(true)
        .tx_enable(true)
        .set_baudrate(115_200);
    uart.open();

    ioport.direction(GioPorts::A, 7, GioDirection::Input);

    let mut v = Vec::new();
    let mut click = 0;
    loop {
        let button = ioport.get(GioPorts::A, 7);
        if button {
            v.push(click);
            if v.pop().unwrap() == click {
                click += 1;
                uart.write(b"Alloc Test done\n");
            } else {
                uart.write(b"Alloc Test fail\n");
            }
        }
    }
}

#[lang = "oom"]
#[no_mangle]
pub fn rust_oom(_: core::alloc::Layout) -> ! {
    unsafe { core::intrinsics::abort() }
}
