extern crate libc;
mod phidgets;
use std::{thread, time};

fn main() {
    phidgets::network::setup_network();
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
