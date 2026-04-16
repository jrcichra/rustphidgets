use core::mem::MaybeUninit;

use super::helpers::str_to_char_arr;
use super::phidget22::{
    PhidgetLCD_PixelState, PhidgetLCD_clear, PhidgetLCD_drawLine, PhidgetLCD_drawPixel,
    PhidgetLCD_drawRect, PhidgetLCD_getContrast, PhidgetLCD_getFrameBuffer, PhidgetLCD_getHeight,
    PhidgetLCD_getMaxBacklight, PhidgetLCD_getMaxContrast, PhidgetLCD_getMinBacklight,
    PhidgetLCD_getMinContrast, PhidgetLCD_setFrameBuffer,
};
use crate::phidget::phidget22::{
    PhidgetHandle, PhidgetLCDHandle, PhidgetLCD_Font, PhidgetLCD_create, PhidgetLCD_delete,
    PhidgetLCD_flush, PhidgetLCD_getBacklight, PhidgetLCD_setBacklight, PhidgetLCD_setFontSize,
    PhidgetLCD_writeText,
};
use crate::phidget::Phidget;

pub struct LCDPhidget {
    handle: PhidgetLCDHandle,
    base_handle: PhidgetHandle,
}

impl LCDPhidget {
    pub fn setup(hub_port: i32, remote: bool, max_wait_millis: u32) -> Result<LCDPhidget, u32> {
        let phidget = LCDPhidget::create()?;
        phidget.base_handle.set_hub_port(hub_port)?;
        phidget.base_handle.set_is_remote(remote)?;
        phidget
            .base_handle
            .open_wait_for_attachment(max_wait_millis)?;
        Ok(phidget)
    }

    pub fn create() -> Result<LCDPhidget, u32> {
        let mut lcd_handle = MaybeUninit::uninit();
        let rc = unsafe { PhidgetLCD_create(lcd_handle.as_mut_ptr()) };
        match rc {
            0 => unsafe {
                Ok(LCDPhidget {
                    handle: lcd_handle.assume_init(),
                    base_handle: lcd_handle.assume_init() as PhidgetHandle,
                })
            },
            _ => Err(rc),
        }
    }

    pub fn delete(&mut self) -> Result<(), u32> {
        let rc = unsafe { PhidgetLCD_delete(&mut self.handle) };
        match rc {
            0 => Ok(()),
            _ => Err(rc),
        }
    }

    pub fn close(&self) -> Result<(), u32> {
        self.base_handle.close()
    }

    pub fn get_backlight(&self) -> Result<f64, u32> {
        let mut backlight = MaybeUninit::uninit();
        let rc = unsafe { PhidgetLCD_getBacklight(self.handle, backlight.as_mut_ptr()) };
        match rc {
            0 => Ok(unsafe { backlight.assume_init() }),
            _ => Err(rc),
        }
    }

    pub fn set_backlight(&self, backlight: f64) -> Result<&Self, u32> {
        let rc = unsafe { PhidgetLCD_setBacklight(self.handle, backlight) };
        match rc {
            0 => Ok(self),
            _ => Err(rc),
        }
    }

    pub fn get_min_backlight(&self) -> Result<f64, u32> {
        let mut min_backlight = MaybeUninit::uninit();
        let rc = unsafe { PhidgetLCD_getMinBacklight(self.handle, min_backlight.as_mut_ptr()) };
        match rc {
            0 => Ok(unsafe { min_backlight.assume_init() }),
            x => Err(x),
        }
    }

    pub fn get_max_backlight(&self) -> Result<f64, u32> {
        let mut max_backlight = MaybeUninit::uninit();
        let rc = unsafe { PhidgetLCD_getMaxBacklight(self.handle, max_backlight.as_mut_ptr()) };
        match rc {
            0 => Ok(unsafe { max_backlight.assume_init() }),
            x => Err(x),
        }
    }

    pub fn clear(&self) -> Result<&Self, u32> {
        let rc = unsafe { PhidgetLCD_clear(self.handle) };
        match rc {
            0 => Ok(self),
            x => Err(x),
        }
    }

