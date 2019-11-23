[![Build Status](https://travis-ci.org/paoloteti/ti-hercules-bsp.svg?branch=master)](https://travis-ci.org/paoloteti/ti-hercules-bsp)

# TMS570 BSP

Bare Metal Board Support Package for Texas Instruments Cortex-R4F/R5F TMS570
MCUs.

TMS570 Transportation MCUs are ARM Cortex-R4F based floating point MCUs that meet IEC61508/SIL3 safety standards. Targeted transportation safety applications include automotive chassis and stability control, electric power steering, hybrid and electric vehicles, aerospace, railway communications, and off-road vehicle engine control.

The TMS570 family integrates dual Cortex-R4F and Cortex-R5F processors in lock-step and is designed to meet automotive and transportation safety standards. These devices provide system-wide protection through seamless support for error detection from the processor, through the bus interconnect, and into the memories.

## Getting started

* Rust nightly as default toolchain (2018 edition)
  * Latest tested release: `rustc 1.41.0-nightly (5c5b8afd8 2019-11-16)`.
    Any other recent nightly release should work as well.
* Add an armebv7r target:
  * Hard-float: `rustup target add armebv7r-none-eabihf`
  * Soft-float: `rustup target add armebv7r-none-eabi`
* GCC v7.3.2 (or newer) for ARM: `sudo apt install gcc-arm-none-eabi`
* JTAG programmer: Lautherbach Trace32 Powerview for ARM, SEGGER JLINK or any other supporting TMS570.

## How to use

```
tms570 = { git = "https://github.com/paoloteti/ti-hercules-bsp.git" }
f021_fapi = { git = "https://github.com/paoloteti/f021-flash-api.git" }
```

Example code for TMS570LS3137 can be found [here](https://github.com/paoloteti/tms570ls3137)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual
licensed as above, without any additional terms or conditions.

Please do not ask for features, but send a Pull Request instead.
