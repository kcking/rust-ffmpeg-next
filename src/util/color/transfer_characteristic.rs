use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use ffi::AVColorTransferCharacteristic::*;
use ffi::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum TransferCharacteristic {
    Reserved0,
    /** Also ITU-R BT1361. */
    BT709,
    Unspecified,
    Reserved,
    /** Also ITU-R BT470M / ITU-R BT1700 625 PAL & SECAM. */
    GAMMA22,
    /** Also ITU-R BT470BG. */
    GAMMA28,
    /**
    * Also ITU-R BT601-6 525 or 625
    * / ITU-R BT1358 525 or 625
    * / ITU-R BT1700 NTSC.
    */
    SMPTE170M,
    SMPTE240M,
    /** Linear transfer characteristics */
    Linear,
    /** Logarithmic transfer characteristic (100:1 range) */
    Log,
    /** Logarithmic transfer characteristic (100 * Sqrt(10) : 1 range) */
    LogSqrt,
    /** IEC 61966-2-4. */
    IEC61966_2_4,
    /** ITU-R BT1361 Extended Color Gamut. */
    BT1361_ECG,
    /** IEC 61966-2-1 (sRGB or sYCC). */
    IEC61966_2_1,
    /** ITU-R BT2020 for 10-bit system. */
    BT2020_10,
    /** ITU-R BT2020 for 12-bit system. */
    BT2020_12,
    /** SMPTE ST 2084 for 10-, 12-, 14-, and 16-bit systems. */
    SMPTE2084,
    /** SMPTE ST 428-1. */
    SMPTE428,
    /** ARIB STD-B67, known as "Hybrid log-gamma". */
    ARIB_STD_B67,
}

impl TransferCharacteristic {
    pub fn name(&self) -> Option<&'static str> {
        if *self == TransferCharacteristic::Unspecified {
            return None;
        }
        unsafe {
            let ptr = av_color_transfer_name((*self).into());
            ptr.as_ref()
                .map(|ptr| from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()))
        }
    }
}

impl From<AVColorTransferCharacteristic> for TransferCharacteristic {
    fn from(value: AVColorTransferCharacteristic) -> TransferCharacteristic {
        match value {
            AVCOL_TRC_RESERVED0 => TransferCharacteristic::Reserved0,
            AVCOL_TRC_BT709 => TransferCharacteristic::BT709,
            AVCOL_TRC_UNSPECIFIED => TransferCharacteristic::Unspecified,
            AVCOL_TRC_RESERVED => TransferCharacteristic::Reserved,
            AVCOL_TRC_GAMMA22 => TransferCharacteristic::GAMMA22,
            AVCOL_TRC_GAMMA28 => TransferCharacteristic::GAMMA28,
            AVCOL_TRC_SMPTE170M => TransferCharacteristic::SMPTE170M,
            AVCOL_TRC_SMPTE240M => TransferCharacteristic::SMPTE240M,
            AVCOL_TRC_LINEAR => TransferCharacteristic::Linear,
            AVCOL_TRC_LOG => TransferCharacteristic::Log,
            AVCOL_TRC_LOG_SQRT => TransferCharacteristic::LogSqrt,
            AVCOL_TRC_IEC61966_2_4 => TransferCharacteristic::IEC61966_2_4,
            AVCOL_TRC_BT1361_ECG => TransferCharacteristic::BT1361_ECG,
            AVCOL_TRC_IEC61966_2_1 => TransferCharacteristic::IEC61966_2_1,
            AVCOL_TRC_BT2020_10 => TransferCharacteristic::BT2020_10,
            AVCOL_TRC_BT2020_12 => TransferCharacteristic::BT2020_12,
            AVCOL_TRC_NB => TransferCharacteristic::Reserved0,
            AVCOL_TRC_SMPTE2084 => TransferCharacteristic::SMPTE2084,
            AVCOL_TRC_SMPTE428 => TransferCharacteristic::SMPTE428,
            AVCOL_TRC_ARIB_STD_B67 => TransferCharacteristic::ARIB_STD_B67,
        }
    }
}

impl From<TransferCharacteristic> for AVColorTransferCharacteristic {
    fn from(value: TransferCharacteristic) -> AVColorTransferCharacteristic {
        match value {
            TransferCharacteristic::Reserved0 => AVCOL_TRC_RESERVED0,
            TransferCharacteristic::BT709 => AVCOL_TRC_BT709,
            TransferCharacteristic::Unspecified => AVCOL_TRC_UNSPECIFIED,
            TransferCharacteristic::Reserved => AVCOL_TRC_RESERVED,
            TransferCharacteristic::GAMMA22 => AVCOL_TRC_GAMMA22,
            TransferCharacteristic::GAMMA28 => AVCOL_TRC_GAMMA28,
            TransferCharacteristic::SMPTE170M => AVCOL_TRC_SMPTE170M,
            TransferCharacteristic::SMPTE240M => AVCOL_TRC_SMPTE240M,
            TransferCharacteristic::Linear => AVCOL_TRC_LINEAR,
            TransferCharacteristic::Log => AVCOL_TRC_LOG,
            TransferCharacteristic::LogSqrt => AVCOL_TRC_LOG_SQRT,
            TransferCharacteristic::IEC61966_2_4 => AVCOL_TRC_IEC61966_2_4,
            TransferCharacteristic::BT1361_ECG => AVCOL_TRC_BT1361_ECG,
            TransferCharacteristic::IEC61966_2_1 => AVCOL_TRC_IEC61966_2_1,
            TransferCharacteristic::BT2020_10 => AVCOL_TRC_BT2020_10,
            TransferCharacteristic::BT2020_12 => AVCOL_TRC_BT2020_12,
            TransferCharacteristic::SMPTE2084 => AVCOL_TRC_SMPTE2084,
            TransferCharacteristic::SMPTE428 => AVCOL_TRC_SMPTE428,
            TransferCharacteristic::ARIB_STD_B67 => AVCOL_TRC_ARIB_STD_B67,
        }
    }
}
