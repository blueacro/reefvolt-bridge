use env_logger::Env;
use log::{info};
use anyhow::{self, Context};
use rusb::{Device, GlobalContext};

fn scan_devices() -> anyhow::Result<Vec<Device<GlobalContext>>> {
    let devices_scan = rusb::devices().context("device enumeration")?;

    let devices: Vec<_> = devices_scan.iter().filter(|device| {
        match device.device_descriptor() {
            Ok(descriptor) if descriptor.vendor_id() == 0x726c => true,
            _ => false,
        }
    }).collect();

    info!("discovered devices: {:?}", devices);
    Ok(devices)
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("reefvolt-bridge starting");
    scan_devices().unwrap();

}
