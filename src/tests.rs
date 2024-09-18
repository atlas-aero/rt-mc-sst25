use crate::device::{CommandError, Flash, Memory, Status};
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
fn test_device_write_disable_hold_error() {
    let error = MockedPeripherals::hold_error().into_flash().write_disable().unwrap_err();
    assert!(matches!(error, CommandError::HoldPinError(10)))
}

#[test]
fn test_device_write_disable_wp_pin_error() {
    let error = MockedPeripherals::wp_error().into_flash().write_disable().unwrap_err();
    assert!(matches!(error, CommandError::WriteProtectionPinError(15)))
}

#[test]
fn test_device_write_disable_enable_error() {
    let error = MockedPeripherals::default()
        .mock_configure()
        .enable_error()
        .into_flash()
        .write_disable()
        .unwrap_err();

    assert!(matches!(error, CommandError::EnablePinError(20)))
}

#[test]
fn test_device_write_disable_transfer_error() {
    let error = MockedPeripherals::default()
        .mock_configure()
        .mock_enable_pin()
        .spi_transfer_error()
        .into_flash()
        .write_disable()
        .unwrap_err();

    assert!(matches!(error, CommandError::TransferError(30)))
}

#[test]
fn test_device_write_disable_success() {
    MockedPeripherals::default()
        .mock_configure()
        .mock_enable_pin()
        .expect_write_disable_command()
        .into_flash()
        .write_disable()
        .unwrap();
}

#[test]
fn test_device_erase_sector_address_error() {
    let error = MockedPeripherals::default().into_flash().erase_sector(16777217).unwrap_err();
    assert!(matches!(error, CommandError::InvalidAddress))
}

#[test]
fn test_device_erase_sector_hold_error() {
    let error = MockedPeripherals::hold_error().into_flash().erase_sector(0x0).unwrap_err();
    assert!(matches!(error, CommandError::HoldPinError(10)))
}

#[test]
fn test_device_erase_sector_full_wp_pin_error() {
    let error = MockedPeripherals::wp_error().into_flash().erase_sector(0x0).unwrap_err();
    assert!(matches!(error, CommandError::WriteProtectionPinError(15)))
}

#[test]
fn test_device_erase_sector_busy() {
    let error = MockedPeripherals::default()
        .mock_configure()
        .mock_enable_pin()
        .expect_write_enable_command()
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0001])
        .into_flash()
        .erase_sector(0x0)
        .unwrap_err();

    assert!(matches!(error, CommandError::Busy))
}

#[test]
fn test_device_erase_sector_transfer_error() {
    let error = MockedPeripherals::default()
        .mock_configure()
        .mock_enable_pin()
        .expect_write_enable_command()
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0010])
        .mock_enable_pin()
        .spi_transfer_error()
        .into_flash()
        .erase_sector(0x0)
        .unwrap_err();

    assert!(matches!(error, CommandError::TransferError(30)))
}

#[test]
fn test_device_erase_sector_blocking() {
    MockedPeripherals::default()
        .mock_configure()
        .mock_enable_pin()
        .expect_write_enable_command()
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0010])
        .mock_enable_pin()
        .expect_transfer(&[0x20, 0x00, 0x80, 0x00], &[0x0])
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0001]) // Still busy
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0001]) // Still busy
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0000])
        .into_flash()
        .erase_sector(0x8000)
        .unwrap();
}

#[test]
fn test_device_erase_sector_non_blocking() {
    let mut flash = MockedPeripherals::default()
        .mock_configure()
        .mock_enable_pin()
        .expect_write_enable_command()
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0010])
        .mock_enable_pin()
        .expect_transfer(&[0x20, 0x00, 0x10, 0x00], &[0x0])
        .into_flash();

    flash.set_non_blocking();
    flash.erase_sector(0x1000).unwrap();
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
fn test_device_program_byte_hold_error() {
    let error = MockedPeripherals::hold_error().into_flash().byte_program(0x0, 0x0).unwrap_err();
    assert!(matches!(error, CommandError::HoldPinError(10)))
}

#[test]
fn test_device_program_byte_wp_pin_error() {
    let error = MockedPeripherals::wp_error().into_flash().byte_program(0x0, 0x0).unwrap_err();
    assert!(matches!(error, CommandError::WriteProtectionPinError(15)))
}

#[test]
fn test_device_program_byte_address_error() {
    let error = MockedPeripherals::default()
        .into_flash()
        .byte_program(16777217, 0x0)
        .unwrap_err();
    assert!(matches!(error, CommandError::InvalidAddress))
}

#[test]
fn test_device_byte_program_transfer_error() {
    let mut flash = MockedPeripherals::default()
        .mock_configure()
        .mock_enable_pin()
        .expect_write_enable_command()
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0010])
        .mock_enable_pin()
        .spi_transfer_error()
        .into_flash();

    flash.set_non_blocking();
    let error = flash.byte_program(0xdbba0, 0x96).unwrap_err();
    assert!(matches!(error, CommandError::TransferError(30)))
}

