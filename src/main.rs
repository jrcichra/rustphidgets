extern crate libc;
mod common;
mod phidget22;

use std::{thread, time};

fn setup_network() {
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

fn setup_temperature() -> phidget22::PhidgetTemperatureSensorHandle {
    unsafe {
        //debug
        let (_debugpath, debugpath) = common::str_to_char_arr("phidgetlog.log");
        phidget22::PhidgetLog_enable(phidget22::Phidget_LogLevel_PHIDGET_LOG_VERBOSE, debugpath);
        #[allow(deprecated)]
        let mut temphandle: phidget22::PhidgetTemperatureSensorHandle = std::mem::uninitialized();
        phidget22::PhidgetTemperatureSensor_create(&mut temphandle);
        phidget22::Phidget_setIsRemote(temphandle as phidget22::PhidgetHandle, 1);
        phidget22::Phidget_setDeviceSerialNumber(temphandle as phidget22::PhidgetHandle, 597101);
        phidget22::Phidget_setHubPort(temphandle as phidget22::PhidgetHandle, 0);
        let rc =
            phidget22::Phidget_openWaitForAttachment(temphandle as phidget22::PhidgetHandle, 5000);
        if rc != phidget22::PhidgetReturnCode_EPHIDGET_OK {
            println!(
                "Something went wrong when attaching to the temperature sensor: {}",
                rc
            );
        }
        return temphandle;
    }
}

fn get_temperature(temphandle: phidget22::PhidgetTemperatureSensorHandle) {
    let mut temp = 0.0_f64;
    let temp_ptr: *mut f64 = &mut temp;
    unsafe {
        phidget22::PhidgetTemperatureSensor_getTemperature(temphandle, temp_ptr);
    }
    println!("Got a temperature of {} F.", temp * 9.0 / 5.0 + 32.0);
}

fn main() {
    setup_network();
    let handle = setup_temperature();
    loop {
        get_temperature(handle);
        thread::sleep(time::Duration::from_secs(1));
    }
}
