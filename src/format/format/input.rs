use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use ffi::*;

pub struct Input {
    ptr: *mut AVInputFormat,
}

impl Input {
    pub unsafe fn wrap(ptr: *mut AVInputFormat) -> Self {
        Input { ptr }
    }

    pub unsafe fn as_ptr(&self) -> *const AVInputFormat {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVInputFormat {
        self.ptr
    }
}

impl Input {
    /** A comma separated list of short names for the format. */
    pub fn name(&self) -> &str {
        unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).name).to_bytes()) }
    }

    /** Descriptive name for the format, meant to be more human readable than the name. */
    pub fn description(&self) -> &str {
        unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).long_name).to_bytes()) }
    }

    /** If extensions are defined, then no probe is done.
     * You should usually not use extension format guessing because it is not reliable
     * enough. */
    pub fn extensions(&self) -> Vec<&str> {
        unsafe {
            let ptr = (*self.as_ptr()).extensions;

            if ptr.is_null() {
                Vec::new()
            } else {
                from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes())
                    .split(',')
                    .collect()
            }
        }
    }

    /** A `Vec` containing the mime types. */
    pub fn mime_types(&self) -> Vec<&str> {
        unsafe {
            let ptr = (*self.as_ptr()).mime_type;

            if ptr.is_null() {
                Vec::new()
            } else {
                from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes())
                    .split(',')
                    .collect()
            }
        }
    }
}
