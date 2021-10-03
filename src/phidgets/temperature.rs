use super::phidget22;
use core::mem::MaybeUninit;

#[derive(Copy, Clone)]
pub struct PhidgetTemperature {
    handle: phidget22::PhidgetTemperatureSensorHandle,
}

pub trait PhidgetTemperatureTraits {
    fn new() -> PhidgetTemperature;
    fn get_temperature(self) -> f64;
}

impl PhidgetTemperatureTraits for PhidgetTemperature {
    fn new() -> PhidgetTemperature {
        let mut temphandle: phidget22::PhidgetTemperatureSensorHandle;
        unsafe {
            temphandle = MaybeUninit::uninit().assume_init();
            phidget22::PhidgetTemperatureSensor_create(&mut temphandle);
        }
        return PhidgetTemperature { handle: temphandle };
    }
    fn get_temperature(self) -> f64 {
        let mut temp = 0.0_f64;
        let temp_ptr: *mut f64 = &mut temp;
        unsafe {
            phidget22::PhidgetTemperatureSensor_getTemperature(self.handle, temp_ptr);
        }
        return temp;
    }
}
