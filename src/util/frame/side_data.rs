use std::ffi::CStr;
use std::marker::PhantomData;
use std::slice;
use std::str::from_utf8_unchecked;

use super::Frame;
use ffi::AVFrameSideDataType::*;
use ffi::*;
use DictionaryRef;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Type {
    /** The data is the AVPanScan struct defined in libavcodec. */
    PanScan,
    /** ATSC A53 Part 4 Closed Captions. */
    A53CC,
    /** Stereoscopic 3d metadata. */
    Stereo3D,
    MatrixEncoding,
    /** Metadata relevant to a downmix procedure. */
    DownMixInfo,
    /** Replay gani information in the form of the AVReplayGain struct. */
    ReplayGain,
    /**
     * This side data contains a 3x3 transformation matrix describing
     * an affine transformation that needs to be applied to the frame
     * for correct presentation.
     *
     * See libavutil/display.h for a detailed description of the data.
     */
    DisplayMatrix,
    /** Active Format Description data consisting of a single byte. */
    AFD,
    /**
     * Motion vectors exported by some codecs (on demand through the
     * export_mvs flag set in libavcodec CodecContext flags2 option).
     */
    MotionVectors,
    /**
     * Reccomend skipping the specified number of samples.
     *
     * This is exported only if the "skip_manual" AVOption is set in
     * libavcodec. This has the same format as TODO: AV_PKT_DATA_SKIP_SAMPLES.
     */
    SkipSamples,
    /**
     * This side data must be associated with an audio frame and
     * corresponds to the enum audio::audio_service::AudioService.
     */
    AudioServiceType,
    /** Mastering display metadata associated with a video frame. */
    MasteringDisplayMetadata,
    /** The GOP timecode in 25-bit timecode format.
     *
     * Data format is 64-bit integer. This is set on the first frame
     * of a GOP that has a temporal reference of 0.
     */
    GOPTimecode,
    /** The data represents TODO: AVSphericalMapping. */
    Spherical,

    /** Content light level (based on CTA-861.3). */
    ContentLightLevel,
    /** The data contains an ICC profile as an opaque octet buffer
     * following the same format described by ISO 15076-1 with an
     * optional name defined in the metadata key entry "name". */
    IccProfile,

    #[cfg(feature = "ffmpeg_4_0")]
    QPTableProperties,
    #[cfg(feature = "ffmpeg_4_0")]
    QPTableData,

    #[cfg(feature = "ffmpeg_4_1")]
    S12M_TIMECODE,

    #[cfg(feature = "ffmpeg_4_2")]
    DYNAMIC_HDR_PLUS,
    #[cfg(feature = "ffmpeg_4_2")]
    REGIONS_OF_INTEREST,

    #[cfg(feature = "ffmpeg_4_3")]
    /** Encoding parameters for a video frame, as described by TODO:
     * AVVideoEncParams. */
    VIDEO_ENC_PARAMS,

    #[cfg(feature = "ffmpeg_4_4")]
    /**
     * User data unregistered metadata associated with a video frame.
     */
    SEI_UNREGISTERED,
    #[cfg(feature = "ffmpeg_4_4")]
    FILM_GRAIN_PARAMS,
}

impl Type {
    #[inline]
    pub fn name(&self) -> &'static str {
        unsafe {
            from_utf8_unchecked(CStr::from_ptr(av_frame_side_data_name((*self).into())).to_bytes())
        }
    }
}

impl From<AVFrameSideDataType> for Type {
    #[inline(always)]
    fn from(value: AVFrameSideDataType) -> Self {
        match value {
            AV_FRAME_DATA_PANSCAN => Type::PanScan,
            AV_FRAME_DATA_A53_CC => Type::A53CC,
            AV_FRAME_DATA_STEREO3D => Type::Stereo3D,
            AV_FRAME_DATA_MATRIXENCODING => Type::MatrixEncoding,
            AV_FRAME_DATA_DOWNMIX_INFO => Type::DownMixInfo,
            AV_FRAME_DATA_REPLAYGAIN => Type::ReplayGain,
            AV_FRAME_DATA_DISPLAYMATRIX => Type::DisplayMatrix,
            AV_FRAME_DATA_AFD => Type::AFD,
            AV_FRAME_DATA_MOTION_VECTORS => Type::MotionVectors,
            AV_FRAME_DATA_SKIP_SAMPLES => Type::SkipSamples,
            AV_FRAME_DATA_AUDIO_SERVICE_TYPE => Type::AudioServiceType,
            AV_FRAME_DATA_MASTERING_DISPLAY_METADATA => Type::MasteringDisplayMetadata,
            AV_FRAME_DATA_GOP_TIMECODE => Type::GOPTimecode,
            AV_FRAME_DATA_SPHERICAL => Type::Spherical,

            AV_FRAME_DATA_CONTENT_LIGHT_LEVEL => Type::ContentLightLevel,
            AV_FRAME_DATA_ICC_PROFILE => Type::IccProfile,

            #[cfg(feature = "ffmpeg_4_0")]
            AV_FRAME_DATA_QP_TABLE_PROPERTIES => Type::QPTableProperties,
            #[cfg(feature = "ffmpeg_4_0")]
            AV_FRAME_DATA_QP_TABLE_DATA => Type::QPTableData,

            #[cfg(feature = "ffmpeg_4_1")]
            AV_FRAME_DATA_S12M_TIMECODE => Type::S12M_TIMECODE,

            #[cfg(feature = "ffmpeg_4_2")]
            AV_FRAME_DATA_DYNAMIC_HDR_PLUS => Type::DYNAMIC_HDR_PLUS,
            #[cfg(feature = "ffmpeg_4_2")]
            AV_FRAME_DATA_REGIONS_OF_INTEREST => Type::REGIONS_OF_INTEREST,

            #[cfg(feature = "ffmpeg_4_3")]
            AV_FRAME_DATA_VIDEO_ENC_PARAMS => Type::VIDEO_ENC_PARAMS,

            #[cfg(feature = "ffmpeg_4_4")]
            AV_FRAME_DATA_SEI_UNREGISTERED => Type::SEI_UNREGISTERED,
            #[cfg(feature = "ffmpeg_4_4")]
            AV_FRAME_DATA_FILM_GRAIN_PARAMS => Type::FILM_GRAIN_PARAMS,
        }
    }
}

