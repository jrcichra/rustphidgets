extern crate libc;
mod phidget22;

mod common;

fn setup_network() {
    unsafe {
        let rc = phidget22::PhidgetNet_addServer(
            common::str_to_char_arr("Justin"),
            common::str_to_char_arr("10.0.0.176"),
            5661,
            common::str_to_char_arr(""),
            0,
        );
        if rc != phidget22::PhidgetReturnCode_EPHIDGET_OK {
            println!("Something went wrong in networking: {}", rc);
        }
    }
}

fn get_temperature() {
    let mut temphandle: phidget22::PhidgetTemperatureSensorHandle =
        0 as *mut phidget22::_PhidgetTemperatureSensor;
    let temphandle_ptr: *mut phidget22::PhidgetTemperatureSensorHandle = &mut temphandle;
    unsafe {
        phidget22::PhidgetTemperatureSensor_create(temphandle_ptr);
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
        } else {
            let mut temp = 0.0_f64;
            let temp_ptr: *mut f64 = &mut temp;
            phidget22::PhidgetTemperatureSensor_getTemperature(temphandle, temp_ptr);
            println!("Got a temperature of {}.", temp);
        }
    }
}

fn main() {
    setup_network();
    get_temperature();
    println!("hello phidgets!");
}