#[test]
fn test_device_byte_program_non_blocking() {
    let mut flash = MockedPeripherals::default()
        .mock_configure()
        .mock_enable_pin()
        .expect_write_enable_command()
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0010])
        .mock_enable_pin()
        .expect_transfer(
            &[0b0000_0010, 0b0000_1101, 0b1011_1011, 0b1010_0000, 0x96],
            &[0x0, 0x0, 0x0, 0x0, 0x0],
        )
        .into_flash();

    flash.set_non_blocking();
    flash.byte_program(0xdbba0, 0x96).unwrap();
}

#[test]
fn test_device_byte_program_blocking() {
    let mut flash = MockedPeripherals::default()
        .mock_configure()
        .mock_enable_pin()
        .expect_write_enable_command()
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0010])
        .mock_enable_pin()
        .expect_transfer(
            &[0b0000_0010, 0b0000_0000, 0b1001_1100, 0b0100_0000, 0x66],
            &[0x0, 0x0, 0x0, 0x0, 0x0],
        )
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0001]) // Still busy
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0001]) // Still busy
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0000])
        .into_flash();

    flash.set_blocking();
    flash.byte_program(0x9c40, 0x66).unwrap();
}

#[test]
fn test_device_write_status_hold_error() {
    let error = MockedPeripherals::hold_error()
        .into_flash()
        .write_status(Status::default())
        .unwrap_err();
    assert!(matches!(error, CommandError::HoldPinError(10)))
}

#[test]
fn test_device_write_status_wp_pin_error() {
    let error = MockedPeripherals::wp_error()
        .into_flash()
        .write_status(Status::default())
        .unwrap_err();
    assert!(matches!(error, CommandError::WriteProtectionPinError(15)))
}

#[test]
fn test_device_write_status_transfer_error() {
    let error = MockedPeripherals::default()
        .mock_configure()
        .mock_enable_pin()
        .expect_write_enable_command()
        .spi_transfer_error()
        .into_flash()
        .write_status(Status::default())
        .unwrap_err();

    assert!(matches!(error, CommandError::TransferError(30)))
}

#[test]
fn test_device_write_status_success() {
    let status = Status {
        block0_protected: true,
        block3_protected: true,
        ..Default::default()
    };

    MockedPeripherals::default()
        .mock_configure()
        .mock_enable_pin()
        .expect_write_enable_command()
        .mock_enable_pin()
        .expect_transfer(&[0x0], &[0x0])
        .expect_transfer(&[0b0000_0001, 0b0010_0100], &[0x0, 0x0])
        .into_flash()
        .write_status(status)
        .unwrap();
}

#[test]
fn test_device_read_hold_error() {
    let error = MockedPeripherals::hold_error().into_flash().read::<1>(0x0).unwrap_err();
    assert!(matches!(error, CommandError::HoldPinError(10)))
}

#[test]
fn test_device_read_wp_pin_error() {
    let error = MockedPeripherals::wp_error().into_flash().read::<1>(0x0).unwrap_err();
    assert!(matches!(error, CommandError::WriteProtectionPinError(15)))
}

#[test]
fn test_device_read_address_error() {
    let error = MockedPeripherals::default().into_flash().read::<1>(16777217).unwrap_err();
    assert!(matches!(error, CommandError::InvalidAddress))
}

#[test]
fn test_device_read_transfer_error_command() {
    let error = MockedPeripherals::default()
        .mock_configure()
        .mock_enable_pin()
        .spi_transfer_error()
        .into_flash()
        .read::<1>(0x0)
        .unwrap_err();

    assert!(matches!(error, CommandError::TransferError(30)))
}

#[test]
fn test_device_read_transfer_error_data() {
    let error = MockedPeripherals::default()
        .mock_configure()
        .mock_enable_pin()
        .expect_transfer(&[0b0000_0011, 0x0, 0x0, 0x0], &[0x0, 0x0, 0x0, 0x0])
        .spi_transfer_error()
        .into_flash()
        .read::<1>(0x0)
        .unwrap_err();

    assert!(matches!(error, CommandError::TransferError(30)))
}

#[test]
fn test_device_read_success() {
    let result = MockedPeripherals::default()
        .mock_configure()
        .mock_enable_pin()
        .expect_transfer(
            &[0b0000_0011, 0b0000_0110, 0b0001_1010, 0b1000_0000],
            &[0x0, 0x0, 0x0, 0x0],
        )
        .expect_transfer(&[0x0, 0x0], &[0x47, 0x20])
        .into_flash()
        .read::<2>(0x61A80)
        .unwrap();

    assert_eq!([0x47, 0x20], result)
}

#[test]
fn test_device_aai_program_hold_error() {
    let error = MockedPeripherals::hold_error()
        .into_flash()
        .aai_program(0x0, &[0x0, 0x0])
        .unwrap_err();
    assert!(matches!(error, CommandError::HoldPinError(10)))
}

#[test]
fn test_device_aai_program_wp_pin_error() {
    let error = MockedPeripherals::wp_error()
        .into_flash()
        .aai_program(0x0, &[0x0, 0x0])
        .unwrap_err();
    assert!(matches!(error, CommandError::WriteProtectionPinError(15)))
}

