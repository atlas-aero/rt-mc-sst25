//! # Non-Blocking & blocking SPI protocol abstraction
//!
//! ## Setup
//!
//! Creating a [device](Flash) instance requires the following peripherals:
//! * An SPI bus implementing [embedded-hal Transfer trait](embedded_hal::blocking::spi::Transfer)
//! * Three GPIO pins connected to EN, WP and HOLD of the flash chip implementing [embedded-hal OutputPin](embedded_hal::digital::v2::OutputPin)
//!
//! The device can be communicated with either in blocking or non-blocking mode:
//! * In the case of blocking mode, the library waits internally until the respective operation is
//! completely finished.
//! * In the case of non-blocking mode, it is up to the caller to check the busy flag of the status register.
//! (s. [Reading status register](#reading-status))
//!
//! ````
//!# use mc_sst25::device::{Flash, Memory};
//!# use mc_sst25::example::{MockBus, MockPin};
//!#
//!# let bus = MockBus::default();
//!# let pin_en = MockPin::default();
//!# let pin_hold = MockPin::default();
//!# let pin_wp = MockPin::default();
//!#
//! let mut device = Flash::new(bus, pin_en, pin_wp, pin_hold);
//!
//! // Blocking mode (default)
//! device.set_blocking();
//!
//! // Non-blocking
//! device.set_non_blocking();
//! ````
//!
//! ## Reading status
//!
//! The device contains eight status bits, which are mapped to [Status] struct.
//!
//! ````
//!# use mc_sst25::device::{Flash, Memory};
//!# use mc_sst25::example::{MockBus, MockPin};
//!#
//!# let bus = MockBus::default();
//!# let pin_en = MockPin::default();
//!# let pin_hold = MockPin::default();
//!# let pin_wp = MockPin::default();
//!#
//!# let mut device = Flash::new(bus, pin_en, pin_wp, pin_hold);
//!#
//! let status = device.read_status().unwrap();
//!
//! assert!(!status.busy);
//! assert!(!status.block0_protected);
//! assert!(!status.write_enabled);
//! ````
//!
//! ## Writing status
//!
//! The following status flags are used for (write) protecting memory segments.
//! On device power-up all memory blocks are protected.
//!
//! ````
//!# use mc_sst25::device::{Flash, Memory, Status};
//!# use mc_sst25::example::{MockBus, MockPin};
//!#
//!# let bus = MockBus::default();
//!# let pin_en = MockPin::default();
//!# let pin_hold = MockPin::default();
//!# let pin_wp = MockPin::default();
//!#
//!# let mut device = Flash::new(bus, pin_en, pin_wp, pin_hold);
//!#
//! let mut  status = Status::default();
//! status.block0_protected = false;
//! status.block1_protected = false;
//! status.block2_protected = true;
//! status.block3_protected = true;
//! status.bits_read_only = false;
//!
//! device.write_status(status).unwrap();
//! ````
//!
//! ## Writing single bytes
//!
//! The following method is used for writing single bytes.
//!
//! *Note: Memory region needs to be unprotected (s. [Reading status](#reading-status)), otherwise
//! write operation is ignored by device*
//! ````
//!# use mc_sst25::device::{Flash, Memory, Status};
//!# use mc_sst25::example::{MockBus, MockPin};
//!#
//!# let bus = MockBus::default();
//!# let pin_en = MockPin::default();
//!# let pin_hold = MockPin::default();
//!# let pin_wp = MockPin::default();
//!#
//!# let mut device = Flash::new(bus, pin_en, pin_wp, pin_hold);
//!#
//! // Writing byte 0x64 to address 0xc8
//! device.byte_program(0xc8, 0x64).unwrap();
//! ````
//!
//! ## Writing larger data
//!
//! Auto-address-increment method is used for writing larger amount of data. The given buffer needs to
//! contain an even amount of data. (e.g 2, 4, 6, 8, ... bytes).
//!
//! *Note: Memory region needs to be unprotected (s. [Reading status](#reading-status)), otherwise
//! write operation is ignored by device*
//! ````
//!# use mc_sst25::device::{Flash, Memory, Status};
//!# use mc_sst25::example::{MockBus, MockPin};
//!#
//!# let bus = MockBus::default();
//!# let pin_en = MockPin::default();
//!# let pin_hold = MockPin::default();
//!# let pin_wp = MockPin::default();
//!#
//!# let mut device = Flash::new(bus, pin_en, pin_wp, pin_hold);
//!#
//! // Writing four bytes to address 0x5, so the following data is written:
//! // Address 0x5 contains byte 0x1
//! // Address 0x6 contains byte 0x2
//! // ...
//! device.aai_program(0x5, &[0x1, 0x2, 0x3, 0x4]).unwrap();
//! ````
//!
//! ## Full chip erase
//!
//! The chip supports erasing the entire memory.
//!
//! *Note: All memory blocks needs to be unprotected (s. [Reading status](#reading-status)), otherwise
//! erase operation is ignored by device*
//! ````
//!# use mc_sst25::device::{Flash, Memory, Status};
//!# use mc_sst25::example::{MockBus, MockPin};
//!#
//!# let bus = MockBus::default();
//!# let pin_en = MockPin::default();
//!# let pin_hold = MockPin::default();
//!# let pin_wp = MockPin::default();
//!#
//!# let mut device = Flash::new(bus, pin_en, pin_wp, pin_hold);
//!#
//! device.erase_full().unwrap();
//! ````
//!
//! ## Reading memory
//!
//! Reading an arbitrary amount of data starting at the given address. The data amount is determined
//! by the generic const L.
//!
//! *Note: If the maximum address is within range, the chip wraps automatically and continuous
//! at the first address.*
//!
//! ````
//!# use mc_sst25::device::{Flash, Status, Memory};
//!# use mc_sst25::example::{MockBus, MockPin};
//!#
//!# let bus = MockBus::default();
//!# let pin_en = MockPin::default();
//!# let pin_hold = MockPin::default();
//!# let pin_wp = MockPin::default();
//!#
//!# let mut device = Flash::new(bus, pin_en, pin_wp, pin_hold);
//!#
//! // Reading four bytes starting at address 0x0
//! let data = device.read::<4>(0x0).unwrap();
//! assert_eq!([0xa, 0xb, 0xc, 0xd], data);
//! ````
use core::fmt::Debug;
use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;

