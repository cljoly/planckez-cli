// Based on https://github.com/littledivy/webusb/blob/c963f3774a32e75fb1e0aea414e1a687b34b425b/examples/blink.rs

use webusb::Context;

fn main() -> Result<(), webusb::Error> {
    let context = Context::init()?;
    let devices = context.devices()?;

    // https://wicg.github.io/webusb/#device-usage

    let mut device = devices
        .into_iter()
        // .find(|d| dbg!(d.vendor_id) == 0x0483 && dbg!(d.product_id) == 0xdf11)
        .find(|d| {
            log::trace!("vendor_id: {}, product_id: {}", d.vendor_id, d.product_id);
            // https://github.com/zsa/wally/blob/623a50d0e0b90486e42ad8ad42b0a7313f7a37b3/wally/usb.go#L16
            // https://github.com/zsa/wally/blob/623a50d0e0b90486e42ad8ad42b0a7313f7a37b3/wally/usb.go#L43
            d.vendor_id == 0x3297 && d.product_id == 0xC6CE
        })
        .expect("Device not found.");
    device.open()?;

    println!("configurations: {:?},configuration: {:?},device_class: {:?},device_subclass: {:?},device_protocol: {:?},device_version_major: {:?},device_version_minor: {:?},device_version_subminor: {:?},manufacturer_name: {:?},product_id: {:?},product_name: {:?},serial_number: {:?},usb_version_major: {:?},usb_version_minor: {:?},usb_version_subminor: {:?},vendor_id: {:?},opened: {:?},url: {:?}",  false,    false,    device.device_class,    device.device_subclass,    device.device_protocol,    device.device_version_major,    device.device_version_minor,    device.device_version_subminor,    device.manufacturer_name,    device.product_id,    device.product_name,    device.serial_number,    device.usb_version_major,    device.usb_version_minor,    device.usb_version_subminor,    device.vendor_id,    device.opened,    device.url,    );

    // Should look into 
    // * https://lib.rs/crates/rusb
    // * https://web.dev/porting-libusb-to-webusb/

    // dbg!(device.configuration.is_some());
    // device.select_configuration(1);

    // device.claim_interface(2)?;
    // device.select_alternate_interface(2, 0)?;

    // device.control_transfer_out(ARDUINO_CONTROL_INIT, &[])?;

    return Ok(());
}
