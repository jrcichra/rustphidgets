use super::common;
use super::phidget22;

pub fn setup_network() {
    unsafe {
        let (_hostname, hostname) = common::str_to_char_arr("Justin");
        let (_ip, ip) = common::str_to_char_arr("10.0.0.176");
        let (_blank, blank) = common::str_to_char_arr("");
        let rc = phidget22::PhidgetNet_addServer(hostname, ip, 5661, blank, 0);
        if rc != phidget22::PhidgetReturnCode_EPHIDGET_OK {
            println!("Something went wrong in networking: {}", rc);
        }
    }
}
