use super::helpers;
use super::phidget22;

pub struct PhidgetNetwork;

impl PhidgetNetwork {
    pub fn setup(hostname: &str, ip_address: &str, port: i32) -> Result<(), u32> {
        let hostname = helpers::str_to_char_arr(hostname).ok_or(1u32)?.as_ptr();
        let ip = helpers::str_to_char_arr(ip_address).ok_or(1u32)?.as_ptr();
        let password = helpers::str_to_char_arr("").ok_or(1u32)?.as_ptr();
        let rc = unsafe { phidget22::PhidgetNet_addServer(hostname, ip, port, password, 0) };
        match rc {
            0 => Ok(()),
            x => Err(x),
        }
    }
}
