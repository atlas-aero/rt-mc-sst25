//! # Mocks for doc examples
use core::convert::Infallible;
use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;

/// Mocked GPIO output pin
#[derive(Default, Debug)]
pub struct MockPin {}

impl OutputPin for MockPin {
    type Error = Infallible;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// Mocked SPI Bus
#[derive(Default, Debug)]
pub struct MockBus {
    /// Was previous transfer a read command?
    read_command: bool,
}

impl Transfer<u8> for MockBus {
    type Error = Infallible;

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        if words[0] == 0b0000_0011 {
            self.read_command = true;
            return Ok(&[0x0]);
        }

        if self.read_command {
            self.read_command = false;

            return if words.len() == 5 {
                Ok(&[0x66, 0x1, 0x2, 0x3, 0x4])
            } else {
                Ok(&[0xa, 0xb, 0xc, 0xd])
            };
        }

        Ok(&[0x0, 0b0000_0000])
    }
}