/// General flash memory interface
pub trait Memory {
    type Error;

    /// Switches to blocking mode
    fn set_blocking(&mut self);

    /// Switches to non-blocking mode
    fn set_non_blocking(&mut self);

    /// Reads and returns the status registers
    fn read_status(&mut self) -> Result<Status, Self::Error>;

    /// Enables write operations
    fn write_enable(&mut self) -> Result<(), Self::Error>;

    /// Enables write operations
    fn write_disable(&mut self) -> Result<(), Self::Error>;

    /// Writes the given status to status registers
    fn write_status(&mut self, status: Status) -> Result<(), Self::Error>;

    /// Erases the full chip.
    fn erase_full(&mut self) -> Result<(), Self::Error>;

    /// Programs/Writes the given byte at the given address.
    fn byte_program(&mut self, address: u32, data: u8) -> Result<(), Self::Error>;

    /// Auto address increment (AAI) programming for writing larger amount of data
    fn aai_program(&mut self, address: u32, buffer: &[u8]) -> Result<(), Self::Error>;

    /// Reads data with length L starting at the given address
    fn read<const L: usize>(&mut self, address: u32) -> Result<[u8; L], Self::Error>;
}

/// SS25* flash memory chip
pub struct Flash<B: Transfer<u8>, P: OutputPin> {
    /// SPI bus
    bus: B,

    /// GPIO EN pin
    pin_enable: P,

    /// GPIO WP pin
    pin_write_protection: P,

    /// GPIO Hold pin
    pin_hold: P,

    /// Is the device configured?
    configured: bool,

    /// True if blocks on longer lasting operations
    blocking: bool,
}

/// Error when communicating with the device
#[derive(Debug, PartialEq, Eq)]
pub enum CommandError<B: Transfer<u8>, P: OutputPin> {
    /// SPI transfer error
    TransferError(B::Error),

    /// Error while setting GPIO state of EN pin
    EnablePinError(P::Error),

    /// Error while setting GPIO state of HOLD pin
    HoldPinError(P::Error),

    /// Error while setting GPIO state of WP pin
    WriteProtectionPinError(P::Error),

