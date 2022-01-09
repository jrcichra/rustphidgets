use super::phidget22::*;
use super::common::str_to_char_arr;
use core::mem::MaybeUninit;

pub struct LCDPhidget {
    handle: PhidgetLCDHandle
}

impl LCDPhidget {
    pub fn setup(hub_port: i32, remote: bool, max_wait_millis: u32) -> Result<LCDPhidget, u32> {
        let phidget = LCDPhidget::create();
        phidget.set_hub_port(hub_port)?;
        phidget.set_is_remote(remote)?;

        phidget.open_wait_for_attachment(max_wait_millis)?;

        Ok(phidget)
    }

    pub fn set_hub_port(&self, hub_port: i32) -> Result<(), u32> {
        unsafe {
            match Phidget_setHubPort(self.handle as PhidgetHandle, hub_port) {
                0 => Ok(()),
                x => Err(x)
            }
        }
    }

    pub fn set_is_remote(&self, remote: bool) -> Result<(), u32> {
        unsafe {
            match Phidget_setIsRemote(self.handle as PhidgetHandle, remote as i32) {
                0 => Ok(()),
                x => Err(x)
            }
        }
    }

    pub fn open_wait_for_attachment(&self, max_wait_millis: u32) -> Result<(), u32> {
        unsafe {
            match Phidget_openWaitForAttachment(self.handle as PhidgetHandle, max_wait_millis) {
                0 => Ok(()),
                x => Err(x)
            }
        }
    }

    pub fn close(&self) {
        unsafe {
            Phidget_close(self.handle as PhidgetHandle);
        }
    }

    pub fn create() -> LCDPhidget {
        unsafe {
            let mut lcd_handle = MaybeUninit::uninit().assume_init();
            PhidgetLCD_create(&mut lcd_handle);

            LCDPhidget {
                handle: lcd_handle
            }
        }
    }

    pub fn delete(&mut self) {
        unsafe {
            PhidgetLCD_delete(&mut self.handle);
        }
    }

    pub fn get_backlight(&self) -> f64 {
        unsafe {
            let mut backlight: f64 = MaybeUninit::uninit().assume_init();
            PhidgetLCD_getBacklight(self.handle, &mut backlight);

            backlight
        }
    }

    pub fn set_backlight(&self, backlight: f64) {
        unsafe {
            PhidgetLCD_setBacklight(self.handle, backlight);
        }
    }

    pub fn flush(&self) {
        unsafe {
            PhidgetLCD_flush(self.handle);
        }
    }

    pub fn set_font_size(&self, font: PhidgetLCD_Font, width: i32, height: i32) {
        unsafe {
            PhidgetLCD_setFontSize(self.handle, font, width, height);
        }
    }

    pub fn write_text(&self, font: PhidgetLCD_Font, x_position: i32, y_position: i32, text: &str) -> PhidgetReturnCode {
        unsafe {
            let (_c_string, char_pointer) = str_to_char_arr(text);
            PhidgetLCD_writeText(self.handle, font, x_position, y_position, char_pointer)
        }
    }
}
