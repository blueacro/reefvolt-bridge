use packed_struct::{PackedStruct, PackedStructSlice, PrimitiveEnum};
use rusb::DeviceHandle;

use crate::structproto::dualdoser::{CommandRead, Response, ResponseStatus};
use crate::{Driver, DriverError};

use std::time::Duration;

use log::info;

const EP_OUT: u8 = 2;
const EP_IN: u8 = 0x81;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("usb error")]
    UsbError(rusb::Error),
    #[error("unknown response error")]
    UnknownResponseError,
    #[error("protocol error")]
    ProtocolError(packed_struct::PackingError),
}

pub struct DualDoser<C: rusb::UsbContext> {
    handle: DeviceHandle<C>,
    serial: String,
}

impl<C> DualDoser<C>
where
    C: rusb::UsbContext,
{
    pub fn new(mut device: DeviceHandle<C>) -> Result<Self, Error> {
        device.reset().map_err(Error::UsbError)?;
        device.claim_interface(0).map_err(Error::UsbError)?;
        let descriptor = device
            .device()
            .device_descriptor()
            .map_err(Error::UsbError)?;
        let serial = device
            .read_serial_number_string_ascii(&descriptor)
            .map_err(Error::UsbError)?;
        Ok(Self {
            handle: device,
            serial,
        })
    }

    fn read_status(&mut self) -> Result<ResponseStatus, Error> {
        let mut buf = [0_u8; 64];
        let command = CommandRead::default()
            .pack()
            .map_err(Error::ProtocolError)?;
        let _b = self
            .handle
            .write_bulk(EP_OUT, &command, Duration::from_millis(100))
            .unwrap();
        let _result = self
            .handle
            .read_bulk(EP_IN, &mut buf, Duration::from_millis(100))
            .map_err(Error::UsbError)?;
        match Response::from_primitive(buf[0]) {
            Some(Response::Status) => {
                ResponseStatus::unpack_from_slice(&buf[1..6]).map_err(Error::ProtocolError)
            }
            _ => Err(Error::UnknownResponseError),
        }
    }
}

impl<C> Driver for DualDoser<C>
where
    C: rusb::UsbContext,
{
    fn poll(&mut self) -> Result<(), DriverError> {
        info!("polling dualdoser {}", self.serial);
        let status = self
            .read_status()
            .map_err(|e| e.into())
            .map_err(DriverError::TransientError)?;
        info!("status: {:?}", status);
        Ok(())
    }
}
