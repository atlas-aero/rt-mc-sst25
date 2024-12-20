use core::fmt::{Debug, Formatter};
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::{Operation, SpiDevice};
use mockall::mock;

#[derive(Debug, Copy, Clone)]
pub enum PinError {
    Error1,
}

#[derive(Debug, Copy, Clone)]
pub enum BusError {
    Error1,
}

mock! {
    pub SPIBus {}

    impl SpiDevice<u8> for SPIBus{
        fn transaction<'a>(&mut self, operations: &mut [Operation<'a, u8>]) -> Result<(), BusError>;
    }
}

impl embedded_hal::spi::ErrorType for MockSPIBus {
    type Error = BusError;
}

mock! {
    pub Pin {}

    impl OutputPin for Pin {
        fn set_low(&mut self) -> Result<(), PinError>;
        fn set_high(&mut self) -> Result<(), PinError>;
    }
}

impl embedded_hal::digital::ErrorType for MockPin {
    type Error = PinError;
}

impl Debug for MockSPIBus {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str("MockSPIBus")
    }
}

impl Debug for MockPin {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str("MockPin")
    }
}

impl embedded_hal::digital::Error for PinError {
    fn kind(&self) -> embedded_hal::digital::ErrorKind {
        embedded_hal::digital::ErrorKind::Other
    }
}

impl embedded_hal::spi::Error for BusError {
    fn kind(&self) -> embedded_hal::spi::ErrorKind {
        embedded_hal::spi::ErrorKind::Other
    }
}
