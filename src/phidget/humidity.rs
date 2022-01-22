use super::phidget22::{PhidgetHumiditySensorHandle, PhidgetHandle, PhidgetHumiditySensor_create, PhidgetHumiditySensor_getHumidity};
use core::mem::MaybeUninit;
use crate::phidget::Phidget;

pub struct HumidityPhidget {
    handle: PhidgetHumiditySensorHandle,
    base_handle: PhidgetHandle
}

impl HumidityPhidget {
    pub fn setup(hub_port: i32, remote: bool, max_wait_millis: u32) -> Result<HumidityPhidget, u32> {
        let phidget = HumidityPhidget::create()?;
        phidget.base_handle.set_hub_port(hub_port)?;
        phidget.base_handle.set_is_remote(remote)?;
        phidget.base_handle.open_wait_for_attachment(max_wait_millis)?;
        Ok(phidget)
    }

    pub fn create() -> Result<HumidityPhidget, u32> {
        let mut humidity_handle;
        let rc = unsafe {
            humidity_handle = MaybeUninit::uninit().assume_init();
            PhidgetHumiditySensor_create(&mut humidity_handle)
        };
        match rc {
            0 => Ok(HumidityPhidget { handle: humidity_handle, base_handle: humidity_handle as PhidgetHandle }),
            x => Err(x)
        }
    }

    pub fn close(&self) -> Result<(), u32> {
        self.base_handle.close()
    }

    pub fn get_humidity(&self) -> Result<f64, u32> {
        let mut humidity;
        let rc = unsafe {
            humidity = MaybeUninit::uninit().assume_init();
            PhidgetHumiditySensor_getHumidity(self.handle, &mut humidity)
        };
        match rc {
            0 => Ok(humidity),
            _ => Err(rc),
        }
    }
}
