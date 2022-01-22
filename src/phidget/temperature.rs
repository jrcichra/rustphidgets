use core::mem::MaybeUninit;

use super::{phidget22::{PhidgetHandle, PhidgetTemperatureSensorHandle, PhidgetTemperatureSensor_create, PhidgetTemperatureSensor_getTemperature}, Phidget};

pub struct TemperaturePhidget {
    handle: PhidgetTemperatureSensorHandle,
    base_handle: PhidgetHandle
}

impl TemperaturePhidget {
    pub fn setup(
        hub_port: i32,
        // serial: i32,
        remote: bool,
        max_wait_millis: u32,
    ) -> Result<TemperaturePhidget, u32> {
        let phidget = TemperaturePhidget::create()?;
        phidget.base_handle.set_hub_port(hub_port)?;
        phidget.base_handle.set_is_remote(remote)?;
        // phidget_handle.set_device_serial_number(serial)?;
        phidget.base_handle.open_wait_for_attachment(max_wait_millis)?;
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
                base_handle: temperature_handle as PhidgetHandle
            }),
            _ => Err(rc),
        }
    }

    pub fn close(&self) -> Result<(), u32> {
        self.base_handle.close()
    }

    pub fn get_temperature(&self) -> Result<f64, u32> {
        let mut temperature;
        let rc = unsafe {
            temperature = MaybeUninit::uninit().assume_init();
            PhidgetTemperatureSensor_getTemperature(self.handle, &mut temperature)
        };
        match rc {
            0 => Ok(temperature),
            _ => Err(rc),
        }
    }
}
