use crate::device::{CommandError, Flash, Status};
use crate::mocks::{MockPin, MockSPIBus};

#[test]
fn test_device_read_status_success() {
    let status = MockedPeripherals::default()
        .mock_configure()
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0x1C])
        .into_flash()
        .read_status()
        .unwrap();

    assert!(!status.busy);
    assert!(!status.write_enabled);
    assert!(status.block0_protected);
    assert!(status.block1_protected);
    assert!(status.block2_protected);
    assert!(!status.block3_protected);
    assert!(!status.aai_programming_mode);
    assert!(!status.bits_read_only);
}

#[test]
fn test_device_read_status_hold_error() {
    let error = MockedPeripherals::hold_error().into_flash().read_status().unwrap_err();
    assert!(matches!(error, CommandError::HoldPinError(10)))
}

#[test]
fn test_device_read_status_wp_pin_error() {
    let error = MockedPeripherals::wp_error().into_flash().read_status().unwrap_err();
    assert!(matches!(error, CommandError::WriteProtectionPinError(15)))
}

#[test]
fn test_device_read_status_enable_error() {
    let error = MockedPeripherals::default()
        .mock_configure()
        .enable_error()
        .into_flash()
        .read_status()
        .unwrap_err();

    assert!(matches!(error, CommandError::EnablePinError(20)))
}

#[test]
fn test_device_read_status_transfer_error() {
    let error = MockedPeripherals::default()
        .mock_configure()
        .mock_enable_pin()
        .spi_transfer_error()
        .into_flash()
        .read_status()
        .unwrap_err();

    assert!(matches!(error, CommandError::TransferError(30)))
}

#[test]
fn test_device_write_enable_hold_error() {
    let error = MockedPeripherals::hold_error().into_flash().write_enable().unwrap_err();
    assert!(matches!(error, CommandError::HoldPinError(10)))
}

#[test]
fn test_device_write_enable_wp_pin_error() {
    let error = MockedPeripherals::wp_error().into_flash().write_enable().unwrap_err();
    assert!(matches!(error, CommandError::WriteProtectionPinError(15)))
}

#[test]
fn test_device_write_enable_enable_error() {
    let error = MockedPeripherals::default()
        .mock_configure()
        .enable_error()
        .into_flash()
        .write_enable()
        .unwrap_err();

    assert!(matches!(error, CommandError::EnablePinError(20)))
}

#[test]
fn test_device_write_enable_transfer_error() {
    let error = MockedPeripherals::default()
        .mock_configure()
        .mock_enable_pin()
        .spi_transfer_error()
        .into_flash()
        .write_enable()
        .unwrap_err();

    assert!(matches!(error, CommandError::TransferError(30)))
}

#[test]
fn test_device_write_enable_success() {
    MockedPeripherals::default()
        .mock_configure()
        .mock_enable_pin()
        .expect_write_enable_command()
        .into_flash()
        .write_enable()
        .unwrap();
}

#[test]
fn test_device_erase_full_hold_error() {
    let error = MockedPeripherals::hold_error().into_flash().erase_full().unwrap_err();
    assert!(matches!(error, CommandError::HoldPinError(10)))
}

#[test]
fn test_device_erase_full_wp_pin_error() {
    let error = MockedPeripherals::wp_error().into_flash().erase_full().unwrap_err();
    assert!(matches!(error, CommandError::WriteProtectionPinError(15)))
}

#[test]
fn test_device_erase_full_busy() {
    let error = MockedPeripherals::default()
        .mock_configure()
        .mock_enable_pin()
        .expect_write_enable_command()
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0001])
        .into_flash()
        .erase_full()
        .unwrap_err();

    assert!(matches!(error, CommandError::Busy))
}

#[test]
fn test_device_erase_transfer_error() {
    let error = MockedPeripherals::default()
        .mock_configure()
        .mock_enable_pin()
        .expect_write_enable_command()
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0010])
        .mock_enable_pin()
        .spi_transfer_error()
        .into_flash()
        .erase_full()
        .unwrap_err();

    assert!(matches!(error, CommandError::TransferError(30)))
}

#[test]
fn test_device_erase_full_blocking() {
    MockedPeripherals::default()
        .mock_configure()
        .mock_enable_pin()
        .expect_write_enable_command()
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0010])
        .mock_enable_pin()
        .expect_full_erase()
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0001]) // Still busy
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0001]) // Still busy
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0000])
        .into_flash()
        .erase_full()
        .unwrap();
}

