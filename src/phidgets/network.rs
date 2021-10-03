use super::common;
use super::phidget22;
use super::Phidget;
use super::PhidgetTraits;
// setup_network takes in a hostname, ip address, and port.

pub struct PhidgetNetwork;

pub trait PhidgetNetworkTraits {
    fn new() -> PhidgetNetwork;
    fn setup_network(
        self,
        hostname: &str,
        ip_address: &str,
        port: i32,
    ) -> phidget22::PhidgetReturnCode;
}

impl PhidgetNetworkTraits for PhidgetNetwork {
    fn new() -> PhidgetNetwork {
        return PhidgetNetwork {};
    }
    fn setup_network(
        self,
        hostname: &str,
        ip_address: &str,
        port: i32,
    ) -> phidget22::PhidgetReturnCode {
        let (_hostname, hostname) = common::str_to_char_arr(hostname);
        let (_ip, ip) = common::str_to_char_arr(ip_address);
        let (_blank, blank) = common::str_to_char_arr("");
        let rc: u32;
        unsafe {
            rc = phidget22::PhidgetNet_addServer(hostname, ip, port, blank, 0);
        }
        if rc != phidget22::PhidgetReturnCode_EPHIDGET_OK {
            println!(
                "Something went wrong in networking: {}",
                Phidget::get_return_code_string(rc)
            );
        }
        return rc;
    }
}
