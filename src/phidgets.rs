mod helpers;
pub mod humidity;
pub mod lcd;
pub mod network;
mod phidget22;
pub mod temperature;
pub use helpers::celcius_to_fahrenheit;
pub use humidity::HumidityPhidget;
pub use lcd::LCDPhidget;
pub use network::PhidgetNetwork;
pub use phidget22::PhidgetLCD_Font_FONT_6x12;
use phidget22::*;
pub use temperature::TemperaturePhidget;
// functions common to all phidgets
pub trait PhidgetTraits {
    fn set_is_remote(self, remote: bool) -> Result<(), u32>;
    fn set_device_serial_number(self, serial: i32) -> Result<(), u32>;
    fn set_hub_port(self, hub_port: i32) -> Result<(), u32>;
    fn set_channel(self, channel: i32) -> Result<(), u32>;
    fn open_wait_for_attachment(self, max_wait_millis: u32) -> Result<(), u32>;
}

impl PhidgetTraits for PhidgetHandle {
    fn set_hub_port(self, hub_port: i32) -> Result<(), u32> {
        unsafe {
            match Phidget_setHubPort(self, hub_port) {
                0 => Ok(()),
                x => Err(x),
            }
        }
    }

    fn set_channel(self, channel: i32) -> Result<(), u32> {
        unsafe {
            match Phidget_setChannel(self, channel) {
                0 => Ok(()),
                x => Err(x),
            }
        }
    }

    fn set_is_remote(self, remote: bool) -> Result<(), u32> {
        unsafe {
            match Phidget_setIsRemote(self, remote as i32) {
                0 => Ok(()),
                x => Err(x),
            }
        }
    }

    fn set_device_serial_number(self, serial: i32) -> Result<(), u32> {
        unsafe {
            match Phidget_setDeviceSerialNumber(self, serial) {
                0 => Ok(()),
                x => Err(x),
            }
        }
    }

    fn open_wait_for_attachment(self, max_wait_millis: u32) -> Result<(), u32> {
        unsafe {
            match Phidget_openWaitForAttachment(self, max_wait_millis) {
                0 => Ok(()),
                x => Err(x),
            }
        }
    }
}
