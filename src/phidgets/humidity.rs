use super::phidget22;
use super::phidget22::*;
use crate::phidgets::PhidgetTraits;

use core::mem::MaybeUninit;
pub struct HumidityPhidget {
    handle: phidget22::PhidgetHumiditySensorHandle,
}

impl HumidityPhidget {
    pub fn setup(
        hub_port: i32,
        remote: bool,
        max_wait_millis: u32,
    ) -> Result<HumidityPhidget, u32> {
        let phidget = HumidityPhidget::create();
        let phidget_handle = phidget.handle as PhidgetHandle;
        phidget_handle.set_hub_port(hub_port)?;
        phidget_handle.set_is_remote(remote)?;
        phidget_handle.open_wait_for_attachment(max_wait_millis)?;
        Ok(phidget)
    }

    pub fn create() -> HumidityPhidget {
        unsafe {
            let mut humidity_handle = MaybeUninit::uninit().assume_init();
            PhidgetHumiditySensor_create(&mut humidity_handle);
            HumidityPhidget {
                handle: humidity_handle,
            }
        }
    }

    pub fn get_humidity(&self) -> Result<f64, u32> {
        let rc;
        let mut humidity;
        unsafe {
            humidity = MaybeUninit::uninit().assume_init();
            rc = phidget22::PhidgetHumiditySensor_getHumidity(self.handle, &mut humidity);
        }
        match rc {
            0 => Ok(humidity),
            _ => Err(rc),
        }
    }
}
