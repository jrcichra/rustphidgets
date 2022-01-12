use super::helpers::str_to_char_arr;
use super::phidget22::PhidgetHandle;
use super::phidget22::*;
use crate::phidgets::PhidgetTraits;
use core::mem::MaybeUninit;

pub struct LCDPhidget {
    handle: PhidgetLCDHandle,
}

impl LCDPhidget {
    pub fn setup(hub_port: i32, remote: bool, max_wait_millis: u32) -> Result<LCDPhidget, u32> {
        let phidget = LCDPhidget::create()?;
        let phidget_handle = phidget.handle as PhidgetHandle;
        phidget_handle.set_hub_port(hub_port)?;
        phidget_handle.set_is_remote(remote)?;
        phidget_handle.open_wait_for_attachment(max_wait_millis)?;
        Ok(phidget)
    }

    pub fn close(&self) -> Result<(), u32> {
        let rc = unsafe {
            Phidget_close(self.handle as PhidgetHandle)
        };
        match rc {
            0 => Ok(()),
            _ => Err(rc),
        }
    }

    pub fn create() -> Result<LCDPhidget, u32> {
        let mut lcd_handle;
        let rc = unsafe {
            lcd_handle = MaybeUninit::uninit().assume_init();
            PhidgetLCD_create(&mut lcd_handle)
        };
        match rc {
            0 => Ok(LCDPhidget { handle: lcd_handle }),
            _ => Err(rc),
        }
    }

    pub fn delete(&mut self) -> Result<(), u32> {
        let rc = unsafe {
            PhidgetLCD_delete(&mut self.handle)
        };
        match rc {
            0 => Ok(()),
            _ => Err(rc),
        }
    }

    pub fn get_backlight(&self) -> Result<f64, u32> {
        let mut backlight;
        let rc = unsafe {
            backlight = MaybeUninit::uninit().assume_init();
            PhidgetLCD_getBacklight(self.handle, &mut backlight)
        };
        match rc {
            0 => Ok(backlight),
            _ => Err(rc),
        }
    }

    pub fn set_backlight(&self, backlight: f64) -> Result<(), u32> {
        let rc = unsafe {
            PhidgetLCD_setBacklight(self.handle, backlight)
        };
        match rc {
            0 => Ok(()),
            _ => Err(rc),
        }
    }

    pub fn flush(&self) -> Result<(), u32> {
        let rc = unsafe {
            PhidgetLCD_flush(self.handle)
        };
        match rc {
            0 => Ok(()),
            _ => Err(rc),
        }
    }

    pub fn set_font_size(&self, font: PhidgetLCD_Font, width: i32, height: i32) -> Result<(), u32> {
        let rc = unsafe {
            PhidgetLCD_setFontSize(self.handle, font, width, height)
        };
        match rc {
            0 => Ok(()),
            _ => Err(rc),
        }
    }

    pub fn write_text(
        &self,
        font: PhidgetLCD_Font,
        x_position: i32,
        y_position: i32,
        text: &str,
    ) -> Result<(), u32> {
        let (_, char_pointer) = str_to_char_arr(text);
        let rc = unsafe {
            PhidgetLCD_writeText(self.handle, font, x_position, y_position, char_pointer)
        };
        match rc {
            0 => Ok(()),
            _ => Err(rc),
        }
    }
}
