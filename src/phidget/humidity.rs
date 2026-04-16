use super::phidget22::{
    PhidgetHandle, PhidgetHumiditySensorHandle, PhidgetHumiditySensor_create,
    PhidgetHumiditySensor_delete, PhidgetHumiditySensor_getHumidity,
};
use crate::phidget::Phidget;
use core::mem::MaybeUninit;

pub struct HumidityPhidget {
    handle: PhidgetHumiditySensorHandle,
    base_handle: PhidgetHandle,
}

impl HumidityPhidget {
    pub fn setup(
        hub_port: i32,
        remote: bool,
        max_wait_millis: u32,
    ) -> Result<HumidityPhidget, u32> {
        let phidget = HumidityPhidget::create()?;
        phidget.base_handle.set_hub_port(hub_port)?;
        phidget.base_handle.set_is_remote(remote)?;
        phidget
            .base_handle
            .open_wait_for_attachment(max_wait_millis)?;
        Ok(phidget)
    }

    pub fn create() -> Result<HumidityPhidget, u32> {
        let mut humidity_handle = MaybeUninit::uninit();
        let rc = unsafe { PhidgetHumiditySensor_create(humidity_handle.as_mut_ptr()) };
        match rc {
            0 => unsafe {
                Ok(HumidityPhidget {
                    handle: humidity_handle.assume_init(),
                    base_handle: humidity_handle.assume_init() as PhidgetHandle,
                })
            },
            x => Err(x),
        }
    }

    pub fn close(&self) -> Result<(), u32> {
        self.base_handle.close()
    }

    pub fn get_humidity(&self) -> Result<f64, u32> {
        let mut humidity = MaybeUninit::uninit();
        let rc = unsafe { PhidgetHumiditySensor_getHumidity(self.handle, humidity.as_mut_ptr()) };
        match rc {
            0 => Ok(unsafe { humidity.assume_init() }),
            _ => Err(rc),
        }
    }
}

impl Drop for HumidityPhidget {
    fn drop(&mut self) {
        unsafe {
            PhidgetHumiditySensor_delete(&mut self.handle);
        }
    }
}
