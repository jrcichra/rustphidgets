use std::mem::MaybeUninit;
use crate::phidget::Phidget;
use crate::phidget::phidget22::{PhidgetLCDHandle, PhidgetHandle, PhidgetLCD_create, PhidgetLCD_delete, PhidgetLCD_getBacklight, PhidgetLCD_setBacklight, PhidgetLCD_flush, PhidgetLCD_Font, PhidgetLCD_setFontSize, PhidgetLCD_writeText};
use super::helpers::str_to_char_arr;

pub struct LCDPhidget {
    handle: PhidgetLCDHandle,
    base_handle: PhidgetHandle
}

impl LCDPhidget {
    pub fn setup(hub_port: i32, remote: bool, max_wait_millis: u32) -> Result<LCDPhidget, u32> {
        let phidget = LCDPhidget::create()?;
        phidget.base_handle.set_hub_port(hub_port)?;
        phidget.base_handle.set_is_remote(remote)?;
        phidget.base_handle.open_wait_for_attachment(max_wait_millis)?;
        Ok(phidget)
    }

    pub fn create() -> Result<LCDPhidget, u32> {
        let mut lcd_handle;
        let rc = unsafe {
            lcd_handle = MaybeUninit::uninit().assume_init();
            PhidgetLCD_create(&mut lcd_handle)
        };
        match rc {
            0 => Ok(LCDPhidget { handle: lcd_handle, base_handle: lcd_handle as PhidgetHandle }),
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

    pub fn close(&self) -> Result<(), u32> {
        self.base_handle.close()
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

    pub fn set_backlight(&self, backlight: f64) -> Result<&Self, u32> {
        let rc = unsafe {
            PhidgetLCD_setBacklight(self.handle, backlight)
        };
        match rc {
            0 => Ok(self),
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

    pub fn set_font_size(&self, font: PhidgetLCD_Font, width: i32, height: i32) -> Result<&Self, u32> {
        let rc = unsafe {
            PhidgetLCD_setFontSize(self.handle, font, width, height)
        };
        match rc {
            0 => Ok(self),
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
        let (a, char_pointer) = str_to_char_arr(text);
        let rc = unsafe {
            PhidgetLCD_writeText(self.handle, font, x_position, y_position, char_pointer)
        };
        match rc {
            0 => Ok(()),
            _ => Err(rc),
        }
    }
}
