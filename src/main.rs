extern crate libusb;

use std::result::Result;
use std::time::Duration;
use std::thread;

fn main() {
    // TODO: parse arguments

    let brightness = 1;
    let result = set_background_brightness(brightness);
    print!("{:?}", result);
}

fn set_background_brightness(brightness:usize) -> Result<(), libusb::Error> {
    let context = try!(libusb::Context::new());
    let devices = try!(context.devices());
    for device in devices.iter() {
        let device_desc = try!(device.device_descriptor());
        if device_desc.vendor_id() == 0x046d && device_desc.product_id() == 0xc333 {
            let mut handle = try!(device.open());
            return adjust_background_lighting(&mut handle, brightness);
        }
    }
    return Err(libusb::Error::NoDevice);
}

fn adjust_background_lighting(handle: &mut libusb::DeviceHandle, brightness: usize) -> Result<(), libusb::Error> {
    let interface : u8 = 0x1;
    try!(handle.detach_kernel_driver(interface));
    try!(handle.claim_interface(interface));

    try!(set_backlight_brightness(handle, brightness));

    try!(handle.release_interface(interface));
    try!(handle.attach_kernel_driver(interface));
    return Ok(())
}

fn set_backlight_brightness(handle: &mut libusb::DeviceHandle, brightness: usize) -> Result<(), libusb::Error> {
    try!(handle.write_control(0x21,0x09,0x0211,1, command_for_brightness(brightness).as_slice(), Duration::from_millis(50)));
    thread::sleep(Duration::from_millis(100));
    try!(handle.write_control(0x21,0x09,0x0211,1, command_for_logo_brightness(brightness).as_slice(), Duration::from_millis(50)));
    return Ok(());
}

fn command_for_brightness(brightness:usize) -> Vec<u8> {
    return generate_command("11ff0d3b0001", brightness);
}

fn command_for_logo_brightness(brightness:usize) -> Vec<u8> {
    return generate_command("11ff0d3b0101", brightness);
}

fn generate_command(prefix: &str, brightness:usize) -> Vec<u8> {
    let postfix = "0200000000000000000000";
    return match brightness {
        0 => hex_string_to_byte_array([ prefix, "000000", postfix ].concat()),
        _ => hex_string_to_byte_array([ prefix, "ffffff", postfix ].concat()),
    }
}

fn hex_string_to_byte_array(hex_string: String) -> Vec<u8>{
    let mut z = hex_string.chars().peekable();
    let mut return_value = Vec::new();
    while z.peek().is_some() {
        let chunk: String = z.by_ref().take(2).collect();
        return_value.push(u8::from_str_radix(&chunk, 16).unwrap());
    }
    return return_value;
}
