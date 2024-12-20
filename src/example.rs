//! # Mocks for doc examples
use core::convert::Infallible;
use embedded_hal::digital::{ErrorType, OutputPin};
use embedded_hal::spi::{Operation, SpiDevice};

/// Mocked GPIO output pin
#[derive(Default, Debug)]
pub struct MockPin {}

impl ErrorType for MockPin {
    type Error = Infallible;
}

impl OutputPin for MockPin {
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

impl embedded_hal::spi::ErrorType for MockBus {
    type Error = Infallible;
}

impl SpiDevice<u8> for MockBus {
    fn transaction(&mut self, operations: &mut [Operation<'_, u8>]) -> Result<(), Self::Error> {
        for operation in operations {
            match operation {
                Operation::Read(buffer) => {
                    if self.read_command {
                        self.read_command = false;

                        if buffer.len() == 5 {
                            buffer.copy_from_slice(&[0x66, 0x1, 0x2, 0x3, 0x4])
                        } else {
                            buffer.copy_from_slice(&[0xa, 0xb, 0xc, 0xd])
                        };
                    }
                }
                Operation::Write(words) => {
                    if words[0] == 0b0000_0011 {
                        self.read_command = true;
                    }
                }
                Operation::Transfer(_, _) => unimplemented!(),
                Operation::TransferInPlace(_) => unimplemented!(),
                Operation::DelayNs(_) => unimplemented!(),
            }
        }

        Ok(())
    }
}
