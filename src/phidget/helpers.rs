use std::ffi::CString;
use std::os::raw::c_char;

#[derive(Debug)]
pub struct CStringPtr {
    _inner: CString,
    ptr: *const c_char,
}

impl CStringPtr {
    pub fn new(s: &str) -> Option<Self> {
        let c_str = CString::new(s).ok()?;
        let ptr = c_str.as_ptr();
        Some(CStringPtr { _inner: c_str, ptr })
    }

    pub fn as_ptr(&self) -> *const c_char {
        self.ptr
    }
}

#[allow(dead_code)]
pub fn str_to_char_arr(to: &str) -> Option<CStringPtr> {
    CStringPtr::new(to)
}

pub fn celcius_to_fahrenheit(celcius: f64) -> f64 {
    celcius * 9.0 / 5.0 + 32.0
}
