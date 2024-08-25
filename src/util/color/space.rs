use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use ffi::AVColorSpace::*;
use ffi::*;

/** enum representation of AVColorSpace, the YUV colorspace type. */
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Space {
    /** Coefficients are actually ordered as GBR. */
    RGB,
    BT709,
    Unspecified,
    /** Reserved for future use by ITU-T and ISO/IEC. */
    Reserved,
    /** FCC Title 47 Code of federal regulations. */
    FCC,
    BT470BG,
    SMPTE170M,
    SMPTE240M,
    YCGCO,
    BT2020NCL,
    BT2020CL,
    SMPTE2085,

    /** Chromacity-derived non-constant luminance system. */
    ChromaDerivedNCL,
    /** Chromacity-derived constant luminance system. */
    ChromaDerivedCL,
    ICTCP,
}

impl Space {
    pub const YCOCG: Space = Space::YCGCO;

    /** Returns the name for provided color space or None if unknown/null. */
    pub fn name(&self) -> Option<&'static str> {
        if *self == Space::Unspecified {
            return None;
        }
        unsafe {
            let ptr = av_color_space_name((*self).into());
            ptr.as_ref()
                .map(|ptr| from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()))
        }
    }
}

impl From<AVColorSpace> for Space {
    fn from(value: AVColorSpace) -> Self {
        match value {
            AVCOL_SPC_RGB => Space::RGB,
            AVCOL_SPC_BT709 => Space::BT709,
            AVCOL_SPC_UNSPECIFIED => Space::Unspecified,
            AVCOL_SPC_RESERVED => Space::Reserved,
            AVCOL_SPC_FCC => Space::FCC,
            AVCOL_SPC_BT470BG => Space::BT470BG,
            AVCOL_SPC_SMPTE170M => Space::SMPTE170M,
            AVCOL_SPC_SMPTE240M => Space::SMPTE240M,
            AVCOL_SPC_YCGCO => Space::YCGCO,
            AVCOL_SPC_BT2020_NCL => Space::BT2020NCL,
            AVCOL_SPC_BT2020_CL => Space::BT2020CL,
            AVCOL_SPC_SMPTE2085 => Space::SMPTE2085,
            AVCOL_SPC_NB => Space::Unspecified,

            AVCOL_SPC_CHROMA_DERIVED_NCL => Space::ChromaDerivedNCL,
            AVCOL_SPC_CHROMA_DERIVED_CL => Space::ChromaDerivedCL,
            AVCOL_SPC_ICTCP => Space::ICTCP,
        }
    }
}

impl From<Space> for AVColorSpace {
    fn from(value: Space) -> AVColorSpace {
        match value {
            Space::RGB => AVCOL_SPC_RGB,
            Space::BT709 => AVCOL_SPC_BT709,
            Space::Unspecified => AVCOL_SPC_UNSPECIFIED,
            Space::Reserved => AVCOL_SPC_RESERVED,
            Space::FCC => AVCOL_SPC_FCC,
            Space::BT470BG => AVCOL_SPC_BT470BG,
            Space::SMPTE170M => AVCOL_SPC_SMPTE170M,
            Space::SMPTE240M => AVCOL_SPC_SMPTE240M,
            Space::YCGCO => AVCOL_SPC_YCGCO,
            Space::BT2020NCL => AVCOL_SPC_BT2020_NCL,
            Space::BT2020CL => AVCOL_SPC_BT2020_CL,
            Space::SMPTE2085 => AVCOL_SPC_SMPTE2085,

            Space::ChromaDerivedNCL => AVCOL_SPC_CHROMA_DERIVED_NCL,
            Space::ChromaDerivedCL => AVCOL_SPC_CHROMA_DERIVED_CL,
            Space::ICTCP => AVCOL_SPC_ICTCP,
        }
    }
}
