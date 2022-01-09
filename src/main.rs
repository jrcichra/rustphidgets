extern crate libc;
mod phidgets;
use phidgets::network::PhidgetNetwork;
use phidgets::network::PhidgetNetworkTraits;
use phidgets::phidget22;
use phidgets::temperature::PhidgetTemperature;
use phidgets::temperature::PhidgetTemperatureTraits;
use phidgets::lcd::{setup_lcd, flush, write_text, delete, close};
use phidgets::Phidget;
use phidgets::PhidgetTraits;
use std::{thread, time};

fn main() {
    //make a phidget network object
    let network = PhidgetNetwork::new();
    network.setup_network("Tim", "10.0.0.190", 5661);

    //make a phidget temperature object
    // temp();

    //make a phidget LCD object
    lcd();
}


fn temp() {
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

fn lcd() {
    let mut phidget = setup_lcd(1, 597101, true);
    println!("It was successful!");

    dbg!(write_text(phidget, phidget22::PhidgetLCD_Font_FONT_6x10, 20, 20, "bob"));
    dbg!(flush(phidget));

    close(phidget);
    delete(&mut phidget);
}