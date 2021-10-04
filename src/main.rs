extern crate libc;
mod phidgets;
use phidgets::network::PhidgetNetwork;
use phidgets::network::PhidgetNetworkTraits;
use phidgets::phidget22;
use phidgets::temperature::PhidgetTemperature;
use phidgets::temperature::PhidgetTemperatureTraits;
use phidgets::Phidget;
use phidgets::PhidgetTraits;
use std::{thread, time};

fn main() {
    //make a phidget network object
    let network = PhidgetNetwork::new();
    network.setup_network("Justin", "10.0.0.176", 5661);
    //make a phidget temperature object
    let phidget = PhidgetTemperature::new();
    println!("Waiting for attachment...");
    let rc = phidget.phidget.open_wait_for_attachment(2000);
    if rc != phidget22::PhidgetReturnCode_EPHIDGET_OK {
        println!(
            "Error opening phidget: {}",
            Phidget::get_return_code_string(rc)
        );
        return;
    }
    println!("Attached!");
    loop {
        let value = phidget.get_temperature();
        println!(
            "Temperature: {} F",
            phidgets::common::celcius_to_fahrenheit(value)
        );
        thread::sleep(time::Duration::from_secs(1));
    }
}