#[test]
fn test_device_aai_program_address_error() {
    let error = MockedPeripherals::default()
        .into_flash()
        .aai_program(16777217, &[0x0, 0x0])
        .unwrap_err();
    assert!(matches!(error, CommandError::InvalidAddress))
}

#[test]
fn test_device_aai_program_buffer_too_small_error() {
    let error = MockedPeripherals::default().into_flash().aai_program(0x0, &[0x0]).unwrap_err();
    assert!(matches!(error, CommandError::BufferTooSmall))
}

#[test]
fn test_device_aai_program_buffer_uneven_error() {
    let error = MockedPeripherals::default()
        .into_flash()
        .aai_program(0x0, &[0x0, 0x0, 0x0])
        .unwrap_err();
    assert!(matches!(error, CommandError::BufferUneven))
}

#[test]
fn test_device_aai_program_busy_error() {
    let error = MockedPeripherals::default()
        .mock_configure()
        .mock_enable_pin()
        .expect_write_enable_command()
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0001])
        .into_flash()
        .aai_program(0x0, &[0x0, 0x0])
        .unwrap_err();
    assert!(matches!(error, CommandError::Busy))
}

#[test]
fn test_device_aai_program_transfer_error() {
    let error = MockedPeripherals::default()
        .mock_configure()
        .mock_enable_pin()
        .expect_write_enable_command()
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0000])
        .mock_enable_pin()
        .spi_transfer_error()
        .into_flash()
        .aai_program(0x0, &[0x0, 0x0])
        .unwrap_err();
    assert!(matches!(error, CommandError::TransferError(30)))
}

#[test]
fn test_device_aai_program_two_bytes() {
    let mut flash = MockedPeripherals::default()
        .mock_configure()
        .mock_enable_pin()
        .expect_write_enable_command()
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0000])
        .mock_enable_pin()
        .expect_transfer(
            &[0b1010_1101, 0b0000_0111, 0b1010_0001, 0b0010_0000, 0x96, 0x64],
            &[0x0; 6],
        )
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0001]) // Still busy
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0000]) // Not busy anymore
        .mock_enable_pin()
        .expect_write_disable_command()
        .into_flash();

    flash.set_non_blocking();
    flash.aai_program(0x7A120, &[0x96, 0x64]).unwrap();
}

#[test]
fn test_device_aai_program_six_bytes() {
    let mut flash = MockedPeripherals::default()
        .mock_configure()
        .mock_enable_pin()
        .expect_write_enable_command()
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0000])
        .mock_enable_pin()
        .expect_transfer(
            &[0b1010_1101, 0b0000_0111, 0b1010_0001, 0b0010_0000, 0x96, 0x64],
            &[0x0; 6],
        )
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0001]) // Still busy
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0000])
        .mock_enable_pin()
        .expect_transfer(&[0b1010_1101, 0x44, 0x55], &[0x0; 3])
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0000]) // Not busy anymore
        .mock_enable_pin()
        .expect_transfer(&[0b1010_1101, 0x66, 0x77], &[0x0; 3])
        .mock_enable_pin()
        .expect_status_request(&[0x0, 0b0000_0000]) // not busy
        .mock_enable_pin()
        .expect_write_disable_command()
        .into_flash();

    flash.set_non_blocking();
    flash.aai_program(0x7A120, &[0x96, 0x64, 0x44, 0x55, 0x66, 0x77]).unwrap();
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

#[test]
fn test_status_to_registers() {
    let status = Status {
        // Assert read-only bits are ignored
        busy: true,
        block0_protected: true,
        block1_protected: true,
        block2_protected: false,
        block3_protected: true,
        bits_read_only: true,
        ..Default::default()
    };
    assert_eq!(0b1010_1100, status.to_registers());

    let status = Status {
        // Assert read-only bits are ignored
        aai_programming_mode: true,
        block1_protected: true,
        block2_protected: true,
        ..Default::default()
    };
    assert_eq!(0b0001_1000, status.to_registers());
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
    pub fn expect_write_enable_command(self) -> Self {
        self.expect_transfer(&[0b0000_0110], &[0x0])
    }

    /// Expects a correct write-disable command
    pub fn expect_write_disable_command(self) -> Self {
        self.expect_transfer(&[0b0000_0100], &[0x0])
    }

    /// Expects a status command request and returns the given raw response
    pub fn expect_status_request(self, response: &'static [u8]) -> Self {
        self.expect_transfer(&[0b0000_0101, 0x0], response)
    }

    /// Expects a full chip erase command
    pub fn expect_full_erase(self) -> Self {
        self.expect_transfer(&[0b0110_0000], &[0x0])
    }

    /// Expects a generic command
    pub fn expect_transfer(mut self, command: &'static [u8], response: &'static [u8]) -> Self {
        self.bus.expect_transfer().times(1).returning(move |data: &mut [u8]| {
            assert_eq!(command, data);
            Ok(response)
        });

        self
    }
}