#[test]
fn test_device_erase_full_non_blocking() {
    let mut flash = MockedPeripherals::default()
        .mock_configure()
        .mock_enable_pin()
        .expect_write_enable_command()
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0010])
        .mock_enable_pin()
        .expect_full_erase()
        .into_flash();

    flash.set_non_blocking();
    flash.erase_full().unwrap();
}

#[test]
fn test_status_from_register() {
    assert!(!Status::from_register(0b1111_1110).busy);
    assert!(Status::from_register(0b0000_0001).busy);

    assert!(!Status::from_register(0b1111_1101).write_enabled);
    assert!(Status::from_register(0b0000_0010).write_enabled);

    assert!(!Status::from_register(0b1111_1011).block0_protected);
    assert!(Status::from_register(0b0000_0100).block0_protected);

    assert!(!Status::from_register(0b1111_0111).block1_protected);
    assert!(Status::from_register(0b0000_1000).block1_protected);

    assert!(!Status::from_register(0b1110_1111).block2_protected);
    assert!(Status::from_register(0b0001_0000).block2_protected);

    assert!(!Status::from_register(0b1101_1111).block3_protected);
    assert!(Status::from_register(0b0010_0000).block3_protected);

    assert!(!Status::from_register(0b1011_1111).aai_programming_mode);
    assert!(Status::from_register(0b0100_0000).aai_programming_mode);

    assert!(!Status::from_register(0b0111_1111).bits_read_only);
    assert!(Status::from_register(0b1000_0000).bits_read_only);
}

#[derive(Default)]
struct MockedPeripherals {
    pub pin_en: MockPin,
    pub pin_hold: MockPin,
    pub pin_wp: MockPin,
    pub bus: MockSPIBus,
}

impl MockedPeripherals {
    /// Returns a new flash device with mocked peripherals
    pub fn into_flash(self) -> Flash<MockSPIBus, MockPin> {
        Flash::new(self.bus, self.pin_en, self.pin_wp, self.pin_hold)
    }

    /// Simulates a error when setting HOLD state
    pub fn hold_error() -> Self {
        let mut peripherals = Self::default();
        peripherals.pin_hold.expect_set_high().times(1).return_const(Err(10));
        peripherals
    }

    /// Simulates a error when setting WP state
    pub fn wp_error() -> Self {
        let mut peripherals = Self::default();
        peripherals.pin_hold.expect_set_high().times(1).return_const(Ok(()));
        peripherals.pin_wp.expect_set_low().times(1).return_const(Err(15));

        peripherals
    }

    /// Mocks a error of EN pin
    pub fn enable_error(mut self) -> Self {
        self.pin_en.expect_set_low().times(1).return_const(Err(20));
        self
    }

    /// Simulates a SPI transfer error
    pub fn spi_transfer_error(mut self) -> Self {
        self.bus.expect_transfer().times(1).return_const(Err(30));
        self
    }

    /// Mocks the one-time configuration logic
    pub fn mock_configure(mut self) -> Self {
        self.pin_hold.expect_set_high().times(1).return_const(Ok(()));
        self.pin_wp.expect_set_low().times(1).return_const(Ok(()));

        self
    }

    /// Mocks the transition of enable pin
    pub fn mock_enable_pin(mut self) -> Self {
        self.pin_en.expect_set_low().times(1).return_const(Ok(()));
        self.pin_en.expect_set_high().times(1).return_const(Ok(()));

        self
    }

    /// Expects a correct write-enable command
    pub fn expect_write_enable_command(mut self) -> Self {
        self.expect_transfer(&[0b0000_0110], &[0x0]);
        self
    }

    /// Expects a status command request and returns the given raw response
    pub fn expect_status_request(mut self, response: &'static [u8]) -> Self {
        self.expect_transfer(&[0b0000_0101, 0x0], response);
        self
    }

    /// Expects a full chip erase command
    pub fn expect_full_erase(mut self) -> Self {
        self.expect_transfer(&[0b0110_0000], &[0x0]);
        self
    }

    fn expect_transfer(&mut self, command: &'static [u8], response: &'static [u8]) {
        self.bus.expect_transfer().times(1).returning(move |data: &mut [u8]| {
            assert_eq!(command, data);
            Ok(response)
        });
    }
}
