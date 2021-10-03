use super::phidget22;

pub fn setup_humidity() -> phidget22::PhidgetHumiditySensorHandle {
    unsafe {
        //debug
        // let (_debugpath, debugpath) = common::str_to_char_arr("phidgetlog.log");
        // phidget22::PhidgetLog_enable(phidget22::Phidget_LogLevel_PHIDGET_LOG_VERBOSE, debugpath);
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

pub fn get_temperature(temphandle: phidget22::PhidgetTemperatureSensorHandle) -> f64 {
    let mut temp = 0.0_f64;
    let temp_ptr: *mut f64 = &mut temp;
    unsafe {
        phidget22::PhidgetTemperatureSensor_getTemperature(temphandle, temp_ptr);
    }
    return temp;
}
