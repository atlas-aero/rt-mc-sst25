//! # Non-blocking I/O library for Microchip SST25 flash memory series
//!
//! Optionally non-blocking crate for interacting with Microchip SST25 flash memory devices like
//! [SST25VF080B](https://ww1.microchip.com/downloads/en/DeviceDoc/20005045C.pdf).
//!
//! # Example
//!
//! For all details see [device] module.
//!
//! ````
//! use mc_sst25::device::{Flash, Memory};
//! use mc_sst25::example::{MockBus, MockPin};
//!
//! let bus = MockBus::default();
//! let pin_hold = MockPin::default();
//! let pin_wp = MockPin::default();
//!
//! let mut device = Flash::new(bus, pin_wp, pin_hold);
//!
//! // Writing a single byte
//! device.erase_full().unwrap();
//! device.byte_program(0x0, 0x66).unwrap();
//!
//! // Writing larger data
//! device.aai_program(0x1, &[0x1, 0x2, 0x3, 0x4]).unwrap();
//!
//! // Reading data starting at address 0x0
//! let data = device.read::<5>(0x0).unwrap();
//! assert_eq!([0x66, 0x1, 0x2, 0x3, 0x4], data);
//! ````
#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "strict", deny(warnings))]

pub mod device;

#[cfg(feature = "example")]
pub mod example;

#[cfg(test)]
mod mocks;
#[cfg(test)]
mod tests;
