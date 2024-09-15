use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use ffi::AVColorRange::*;
use ffi::*;

/**
 * An enum representation of an AVColorRange.
 *
 * ```
 * Unspecified => AVCOL_RANGE_UNSPECIFIED | AVCOL_RANGE_NB
 * MPEG        => AVCOL_RANGE_MPEG
 * JPEG        => AVCOL_RANGE_JPEG
 * ```
 *
 * Represents the visual content value range.
 *
 * > For RGB and luma planes such as Y in YCbCr and I in ICtCp, 'E' is the original value in range of 0.0 to 1.0.
 * > For chroma planes such as Cb,Cr and Ct,Cp, 'E' is the original value in range of -0.5 to 0.5.
 * > 'n' is the output bit depth.
 * > For additional definitions such as rounding and clipping to valid n bit unsigned integer range, please refer to BT.2100 (Table 9).
 */
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Range {
    Unspecified,
    /**
     * Narrow or limited range content.
     *
     * For luma planes:
     * `(219 * E + 16) * 2^(n-8)`
     *
     * For chroma planes:
     * `(224 * E + 128) * 2^(n-8)`
     */
    MPEG,
    /**
     * Full range content
     *
     * For RGB and luma planes:
     * `(2^n - 1) * E`
     *
     * For chroma planes:
     * `(2^n - 1) * E + 2^(n - 1)`
     */
    JPEG,
}

impl Range {
    pub fn name(&self) -> Option<&'static str> {
        if *self == Range::Unspecified {
            return None;
        }
        unsafe {
            let ptr = av_color_range_name((*self).into());
            ptr.as_ref()
                .map(|ptr| from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()))
        }
    }
}

impl From<AVColorRange> for Range {
    fn from(value: AVColorRange) -> Self {
        match value {
            AVCOL_RANGE_UNSPECIFIED => Range::Unspecified,
            AVCOL_RANGE_MPEG => Range::MPEG,
            AVCOL_RANGE_JPEG => Range::JPEG,
            AVCOL_RANGE_NB => Range::Unspecified,
        }
    }
}

impl From<Range> for AVColorRange {
    fn from(value: Range) -> AVColorRange {
        match value {
            Range::Unspecified => AVCOL_RANGE_UNSPECIFIED,
            Range::MPEG => AVCOL_RANGE_MPEG,
            Range::JPEG => AVCOL_RANGE_JPEG,
        }
    }
}
