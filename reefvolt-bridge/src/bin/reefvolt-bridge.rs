use anyhow::{self, Context};
use env_logger::Env;
use log::info;
use rusb::{Device, GlobalContext};

use std::collections::{HashMap, HashSet};
use std::time::Duration;

use reefvolt_bridge::{drivers::dualdoser::DualDoser, Driver};

fn scan_devices() -> anyhow::Result<Vec<Device<GlobalContext>>> {
    let devices_scan = rusb::devices().context("device enumeration")?;

    let devices: Vec<_> = devices_scan
        .iter()
        .filter(|device| match device.device_descriptor() {
            Ok(descriptor)
                if descriptor.vendor_id() == 0x726c && descriptor.product_id() == 0x3101 =>
            {
                true
            }
            _ => false,
        })
        .collect();

    Ok(devices)
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct Address {
    bus_number: u8,
    port_number: u8,
    address: u8,
}

impl<C> From<&Device<C>> for Address
where
    C: rusb::UsbContext,
{
    fn from(d: &Device<C>) -> Self {
        Address {
            bus_number: d.bus_number(),
            port_number: d.port_number(),
            address: d.address(),
        }
    }
}

fn main() {
    let mut drivers: HashMap<Address, Box<dyn Driver>> = HashMap::default();

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("reefvolt-bridge starting");
    loop {
        // Try to map new devices
        let devices = scan_devices().unwrap();

        // Filter out old devices
        let addresses: HashSet<Address> = devices.iter().map(Address::from).collect();
        drivers.retain(|k, _v| addresses.contains(k));

        for device_handle in devices.iter() {
            let address: Address = device_handle.into();
            if drivers.contains_key(&address) {
                continue;
            }

            let device = device_handle.open().unwrap();
            let driver = DualDoser::new(device).unwrap();
            info!("new driver created for {:?}", address);
            drivers.insert(address, Box::new(driver));
        }

        for (_address, driver) in drivers.iter_mut() {
            driver.poll().unwrap();
        }

        std::thread::sleep(Duration::from_secs(1));
    }
}
