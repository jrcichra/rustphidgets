use std::mem::MaybeUninit;
use crate::phidget::Phidget;
use crate::phidget::phidget22::{PhidgetLCDHandle, PhidgetHandle, PhidgetLCD_create, PhidgetLCD_delete, PhidgetLCD_getBacklight, PhidgetLCD_setBacklight, PhidgetLCD_flush, PhidgetLCD_Font, PhidgetLCD_setFontSize, PhidgetLCD_writeText};
use super::helpers::str_to_char_arr;
use super::phidget22::{PhidgetLCD_getMinBacklight, PhidgetLCD_getMaxBacklight, PhidgetLCD_clear, PhidgetLCD_getContrast, PhidgetLCD_setContrast, PhidgetLCD_getMinContrast, PhidgetLCD_getMaxContrast, PhidgetLCD_getFrameBuffer, PhidgetLCD_setFrameBuffer, PhidgetLCD_getHeight, PhidgetLCD_drawLine, PhidgetLCD_drawPixel, PhidgetLCD_PixelState, PhidgetLCD_drawRect};

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

    pub fn get_min_backlight(&self) -> Result<f64, u32> {
        let mut min_backlight;
        let rc = unsafe {
            min_backlight = MaybeUninit::uninit().assume_init();
            PhidgetLCD_getMinBacklight(self.handle, &mut min_backlight)
        };
        match rc {
            0 => Ok(min_backlight),
            x => Err(x)
        }
    }

    pub fn get_max_backlight(&self) -> Result<f64, u32> {
        let mut max_backlight;
        let rc = unsafe {
            max_backlight = MaybeUninit::uninit().assume_init();
            PhidgetLCD_getMaxBacklight(self.handle, &mut max_backlight)
        };
        match rc {
            0 => Ok(max_backlight),
            x => Err(x)
        }
    }

    pub fn clear(&self) -> Result<&Self, u32> {
        let rc = unsafe {
            PhidgetLCD_clear(self.handle)
        };
        match rc {
            0 => Ok(self),
            x => Err(x)
        }
    }

    pub fn get_contrast(&self) -> Result<f64, u32> {
        let mut constrast;
        let rc = unsafe {
            constrast = MaybeUninit::uninit().assume_init();
            PhidgetLCD_getContrast(self.handle, &mut constrast)
        };
        match rc {
            0 => Ok(constrast),
            x => Err(x)
        }
    }

    pub fn get_min_contrast(&self) -> Result<f64, u32> {
        let mut min_constrast;
        let rc = unsafe {
            min_constrast = MaybeUninit::uninit().assume_init();
            PhidgetLCD_getMinContrast(self.handle, &mut min_constrast)
        };
        match rc {
            0 => Ok(min_constrast),
            x => Err(x)
        }
    }

    pub fn get_max_contrast(&self) -> Result<f64, u32> {
        let mut max_constrast;
        let rc = unsafe {
            max_constrast = MaybeUninit::uninit().assume_init();
            PhidgetLCD_getMaxContrast(self.handle, &mut max_constrast)
        };
        match rc {
            0 => Ok(max_constrast),
            x => Err(x)
        }
    }

    pub fn draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32) -> Result<&Self, u32> {
        let rc = unsafe {
            PhidgetLCD_drawLine(self.handle, x1, y1, x2, y2)
        };
        match rc {
            0 => Ok(self),
            x => Err(x)
        }
    }

    pub fn draw_pixel(&self, x1: i32, y1: i32, pixel_state: PhidgetLCD_PixelState) -> Result<&Self, u32> {
        let rc = unsafe {
            PhidgetLCD_drawPixel(self.handle, x1, y1, pixel_state)
        };
        match rc {
            0 => Ok(self),
            x => Err(x)
        }
    }

    /// Draws a rectangle in the current frame buffer using the specified points.
    /// 
    /// Changes made to the frame buffer must be flushed to the LCD screen using [`flush`][flush].
    /// 
    /// [flush]: struct.LCDPhidget.html#method.flush
    pub fn draw_rect(&self, x1: i32, y1: i32, x2: i32, y2: i32, filled: bool, inverted: bool) -> Result<&Self, u32> {
        let rc = unsafe {
            PhidgetLCD_drawRect(self.handle, x1, y1, x2, y2, filled as i32, inverted as i32)
        };
        match rc {
            0 => Ok(self),
            x => Err(x)
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

    pub fn get_frame_buffer(&self) -> Result<i32, u32> {
        let mut frame_buffer;
        let rc = unsafe {
            frame_buffer = MaybeUninit::uninit().assume_init();
            PhidgetLCD_getFrameBuffer(self.handle, &mut frame_buffer)
        };
        match rc {
            0 => Ok(frame_buffer),
            x => Err(x)
        }
    }

    pub fn set_frame_buffer(&self, frame_buffer: i32) -> Result<&Self, u32> {
        let rc = unsafe {
            PhidgetLCD_setFrameBuffer(self.handle, frame_buffer)
        };
        match rc {
            0 => Ok(self),
            x => Err(x)
        }
    }

    pub fn get_height(&self) -> Result<i32, u32> {
        let mut height;
        let rc = unsafe {
            height = MaybeUninit::uninit().assume_init();
            PhidgetLCD_getHeight(self.handle, &mut height)
        };
        match rc {
            0 => Ok(height),
            x => Err(x)
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