    /// Chip is still busy executing another operation
    Busy,

    /// The given memory address is out of range
    InvalidAddress,

    /// The given buffer size is too small for the called operation
    BufferTooSmall,

    /// The called operation requires an even buffer size
    BufferUneven,
}

const CMD_AAI_PROGRAM: u8 = 0b1010_1101;

impl<B: Transfer<u8>, P: OutputPin> Memory for Flash<B, P> {
    type Error = CommandError<B, P>;

    /// Switches to blocking mode
    fn set_blocking(&mut self) {
        self.blocking = true;
    }

    /// Switches to non-blocking mode
    fn set_non_blocking(&mut self) {
        self.blocking = false;
    }

    /// Reads and returns the status registers
    fn read_status(&mut self) -> Result<Status, CommandError<B, P>> {
        Ok(Status::from_register(self.transfer(&mut [0b0000_0101, 0x0])?[1]))
    }

    /// Enables write operations
    fn write_enable(&mut self) -> Result<(), CommandError<B, P>> {
        self.transfer(&mut [0b0000_0110])?;
        Ok(())
    }

    /// Enables write operations
    fn write_disable(&mut self) -> Result<(), CommandError<B, P>> {
        self.transfer(&mut [0b0000_0100])?;
        Ok(())
    }

    /// Writes the given status to status registers
    fn write_status(&mut self, status: Status) -> Result<(), CommandError<B, P>> {
        self.write_enable()?;

        self.bus.transfer(&mut [0x0]).map_err(CommandError::TransferError)?;
        let _ = self.transfer(&mut [0b0000_0001, status.to_registers()])?;

        Ok(())
    }

    /// Erases the full chip.
    /// Waits until operation is completed in blocking mode, otherwise returns when command is sent
    fn erase_full(&mut self) -> Result<(), CommandError<B, P>> {
        self.write_enable()?;
        self.assert_not_busy()?;

        self.transfer(&mut [0b0110_0000])?;
        self.wait(false)
    }

    /// Programs/Writes the given byte at the given address. Disables internal write protection.
    /// Waits until operation is completed in blocking mode, otherwise returns when command is sent
    fn byte_program(&mut self, address: u32, data: u8) -> Result<(), CommandError<B, P>> {
        self.assert_valid_address(address)?;

        self.write_enable()?;
        self.assert_not_busy()?;

        let mut frame = [0b0000_0010, 0x0, 0x0, 0x0, data];
        self.address_command(address, &mut frame);

        self.transfer(&mut frame)?;
        self.wait(false)
    }

    /// Auto address increment (AAI) programming for writing larger amount of data
    /// Buffer needs to contain at least two bytes and an even data amount
    fn aai_program(&mut self, address: u32, buffer: &[u8]) -> Result<(), CommandError<B, P>> {
        self.assert_valid_address(address)?;

        if buffer.len() < 2 {
            return Err(CommandError::BufferTooSmall);
        }

        if buffer.len() & 1 == 1 {
            return Err(CommandError::BufferUneven);
        }

        self.write_enable()?;
        self.assert_not_busy()?;

        let mut frame = [CMD_AAI_PROGRAM, 0x0, 0x0, 0x0, buffer[0], buffer[1]];
        self.address_command(address, &mut frame);
        self.transfer(&mut frame)?;
        self.wait(true)?;

        for chunk in buffer[2..].chunks(2) {
            self.transfer(&mut [CMD_AAI_PROGRAM, chunk[0], chunk[1]])?;
            self.wait(true)?;
        }

        self.write_disable()
    }

    /// Reads data with length L starting at the given address
    fn read<const L: usize>(&mut self, address: u32) -> Result<[u8; L], CommandError<B, P>> {
        self.assert_valid_address(address)?;
        self.configure()?;

        let mut frame = [0b0000_0011, 0x0, 0x0, 0x0];
        self.address_command(address, &mut frame);

        self.pin_enable.set_low().map_err(CommandError::EnablePinError)?;
        if let Err(error) = self.bus.transfer(&mut frame) {
            self.pin_enable.set_high().map_err(CommandError::EnablePinError)?;
            return Err(CommandError::TransferError(error));
        }

        let mut buffer = [0x0; L];

        match self.bus.transfer(&mut [0x0; L]) {
            Ok(data) => {
                buffer.clone_from_slice(data);
            }
            Err(error) => {
                self.pin_enable.set_high().map_err(CommandError::EnablePinError)?;
                return Err(CommandError::TransferError(error));
            }
        }

        self.pin_enable.set_high().map_err(CommandError::EnablePinError)?;
        Ok(buffer)
    }
}

