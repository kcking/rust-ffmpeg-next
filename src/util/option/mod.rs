mod traits;
pub use self::traits::{Gettable, Iterable, Settable, Target};

use ffi::AVOptionType::*;
use ffi::*;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Type {
    /** Underlying C type is `unsigned int`. */
    Flags,
    /** Underlying C type is `int`. */
    Int,
    /** Underlying C type is `int64_t`. */
    Int64,
    /** Underlying C type is `double`. */
    Double,
    /** Underlying C type is `float`. */
    Float,
    /** Underlying C type is a `uint8_t*` that is either `NULL` or points to a C
     * string allocated with the `av_malloc()` family of functions. */
    String,
    /** Underlying C type is `AVRational` aka `Rational`. */
    Rational,
    /** Underlying C type is a `uint8_t*` that is either `NULL` or points to an
     * array allocated with the `av_malloc()` family of functions.
     *
     * The pointer is immediately followed by an int containing the array length
     * in bytes. */
    Binary,
    /** Underlying C type is `AVDictionary` aka `Dictionary`. */
    Dictionary,
    /** Underlying C type is `uint64_t`. */
    Constant,

    /** Underlying C type is two consecutive integers. */
    ImageSize,
    /** Underlying C type is `AVPixelFormat` aka `Pixel`. */
    PixelFormat,
    /** Underlying C type is `AVSampleFormat` aka `Sample`. */
    SampleFormat,
    /** Underlying C type is `AVRational` aka `Rational`. */
    VideoRate,
    /** Underlying C type is `int64_t`. */
    Duration,
    /** Underlying C type is `uint8_t[4]`. */
    Color,
    /** Underlying C type is `AVChannelLayout`. */
    ChannelLayout,
    /** Underlying C type is `unsigned int`. */
    c_ulong,
    /** Underlying C type is `int`. */
    bool,
}

impl From<AVOptionType> for Type {
    fn from(value: AVOptionType) -> Self {
        match value {
            AV_OPT_TYPE_FLAGS => Type::Flags,
            AV_OPT_TYPE_INT => Type::Int,
            AV_OPT_TYPE_INT64 => Type::Int64,
            AV_OPT_TYPE_DOUBLE => Type::Double,
            AV_OPT_TYPE_FLOAT => Type::Float,
            AV_OPT_TYPE_STRING => Type::String,
            AV_OPT_TYPE_RATIONAL => Type::Rational,
            AV_OPT_TYPE_BINARY => Type::Binary,
            AV_OPT_TYPE_DICT => Type::Dictionary,
            AV_OPT_TYPE_CONST => Type::Constant,
            AV_OPT_TYPE_UINT64 => Type::c_ulong,
            AV_OPT_TYPE_BOOL => Type::bool,

            AV_OPT_TYPE_IMAGE_SIZE => Type::ImageSize,
            AV_OPT_TYPE_PIXEL_FMT => Type::PixelFormat,
            AV_OPT_TYPE_SAMPLE_FMT => Type::SampleFormat,
            AV_OPT_TYPE_VIDEO_RATE => Type::VideoRate,
            AV_OPT_TYPE_DURATION => Type::Duration,
            AV_OPT_TYPE_COLOR => Type::Color,
            AV_OPT_TYPE_CHANNEL_LAYOUT => Type::ChannelLayout,
        }
    }
}

impl From<Type> for AVOptionType {
    fn from(value: Type) -> AVOptionType {
        match value {
            Type::Flags => AV_OPT_TYPE_FLAGS,
            Type::Int => AV_OPT_TYPE_INT,
            Type::Int64 => AV_OPT_TYPE_INT64,
            Type::Double => AV_OPT_TYPE_DOUBLE,
            Type::Float => AV_OPT_TYPE_FLOAT,
            Type::String => AV_OPT_TYPE_STRING,
            Type::Rational => AV_OPT_TYPE_RATIONAL,
            Type::Binary => AV_OPT_TYPE_BINARY,
            Type::Dictionary => AV_OPT_TYPE_DICT,
            Type::Constant => AV_OPT_TYPE_CONST,
            Type::c_ulong => AV_OPT_TYPE_UINT64,
            Type::bool => AV_OPT_TYPE_BOOL,

            Type::ImageSize => AV_OPT_TYPE_IMAGE_SIZE,
            Type::PixelFormat => AV_OPT_TYPE_PIXEL_FMT,
            Type::SampleFormat => AV_OPT_TYPE_SAMPLE_FMT,
            Type::VideoRate => AV_OPT_TYPE_VIDEO_RATE,
            Type::Duration => AV_OPT_TYPE_DURATION,
            Type::Color => AV_OPT_TYPE_COLOR,
            Type::ChannelLayout => AV_OPT_TYPE_CHANNEL_LAYOUT,
        }
    }
}
