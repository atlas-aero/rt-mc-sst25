use core::fmt::Debug;
use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;

/// SS25* flash memory chip
pub struct Flash<B: Transfer<u8>, P: OutputPin> {
    /// SPI bus
    bus: B,

    /// GPIO EN pin
    pin_enable: P,

    /// GPIO WP pin
    #[allow(unused)]
    pin_write_protection: P,

    /// GPIO Hold pin
    pin_hold: P,

    /// Is the device configured?
    configured: bool,
}

/// Error when reading data of device
#[derive(Debug, PartialEq, Eq)]
pub enum ReadError<B: Transfer<u8>, P: OutputPin> {
    /// SPI transfer error
    TransferError(B::Error),

    /// Error while setting GPIO state of EN pin
    EnablePinError(P::Error),

    /// Error while setting GPIO state of HOLD pin
    HoldPinError(P::Error),
}

impl<B: Transfer<u8>, P: OutputPin> Flash<B, P> {
    pub fn new(bus: B, pin_enable: P, pin_write_protection: P, pin_hold: P) -> Self {
        Self {
            bus,
            pin_enable,
            pin_write_protection,
            pin_hold,
            configured: false,
        }
    }

    /// Reads and returns the status registers
    pub fn read_status(&mut self) -> Result<Status, ReadError<B, P>> {
        self.configure()?;

        self.pin_enable.set_low().map_err(ReadError::EnablePinError)?;
        let status = self.bus.transfer(&mut [0b0000_0101, 0x0]).map_err(ReadError::TransferError)?[1];
        self.pin_enable.set_high().map_err(ReadError::EnablePinError)?;

        Ok(Status::from_register(status))
    }

    /// Sets the base GPIO states once
    fn configure(&mut self) -> Result<(), ReadError<B, P>> {
        if self.configured {
            return Ok(());
        }

        self.pin_hold.set_high().map_err(ReadError::HoldPinError)?;
        self.configured = true;

        Ok(())
    }
}

/// Mapped status register
#[derive(Clone, Debug)]
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
    pub fn from_register(data: u8) -> Self {
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
}
