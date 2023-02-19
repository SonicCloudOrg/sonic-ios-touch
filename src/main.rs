use bluetooth_serial_port::{BtProtocol, BtSocket};
use std::io::{Read, Write};
use mio::{Poll, PollOpt, Token, Ready};

fn main() {
    // scan for devices
    let devices = bluetooth_serial_port::scan_devices().unwrap();
    if devices.len() == 0 { panic!("No devices found"); }

    // "device.addr" is the MAC address of the device
    let device = &devices[0];
    println!("Connecting to `{}` ({})", device.name, device.addr.to_string());

}