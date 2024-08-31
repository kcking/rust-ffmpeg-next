use ffi::*;
use Error;

/** Get the current time (`av_gettime()`). */
#[inline(always)]
pub fn current() -> i64 {
    unsafe { av_gettime() as i64 }
}

/** Get the relative time (`av_gettime_relative()`). */
#[inline(always)]
pub fn relative() -> i64 {
    unsafe { av_gettime_relative() as i64 }
}

/** Is the time monotonic? (`av_gettime_relative_is_monotonic()`). */
#[inline(always)]
pub fn is_monotonic() -> bool {
    unsafe { av_gettime_relative_is_monotonic() != 0 }
}

/** Sleep for `usec` seconds (`av_usleep`). */
#[inline(always)]
pub fn sleep(usec: u32) -> Result<(), Error> {
    unsafe {
        match av_usleep(usec) {
            0 => Ok(()),
            e => Err(Error::from(e)),
        }
    }
}
