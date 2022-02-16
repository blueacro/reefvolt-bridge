use anyhow::Context;
use std::time::Duration;


pub fn main()  -> anyhow::Result<()> {
    let devices_scan = rusb::devices().context("device enumeration")?;

    let devices: Vec<_> = devices_scan.iter().filter(|device| {
        match device.device_descriptor() {
            Ok(descriptor) if descriptor.vendor_id() == 0x726c => true,
            _ => false,
        }
    }).collect();

    println!("discovered devices: {:?}", devices);

    for device in devices {

        let mut open = device.open()?;
        
        //open.set_active_configuration(1)?;
        open.claim_interface(0)?;
        open.write_bulk(2, vec![1 as u8].as_ref(), Duration::from_millis(50))?;
        println!("wrote to endpoint");
        let mut buf = vec![0 as u8; 64];
        open.read_bulk(0x81, &mut buf, Duration::from_millis(500))?;
        println!("{:?}", buf);
        //open.release_interface(0)?;
    }

    Ok(())
}
