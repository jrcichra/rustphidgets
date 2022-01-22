pub mod humidity;
pub mod temperature;
pub mod lcd;
pub mod network;
pub mod phidget22;
pub mod helpers;

use crate::phidget::phidget22::{PhidgetHandle, Phidget_setIsRemote, Phidget_setDeviceSerialNumber, Phidget_setHubPort, Phidget_setChannel, Phidget_openWaitForAttachment, Phidget_close};

// functions common to all phidget
trait Phidget {
    fn set_is_remote(self, remote: bool) -> Result<(), u32>;
    fn set_device_serial_number(self, serial: i32) -> Result<(), u32>;
    fn set_hub_port(self, hub_port: i32) -> Result<(), u32>;
    fn set_channel(self, channel: i32) -> Result<(), u32>;
    fn open_wait_for_attachment(self, max_wait_millis: u32) -> Result<(), u32>;
    fn close(self) -> Result<(), u32>;
}

impl Phidget for PhidgetHandle {
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

    fn open_wait_for_attachment(self, max_wait_millis: u32) -> Result<(), u32> {
        unsafe {
            match Phidget_openWaitForAttachment(self, max_wait_millis) {
                0 => Ok(()),
                x => Err(x),
            }
        }
    }

    fn close(self) -> Result<(), u32> {
        let rc = unsafe {
            Phidget_close(self)
        };
        match rc {
            0 => Ok(()),
            _ => Err(rc),
        }
    }
}