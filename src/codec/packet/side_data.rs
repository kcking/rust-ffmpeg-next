use std::marker::PhantomData;
use std::slice;

use super::Packet;
use ffi::AVPacketSideDataType::*;
use ffi::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Type {
    /** A `Type::Palette` side data packet contains exactly
     * `AVPALETTE_SIZE bytes worth of palette.
     *
     * This side data signals that a new palette is present. */
    Palette,
    /** The `Type::NewExtraData` is used to notify the codec or the
     * format that the extradata buffer was changed and the receiving
     * side should act upon it appropriately.
     *
     * The new extradata is embedded in the side data buffer and
     * should be immediately used for processing the current frame or
     * packet. */
    NewExtraData,
    /** A `Type::ParamChange` side data packet is laid out as follows:
     *
     * ```
     * u32le param_flags
     * if (param_flags & AV_SIDE_DATA_PARAM_CHANGE_SAMPLE_RATE)
     *     s32le sample_rate
     * if (param_flags & AV_SIDE_DATA_PARAM_CHANGE_DIMENSIONS)
     *     s32le width
     *     s32le height
     * ```
     */
    ParamChange,
    /** A `Type::H263MbInfo` side data packet contains a number of
     * structures with info about macroblocks relevant to splitting
     * the packet into smaller packets on macroblock edges (e.g.
     * as for RFC 2190). That is, it does not necessarily contain info
     * about all macroblocks, as long as the distance between
     * macroblocks in the info is smaller than the target payload
     * size. Each MB info structure is 12 bytes, and is laid out as
     * follows:
     *
     * ```
     * u32le bit offset from the start of the packet
     * u8    current quantizer at the start of the macroblock
     * u8    GOB number
     * u16le macroblock address within the GOB
     * u8    horizontal MV predictor
     * u8    vertical MV predictor
     * u8    horizontal MV predictor for block number 3
     * u8    vertical MV predictor for block number 3
     * ```
     */
    H263MbInfo,
    /** This side data should be associated with an audio stream and contains ReplayGain
     * information in form of the AVReplayGain struct. */
    ReplayGain,
    /** This side data contains a 3x3 transformation matrix describing an affine transformation
     * that needs to be applied to the decoded video frames for correct presentation. */
    DisplayMatrix,
    /** This side data should be associated with a video stream and contains Stereoscopic 3D
     * information in form of the AVStereo3D struct. */
    Stereo3d,
    /** This side data should be associated with an audio stream and corresponds to enum
     * AVAudioServiceType. */
    AudioServiceType,
    /** This side data contains quality related information from the encoder.
     *
     * ```
     * u32le quality factor of the compressed frame. Allowed range is between 1 (good) and FF_LAMBDA_MAX (bad).
     * u8    picture type
     * u8    error count
     * u16   reserved
     * u64le[error count] sum of squared differences between encoder in and output
     * ```
     */
    QualityStats,
    /** This side data contains an integer value representing the stream index of a "fallback" track.
    *
    * A fallback track indicates an alternate track to use when the current track can not be decoded for some reason. e.g. no decoder available for codec. */
    FallbackTrack,
    /** This side data corresponds to the AVCPBProperties struct. */
    CBPProperties,
    /** Recommmends skipping the specified number of samples.
     *
     * ```
     * u32le number of samples to skip from start of this packet
     * u32le number of samples to skip from end of this packet
     * u8    reason for start skip
     * u8    reason for end   skip (0=padding silence, 1=convergence)
     * ```
     */
    SkipSamples,
    /** A `Type::JpDualMono` side data packet [200~indicates that the packet may contain "dual
     * mono" audio specific to Japanese DTV and if it is true, recommends only the selected channel
     * to be used.
     *
     * ```
     * u8    selected channels (0=main/left, 1=sub/right, 2=both)
     * ```
     */
    JpDualMono,
    /** 	
     * A list of zero terminated key/value strings.
     * 
     * There is no end marker for the list, so it is required to rely
     * on the side data size to stop. */
    StringsMetadata,
    /**
     * Subtitle event position.
     *
     * ```
     * u32le x1
     * u32le y1
     * u32le x2
     * u32le y2
     * ```
     */
    SubtitlePosition,
    /** Data found in BlockAdditional element of matroska container.
     *
     * There is no end marker for the data, so it is required to rely on the side data size to recognize the end. 8 byte id (as found in BlockAddId) followed by data. */
    MatroskaBlockAdditional,
    /** The optional first identifier line of a WebVTT cue. */
    WebVTTIdentifier,
    /** The optional settings (rendering instructions) that immediately follow the timestamp
     * specifier of a WebVTT cue. */
    WebVTTSettings,
    /** A list of zero terminated key/value strings.

There is no end marker for the list, so it is required to rely on the side data size to stop. This side data includes updated metadata which appeared in the stream. */
    MetadataUpdate,
    /** MPEGTS stream ID as uint8_t, this is required to pass the stream ID information from the
     * demuxer to the corresponding muxer. */
    MPEGTSStreamID,
    /** Mastering display metadata (based on SMPTE-2086:2014).
     *
     * This metadata should be associated with a video stream and contains data in the form of the AVMasteringDisplayMetadata struct. */
    MasteringDisplayMetadata,
    /** This side data should be associated with a video stream and corresponds to the
     * AVSphericalMapping structure. */
    DataSpherical,
    DataNb,

    /** Content light level (based on CTA-861.3).
     *
     * This metadata should be associated with a video stream and contains data in the form of the AVContentLightMetadata struct. */
    ContentLightLevel,
    /** ATSC A53 Part 4 Closed Captions.
    *
    * This metadata should be associated with a video stream. A53 CC bitstream is stored as uint8_t in AVPacketSideData.data. The number of bytes of CC data is AVPacketSideData.size. */
    A53CC,

    #[cfg(feature = "ffmpeg_4_0")]
    /**
     * This side data is encryption initialization data.
     *
     * The format is not part of ABI, use av_encryption_init_info_* methods to access. 
    */
    EncryptionInitInfo,
    #[cfg(feature = "ffmpeg_4_0")]
    /** This side data contains encryption info for how to decrypt the packet.
     *
     * The format is not part of ABI, use av_encryption_info_* methods to access. */
    EncryptionInfo,

    #[cfg(feature = "ffmpeg_4_1")]
    /** Active Format Description data consisting of a single byte as specified in ETSI TS 101 154
     * using AVActiveFormatDescription enum. */
    AFD,

    #[cfg(feature = "ffmpeg_4_3")]
    /** Producer Reference Time data corresponding to the AVProducerReferenceTime struct, usually
     * exported by some encoders (on demand through the prft flag set in the AVCodecContext
     * export_side_data field). */
    PRFT,
    #[cfg(feature = "ffmpeg_4_3")]
    /** ICC profile data consisting of an opaque octet buffer following the format described by ISO
     * 15076-1. */
    ICC_PROFILE,
    #[cfg(feature = "ffmpeg_4_3")]
    /** DOVI configuration ref:
     * dolby-vision-bitstreams-within-the-iso-base-media-file-format-v2.1.2, section 2.2
     * dolby-vision-bitstreams-in-mpeg-2-transport-stream-multiplex-v1.2, section 3.3 Tags are
     * stored in struct AVDOVIDecoderConfigurationRecord. */
    DOVI_CONF,

    #[cfg(feature = "ffmpeg_4_4")]
    /** Timecode which conforms to SMPTE ST 12-1:2014.
     *
     * The data is an array of 4 uint32_t where the first uint32_t describes how many (1-3) of the other timecodes are used. The timecode format is described in the documentation of av_timecode_get_smpte_from_framenum() function in libavutil/timecode.h. */
    S12M_TIMECODE,
}

