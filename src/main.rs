fn main() {
    let api = hidapi::HidApi::new().unwrap();
    for device in api.device_list() {
        println!("{:#?}", device);
    }

    // let (VID, PID) = (0x1200, 0x038F);
    // let device = api.open(VID, PID).unwrap();
    //
    // let mut buf = [0u8; 8];
    // let res = device.read(&mut buf[..]).unwrap();
    // println!("Read: {:?}", &buf[..res]);
    //
    // let buf = [0u8, 1, 2, 3, 4];
    // let res = device.write(&buf).unwrap();
    // println!("Wrote: {:?} byte(s)", res);
}

