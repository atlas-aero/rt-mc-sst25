# I/O library for Microchip SST25 flash memory series
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Crates.io](https://img.shields.io/crates/v/mc-sst25.svg)](https://crates.io/crates/mc-sst25)
[![Actions Status](https://github.com/atlas-aero/rt-mc-sst25/workflows/QA/badge.svg)](http://github.com/pegasus-aero/rt-mc-sst25/actions)

Non-blocking crate for interacting with Microchip SST25 flash memory devices like 
[SST25VF080B](https://ww1.microchip.com/downloads/en/DeviceDoc/20005045C.pdf).

Currently, the following features are implemented:
* [Reading memory](https://docs.rs/mc-sst25/latest/mc-sst25/device/index.html#reading-memory)
* [Writing single bytes](https://docs.rs/mc-sst25/latest/mc-sst25/device/index.html#writing-single-bytes)
* [Auto-address-increment writes](https://docs.rs/mc-sst25/latest/mc-sst25/device/index.html#writing-larger-data)
* [Full chip erase](https://docs.rs/mc-sst25/latest/mc-sst25/device/index.html#full-chip-erase)
* [Reading status](https://docs.rs/mc-sst25/latest/mc-sst25/device/index.html#reading-status)
* [Writing status](https://docs.rs/mc-sst25/latest/mc-sst25/device/index.html#writing-status)

## Example
For all details see [monitor](https://docs.rs/ltc681x/latest/ltc681x/monitor/index.html) module.

````rust
use mc_sst25::device::Flash;
use mc_sst25::example::{MockBus, MockPin};

let bus = MockBus::default();
let pin_en = MockPin::default();
let pin_hold = MockPin::default();
let pin_wp = MockPin::default();

let mut device = Flash::new(bus, pin_en, pin_wp, pin_hold);

// Writing a single byte
device.erase_full().unwrap();
device.byte_program(0x0, 0x66).unwrap();

// Writing larger data
device.aai_program(0x1, &[0x1, 0x2, 0x3, 0x4]).unwrap();

// Reading data starting at address 0x0
let data = device.read::<5>(0x0).unwrap();
assert_eq!([0x66, 0x1, 0x2, 0x3, 0x4], data);
````

## State

> :warning: The crate has only been tested for the SST25VF080B variant.

## Development

Any form of support is greatly appreciated. Feel free to create issues and PRs.
See [DEVELOPMENT](DEVELOPMENT.md) for more details.  

## License
Licensed under either of

* Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
at your option.

Each contributor agrees that his/her contribution covers both licenses.