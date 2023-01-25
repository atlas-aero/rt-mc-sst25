use crate::device::{Flash, ReadError, Status};
use crate::mocks::{MockPin, MockSPIBus};

#[test]
fn test_device_read_status_success() {
    let pin_wp = MockPin::new();
    let mut pin_en = MockPin::new();
    pin_en.expect_set_low().times(1).return_const(Ok(()));
    pin_en.expect_set_high().times(1).return_const(Ok(()));

    let mut pin_hold = MockPin::new();
    pin_hold.expect_set_high().times(1).return_const(Ok(()));

    let mut bus = MockSPIBus::new();
    bus.expect_transfer().times(1).returning(move |data: &mut [u8]| {
        assert_eq!(0b0000_0101, data[0]);
        assert_eq!(0x0, data[1]);

        Ok(&[0x0, 0x1C])
    });

    let mut device = Flash::new(bus, pin_en, pin_wp, pin_hold);
    let status = device.read_status().unwrap();

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
    let pin_wp = MockPin::new();
    let pin_en = MockPin::new();

    let mut pin_hold = MockPin::new();
    pin_hold.expect_set_high().times(1).return_const(Err(10));

    let bus = MockSPIBus::new();

    let mut device = Flash::new(bus, pin_en, pin_wp, pin_hold);
    let error = device.read_status().unwrap_err();

    assert!(matches!(error, ReadError::HoldPinError(10)))
}

#[test]
fn test_device_read_status_enable_error() {
    let pin_wp = MockPin::new();
    let mut pin_en = MockPin::new();
    pin_en.expect_set_low().times(1).return_const(Err(20));

    let mut pin_hold = MockPin::new();
    pin_hold.expect_set_high().times(1).return_const(Ok(()));

    let bus = MockSPIBus::new();

    let mut device = Flash::new(bus, pin_en, pin_wp, pin_hold);
    let error = device.read_status().unwrap_err();

    assert!(matches!(error, ReadError::EnablePinError(20)))
}

#[test]
fn test_device_read_status_transfer_error() {
    let pin_wp = MockPin::new();
    let mut pin_en = MockPin::new();
    pin_en.expect_set_low().times(1).return_const(Ok(()));

    let mut pin_hold = MockPin::new();
    pin_hold.expect_set_high().times(1).return_const(Ok(()));

    let mut bus = MockSPIBus::new();
    bus.expect_transfer().times(1).return_const(Err(30));

    let mut device = Flash::new(bus, pin_en, pin_wp, pin_hold);
    let error = device.read_status().unwrap_err();

    assert!(matches!(error, ReadError::TransferError(30)))
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
