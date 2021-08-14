extern crate libc;
mod phidgets;
use std::{thread, time};

fn main() {
    //make a &str for hostname
    phidgets::network::setup_network("Justin", "10.0.0.176", 5661);
    let handle = phidgets::temperature::setup_temperature();
    loop {
        let value = phidgets::temperature::get_temperature(handle);
        println!(
            "Temperature: {} F",
            phidgets::common::celcius_to_fahrenheit(value)
        );
        thread::sleep(time::Duration::from_secs(1));
    }
}
