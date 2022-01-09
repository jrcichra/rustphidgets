use super::phidget22::*;
use super::common::str_to_char_arr;
use core::mem::MaybeUninit;

pub fn setup_lcd(hub_port: i32, serial_number: i32, remote: bool) -> PhidgetLCDHandle {
    let mut lcd_handle: PhidgetLCDHandle;
    let rc;

    unsafe {
        lcd_handle = MaybeUninit::uninit().assume_init();

        PhidgetLCD_create(&mut lcd_handle);
        match Phidget_setHubPort(lcd_handle as PhidgetHandle, hub_port) {
            PhidgetReturnCode_EPHIDGET_OK => (),
            _ => panic!("This is not Ok")
        };
        match Phidget_setIsRemote(lcd_handle as PhidgetHandle, remote as i32) {
            PhidgetReturnCode_EPHIDGET_OK => (),
            _ => panic!("This is not Ok")
        };

        rc = Phidget_openWaitForAttachment(lcd_handle as PhidgetHandle, 5000);
    }

    if rc != PhidgetReturnCode_EPHIDGET_OK {
        println!("Something went wrong when attaching to the lcd sensor: {}", rc);
        panic!("You are a stupid nerd");
    }

    lcd_handle
}

// pub fn create

pub fn close(ch: PhidgetLCDHandle) {
    unsafe {
        Phidget_close(ch as PhidgetHandle);
    }
}

pub fn delete(ch: &mut PhidgetLCDHandle) {
    unsafe {
        PhidgetLCD_delete(ch);
    }
}

pub fn flush(ch: PhidgetLCDHandle) -> PhidgetReturnCode {
    unsafe {
        PhidgetLCD_flush(ch)
    }
}

pub fn write_text(ch: PhidgetLCDHandle, font: PhidgetLCD_Font, x_position: i32, y_position: i32, text: &str) -> PhidgetReturnCode {
    unsafe {
        let (a, b) = str_to_char_arr(text);
        dbg!(b);
        PhidgetLCD_writeText(ch, font, x_position, y_position, b)
    }
}