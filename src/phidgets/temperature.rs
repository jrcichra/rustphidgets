use super::phidget22;
use super::phidget22::*;
use crate::phidgets::PhidgetTraits;

use core::mem::MaybeUninit;
pub struct TemperaturePhidget {
    handle: phidget22::PhidgetTemperatureSensorHandle,
}

impl TemperaturePhidget {
    pub fn setup(
        hub_port: i32,
        // serial: i32,
        remote: bool,
        max_wait_millis: u32,
    ) -> Result<TemperaturePhidget, u32> {
        let phidget = TemperaturePhidget::create()?;
        let phidget_handle = phidget.handle as PhidgetHandle;
        phidget_handle.set_hub_port(hub_port)?;
        phidget_handle.set_is_remote(remote)?;
        // phidget_handle.set_device_serial_number(serial)?;
        phidget_handle.open_wait_for_attachment(max_wait_millis)?;
        Ok(phidget)
    }

    pub fn create() -> Result<TemperaturePhidget, u32> {
        let mut temperature_handle;
        let rc = unsafe {
            temperature_handle = MaybeUninit::uninit().assume_init();
            PhidgetTemperatureSensor_create(&mut temperature_handle)
        };
        match rc {
            0 => Ok(TemperaturePhidget {
                handle: temperature_handle,
            }),
            _ => Err(rc),
        }
    }

    pub fn get_temperature(&self) -> Result<f64, u32> {
        let mut temperature;
        let rc = unsafe {
            temperature = MaybeUninit::uninit().assume_init();
            phidget22::PhidgetTemperatureSensor_getTemperature(self.handle, &mut temperature)
        };
        match rc {
            0 => Ok(temperature),
            _ => Err(rc),
        }
    }
}
