use std::{io, iter};
use io_bluetooth::bt;
use io_bluetooth::bt::BtStream;

fn main() -> std::io::Result<()> {
    list_devices()
}

pub fn list_devices() -> io::Result<()> {
    println!("Scanning...");
    let devices = bt::discover_devices()?;
    println!("Devices:");
    for (idx, device) in devices.iter().enumerate() {
        println!("{}: {}", idx, *device);
    }

    if devices.len() == 0 {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No devices found.",
        ));
    }

    let device_idx = request_device_idx(devices.len())?;

    let socket = BtStream::connect(iter::once(&devices[device_idx]), bt::BtProtocol::RFCOMM)?;

    match socket.peer_addr() {
        Ok(name) => println!("Peername: {}.", name.to_string()),
        Err(err) => println!("An error occured while retrieving the peername: {:?}", err),
    }

    match socket.local_addr() {
        Ok(name) => println!("Socket name: {}", name.to_string()),
        Err(err) => println!("An error occured while retrieving the sockname: {:?}", err),
    }

    let mut buffer = vec![0; 1024];
    loop {
        match socket.recv(&mut buffer[..]) {
            Ok(len) => println!("Received {} bytes.", len),
            Err(err) => return Err(err),
        }
    }
}

fn request_device_idx(len: usize) -> io::Result<usize> {
    println!("Please specify the index of the device you want to connect to:");

    let mut buffer = String::new();
    loop {
        io::stdin().read_line(&mut buffer)?;
        if let Ok(idx) = buffer.trim_end().parse::<usize>() {
            if idx < len {
                return Ok(idx);
            }
        }
        buffer.clear();
        println!("Invalid index. Please try again.");
    }
}

