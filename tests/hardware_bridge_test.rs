use pakem::prelude::*;

#[test]
fn test_escpos_thermal_printer_builder() {
    let mut builder = EscPosBuilder::new();
    builder
        .align_center()
        .bold(true)
        .double_size(true)
        .line("TOKO ROTI LUMINA")
        .bold(false)
        .double_size(false)
        .line("Jl. Jenderal Sudirman No. 45")
        .separator(32)
        .align_left()
        .two_columns("Roti Coklat x2", "Rp 24.000", 32)
        .two_columns("Kopi Susu x1", "Rp 18.000", 32)
        .separator(32)
        .bold(true)
        .two_columns("TOTAL", "Rp 42.000", 32)
        .bold(false)
        .feed(3)
        .cut()
        .open_drawer();

    let bytes = builder.clone().build();
    assert!(!bytes.is_empty());

    // Check ESC @ (Initialize) = [0x1B, 0x40]
    assert_eq!(bytes[0], 0x1B);
    assert_eq!(bytes[1], 0x40);

    // Check text appears in buffer
    let text_repr = String::from_utf8_lossy(&bytes);
    assert!(text_repr.contains("TOKO ROTI LUMINA"));
    assert!(text_repr.contains("Roti Coklat"));
    assert!(text_repr.contains("Rp 42.000"));

    // Check Base64 serialization
    let b64 = builder.to_base64();
    assert!(!b64.is_empty());
    assert!(b64.len() > 50);
}

#[test]
fn test_barcode_scanner_event_and_ean13_validation() {
    // Valid EAN-13 example: 9780201379624
    let valid_event = BarcodeScanEvent::new("9780201379624").with_format("EAN_13");
    assert!(valid_event.is_valid_ean13());
    assert_eq!(valid_event.format.as_deref(), Some("EAN_13"));

    // Invalid EAN-13 (bad check digit or length)
    let invalid_event = BarcodeScanEvent::new("9780201379629");
    assert!(!invalid_event.is_valid_ean13());

    let short_event = BarcodeScanEvent::new("12345");
    assert!(!short_event.is_valid_ean13());
}

#[test]
fn test_fingerprint_template_quality() {
    let good_fp = FingerprintTemplate::new("SecuGen-Hamster-Pro", "dGVtcGxhdGVfZGF0YQ==", 85);
    assert!(good_fp.is_usable());

    let low_quality_fp = FingerprintTemplate::new("Generic-Sensor", "c29tZV9kYXRh", 35);
    assert!(!low_quality_fp.is_usable());

    let empty_fp = FingerprintTemplate::new("SecuGen", "", 90);
    assert!(!empty_fp.is_usable());
}

#[test]
fn test_hardware_bridge_device_message_serialization() {
    let msg = DeviceMessage::PrintRaw {
        device: "thermal_printer_01".to_string(),
        data_base64: "G0BMT0tP".to_string(),
    };

    let serialized = serde_json::to_string(&msg).expect("Serialize DeviceMessage");
    assert!(serialized.contains("PrintRaw"));
    assert!(serialized.contains("thermal_printer_01"));

    let deserialized: DeviceMessage = serde_json::from_str(&serialized).expect("Deserialize DeviceMessage");
    assert_eq!(deserialized, msg);

    let resp = DeviceResponse::Success {
        device: "thermal_printer_01".to_string(),
        message: "Receipt printed successfully".to_string(),
        data: None,
    };

    let resp_json = serde_json::to_string(&resp).expect("Serialize DeviceResponse");
    assert!(resp_json.contains("Success"));
    assert!(resp_json.contains("Receipt printed successfully"));
}

#[test]
fn test_device_bridge_script_generation() {
    let script = device_bridge_script();
    assert!(script.contains("window.LuminaDevice"));
    assert!(script.contains("navigator.usb.requestDevice"));
    assert!(script.contains("navigator.serial.requestPort"));
    assert!(script.contains("connectServerBridge"));
}
