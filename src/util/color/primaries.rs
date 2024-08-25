use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use ffi::AVColorPrimaries::*;
use ffi::*;

/** Chromacity coordinates of the source primaries. */
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Primaries {
    Reserved0,
    /** Also ITU-R BT1361 / IEC 61966-2-4 / SMPTE RP 177 Annex B. */
    BT709,
    Unspecified,
    Reserved,
    /** Also FCC Title 47 Code of Federal Regulations 73.682 (a)(20) */
    BT470M,

    /**
     * Also ITU-R BT601-6 625
     * / ITU-R BT1358 625
     * / ITU-R BT1700 625 PAL & SECAM.
     */
    BT470BG,
    /** 
     * Also ITU-R BT601-6 525 / ITU-R BT1358 525 / ITU-R BT1700 NTSC.
     */
    SMPTE170M,
    /** Identical to SMPTE170M, also called "SMPTE C" even though it uses D65. */
    SMPTE240M,
    /** Color filters using Illuminant C. */
    Film,
    /** ITU-R BT2020. */
    BT2020,

    /** SMPTE ST 428-1 (CIE 1931 XYZ). */
    SMPTE428,
    /** SMPTE ST 431-2 (2011) / DCI P3. */
    SMPTE431,
    /** SMPTE ST 432-1 (2010) / P3 D65 / Display P3. */
    SMPTE432,
    #[cfg(not(feature = "ffmpeg_4_3"))]
    JEDEC_P22,
    #[cfg(feature = "ffmpeg_4_3")]
    EBU3213,
}

impl Primaries {
    #[cfg(feature = "ffmpeg_4_3")]
    pub const JEDEC_P22: Primaries = Primaries::EBU3213;

    pub fn name(&self) -> Option<&'static str> {
        if *self == Primaries::Unspecified {
            return None;
        }
        unsafe {
            let ptr = av_color_primaries_name((*self).into());
            ptr.as_ref()
                .map(|ptr| from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()))
        }
    }
}

impl From<AVColorPrimaries> for Primaries {
    fn from(value: AVColorPrimaries) -> Primaries {
        match value {
            AVCOL_PRI_RESERVED0 => Primaries::Reserved0,
            AVCOL_PRI_BT709 => Primaries::BT709,
            AVCOL_PRI_UNSPECIFIED => Primaries::Unspecified,
            AVCOL_PRI_RESERVED => Primaries::Reserved,
            AVCOL_PRI_BT470M => Primaries::BT470M,

            AVCOL_PRI_BT470BG => Primaries::BT470BG,
            AVCOL_PRI_SMPTE170M => Primaries::SMPTE170M,
            AVCOL_PRI_SMPTE240M => Primaries::SMPTE240M,
            AVCOL_PRI_FILM => Primaries::Film,
            AVCOL_PRI_BT2020 => Primaries::BT2020,
            AVCOL_PRI_NB => Primaries::Reserved0,

            AVCOL_PRI_SMPTE428 => Primaries::SMPTE428,
            AVCOL_PRI_SMPTE431 => Primaries::SMPTE431,
            AVCOL_PRI_SMPTE432 => Primaries::SMPTE432,
            #[cfg(not(feature = "ffmpeg_4_3"))]
            AVCOL_PRI_JEDEC_P22 => Primaries::JEDEC_P22,
            #[cfg(feature = "ffmpeg_4_3")]
            AVCOL_PRI_EBU3213 => Primaries::EBU3213,
        }
    }
}

impl From<Primaries> for AVColorPrimaries {
    fn from(value: Primaries) -> AVColorPrimaries {
        match value {
            Primaries::Reserved0 => AVCOL_PRI_RESERVED0,
            Primaries::BT709 => AVCOL_PRI_BT709,
            Primaries::Unspecified => AVCOL_PRI_UNSPECIFIED,
            Primaries::Reserved => AVCOL_PRI_RESERVED,
            Primaries::BT470M => AVCOL_PRI_BT470M,

            Primaries::BT470BG => AVCOL_PRI_BT470BG,
            Primaries::SMPTE170M => AVCOL_PRI_SMPTE170M,
            Primaries::SMPTE240M => AVCOL_PRI_SMPTE240M,
            Primaries::Film => AVCOL_PRI_FILM,
            Primaries::BT2020 => AVCOL_PRI_BT2020,

            Primaries::SMPTE428 => AVCOL_PRI_SMPTE428,
            Primaries::SMPTE431 => AVCOL_PRI_SMPTE431,
            Primaries::SMPTE432 => AVCOL_PRI_SMPTE432,
            #[cfg(not(feature = "ffmpeg_4_3"))]
            Primaries::JEDEC_P22 => AVCOL_PRI_JEDEC_P22,
            #[cfg(feature = "ffmpeg_4_3")]
            Primaries::EBU3213 => AVCOL_PRI_EBU3213,
        }
    }
}
