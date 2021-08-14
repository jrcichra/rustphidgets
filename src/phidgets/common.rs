use std::ffi::CString;
use std::os::raw::c_char;

pub fn str_to_char_arr(to: &str) -> (std::ffi::CString, *const c_char) {
    let c_str = CString::new(to).unwrap();
    let c_world: *const c_char = c_str.as_ptr() as *const c_char;
    return (c_str, c_world);
}

pub fn celcius_to_fahrenheit(celcius: f64) -> f64 {
    return celcius * 9.0 / 5.0 + 32.0;
}
