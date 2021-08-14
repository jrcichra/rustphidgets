use super::common;
use super::phidget22;
// setup_network takes in a hostname, ip address, and port.
pub fn setup_network(hostname: &str, ip_address: &str, port: i32) {
    let (_hostname, hostname) = common::str_to_char_arr(hostname);
    let (_ip, ip) = common::str_to_char_arr(ip_address);
    let (_blank, blank) = common::str_to_char_arr("");
    unsafe {
        let rc = phidget22::PhidgetNet_addServer(hostname, ip, port, blank, 0);
        if rc != phidget22::PhidgetReturnCode_EPHIDGET_OK {
            println!("Something went wrong in networking: {}", rc);
        }
    }
}