impl From<Type> for AVFrameSideDataType {
    #[inline(always)]
    fn from(value: Type) -> AVFrameSideDataType {
        match value {
            Type::PanScan => AV_FRAME_DATA_PANSCAN,
            Type::A53CC => AV_FRAME_DATA_A53_CC,
            Type::Stereo3D => AV_FRAME_DATA_STEREO3D,
            Type::MatrixEncoding => AV_FRAME_DATA_MATRIXENCODING,
            Type::DownMixInfo => AV_FRAME_DATA_DOWNMIX_INFO,
            Type::ReplayGain => AV_FRAME_DATA_REPLAYGAIN,
            Type::DisplayMatrix => AV_FRAME_DATA_DISPLAYMATRIX,
            Type::AFD => AV_FRAME_DATA_AFD,
            Type::MotionVectors => AV_FRAME_DATA_MOTION_VECTORS,
            Type::SkipSamples => AV_FRAME_DATA_SKIP_SAMPLES,
            Type::AudioServiceType => AV_FRAME_DATA_AUDIO_SERVICE_TYPE,
            Type::MasteringDisplayMetadata => AV_FRAME_DATA_MASTERING_DISPLAY_METADATA,
            Type::GOPTimecode => AV_FRAME_DATA_GOP_TIMECODE,
            Type::Spherical => AV_FRAME_DATA_SPHERICAL,

            Type::ContentLightLevel => AV_FRAME_DATA_CONTENT_LIGHT_LEVEL,
            Type::IccProfile => AV_FRAME_DATA_ICC_PROFILE,

            #[cfg(feature = "ffmpeg_4_0")]
            Type::QPTableProperties => AV_FRAME_DATA_QP_TABLE_PROPERTIES,
            #[cfg(feature = "ffmpeg_4_0")]
            Type::QPTableData => AV_FRAME_DATA_QP_TABLE_DATA,

            #[cfg(feature = "ffmpeg_4_1")]
            Type::S12M_TIMECODE => AV_FRAME_DATA_S12M_TIMECODE,

            #[cfg(feature = "ffmpeg_4_2")]
            Type::DYNAMIC_HDR_PLUS => AV_FRAME_DATA_DYNAMIC_HDR_PLUS,
            #[cfg(feature = "ffmpeg_4_2")]
            Type::REGIONS_OF_INTEREST => AV_FRAME_DATA_REGIONS_OF_INTEREST,

            #[cfg(feature = "ffmpeg_4_3")]
            Type::VIDEO_ENC_PARAMS => AV_FRAME_DATA_VIDEO_ENC_PARAMS,

            #[cfg(feature = "ffmpeg_4_4")]
            Type::SEI_UNREGISTERED => AV_FRAME_DATA_SEI_UNREGISTERED,
            #[cfg(feature = "ffmpeg_4_4")]
            Type::FILM_GRAIN_PARAMS => AV_FRAME_DATA_FILM_GRAIN_PARAMS,
        }
    }
}

pub struct SideData<'a> {
    ptr: *mut AVFrameSideData,

    _marker: PhantomData<&'a Frame>,
}

impl<'a> SideData<'a> {
    #[inline(always)]
    pub unsafe fn wrap(ptr: *mut AVFrameSideData) -> Self {
        SideData {
            ptr,
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    pub unsafe fn as_ptr(&self) -> *const AVFrameSideData {
        self.ptr as *const _
    }

    #[inline(always)]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFrameSideData {
        self.ptr
    }
}

impl<'a> SideData<'a> {
    #[inline]
    pub fn kind(&self) -> Type {
        unsafe { Type::from((*self.as_ptr()).type_) }
    }

    #[inline]
    pub fn data(&self) -> &[u8] {
        unsafe { slice::from_raw_parts((*self.as_ptr()).data, (*self.as_ptr()).size as usize) }
    }

    #[inline]
    pub fn metadata(&self) -> DictionaryRef {
        unsafe { DictionaryRef::wrap((*self.as_ptr()).metadata) }
    }
}
