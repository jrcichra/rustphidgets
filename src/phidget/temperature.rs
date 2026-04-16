use core::mem::MaybeUninit;

use super::{
    phidget22::{
        PhidgetHandle, PhidgetTemperatureSensorHandle, PhidgetTemperatureSensor_create,
        PhidgetTemperatureSensor_delete, PhidgetTemperatureSensor_getTemperature,
    },
    Phidget,
};

pub struct TemperaturePhidget {
    handle: PhidgetTemperatureSensorHandle,
    base_handle: PhidgetHandle,
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
        phidget
            .base_handle
            .open_wait_for_attachment(max_wait_millis)?;
        Ok(phidget)
    }

    pub fn create() -> Result<TemperaturePhidget, u32> {
        let mut temperature_handle = MaybeUninit::uninit();
        let rc = unsafe { PhidgetTemperatureSensor_create(temperature_handle.as_mut_ptr()) };
        match rc {
            0 => unsafe {
                Ok(TemperaturePhidget {
                    handle: temperature_handle.assume_init(),
                    base_handle: temperature_handle.assume_init() as PhidgetHandle,
                })
            },
            _ => Err(rc),
        }
    }

    pub fn close(&self) -> Result<(), u32> {
        self.base_handle.close()
    }

    pub fn get_temperature(&self) -> Result<f64, u32> {
        let mut temperature = MaybeUninit::uninit();
        let rc = unsafe {
            PhidgetTemperatureSensor_getTemperature(self.handle, temperature.as_mut_ptr())
        };
        match rc {
            0 => Ok(unsafe { temperature.assume_init() }),
            _ => Err(rc),
        }
    }
}

impl Drop for TemperaturePhidget {
    fn drop(&mut self) {
        unsafe {
            PhidgetTemperatureSensor_delete(&mut self.handle);
        }
    }
}
