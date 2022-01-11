use super::helpers;
use super::phidget22;

pub struct PhidgetNetwork;

impl PhidgetNetwork {
    pub fn setup(hostname: &str, ip_address: &str, port: i32) -> Result<(), u32> {
        let (_hostname, hostname) = helpers::str_to_char_arr(hostname);
        let (_ip, ip) = helpers::str_to_char_arr(ip_address);
        let (_password, password) = helpers::str_to_char_arr("");
        let rc: u32;
        unsafe {
            rc = phidget22::PhidgetNet_addServer(hostname, ip, port, password, 0);
        }
        match rc {
            0 => Ok(()),
            x => Err(x),
        }
    }
}
