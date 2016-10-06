extern crate libusb;
use std::time::Duration;

fn main() {
    let context = libusb::Context::new().unwrap();

    for device in context.devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();
        if device_desc.vendor_id() == 0x046d && device_desc.product_id() == 0xc333 {
	    println!("Found G610 Device.");
            match device.open() {
                Ok(mut handle) => adjust_background_lighting(&mut handle),
                Err(e) => println!("Error {:?}", e)
            }
        }
    }
}

fn adjust_background_lighting(handle: &mut libusb::DeviceHandle){
    let interface :u8 = 0x1;
    handle.detach_kernel_driver(interface);
    handle.claim_interface(interface);
    println!("Kernel driver is loaded: {:?}", handle.kernel_driver_active(interface));

    let prefix = "11ff0d3b0001";
    let postfix = "0200000000000000000000";
    let no_backlight = [ prefix, "000000", postfix ].concat();
    let full_backlight = [ prefix, "ffffff", postfix ].concat();
    let command = hex_string_to_byte_array(full_backlight);
    let write_result = handle.write_control(0x21,0x09,0x0211,1, command.as_slice(), Duration::from_millis(100));
    println!("{:?}", write_result);
    handle.release_interface(interface);
    handle.attach_kernel_driver(interface);
    println!("Kernel driver is loaded: {:?}", handle.kernel_driver_active(interface));
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