impl From<AVPacketSideDataType> for Type {
    fn from(value: AVPacketSideDataType) -> Self {
        match value {
            AV_PKT_DATA_PALETTE => Type::Palette,
            AV_PKT_DATA_NEW_EXTRADATA => Type::NewExtraData,
            AV_PKT_DATA_PARAM_CHANGE => Type::ParamChange,
            AV_PKT_DATA_H263_MB_INFO => Type::H263MbInfo,
            AV_PKT_DATA_REPLAYGAIN => Type::ReplayGain,
            AV_PKT_DATA_DISPLAYMATRIX => Type::DisplayMatrix,
            AV_PKT_DATA_STEREO3D => Type::Stereo3d,
            AV_PKT_DATA_AUDIO_SERVICE_TYPE => Type::AudioServiceType,
            AV_PKT_DATA_QUALITY_STATS => Type::QualityStats,
            AV_PKT_DATA_FALLBACK_TRACK => Type::FallbackTrack,
            AV_PKT_DATA_CPB_PROPERTIES => Type::CBPProperties,
            AV_PKT_DATA_SKIP_SAMPLES => Type::SkipSamples,
            AV_PKT_DATA_JP_DUALMONO => Type::JpDualMono,
            AV_PKT_DATA_STRINGS_METADATA => Type::StringsMetadata,
            AV_PKT_DATA_SUBTITLE_POSITION => Type::SubtitlePosition,
            AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL => Type::MatroskaBlockAdditional,
            AV_PKT_DATA_WEBVTT_IDENTIFIER => Type::WebVTTIdentifier,
            AV_PKT_DATA_WEBVTT_SETTINGS => Type::WebVTTSettings,
            AV_PKT_DATA_METADATA_UPDATE => Type::MetadataUpdate,
            AV_PKT_DATA_MPEGTS_STREAM_ID => Type::MPEGTSStreamID,
            AV_PKT_DATA_MASTERING_DISPLAY_METADATA => Type::MasteringDisplayMetadata,
            AV_PKT_DATA_SPHERICAL => Type::DataSpherical,
            AV_PKT_DATA_NB => Type::DataNb,

            AV_PKT_DATA_CONTENT_LIGHT_LEVEL => Type::ContentLightLevel,
            AV_PKT_DATA_A53_CC => Type::A53CC,

            #[cfg(feature = "ffmpeg_4_0")]
            AV_PKT_DATA_ENCRYPTION_INIT_INFO => Type::EncryptionInitInfo,
            #[cfg(feature = "ffmpeg_4_0")]
            AV_PKT_DATA_ENCRYPTION_INFO => Type::EncryptionInfo,

            #[cfg(feature = "ffmpeg_4_1")]
            AV_PKT_DATA_AFD => Type::AFD,

            #[cfg(feature = "ffmpeg_4_3")]
            AV_PKT_DATA_PRFT => Type::PRFT,
            #[cfg(feature = "ffmpeg_4_3")]
            AV_PKT_DATA_ICC_PROFILE => Type::ICC_PROFILE,
            #[cfg(feature = "ffmpeg_4_3")]
            AV_PKT_DATA_DOVI_CONF => Type::DOVI_CONF,

            #[cfg(feature = "ffmpeg_4_4")]
            AV_PKT_DATA_S12M_TIMECODE => Type::S12M_TIMECODE,
        }
    }
}

