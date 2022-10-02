// A mini time library
// for making basic timestamps
use effi::*;
use std::str;

// C FFI
extern "C" {
    fn time(__timer: *mut time_t) -> time_t;
    fn strftime(
        __s: *mut c_char,
        __maxsize: size_t,
        __format: *const c_char,
        __tp: *const tm,
    ) -> size_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
}

pub type size_t = c_ulong;
pub type __time_t = c_long;
pub type time_t = __time_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
    pub tm_wday: c_int,
    pub tm_yday: c_int,
    pub tm_isdst: c_int,
    pub tm_gmtoff: c_long,
    pub tm_zone: *const c_char,
}

// Returns the time given a format string
pub fn fmt(fmt_str: &str) -> String {
    unsafe {
        let mut result: [u8; 100] = [0; 100];
        let mut t: time_t = time(std::ptr::null_mut::<time_t>());
        strftime(
            result.as_mut_ptr() as *mut i8,
            100 as c_ulong,
            fmt_str.as_bytes().to_owned().as_mut_ptr() as *const c_char,
            localtime(&mut t));
        let res_str = str::from_utf8(&result).unwrap();
        res_str.to_string()
    }
}

