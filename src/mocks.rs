use core::fmt::{Debug, Formatter};
use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;
use mockall::mock;

mock! {
    pub SPIBus {}

    impl Transfer<u8> for SPIBus{
        type Error = u32;

        fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'static [u8], u32>;
    }
}

mock! {
    pub Pin {}

    impl OutputPin for Pin {
        type Error = u32;

        fn set_low(&mut self) -> Result<(), u32>;
        fn set_high(&mut self) -> Result<(), u32>;
    }
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