    pub fn get_contrast(&self) -> Result<f64, u32> {
        let mut constrast = MaybeUninit::uninit();
        let rc = unsafe { PhidgetLCD_getContrast(self.handle, constrast.as_mut_ptr()) };
        match rc {
            0 => Ok(unsafe { constrast.assume_init() }),
            x => Err(x),
        }
    }

    pub fn get_min_contrast(&self) -> Result<f64, u32> {
        let mut min_constrast = MaybeUninit::uninit();
        let rc = unsafe { PhidgetLCD_getMinContrast(self.handle, min_constrast.as_mut_ptr()) };
        match rc {
            0 => Ok(unsafe { min_constrast.assume_init() }),
            x => Err(x),
        }
    }

    pub fn get_max_contrast(&self) -> Result<f64, u32> {
        let mut max_constrast = MaybeUninit::uninit();
        let rc = unsafe { PhidgetLCD_getMaxContrast(self.handle, max_constrast.as_mut_ptr()) };
        match rc {
            0 => Ok(unsafe { max_constrast.assume_init() }),
            x => Err(x),
        }
    }

    pub fn draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32) -> Result<&Self, u32> {
        let rc = unsafe { PhidgetLCD_drawLine(self.handle, x1, y1, x2, y2) };
        match rc {
            0 => Ok(self),
            x => Err(x),
        }
    }

    pub fn draw_pixel(
        &self,
        x1: i32,
        y1: i32,
        pixel_state: PhidgetLCD_PixelState,
    ) -> Result<&Self, u32> {
        let rc = unsafe { PhidgetLCD_drawPixel(self.handle, x1, y1, pixel_state) };
        match rc {
            0 => Ok(self),
            x => Err(x),
        }
    }

    pub fn draw_rect(
        &self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        filled: bool,
        inverted: bool,
    ) -> Result<&Self, u32> {
        let rc = unsafe {
            PhidgetLCD_drawRect(self.handle, x1, y1, x2, y2, filled as i32, inverted as i32)
        };
        match rc {
            0 => Ok(self),
            x => Err(x),
        }
    }

    pub fn flush(&self) -> Result<(), u32> {
        let rc = unsafe { PhidgetLCD_flush(self.handle) };
        match rc {
            0 => Ok(()),
            _ => Err(rc),
        }
    }

    pub fn set_font_size(
        &self,
        font: PhidgetLCD_Font,
        width: i32,
        height: i32,
    ) -> Result<&Self, u32> {
        let rc = unsafe { PhidgetLCD_setFontSize(self.handle, font, width, height) };
        match rc {
            0 => Ok(self),
            _ => Err(rc),
        }
    }

    pub fn get_frame_buffer(&self) -> Result<i32, u32> {
        let mut frame_buffer = MaybeUninit::uninit();
        let rc = unsafe { PhidgetLCD_getFrameBuffer(self.handle, frame_buffer.as_mut_ptr()) };
        match rc {
            0 => Ok(unsafe { frame_buffer.assume_init() }),
            x => Err(x),
        }
    }

    pub fn set_frame_buffer(&self, frame_buffer: i32) -> Result<&Self, u32> {
        let rc = unsafe { PhidgetLCD_setFrameBuffer(self.handle, frame_buffer) };
        match rc {
            0 => Ok(self),
            x => Err(x),
        }
    }

    pub fn get_height(&self) -> Result<i32, u32> {
        let mut height = MaybeUninit::uninit();
        let rc = unsafe { PhidgetLCD_getHeight(self.handle, height.as_mut_ptr()) };
        match rc {
            0 => Ok(unsafe { height.assume_init() }),
            x => Err(x),
        }
    }

    pub fn write_text(
        &self,
        font: PhidgetLCD_Font,
        x_position: i32,
        y_position: i32,
        text: &str,
    ) -> Result<(), u32> {
        let c_str = str_to_char_arr(text).ok_or(1u32)?;
        let rc = unsafe {
            PhidgetLCD_writeText(self.handle, font, x_position, y_position, c_str.as_ptr())
        };
        match rc {
            0 => Ok(()),
            _ => Err(rc),
        }
    }
}