impl From<Type> for AVPacketSideDataType {
    fn from(value: Type) -> AVPacketSideDataType {
        match value {
            Type::Palette => AV_PKT_DATA_PALETTE,
            Type::NewExtraData => AV_PKT_DATA_NEW_EXTRADATA,
            Type::ParamChange => AV_PKT_DATA_PARAM_CHANGE,
            Type::H263MbInfo => AV_PKT_DATA_H263_MB_INFO,
            Type::ReplayGain => AV_PKT_DATA_REPLAYGAIN,
            Type::DisplayMatrix => AV_PKT_DATA_DISPLAYMATRIX,
            Type::Stereo3d => AV_PKT_DATA_STEREO3D,
            Type::AudioServiceType => AV_PKT_DATA_AUDIO_SERVICE_TYPE,
            Type::QualityStats => AV_PKT_DATA_QUALITY_STATS,
            Type::FallbackTrack => AV_PKT_DATA_FALLBACK_TRACK,
            Type::CBPProperties => AV_PKT_DATA_CPB_PROPERTIES,
            Type::SkipSamples => AV_PKT_DATA_SKIP_SAMPLES,
            Type::JpDualMono => AV_PKT_DATA_JP_DUALMONO,
            Type::StringsMetadata => AV_PKT_DATA_STRINGS_METADATA,
            Type::SubtitlePosition => AV_PKT_DATA_SUBTITLE_POSITION,
            Type::MatroskaBlockAdditional => AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL,
            Type::WebVTTIdentifier => AV_PKT_DATA_WEBVTT_IDENTIFIER,
            Type::WebVTTSettings => AV_PKT_DATA_WEBVTT_SETTINGS,
            Type::MetadataUpdate => AV_PKT_DATA_METADATA_UPDATE,
            Type::MPEGTSStreamID => AV_PKT_DATA_MPEGTS_STREAM_ID,
            Type::MasteringDisplayMetadata => AV_PKT_DATA_MASTERING_DISPLAY_METADATA,
            Type::DataSpherical => AV_PKT_DATA_SPHERICAL,
            Type::DataNb => AV_PKT_DATA_NB,

            Type::ContentLightLevel => AV_PKT_DATA_CONTENT_LIGHT_LEVEL,
            Type::A53CC => AV_PKT_DATA_A53_CC,

            #[cfg(feature = "ffmpeg_4_0")]
            Type::EncryptionInitInfo => AV_PKT_DATA_ENCRYPTION_INIT_INFO,
            #[cfg(feature = "ffmpeg_4_0")]
            Type::EncryptionInfo => AV_PKT_DATA_ENCRYPTION_INFO,

            #[cfg(feature = "ffmpeg_4_1")]
            Type::AFD => AV_PKT_DATA_AFD,

            #[cfg(feature = "ffmpeg_4_3")]
            Type::PRFT => AV_PKT_DATA_PRFT,
            #[cfg(feature = "ffmpeg_4_3")]
            Type::ICC_PROFILE => AV_PKT_DATA_ICC_PROFILE,
            #[cfg(feature = "ffmpeg_4_3")]
            Type::DOVI_CONF => AV_PKT_DATA_DOVI_CONF,

            #[cfg(feature = "ffmpeg_4_4")]
            Type::S12M_TIMECODE => AV_PKT_DATA_S12M_TIMECODE,
        }
    }
}

pub struct SideData<'a> {
    ptr: *mut AVPacketSideData,

    _marker: PhantomData<&'a Packet>,
}

impl<'a> SideData<'a> {
    pub unsafe fn wrap(ptr: *mut AVPacketSideData) -> Self {
        SideData {
            ptr,
            _marker: PhantomData,
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVPacketSideData {
        self.ptr as *const _
    }
}

impl<'a> SideData<'a> {
    pub fn kind(&self) -> Type {
        unsafe { Type::from((*self.as_ptr()).type_) }
    }

    pub fn data(&self) -> &[u8] {
        unsafe { slice::from_raw_parts((*self.as_ptr()).data, (*self.as_ptr()).size as usize) }
    }
}
