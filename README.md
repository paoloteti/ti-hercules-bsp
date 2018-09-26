[![Build Status](https://travis-ci.org/paoloteti/ti-hercules-bsp.svg?branch=master)](https://travis-ci.org/paoloteti/ti-hercules-bsp)

# TMS570 BSP

Bare Metal Board Support Package for Texas Instruments Cortex-R4F/R5F TMS570
MCUs.

TMS570 Transportation MCUs are ARM Cortex-R4F based floating point MCUs that meet IEC61508/SIL3 safety standards. Targeted transportation safety applications include automotive chassis and stability control, electric power steering, hybrid and electric vehicles, aerospace, railway communications, and off-road vehicle engine control.

The TMS570 family integrates dual Cortex-R4F and Cortex-R5F processors in lock-step and is designed to meet automotive and transportation safety standards. These devices provide system-wide protection through seamless support for error detection from the processor, through the bus interconnect, and into the memories.

## Getting started

* Rust nightly as default toolchain
  * Latest tested: `rustc 1.30.0-nightly (4141a4079 2018-09-25)`
* Add an armebv7r target:
  * Hard-float: `rustup target add armebv7r-none-eabihf`
  * Soft-float: `rustup target add armebv7r-none-eabi`
* GCC v7.3.2 for ARM: `sudo apt-get install gcc-arm-none-eabi`
* JTAG programmer: Lautherbach Trace32 Powerview for ARM or OpenOCD

## Build

Just run `cargo build` or `cargo build --release`

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual
licensed as above, without any additional terms or conditions.