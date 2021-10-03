extern crate libc;
mod phidgets;
use phidgets::network::PhidgetNetwork;
use phidgets::network::PhidgetNetworkTraits;
use phidgets::temperature::PhidgetTemperature;
use phidgets::temperature::PhidgetTemperatureTraits;
use std::{thread, time};

fn main() {
    //make a phidget network object
    let network = PhidgetNetwork::new();
    network.setup_network("Justin", "10.0.0.176", 5661);
    //make a phidget temperature object
    let phidget = PhidgetTemperature::new();
    loop {
        let value = phidget.get_temperature();
        println!(
            "Temperature: {} F",
            phidgets::common::celcius_to_fahrenheit(value)
        );
        thread::sleep(time::Duration::from_secs(1));
    }
}
