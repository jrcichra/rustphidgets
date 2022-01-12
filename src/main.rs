/*

    An example program that uses the rustphidgets library.
    jrcichra / tjcichra - 2022

*/

extern crate libc;

mod phidgets;

use clap::Parser;
use phidgets::{
    celcius_to_fahrenheit, HumidityPhidget, LCDPhidget, PhidgetLCD_Font_FONT_6x12, PhidgetNetwork,
    TemperaturePhidget,
};
use std::thread::sleep;
use std::time::Duration;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name
    #[clap(short, long, default_value = "Phidget")]
    name: String,

    /// IP Address
    #[clap(short, long, default_value = "127.0.0.1")]
    ip: String,

    // Port
    #[clap(short, long, default_value_t = 5661)]
    port: i32,
}

fn main() {
    // parse args
    let args = Args::parse();

    // make a phidget network object
    PhidgetNetwork::setup(&args.name, &args.ip, args.port).unwrap();

    // make a phidget temperature object
    temp().unwrap();

    // make a phidget LCD object
    // lcd().unwrap();
}

fn temp() -> Result<(), u32> {
    // create a phidget temperature object
    let phidget = TemperaturePhidget::setup(0, true, 5000)?;
    println!("Attached!");
    loop {
        let value = phidget.get_temperature()?;
        println!("Temperature: {:.2} F", celcius_to_fahrenheit(value));
        sleep(Duration::from_secs(1));
    }
}

fn lcd() -> Result<(), u32> {
    let mut phidget = LCDPhidget::setup(1, true, 5000)?;
    println!("It was successful!");

    phidget.set_backlight(0.05)?;
    phidget.write_text(PhidgetLCD_Font_FONT_6x12, 20, 20, "BOB")?;
    phidget.flush()?;

    let mut x = String::new();
    std::io::stdin().read_line(&mut x).unwrap();

    phidget.close()?;
    phidget.delete()?;

    Ok(())
}
