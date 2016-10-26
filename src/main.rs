extern crate libusb;
use std::time::Duration;

fn main() {
    // TODO: parse arguments

    let context = libusb::Context::new().unwrap();
    let brightness = 1;

    let mut found = false;
    for device in context.devices().unwrap().iter() { // TODO: handle error
        let device_desc = device.device_descriptor().unwrap(); // TODO: handle error
        if device_desc.vendor_id() == 0x046d && device_desc.product_id() == 0xc333 {
	    println!("Found G610 Device.");
            found = true;
            match device.open() {
                Ok(mut handle) => adjust_background_lighting(&mut handle, brightness),
                Err(e) => println!("Error {:?}", e)
                // TODO: handle error
            }
        }
    }
    if !found {
        println!("No G610 Device found.");
    }
}

fn adjust_background_lighting(handle: &mut libusb::DeviceHandle, brightness: usize) {
    let interface : u8 = 0x1;
    let detach_result = handle.detach_kernel_driver(interface);
    println!("{:?}", detach_result);

    let claim_result = handle.claim_interface(interface);
    println!("{:?}", claim_result);

    set_backlight_brightness(handle, brightness);

    let release_result = handle.release_interface(interface);
    println!("{:?}", release_result);
    let attach_result = handle.attach_kernel_driver(interface);
    println!("{:?}", attach_result);
}

fn set_backlight_brightness(handle: &mut libusb::DeviceHandle, brightness: usize) {
    let write_result = handle.write_control(0x21,0x09,0x0211,1, command_for_logo_brightness(0).as_slice(), Duration::from_millis(100));
    println!("{:?}", write_result);
    let write_result = handle.write_control(0x21,0x09,0x0211,1, command_for_brightness(brightness).as_slice(), Duration::from_millis(100));
    println!("{:?}", write_result);
}

fn command_for_brightness(brightness:usize) -> Vec<u8> {
    let prefix = "11ff0d3b0001";
    return generate_command(prefix, brightness);
}

fn command_for_logo_brightness(brightness:usize) -> Vec<u8> {
    let prefix = "11ff0d3b0101";
    return generate_command(prefix, brightness);
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
