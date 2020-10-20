extern crate libc;
mod phidget22;

mod common;

fn setup_network() {
    unsafe {
        let (_garb, justin) = common::str_to_char_arr("Justin");
        let (_garb2, ip) = common::str_to_char_arr("10.0.0.176");
        let (_garb3, blank) = common::str_to_char_arr("");
        let rc = phidget22::PhidgetNet_addServer(justin, ip, 5661, blank, 0);
        if rc != phidget22::PhidgetReturnCode_EPHIDGET_OK {
            println!("Something went wrong in networking: {}", rc);
        }
    }
}

fn get_temperature() {
    unsafe {
        //debug
        let (_garb4, output) = common::str_to_char_arr("phidgetlog.log");
        phidget22::PhidgetLog_enable(phidget22::Phidget_LogLevel_PHIDGET_LOG_VERBOSE, output);
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
