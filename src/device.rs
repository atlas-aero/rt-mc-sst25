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

    /// Switches to blocking mode
    pub fn set_blocking(&mut self) {
        self.blocking = true;
    }

    /// Switches to non-blocking mode
    pub fn set_non_blocking(&mut self) {
        self.blocking = false;
    }

    /// Reads and returns the status registers
    pub fn read_status(&mut self) -> Result<Status, CommandError<B, P>> {
        Ok(Status::from_register(self.transfer(&mut [0b0000_0101, 0x0])?[1]))
    }

    /// Enables write operations
    pub fn write_enable(&mut self) -> Result<(), CommandError<B, P>> {
        self.transfer(&mut [0b0000_0110])?;
        Ok(())
    }

    /// Erases the full chip. Disables internal write protection.
    /// Waits until operation is completed in blocking mode, otherwise returns when command is sent
    pub fn erase_full(&mut self) -> Result<(), CommandError<B, P>> {
        self.write_enable()?;
        self.assert_not_busy()?;

        self.transfer(&mut [0b0110_0000])?;
        self.wait()
    }

    /// Programs/Writes the given byte at the given address. Disables internal write protection.
    /// Waits until operation is completed in blocking mode, otherwise returns when command is sent
    pub fn byte_program(&mut self, address: u32, data: u8) -> Result<(), CommandError<B, P>> {
        if address > 16777216 {
            return Err(CommandError::InvalidAddress);
        }

        self.write_enable()?;
        self.assert_not_busy()?;

        self.transfer(&mut [
            0b0000_0010,
            (address >> 16) as u8,
            (address >> 8) as u8,
            address as u8,
            data,
        ])?;
        self.wait()
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

    /// Returns an error in case device is busy
    fn assert_not_busy(&mut self) -> Result<(), CommandError<B, P>> {
        if self.read_status()?.busy {
            return Err(CommandError::Busy);
        }

        Ok(())
    }

    /// Blocks until device is not busy anymore
    fn wait(&mut self) -> Result<(), CommandError<B, P>> {
        while self.blocking && self.read_status()?.busy {}
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