impl<B: Transfer<u8>, P: OutputPin> Flash<B, P> {
    pub fn new(bus: B, pin_enable: P, pin_write_protection: P, pin_hold: P) -> Self {
        Self {
            bus,
            pin_enable,
            pin_write_protection,
            pin_hold,
            configured: false,
            blocking: true,
        }
    }

    /// Transfers the given data and returns the result
    /// Handles the EN pin status and sets the pin back to HIGH even on error
    fn transfer<'a>(&'a mut self, data: &'a mut [u8]) -> Result<&'a [u8], CommandError<B, P>> {
        self.configure()?;

        self.pin_enable.set_low().map_err(CommandError::EnablePinError)?;
        let result = self.bus.transfer(data).map_err(CommandError::TransferError);
        self.pin_enable.set_high().map_err(CommandError::EnablePinError)?;

        result
    }

    /// Adds the given memory address to the command frame
    fn address_command(&mut self, address: u32, frame: &mut [u8]) {
        frame[1] = (address >> 16) as u8;
        frame[2] = (address >> 8) as u8;
        frame[3] = address as u8;
    }

    /// Returns an error in case device is busy
    fn assert_not_busy(&mut self) -> Result<(), CommandError<B, P>> {
        if self.read_status()?.busy {
            return Err(CommandError::Busy);
        }

        Ok(())
    }

    /// Returns an error if the given address is out of range
    fn assert_valid_address(&self, address: u32) -> Result<(), CommandError<B, P>> {
        if address > 16777216 {
            return Err(CommandError::InvalidAddress);
        }

        Ok(())
    }

    /// Blocks until device is not busy anymore
    fn wait(&mut self, force: bool) -> Result<(), CommandError<B, P>> {
        while (self.blocking || force) && self.read_status()?.busy {}
        Ok(())
    }

    /// Sets the base GPIO states once
    fn configure(&mut self) -> Result<(), CommandError<B, P>> {
        if self.configured {
            return Ok(());
        }

        self.pin_hold.set_high().map_err(CommandError::HoldPinError)?;
        self.pin_write_protection
            .set_low()
            .map_err(CommandError::WriteProtectionPinError)?;
        self.configured = true;

        Ok(())
    }
}

/// Mapped status register
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Status {
    /// True if internal write operation is in progress
    pub busy: bool,

    /// True if device memory write is enabled
    pub write_enabled: bool,

    /// True if first block is write-protected
    pub block0_protected: bool,

    /// True if second block is write-protected
    pub block1_protected: bool,

    /// True if third block is write-protected
    pub block2_protected: bool,

    /// True if fourth block is write-protected
    pub block3_protected: bool,

    /// True => AAI programming mode,
    /// False => Byte-Program mode
    pub aai_programming_mode: bool,

    /// True if  BP3, BP2, BP1, BP0 are read-only
    pub bits_read_only: bool,
}

impl Status {
    /// Maps status bits to object
    pub(crate) fn from_register(data: u8) -> Self {
        Self {
            busy: data & (1 << 0) != 0,
            write_enabled: data & (1 << 1) != 0,
            block0_protected: data & (1 << 2) != 0,
            block1_protected: data & (1 << 3) != 0,
            block2_protected: data & (1 << 4) != 0,
            block3_protected: data & (1 << 5) != 0,
            aai_programming_mode: data & (1 << 6) != 0,
            bits_read_only: data & (1 << 7) != 0,
        }
    }

    /// Converts the status to register byte. Only writable bits are used
    pub(crate) fn to_registers(&self) -> u8 {
        let mut result = 0x0;

        if self.block0_protected {
            result |= 0x1 << 2;
        }

        if self.block1_protected {
            result |= 0x1 << 3;
        }

        if self.block2_protected {
            result |= 0x1 << 4;
        }

        if self.block3_protected {
            result |= 0x1 << 5;
        }

        if self.bits_read_only {
            result |= 0x1 << 7;
        }

        result
    }
}
