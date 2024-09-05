use std::error;
use std::ffi::{CStr, CString, NulError};
use std::fmt;
use std::str::{from_utf8_unchecked, FromStr};

use ffi::AVPixelFormat::*;
use ffi::*;

/**
 * Corresponds to AVPixelFormat.
 * Designates a pixel format.
 */
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Pixel {
    None,

    /** planar YUV 4:2:0, 12bpp, (1 Cr & Cb sample per 2x2 Y samples). */
    YUV420P,
    /** packed YUV 4:2:2, 16bpp, Y0 Cb Y1 Cr. */
    YUYV422,
    /** packed RGB 8:8:8, 24bpp, RGBRGB... */
    RGB24,
    /** packed RGB 8:8:8, 24bpp, BGRBGR...  */
    BGR24,
    /** planar YUV 4:2:2, 16bpp, (1 Cr & Cb sample per 2x1 Y samples). */
    YUV422P,
    /** planar YUV 4:4:4, 24bpp, (1 Cr & Cb sample per 1x1 Y samples). */
    YUV444P,
    /** planar YUV 4:1:0, 9bpp, (1 Cr & Cb sample per 4x4 Y samples). */
    YUV410P,
    /** planar YUV 4:1:1, 12bpp, (1 Cr & Cb sample per 4x1 Y samples). */
    YUV411P,
    /** Y , 8bpp. */
    GRAY8,
    /** Y , 1bpp, 0 is white, 1 is black, in each byte pixels are ordered from
     * the msb to the lsb. */
    MonoWhite,
    /**
     * Y , 1bpp, 0 is black, 1 is white, in each byte pixels are ordered from
     * the msb to the lsb.
     */
    MonoBlack,
    /** 8 bits with AV_PIX_FMT_RGB32 palette. */
    PAL8,
    /**
     * planar YUV 4:2:0, 12bpp, full scale (JPEG), deprecated in favor of
     * Pixel::YUV420P and setting color_range.
     */
    YUVJ420P,
    /**
     * planar YUV 4:2:2, 16bpp, full scale (JPEG), deprecated in favor of
     * Pixel::YUV422P and setting color_range.
     */
    YUVJ422P,
    /**
     * planar YUV 4:4:4, 24bpp, full scale (JPEG), deprecated in favor of
     * Pixel::YUV444P and setting color_range.
     */
    YUVJ444P,
    #[cfg(feature = "ff_api_xvmc")]
    XVMC_MPEG2_MC,
    #[cfg(feature = "ff_api_xvmc")]
    XVMC_MPEG2_IDCT,
    /** packed YUV 4:2:2, 16bpp, Cb Y0 Cr Y1. */
    UYVY422,
    /** packed YUV 4:1:1, 12bpp, Cb Y0 Y1 Cr Y2 Y3. */
    UYYVYY411,
    /** packed RGB 3:3:2, 8bpp, (msb)2B 3G 3R(lsb). */
    BGR8,
    /**
     * packed RGB 1:2:1 bitstream, 4bpp, (msb)1B 2G 1R(lsb), a byte contains two
     * pixels, the first pixel in the byte is the one composed by the 4 msb bits.
     */
    BGR4,
    /** packed RGB 1:2:1, 8bpp, (msb)1B 2G 1R(lsb). */
    BGR4_BYTE,
    /** packed RGB 3:3:2, 8bpp, (msb)3R 3G 2B(lsb). */
    RGB8,
    /**
     * packed RGB 1:2:1 bitstream, 4bpp, (msb)1R 2G 1B(lsb), a byte contains two
     * pixels, the first pixel in the byte is the one composed by the 4 msb bits.
     */
    RGB4,
    /** packed RGB 1:2:1, 8bpp, (msb)1R 2G 1B(lsb). */
    RGB4_BYTE,
    /**
     * planar YUV 4:2:0, 12bpp, 1 plane for Y and 1 plane for the UV components,
     * which are interleaved (first byte U and the following byte V).
     */
    NV12,
    /** same as Pixel::NV12, but U and V bytes are swapped. */
    NV21,

    /** packed ARGB 8:8:8:8, 32bpp, ARGBARGB... */
    ARGB,
    /** packed RGBA 8:8:8:8, 32bpp, RGBARGBA... */
    RGBA,
    /** packed ABGR 8:8:8:8, 32bpp, ABGRABGR... */
    ABGR,
    /** packed BGRA 8:8:8:8, 32bpp, BGRABGRA... */
    BGRA,

    /** Y , 16bpp, big-endian. */
    GRAY16BE,
    /** Y , 16bpp, little-endian. */
    GRAY16LE,
    /** planar YUV 4:4:0 (1 Cr & Cb sample per 1x2 Y samples). */
    YUV440P,
    /**
     * planar YUV 4:4:0 full scale (JPEG), deprecated in favor of Pixel::YUV440P
     * and setting color_range.
     */
    YUVJ440P,
    /** planar YUV 4:2:0, 20bpp, (1 Cr & Cb sample per 2x2 Y & A samples). */
    YUVA420P,
    #[cfg(feature = "ff_api_vdpau")]
    VDPAU_H264,
    #[cfg(feature = "ff_api_vdpau")]
    VDPAU_MPEG1,
    #[cfg(feature = "ff_api_vdpau")]
    VDPAU_MPEG2,
    #[cfg(feature = "ff_api_vdpau")]
    VDPAU_WMV3,
    #[cfg(feature = "ff_api_vdpau")]
    VDPAU_VC1,
    /**
     * packed RGB 16:16:16, 48bpp, 16R, 16G, 16B, the 2-byte value for each R/G/B
     * component is stored as big-endian.
     */
    RGB48BE,
    /**
     * packed RGB 16:16:16, 48bpp, 16R, 16G, 16B, the 2-byte value for each R/G/B
     * component is stored as little-endian.
     */
    RGB48LE,

    /** packed RGB 5:6:5, 16bpp, (msb) 5R 6G 5B(lsb), big-endian. */
    RGB565BE,
    /** packed RGB 5:6:5, 16bpp, (msb) 5R 6G 5B(lsb), little-endian. */
    RGB565LE,
    /** packed RGB 5:5:5, 16bpp, (msb)1X 5R 5G 5B(lsb), big-endian ,
     * X=unused/undefined. */
    RGB555BE,
    /** packed RGB 5:5:5, 16bpp, (msb)1X 5R 5G 5B(lsb), little-endian,
     * X=unused/undefined. */
    RGB555LE,

    /** packed BGR 5:6:5, 16bpp, (msb) 5B 6G 5R(lsb), big-endian. */
    BGR565BE,
    /** packed BGR 5:6:5, 16bpp, (msb) 5B 6G 5R(lsb), little-endian. */
    BGR565LE,
    /** packed BGR 5:5:5, 16bpp, (msb)1X 5B 5G 5R(lsb), big-endian ,
     * X=unused/undefined. */
    BGR555BE,
    /** packed BGR 5:5:5, 16bpp, (msb)1X 5B 5G 5R(lsb), little-endian,
     * X=unused/undefined. */
    BGR555LE,

    #[cfg(feature = "ff_api_vaapi")]
    VAAPI_MOCO,
    #[cfg(feature = "ff_api_vaapi")]
    VAAPI_IDCT,
    #[cfg(feature = "ff_api_vaapi")]
    VAAPI_VLD,
    #[cfg(not(feature = "ff_api_vaapi"))]
    /** Hardware acceleration through VA-API. */
    VAAPI,

    /** planar YUV 4:2:0, 24bpp, (1 Cr & Cb sample per 2x2 Y samples),
     * little-endian. */
    YUV420P16LE,
    /**
     * planar YUV 4:2:0, 24bpp, (1 Cr & Cb sample per 2x2 Y samples), big-endian.
     */
    YUV420P16BE,
    /**
     * planar YUV 4:2:2, 32bpp, (1 Cr & Cb sample per 2x1 Y samples),
     * little-endian.
     */
    YUV422P16LE,
    /**
     * planar YUV 4:2:2, 32bpp, (1 Cr & Cb sample per 2x1 Y samples), big-endian.
     */
    YUV422P16BE,
    /** planar YUV 4:4:4, 48bpp, (1 Cr & Cb sample per 1x1 Y samples),
     * little-endian. */
    YUV444P16LE,
    /**
     * planar YUV 4:4:4, 48bpp, (1 Cr & Cb sample per 1x1 Y samples), big-endian.
     */
    YUV444P16BE,
    #[cfg(feature = "ff_api_vdpau")]
    VDPAU_MPEG4,
    /** HW decoding through DXVA2. */
    DXVA2_VLD,

    /** packed RGB 4:4:4, 16bpp, (msb)4X 4R 4G 4B(lsb), little-endian,
     * X=unused/undefined. */
    RGB444LE,
    /** packed RGB 4:4:4, 16bpp, (msb)4X 4R 4G 4B(lsb), big-endian,
     * X=unused/undefined. */
    RGB444BE,
    /** packed BGR 4:4:4, 16bpp, (msb)4X 4B 4G 4R(lsb), little-endian,
     * X=unused/undefined */
    BGR444LE,
    /** packed BGR 4:4:4, 16bpp, (msb)4X 4B 4G 4R(lsb), big-endian,
     * X=unused/undefined. */
    BGR444BE,
    /** 8 bits gray, 8 bits alpha. */
    YA8,

    /** packed RGB 16:16:16, 48bpp, 16B, 16G, 16R, the 2-byte value for each
     * R/G/B component is stored as big-endian. */
    BGR48BE,
    /**
     * packed RGB 16:16:16, 48bpp, 16B, 16G, 16R, the 2-byte value for each
     * R/G/B component is stored as little-endian.
     */
    BGR48LE,

    /**
     * The following 12 formats have the disadvantage of needing 1 format for
     * each bit depth.
     *
     * Notice that each 9/10 bits sample is stored in 16 bits with extra padding.
     * If you want to support multiple bit depths, then using Pixel::YUV420P16*
     * with the bpp stored separately is better. planar YUV 4:2:0, 13.5bpp,
     * (1 Cr & Cb sample per 2x2 Y samples), big-endian.
     */
    YUV420P9BE,
    /** planar YUV 4:2:0, 13.5bpp, (1 Cr & Cb sample per 2x2 Y samples),
     * little-endian. */
    YUV420P9LE,
    /** planar YUV 4:2:0, 15bpp, (1 Cr & Cb sample per 2x2 Y samples),
     * big-endian. */
    YUV420P10BE,
    /** planar YUV 4:2:0, 15bpp, (1 Cr & Cb sample per 2x2 Y samples),
     * little-endian */
    YUV420P10LE,
    /** planar YUV 4:2:2, 20bpp, (1 Cr & Cb sample per 2x1 Y samples),
     * big-endian. */
    YUV422P10BE,
    /** planar YUV 4:2:2, 20bpp, (1 Cr & Cb sample per 2x1 Y samples),
     * little-endian. */
    YUV422P10LE,
    /** planar YUV 4:4:4, 27bpp, (1 Cr & Cb sample per 1x1 Y samples),
     * big-endian. */
    YUV444P9BE,
    /** planar YUV 4:4:4, 27bpp, (1 Cr & Cb sample per 1x1 Y samples),
     * little-endian. */
    YUV444P9LE,
    /** planar YUV 4:4:4, 30bpp, (1 Cr & Cb sample per 1x1 Y samples),
     * big-endian. */
    YUV444P10BE,
    /**
     * planar YUV 4:4:4, 30bpp, (1 Cr & Cb sample per 1x1 Y samples),
     * little-endian.
     */
    YUV444P10LE,
    /**
     * planar YUV 4:2:2, 18bpp, (1 Cr & Cb sample per 2x1 Y samples), big-endian.
     */
    YUV422P9BE,
    /** planar YUV 4:2:2, 18bpp, (1 Cr & Cb sample per 2x1 Y samples),
     * little-endian. */
    YUV422P9LE,
    #[cfg(not(feature = "ffmpeg_4_0"))]
    VDA_VLD,

    /** planar GBR 4:4:4 24bpp. */
    GBRP,
    /** planar GBR 4:4:4 27bpp, big-endian. */
    GBRP9BE,
    /** planar GBR 4:4:4 27bpp, little-endian. */
    GBRP9LE,
    /** [200~planar GBR 4:4:4 30bpp, big-endian. */
    GBRP10BE,
    /** planar GBR 4:4:4 30bpp, little-endian. */
    GBRP10LE,
    /** planar GBR 4:4:4 48bpp, big-endian. */
    GBRP16BE,
    /** planar GBR 4:4:4 48bpp, little-endian. */
    GBRP16LE,

    /** planar YUV 4:2:0 22.5bpp, (1 Cr & Cb sample per 2x2 Y & A samples),
     * big-endian. */
    YUVA420P9BE,
    /** planar YUV 4:2:0 22.5bpp, (1 Cr & Cb sample per 2x2 Y & A samples),
     * little-endian. */
    YUVA420P9LE,
    /** planar YUV 4:2:2 27bpp, (1 Cr & Cb sample per 2x1 Y & A samples),
     * big-endian. */
    YUVA422P9BE,
    /** planar YUV 4:2:2 27bpp, (1 Cr & Cb sample per 2x1 Y & A samples),
     * little-endian. */
    YUVA422P9LE,
    /** planar YUV 4:4:4 36bpp, (1 Cr & Cb sample per 1x1 Y & A samples),
     * big-endian. */
    YUVA444P9BE,
    /** planar YUV 4:4:4 36bpp, (1 Cr & Cb sample per 1x1 Y & A samples),
     * little-endian. */
    YUVA444P9LE,
    /** planar YUV 4:2:0 25bpp, (1 Cr & Cb sample per 2x2 Y & A samples,
     * big-endian). */
    YUVA420P10BE,
    /** planar YUV 4:2:0 25bpp, (1 Cr & Cb sample per 2x2 Y & A samples,
     * little-endian). */
    YUVA420P10LE,
    /** planar YUV 4:2:2 30bpp, (1 Cr & Cb sample per 2x1 Y & A samples,
     * big-endian). */
    YUVA422P10BE,
    /** planar YUV 4:2:2 30bpp, (1 Cr & Cb sample per 2x1 Y & A samples,
     * little-endian). */
    YUVA422P10LE,
    /** planar YUV 4:4:4 40bpp, (1 Cr & Cb sample per 1x1 Y & A samples,
     * big-endian). */
    YUVA444P10BE,
    /** planar YUV 4:4:4 40bpp, (1 Cr & Cb sample per 1x1 Y & A samples,
     * little-endian). */
    YUVA444P10LE,
    /** planar YUV 4:2:0 40bpp, (1 Cr & Cb sample per 2x2 Y & A samples, big-endian). */
    YUVA420P16BE,
    /** planar YUV 4:2:0 40bpp, (1 Cr & Cb sample per 2x2 Y & A samples, little-endian). */
    YUVA420P16LE,
    /** planar YUV 4:2:2 48bpp, (1 Cr & Cb sample per 2x1 Y & A samples, big-endian). */
    YUVA422P16BE,
    /** planar YUV 4:2:2 48bpp, (1 Cr & Cb sample per 2x1 Y & A samples, little-endian). */
    YUVA422P16LE,
    /** planar YUV 4:4:4 64bpp, (1 Cr & Cb sample per 1x1 Y & A samples, big-endian). */
    YUVA444P16BE,
    /** planar YUV 4:4:4 64bpp, (1 Cr & Cb sample per 1x1 Y & A samples, little-endian). */
    YUVA444P16LE,

    /** HW acceleration through VDPAU. */
    VDPAU,

    /** packed XYZ 4:4:4, 36 bpp, (msb) 12X, 12Y, 12Z (lsb), the 2-byte value for each X/Y/Z is
     * stored as little-endian, the 4 lower bits are set to 0. */
    XYZ12LE,
    /** packed XYZ 4:4:4, 36 bpp, (msb) 12X, 12Y, 12Z (lsb), the 2-byte value for each X/Y/Z is
     * stored as big-endian, the 4 lower bits are set to 0. */
    XYZ12BE,
    /** interleaved chroma YUV 4:2:2, 16bpp, (1 Cr & Cb sample per 2x1 Y samples). */
    NV16,
    /** interleaved chroma YUV 4:2:2, 20bpp, (1 Cr & Cb sample per 2x1 Y samples), little-endian.
     */
    NV20LE,
    /** interleaved chroma YUV 4:2:2, 20bpp, (1 Cr & Cb sample per 2x1 Y samples), big-endian. */
    NV20BE,

    /**
     * packed RGBA 16:16:16:16, 64bpp, 16R, 16G, 16B, 16A, the 2-byte value for each R/G/B/A
     * component is stored as big-endian.
     */
    RGBA64BE,
    /** packed RGBA 16:16:16:16, 64bpp, 16R, 16G, 16B, 16A, the 2-byte value for each R/G/B/A
     * component is stored as little-endian. */
    RGBA64LE,
    /**
     * packed RGBA 16:16:16:16, 64bpp, 16B, 16G, 16R, 16A, the 2-byte value for each R/G/B/A
     * component is stored as big-endian.
     */
    BGRA64BE,
    /**
     * packed RGBA 16:16:16:16, 64bpp, 16B, 16G, 16R, 16A, the 2-byte value for each R/G/B/A
     * component is stored as little-endian.
     */
    BGRA64LE,

    /** packed YUV 4:2:2, 16bpp, Y0 Cr Y1 Cb. */
    YVYU422,

    #[cfg(not(feature = "ffmpeg_4_0"))]
    VDA,

    /** 16 bits gray, 16 bits alpha (big-endian). */
    YA16BE,
    /** 16 bits gray, 16 bits alpha (little-endian). */
    YA16LE,

    /** HW acceleration through QSV. */
    QSV,
    /** HW acceleration though MMAL. */
    MMAL,

    /** HW decoding through Direct3D11 via old API. */
    D3D11VA_VLD,

    /** HW excelleration through CUDA. */
    CUDA,

    ZRGB,
    RGBZ,
    ZBGR,
    BGRZ,
    YUVA444P,
    YUVA422P,

    /** planar YUV 4:2:0,18bpp, (1 Cr & Cb sample per 2x2 Y samples), big-endian. */
    YUV420P12BE,
    /** planar YUV 4:2:0,18bpp, (1 Cr & Cb sample per 2x2 Y samples), little-endian. */
    YUV420P12LE,
    /** planar YUV 4:2:0,21bpp, (1 Cr & Cb sample per 2x2 Y samples), big-endian. */
    YUV420P14BE,
    /** planar YUV 4:2:0,21bpp, (1 Cr & Cb sample per 2x2 Y samples), little-endian. */
    YUV420P14LE,
    /** planar YUV 4:2:2,24bpp, (1 Cr & Cb sample per 2x1 Y samples), big-endian. */
    YUV422P12BE,
    /** planar YUV 4:2:2,24bpp, (1 Cr & Cb sample per 2x1 Y samples), little-endian. */
    YUV422P12LE,
    /** planar YUV 4:2:2,28bpp, (1 Cr & Cb sample per 2x1 Y samples), big-endian. */
    YUV422P14BE,
    /** planar YUV 4:2:2,28bpp, (1 Cr & Cb sample per 2x1 Y samples), little-endian. */
    YUV422P14LE,
    /** planar YUV 4:4:4,36bpp, (1 Cr & Cb sample per 1x1 Y samples), big-endian. */
    YUV444P12BE,
    /** planar YUV 4:4:4,36bpp, (1 Cr & Cb sample per 1x1 Y samples), little-endian. */
    YUV444P12LE,
    /** planar YUV 4:4:4,42bpp, (1 Cr & Cb sample per 1x1 Y samples), big-endian. */
    YUV444P14BE,
    /** planar YUV 4:4:4,42bpp, (1 Cr & Cb sample per 1x1 Y samples), little-endian. */
    YUV444P14LE,
    /** planar GBR 4:4:4 36bpp, big-endian. */
    GBRP12BE,
    /** planar GBR 4:4:4 36bpp, little-endian. */
    GBRP12LE,
    /** planar GBR 4:4:4 42bpp, big-endian. */
    GBRP14BE,
    /** planar GBR 4:4:4 42bpp, little-endian. */
    GBRP14LE,
    GBRAP,
    GBRAP16BE,
    GBRAP16LE,
    /** planar YUV 4:1:1, 12bpp, (1 Cr & Cb sample per 4x1 Y samples) full scale (JPEG), deprecated
     * in favor of AV_PIX_FMT_YUV411P and setting color_range. */
    YUVJ411P,

    /** bayer, BGBG..(odd line), GRGR..(even line), 8-bit samples. */
    BAYER_BGGR8,
    /** bayer, RGRG..(odd line), GBGB..(even line), 8-bit samples. */
    BAYER_RGGB8,
    /** bayer, GBGB..(odd line), RGRG..(even line), 8-bit samples. */
    BAYER_GBRG8,
    /** bayer, GRGR..(odd line), BGBG..(even line), 8-bit samples. */
    BAYER_GRBG8,
    /** bayer, BGBG..(odd line), GRGR..(even line), 16-bit samples, little-endian. */
    BAYER_BGGR16LE,
    /** bayer, BGBG..(odd line), GRGR..(even line), 16-bit samples, big-endian. */
    BAYER_BGGR16BE,
    /** bayer, RGRG..(odd line), GBGB..(even line), 16-bit samples, little-endian. */
    BAYER_RGGB16LE,
    /** bayer, RGRG..(odd line), GBGB..(even line), 16-bit samples, big-endian. */
    BAYER_RGGB16BE,
    /** bayer, GBGB..(odd line), RGRG..(even line), 16-bit samples, little-endian. */
    BAYER_GBRG16LE,
    /** bayer, GBGB..(odd line), RGRG..(even line), 16-bit samples, big-endian. */
    BAYER_GBRG16BE,
    /** bayer, GRGR..(odd line), BGBG..(even line), 16-bit samples, little-endian. */
    BAYER_GRBG16LE,
    /** bayer, GRGR..(odd line), BGBG..(even line), 16-bit samples, big-endian. */
    BAYER_GRBG16BE,

    /** planar YUV 4:4:0,20bpp, (1 Cr & Cb sample per 1x2 Y samples), little-endian. */
    YUV440P10LE,
    /** planar YUV 4:4:0,20bpp, (1 Cr & Cb sample per 1x2 Y samples), big-endian. */
    YUV440P10BE,
    /** planar YUV 4:4:0,24bpp, (1 Cr & Cb sample per 1x2 Y samples), little-endian. */
    YUV440P12LE,
    /** planar YUV 4:4:0,24bpp, (1 Cr & Cb sample per 1x2 Y samples), big-endian. */
    YUV440P12BE,
    /** packed AYUV 4:4:4,64bpp (1 Cr & Cb sample per 1x1 Y & A samples), little-endian. */
    AYUV64LE,
    /** packed AYUV 4:4:4,64bpp (1 Cr & Cb sample per 1x1 Y & A samples), big-endian. */
    AYUV64BE,

    /** hardware decoding through Videotoolbox. */
    VIDEOTOOLBOX,

    // --- defaults
    #[cfg(not(feature = "ff_api_xvmc"))]
    XVMC,

    RGB32,
    RGB32_1,
    BGR32,
    BGR32_1,
    ZRGB32,
    ZBGR32,

    /** Y , 16bpp, big and little endian. */
    GRAY16,
    YA16,
    /** packed RGB 16:16:16, 48bpp, 16R, 16G, 16B, the 2-byte value for each
     * R/G/B component is stored as big or little endian. */
    RGB48,
    /** packed RGB 5:6:5, 16bpp, (msb) 5R 6G 5B(lsb) */
    RGB565,
    /** packed RGB 5:5:5, 16bpp, (msb) 1X 5R 5G 5B(lsb), X=unused/undefined. */
    RGB555,
    RGB444,
    BGR48,
    BGR565,
    BGR555,
    BGR444,

    YUV420P9,
    YUV422P9,
    YUV444P9,
    YUV420P10,
    YUV422P10,
    YUV440P10,
    YUV444P10,
    YUV420P12,
    YUV422P12,
    YUV440P12,
    YUV444P12,
    YUV420P14,
    YUV422P14,
    YUV444P14,
    YUV420P16,
    YUV422P16,
    YUV444P16,

    GBRP9,
    GBRP10,
    GBRP12,
    GBRP14,
    GBRP16,
    GBRAP16,

    BAYER_BGGR16,
    BAYER_RGGB16,
    BAYER_GBRG16,
    BAYER_GRBG16,

    YUVA420P9,
    YUVA422P9,
    YUVA444P9,
    YUVA420P10,
    YUVA422P10,
    YUVA444P10,
    YUVA420P16,
    YUVA422P16,
    YUVA444P16,

    XYZ12,
    NV20,
    AYUV64,

    P010LE,
    P010BE,
    GBRAP12BE,
    GBRAP12LE,
    GBRAP10LE,
    GBRAP10BE,
    MEDIACODEC,
    GRAY12BE,
    GRAY12LE,
    GRAY10BE,
    GRAY10LE,
    P016LE,
    P016BE,

    D3D11,
    GRAY9BE,
    GRAY9LE,
    GBRPF32BE,
    GBRPF32LE,
    GBRAPF32BE,
    GBRAPF32LE,
    DRM_PRIME,

    #[cfg(feature = "ffmpeg_4_0")]
    OPENCL,

    #[cfg(feature = "ffmpeg_4_1")]
    GRAY14BE,
    #[cfg(feature = "ffmpeg_4_1")]
    GRAY14LE,
    #[cfg(feature = "ffmpeg_4_1")]
    GRAYF32BE,
    #[cfg(feature = "ffmpeg_4_1")]
    GRAYF32LE,

    #[cfg(feature = "ffmpeg_4_2")]
    YUVA422P12BE,
    #[cfg(feature = "ffmpeg_4_2")]
    YUVA422P12LE,
    #[cfg(feature = "ffmpeg_4_2")]
    YUVA444P12BE,
    #[cfg(feature = "ffmpeg_4_2")]
    YUVA444P12LE,
    #[cfg(feature = "ffmpeg_4_2")]
    NV24,
    #[cfg(feature = "ffmpeg_4_2")]
    NV42,

    #[cfg(feature = "ffmpeg_4_3")]
    VULKAN,
    #[cfg(feature = "ffmpeg_4_3")]
    Y210BE,
    #[cfg(feature = "ffmpeg_4_3")]
    Y210LE,

    #[cfg(feature = "ffmpeg_4_4")]
    X2RGB10LE,
    #[cfg(feature = "ffmpeg_4_4")]
    X2RGB10BE,

    #[cfg(feature = "rpi")]
    RPI,
    #[cfg(feature = "rpi")]
    SAND128,
    #[cfg(feature = "rpi")]
    SAND64_10,
    #[cfg(feature = "rpi")]
    SAND64_16,
    #[cfg(feature = "rpi")]
    RPI4_8,
    #[cfg(feature = "rpi")]
    RPI4_10,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Descriptor {
    ptr: *const AVPixFmtDescriptor,
}

unsafe impl Send for Descriptor {}
unsafe impl Sync for Descriptor {}

impl Pixel {
    pub const Y400A: Pixel = Pixel::YA8;
    pub const GRAY8A: Pixel = Pixel::YA8;
    pub const GBR24P: Pixel = Pixel::GBRP;
    #[cfg(feature = "ff_api_xvmc")]
    pub const XVMC: Pixel = Pixel::XVMC_MPEG2_IDCT;

    pub fn descriptor(self) -> Option<Descriptor> {
        unsafe {
            let ptr = av_pix_fmt_desc_get(self.into());

            ptr.as_ref().map(|ptr| Descriptor { ptr })
        }
    }
}

impl Descriptor {
    pub fn as_ptr(self) -> *const AVPixFmtDescriptor {
        self.ptr
    }

    pub fn name(self) -> &'static str {
        unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).name).to_bytes()) }
    }

    pub fn nb_components(self) -> u8 {
        unsafe { (*self.as_ptr()).nb_components }
    }

    pub fn log2_chroma_w(self) -> u8 {
        unsafe { (*self.as_ptr()).log2_chroma_w }
    }

    pub fn log2_chroma_h(self) -> u8 {
        unsafe { (*self.as_ptr()).log2_chroma_h }
    }
}

impl From<AVPixelFormat> for Pixel {
    #[inline]
    fn from(value: AVPixelFormat) -> Self {
        match value {
            AV_PIX_FMT_NONE => Pixel::None,

            AV_PIX_FMT_YUV420P => Pixel::YUV420P,
            AV_PIX_FMT_YUYV422 => Pixel::YUYV422,
            AV_PIX_FMT_RGB24 => Pixel::RGB24,
            AV_PIX_FMT_BGR24 => Pixel::BGR24,
            AV_PIX_FMT_YUV422P => Pixel::YUV422P,
            AV_PIX_FMT_YUV444P => Pixel::YUV444P,
            AV_PIX_FMT_YUV410P => Pixel::YUV410P,
            AV_PIX_FMT_YUV411P => Pixel::YUV411P,
            AV_PIX_FMT_GRAY8 => Pixel::GRAY8,
            AV_PIX_FMT_MONOWHITE => Pixel::MonoWhite,
            AV_PIX_FMT_MONOBLACK => Pixel::MonoBlack,
            AV_PIX_FMT_PAL8 => Pixel::PAL8,
            AV_PIX_FMT_YUVJ420P => Pixel::YUVJ420P,
            AV_PIX_FMT_YUVJ422P => Pixel::YUVJ422P,
            AV_PIX_FMT_YUVJ444P => Pixel::YUVJ444P,
            #[cfg(not(feature = "ff_api_xvmc"))]
            AV_PIX_FMT_XVMC => Pixel::XVMC,
            #[cfg(feature = "ff_api_xvmc")]
            AV_PIX_FMT_XVMC_MPEG2_MC => Pixel::XVMC_MPEG2_MC,
            #[cfg(feature = "ff_api_xvmc")]
            AV_PIX_FMT_XVMC_MPEG2_IDCT => Pixel::XVMC_MPEG2_IDCT,
            AV_PIX_FMT_UYVY422 => Pixel::UYVY422,
            AV_PIX_FMT_UYYVYY411 => Pixel::UYYVYY411,
            AV_PIX_FMT_BGR8 => Pixel::BGR8,
            AV_PIX_FMT_BGR4 => Pixel::BGR4,
            AV_PIX_FMT_BGR4_BYTE => Pixel::BGR4_BYTE,
            AV_PIX_FMT_RGB8 => Pixel::RGB8,
            AV_PIX_FMT_RGB4 => Pixel::RGB4,
            AV_PIX_FMT_RGB4_BYTE => Pixel::RGB4_BYTE,
            AV_PIX_FMT_NV12 => Pixel::NV12,
            AV_PIX_FMT_NV21 => Pixel::NV21,

            AV_PIX_FMT_ARGB => Pixel::ARGB,
            AV_PIX_FMT_RGBA => Pixel::RGBA,
            AV_PIX_FMT_ABGR => Pixel::ABGR,
            AV_PIX_FMT_BGRA => Pixel::BGRA,

            AV_PIX_FMT_GRAY16BE => Pixel::GRAY16BE,
            AV_PIX_FMT_GRAY16LE => Pixel::GRAY16LE,
            AV_PIX_FMT_YUV440P => Pixel::YUV440P,
            AV_PIX_FMT_YUVJ440P => Pixel::YUVJ440P,
            AV_PIX_FMT_YUVA420P => Pixel::YUVA420P,
            #[cfg(feature = "ff_api_vdpau")]
            AV_PIX_FMT_VDPAU_H264 => Pixel::VDPAU_H264,
            #[cfg(feature = "ff_api_vdpau")]
            AV_PIX_FMT_VDPAU_MPEG1 => Pixel::VDPAU_MPEG1,
            #[cfg(feature = "ff_api_vdpau")]
            AV_PIX_FMT_VDPAU_MPEG2 => Pixel::VDPAU_MPEG2,
            #[cfg(feature = "ff_api_vdpau")]
            AV_PIX_FMT_VDPAU_WMV3 => Pixel::VDPAU_WMV3,
            #[cfg(feature = "ff_api_vdpau")]
            AV_PIX_FMT_VDPAU_VC1 => Pixel::VDPAU_VC1,
            AV_PIX_FMT_RGB48BE => Pixel::RGB48BE,
            AV_PIX_FMT_RGB48LE => Pixel::RGB48LE,

            AV_PIX_FMT_RGB565BE => Pixel::RGB565BE,
            AV_PIX_FMT_RGB565LE => Pixel::RGB565LE,
            AV_PIX_FMT_RGB555BE => Pixel::RGB555BE,
            AV_PIX_FMT_RGB555LE => Pixel::RGB555LE,

            AV_PIX_FMT_BGR565BE => Pixel::BGR565BE,
            AV_PIX_FMT_BGR565LE => Pixel::BGR565LE,
            AV_PIX_FMT_BGR555BE => Pixel::BGR555BE,
            AV_PIX_FMT_BGR555LE => Pixel::BGR555LE,

            #[cfg(feature = "ff_api_vaapi")]
            AV_PIX_FMT_VAAPI_MOCO => Pixel::VAAPI_MOCO,
            #[cfg(feature = "ff_api_vaapi")]
            AV_PIX_FMT_VAAPI_IDCT => Pixel::VAAPI_IDCT,
            #[cfg(feature = "ff_api_vaapi")]
            AV_PIX_FMT_VAAPI_VLD => Pixel::VAAPI_VLD,

            AV_PIX_FMT_YUV420P16LE => Pixel::YUV420P16LE,
            AV_PIX_FMT_YUV420P16BE => Pixel::YUV420P16BE,
            AV_PIX_FMT_YUV422P16LE => Pixel::YUV422P16LE,
            AV_PIX_FMT_YUV422P16BE => Pixel::YUV422P16BE,
            AV_PIX_FMT_YUV444P16LE => Pixel::YUV444P16LE,
            AV_PIX_FMT_YUV444P16BE => Pixel::YUV444P16BE,
            #[cfg(feature = "ff_api_vdpau")]
            AV_PIX_FMT_VDPAU_MPEG4 => Pixel::VDPAU_MPEG4,
            AV_PIX_FMT_DXVA2_VLD => Pixel::DXVA2_VLD,

            AV_PIX_FMT_RGB444LE => Pixel::RGB444LE,
            AV_PIX_FMT_RGB444BE => Pixel::RGB444BE,
            AV_PIX_FMT_BGR444LE => Pixel::BGR444LE,
            AV_PIX_FMT_BGR444BE => Pixel::BGR444BE,
            AV_PIX_FMT_YA8 => Pixel::YA8,

            AV_PIX_FMT_BGR48BE => Pixel::BGR48BE,
            AV_PIX_FMT_BGR48LE => Pixel::BGR48LE,

            AV_PIX_FMT_YUV420P9BE => Pixel::YUV420P9BE,
            AV_PIX_FMT_YUV420P9LE => Pixel::YUV420P9LE,
            AV_PIX_FMT_YUV420P10BE => Pixel::YUV420P10BE,
            AV_PIX_FMT_YUV420P10LE => Pixel::YUV420P10LE,
            AV_PIX_FMT_YUV422P10BE => Pixel::YUV422P10BE,
            AV_PIX_FMT_YUV422P10LE => Pixel::YUV422P10LE,
            AV_PIX_FMT_YUV444P9BE => Pixel::YUV444P9BE,
            AV_PIX_FMT_YUV444P9LE => Pixel::YUV444P9LE,
            AV_PIX_FMT_YUV444P10BE => Pixel::YUV444P10BE,
            AV_PIX_FMT_YUV444P10LE => Pixel::YUV444P10LE,
            AV_PIX_FMT_YUV422P9BE => Pixel::YUV422P9BE,
            AV_PIX_FMT_YUV422P9LE => Pixel::YUV422P9LE,
            #[cfg(not(feature = "ffmpeg_4_0"))]
            AV_PIX_FMT_VDA_VLD => Pixel::VDA_VLD,

            AV_PIX_FMT_GBRP => Pixel::GBRP,
            AV_PIX_FMT_GBRP9BE => Pixel::GBRP9BE,
            AV_PIX_FMT_GBRP9LE => Pixel::GBRP9LE,
            AV_PIX_FMT_GBRP10BE => Pixel::GBRP10BE,
            AV_PIX_FMT_GBRP10LE => Pixel::GBRP10LE,
            AV_PIX_FMT_GBRP16BE => Pixel::GBRP16BE,
            AV_PIX_FMT_GBRP16LE => Pixel::GBRP16LE,

            AV_PIX_FMT_YUVA420P9BE => Pixel::YUVA420P9BE,
            AV_PIX_FMT_YUVA420P9LE => Pixel::YUVA420P9LE,
            AV_PIX_FMT_YUVA422P9BE => Pixel::YUVA422P9BE,
            AV_PIX_FMT_YUVA422P9LE => Pixel::YUVA422P9LE,
            AV_PIX_FMT_YUVA444P9BE => Pixel::YUVA444P9BE,
            AV_PIX_FMT_YUVA444P9LE => Pixel::YUVA444P9LE,
            AV_PIX_FMT_YUVA420P10BE => Pixel::YUVA420P10BE,
            AV_PIX_FMT_YUVA420P10LE => Pixel::YUVA420P10LE,
            AV_PIX_FMT_YUVA422P10BE => Pixel::YUVA422P10BE,
            AV_PIX_FMT_YUVA422P10LE => Pixel::YUVA422P10LE,
            AV_PIX_FMT_YUVA444P10BE => Pixel::YUVA444P10BE,
            AV_PIX_FMT_YUVA444P10LE => Pixel::YUVA444P10LE,
            AV_PIX_FMT_YUVA420P16BE => Pixel::YUVA420P16BE,
            AV_PIX_FMT_YUVA420P16LE => Pixel::YUVA420P16LE,
            AV_PIX_FMT_YUVA422P16BE => Pixel::YUVA422P16BE,
            AV_PIX_FMT_YUVA422P16LE => Pixel::YUVA422P16LE,
            AV_PIX_FMT_YUVA444P16BE => Pixel::YUVA444P16BE,
            AV_PIX_FMT_YUVA444P16LE => Pixel::YUVA444P16LE,

            AV_PIX_FMT_VDPAU => Pixel::VDPAU,

            AV_PIX_FMT_XYZ12LE => Pixel::XYZ12LE,
            AV_PIX_FMT_XYZ12BE => Pixel::XYZ12BE,
            AV_PIX_FMT_NV16 => Pixel::NV16,
            AV_PIX_FMT_NV20LE => Pixel::NV20LE,
            AV_PIX_FMT_NV20BE => Pixel::NV20BE,

            AV_PIX_FMT_RGBA64BE => Pixel::RGBA64BE,
            AV_PIX_FMT_RGBA64LE => Pixel::RGBA64LE,
            AV_PIX_FMT_BGRA64BE => Pixel::BGRA64BE,
            AV_PIX_FMT_BGRA64LE => Pixel::BGRA64LE,

            AV_PIX_FMT_YVYU422 => Pixel::YVYU422,

            #[cfg(not(feature = "ffmpeg_4_0"))]
            AV_PIX_FMT_VDA => Pixel::VDA,

            AV_PIX_FMT_YA16BE => Pixel::YA16BE,
            AV_PIX_FMT_YA16LE => Pixel::YA16LE,

            AV_PIX_FMT_QSV => Pixel::QSV,
            AV_PIX_FMT_MMAL => Pixel::MMAL,

            AV_PIX_FMT_D3D11VA_VLD => Pixel::D3D11VA_VLD,

            AV_PIX_FMT_CUDA => Pixel::CUDA,

            AV_PIX_FMT_0RGB => Pixel::ZRGB,
            AV_PIX_FMT_RGB0 => Pixel::RGBZ,
            AV_PIX_FMT_0BGR => Pixel::ZBGR,
            AV_PIX_FMT_BGR0 => Pixel::BGRZ,
            AV_PIX_FMT_YUVA444P => Pixel::YUVA444P,
            AV_PIX_FMT_YUVA422P => Pixel::YUVA422P,

            AV_PIX_FMT_YUV420P12BE => Pixel::YUV420P12BE,
            AV_PIX_FMT_YUV420P12LE => Pixel::YUV420P12LE,
            AV_PIX_FMT_YUV420P14BE => Pixel::YUV420P14BE,
            AV_PIX_FMT_YUV420P14LE => Pixel::YUV420P14LE,
            AV_PIX_FMT_YUV422P12BE => Pixel::YUV422P12BE,
            AV_PIX_FMT_YUV422P12LE => Pixel::YUV422P12LE,
            AV_PIX_FMT_YUV422P14BE => Pixel::YUV422P14BE,
            AV_PIX_FMT_YUV422P14LE => Pixel::YUV422P14LE,
            AV_PIX_FMT_YUV444P12BE => Pixel::YUV444P12BE,
            AV_PIX_FMT_YUV444P12LE => Pixel::YUV444P12LE,
            AV_PIX_FMT_YUV444P14BE => Pixel::YUV444P14BE,
            AV_PIX_FMT_YUV444P14LE => Pixel::YUV444P14LE,
            AV_PIX_FMT_GBRP12BE => Pixel::GBRP12BE,
            AV_PIX_FMT_GBRP12LE => Pixel::GBRP12LE,
            AV_PIX_FMT_GBRP14BE => Pixel::GBRP14BE,
            AV_PIX_FMT_GBRP14LE => Pixel::GBRP14LE,
            AV_PIX_FMT_GBRAP => Pixel::GBRAP,
            AV_PIX_FMT_GBRAP16BE => Pixel::GBRAP16BE,
            AV_PIX_FMT_GBRAP16LE => Pixel::GBRAP16LE,
            AV_PIX_FMT_YUVJ411P => Pixel::YUVJ411P,

            AV_PIX_FMT_BAYER_BGGR8 => Pixel::BAYER_BGGR8,
            AV_PIX_FMT_BAYER_RGGB8 => Pixel::BAYER_RGGB8,
            AV_PIX_FMT_BAYER_GBRG8 => Pixel::BAYER_GBRG8,
            AV_PIX_FMT_BAYER_GRBG8 => Pixel::BAYER_GRBG8,
            AV_PIX_FMT_BAYER_BGGR16LE => Pixel::BAYER_BGGR16LE,
            AV_PIX_FMT_BAYER_BGGR16BE => Pixel::BAYER_BGGR16BE,
            AV_PIX_FMT_BAYER_RGGB16LE => Pixel::BAYER_RGGB16LE,
            AV_PIX_FMT_BAYER_RGGB16BE => Pixel::BAYER_RGGB16BE,
            AV_PIX_FMT_BAYER_GBRG16LE => Pixel::BAYER_GBRG16LE,
            AV_PIX_FMT_BAYER_GBRG16BE => Pixel::BAYER_GBRG16BE,
            AV_PIX_FMT_BAYER_GRBG16LE => Pixel::BAYER_GRBG16LE,
            AV_PIX_FMT_BAYER_GRBG16BE => Pixel::BAYER_GRBG16BE,

            AV_PIX_FMT_YUV440P10LE => Pixel::YUV440P10LE,
            AV_PIX_FMT_YUV440P10BE => Pixel::YUV440P10BE,
            AV_PIX_FMT_YUV440P12LE => Pixel::YUV440P12LE,
            AV_PIX_FMT_YUV440P12BE => Pixel::YUV440P12BE,
            AV_PIX_FMT_AYUV64LE => Pixel::AYUV64LE,
            AV_PIX_FMT_AYUV64BE => Pixel::AYUV64BE,

            AV_PIX_FMT_VIDEOTOOLBOX => Pixel::VIDEOTOOLBOX,

            AV_PIX_FMT_P010LE => Pixel::P010LE,
            AV_PIX_FMT_P010BE => Pixel::P010BE,
            AV_PIX_FMT_GBRAP12BE => Pixel::GBRAP12BE,
            AV_PIX_FMT_GBRAP12LE => Pixel::GBRAP12LE,
            AV_PIX_FMT_GBRAP10LE => Pixel::GBRAP10LE,
            AV_PIX_FMT_GBRAP10BE => Pixel::GBRAP10BE,
            AV_PIX_FMT_MEDIACODEC => Pixel::MEDIACODEC,
            AV_PIX_FMT_GRAY12BE => Pixel::GRAY12BE,
            AV_PIX_FMT_GRAY12LE => Pixel::GRAY12LE,
            AV_PIX_FMT_GRAY10BE => Pixel::GRAY10BE,
            AV_PIX_FMT_GRAY10LE => Pixel::GRAY10LE,
            AV_PIX_FMT_P016LE => Pixel::P016LE,
            AV_PIX_FMT_P016BE => Pixel::P016BE,

            AV_PIX_FMT_NB => Pixel::None,

            AV_PIX_FMT_D3D11 => Pixel::D3D11,
            AV_PIX_FMT_GRAY9BE => Pixel::GRAY9BE,
            AV_PIX_FMT_GRAY9LE => Pixel::GRAY9LE,
            AV_PIX_FMT_GBRPF32BE => Pixel::GBRPF32BE,
            AV_PIX_FMT_GBRPF32LE => Pixel::GBRPF32LE,
            AV_PIX_FMT_GBRAPF32BE => Pixel::GBRAPF32BE,
            AV_PIX_FMT_GBRAPF32LE => Pixel::GBRAPF32LE,
            AV_PIX_FMT_DRM_PRIME => Pixel::DRM_PRIME,

            #[cfg(feature = "ffmpeg_4_0")]
            AV_PIX_FMT_OPENCL => Pixel::OPENCL,

            #[cfg(feature = "ffmpeg_4_1")]
            AV_PIX_FMT_GRAY14BE => Pixel::GRAY14BE,
            #[cfg(feature = "ffmpeg_4_1")]
            AV_PIX_FMT_GRAY14LE => Pixel::GRAY14LE,
            #[cfg(feature = "ffmpeg_4_1")]
            AV_PIX_FMT_GRAYF32BE => Pixel::GRAYF32BE,
            #[cfg(feature = "ffmpeg_4_1")]
            AV_PIX_FMT_GRAYF32LE => Pixel::GRAYF32LE,

            #[cfg(feature = "ffmpeg_4_2")]
            AV_PIX_FMT_YUVA422P12BE => Pixel::YUVA422P12BE,
            #[cfg(feature = "ffmpeg_4_2")]
            AV_PIX_FMT_YUVA422P12LE => Pixel::YUVA422P12LE,
            #[cfg(feature = "ffmpeg_4_2")]
            AV_PIX_FMT_YUVA444P12BE => Pixel::YUVA444P12BE,
            #[cfg(feature = "ffmpeg_4_2")]
            AV_PIX_FMT_YUVA444P12LE => Pixel::YUVA444P12LE,
            #[cfg(feature = "ffmpeg_4_2")]
            AV_PIX_FMT_NV24 => Pixel::NV24,
            #[cfg(feature = "ffmpeg_4_2")]
            AV_PIX_FMT_NV42 => Pixel::NV42,

            #[cfg(feature = "ffmpeg_4_3")]
            AV_PIX_FMT_VULKAN => Pixel::VULKAN,
            #[cfg(feature = "ffmpeg_4_3")]
            AV_PIX_FMT_Y210BE => Pixel::Y210BE,
            #[cfg(feature = "ffmpeg_4_3")]
            AV_PIX_FMT_Y210LE => Pixel::Y210LE,

            #[cfg(feature = "ffmpeg_4_4")]
            AV_PIX_FMT_X2RGB10LE => Pixel::X2RGB10LE,
            #[cfg(feature = "ffmpeg_4_4")]
            AV_PIX_FMT_X2RGB10BE => Pixel::X2RGB10BE,

            #[cfg(feature = "rpi")]
            AV_PIX_FMT_RPI => Pixel::RPI,
            #[cfg(feature = "rpi")]
            AV_PIX_FMT_SAND128 => Pixel::SAND128,
            #[cfg(feature = "rpi")]
            AV_PIX_FMT_SAND64_10 => Pixel::SAND64_10,
            #[cfg(feature = "rpi")]
            AV_PIX_FMT_SAND64_16 => Pixel::SAND64_16,
            #[cfg(feature = "rpi")]
            AV_PIX_FMT_RPI4_8 => Pixel::RPI4_8,
            #[cfg(feature = "rpi")]
            AV_PIX_FMT_RPI4_10 => Pixel::RPI4_10,
        }
    }
}

impl From<Pixel> for AVPixelFormat {
    #[inline]
    fn from(value: Pixel) -> AVPixelFormat {
        match value {
            Pixel::None => AV_PIX_FMT_NONE,

            Pixel::YUV420P => AV_PIX_FMT_YUV420P,
            Pixel::YUYV422 => AV_PIX_FMT_YUYV422,
            Pixel::RGB24 => AV_PIX_FMT_RGB24,
            Pixel::BGR24 => AV_PIX_FMT_BGR24,
            Pixel::YUV422P => AV_PIX_FMT_YUV422P,
            Pixel::YUV444P => AV_PIX_FMT_YUV444P,
            Pixel::YUV410P => AV_PIX_FMT_YUV410P,
            Pixel::YUV411P => AV_PIX_FMT_YUV411P,
            Pixel::GRAY8 => AV_PIX_FMT_GRAY8,
            Pixel::MonoWhite => AV_PIX_FMT_MONOWHITE,
            Pixel::MonoBlack => AV_PIX_FMT_MONOBLACK,
            Pixel::PAL8 => AV_PIX_FMT_PAL8,
            Pixel::YUVJ420P => AV_PIX_FMT_YUVJ420P,
            Pixel::YUVJ422P => AV_PIX_FMT_YUVJ422P,
            Pixel::YUVJ444P => AV_PIX_FMT_YUVJ444P,
            #[cfg(feature = "ff_api_xvmc")]
            Pixel::XVMC_MPEG2_MC => AV_PIX_FMT_XVMC_MPEG2_MC,
            #[cfg(feature = "ff_api_xvmc")]
            Pixel::XVMC_MPEG2_IDCT => AV_PIX_FMT_XVMC_MPEG2_IDCT,
            Pixel::UYVY422 => AV_PIX_FMT_UYVY422,
            Pixel::UYYVYY411 => AV_PIX_FMT_UYYVYY411,
            Pixel::BGR8 => AV_PIX_FMT_BGR8,
            Pixel::BGR4 => AV_PIX_FMT_BGR4,
            Pixel::BGR4_BYTE => AV_PIX_FMT_BGR4_BYTE,
            Pixel::RGB8 => AV_PIX_FMT_RGB8,
            Pixel::RGB4 => AV_PIX_FMT_RGB4,
            Pixel::RGB4_BYTE => AV_PIX_FMT_RGB4_BYTE,
            Pixel::NV12 => AV_PIX_FMT_NV12,
            Pixel::NV21 => AV_PIX_FMT_NV21,

            Pixel::ARGB => AV_PIX_FMT_ARGB,
            Pixel::RGBA => AV_PIX_FMT_RGBA,
            Pixel::ABGR => AV_PIX_FMT_ABGR,
            Pixel::BGRA => AV_PIX_FMT_BGRA,

            Pixel::GRAY16BE => AV_PIX_FMT_GRAY16BE,
            Pixel::GRAY16LE => AV_PIX_FMT_GRAY16LE,
            Pixel::YUV440P => AV_PIX_FMT_YUV440P,
            Pixel::YUVJ440P => AV_PIX_FMT_YUVJ440P,
            Pixel::YUVA420P => AV_PIX_FMT_YUVA420P,
            #[cfg(feature = "ff_api_vdpau")]
            Pixel::VDPAU_H264 => AV_PIX_FMT_VDPAU_H264,
            #[cfg(feature = "ff_api_vdpau")]
            Pixel::VDPAU_MPEG1 => AV_PIX_FMT_VDPAU_MPEG1,
            #[cfg(feature = "ff_api_vdpau")]
            Pixel::VDPAU_MPEG2 => AV_PIX_FMT_VDPAU_MPEG2,
            #[cfg(feature = "ff_api_vdpau")]
            Pixel::VDPAU_WMV3 => AV_PIX_FMT_VDPAU_WMV3,
            #[cfg(feature = "ff_api_vdpau")]
            Pixel::VDPAU_VC1 => AV_PIX_FMT_VDPAU_VC1,
            Pixel::RGB48BE => AV_PIX_FMT_RGB48BE,
            Pixel::RGB48LE => AV_PIX_FMT_RGB48LE,

            Pixel::RGB565BE => AV_PIX_FMT_RGB565BE,
            Pixel::RGB565LE => AV_PIX_FMT_RGB565LE,
            Pixel::RGB555BE => AV_PIX_FMT_RGB555BE,
            Pixel::RGB555LE => AV_PIX_FMT_RGB555LE,

            Pixel::BGR565BE => AV_PIX_FMT_BGR565BE,
            Pixel::BGR565LE => AV_PIX_FMT_BGR565LE,
            Pixel::BGR555BE => AV_PIX_FMT_BGR555BE,
            Pixel::BGR555LE => AV_PIX_FMT_BGR555LE,

            #[cfg(feature = "ff_api_vaapi")]
            Pixel::VAAPI_MOCO => AV_PIX_FMT_VAAPI_MOCO,
            #[cfg(feature = "ff_api_vaapi")]
            Pixel::VAAPI_IDCT => AV_PIX_FMT_VAAPI_IDCT,
            #[cfg(feature = "ff_api_vaapi")]
            Pixel::VAAPI_VLD => AV_PIX_FMT_VAAPI_VLD,
            #[cfg(not(feature = "ff_api_vaapi"))]
            Pixel::VAAPI => AV_PIX_FMT_VAAPI,

            Pixel::YUV420P16LE => AV_PIX_FMT_YUV420P16LE,
            Pixel::YUV420P16BE => AV_PIX_FMT_YUV420P16BE,
            Pixel::YUV422P16LE => AV_PIX_FMT_YUV422P16LE,
            Pixel::YUV422P16BE => AV_PIX_FMT_YUV422P16BE,
            Pixel::YUV444P16LE => AV_PIX_FMT_YUV444P16LE,
            Pixel::YUV444P16BE => AV_PIX_FMT_YUV444P16BE,
            #[cfg(feature = "ff_api_vdpau")]
            Pixel::VDPAU_MPEG4 => AV_PIX_FMT_VDPAU_MPEG4,
            Pixel::DXVA2_VLD => AV_PIX_FMT_DXVA2_VLD,

            Pixel::RGB444LE => AV_PIX_FMT_RGB444LE,
            Pixel::RGB444BE => AV_PIX_FMT_RGB444BE,
            Pixel::BGR444LE => AV_PIX_FMT_BGR444LE,
            Pixel::BGR444BE => AV_PIX_FMT_BGR444BE,
            Pixel::YA8 => AV_PIX_FMT_YA8,

            Pixel::BGR48BE => AV_PIX_FMT_BGR48BE,
            Pixel::BGR48LE => AV_PIX_FMT_BGR48LE,

            Pixel::YUV420P9BE => AV_PIX_FMT_YUV420P9BE,
            Pixel::YUV420P9LE => AV_PIX_FMT_YUV420P9LE,
            Pixel::YUV420P10BE => AV_PIX_FMT_YUV420P10BE,
            Pixel::YUV420P10LE => AV_PIX_FMT_YUV420P10LE,
            Pixel::YUV422P10BE => AV_PIX_FMT_YUV422P10BE,
            Pixel::YUV422P10LE => AV_PIX_FMT_YUV422P10LE,
            Pixel::YUV444P9BE => AV_PIX_FMT_YUV444P9BE,
            Pixel::YUV444P9LE => AV_PIX_FMT_YUV444P9LE,
            Pixel::YUV444P10BE => AV_PIX_FMT_YUV444P10BE,
            Pixel::YUV444P10LE => AV_PIX_FMT_YUV444P10LE,
            Pixel::YUV422P9BE => AV_PIX_FMT_YUV422P9BE,
            Pixel::YUV422P9LE => AV_PIX_FMT_YUV422P9LE,
            #[cfg(not(feature = "ffmpeg_4_0"))]
            Pixel::VDA_VLD => AV_PIX_FMT_VDA_VLD,

            Pixel::GBRP => AV_PIX_FMT_GBRP,
            Pixel::GBRP9BE => AV_PIX_FMT_GBRP9BE,
            Pixel::GBRP9LE => AV_PIX_FMT_GBRP9LE,
            Pixel::GBRP10BE => AV_PIX_FMT_GBRP10BE,
            Pixel::GBRP10LE => AV_PIX_FMT_GBRP10LE,
            Pixel::GBRP16BE => AV_PIX_FMT_GBRP16BE,
            Pixel::GBRP16LE => AV_PIX_FMT_GBRP16LE,

            Pixel::YUVA420P9BE => AV_PIX_FMT_YUVA420P9BE,
            Pixel::YUVA420P9LE => AV_PIX_FMT_YUVA420P9LE,
            Pixel::YUVA422P9BE => AV_PIX_FMT_YUVA422P9BE,
            Pixel::YUVA422P9LE => AV_PIX_FMT_YUVA422P9LE,
            Pixel::YUVA444P9BE => AV_PIX_FMT_YUVA444P9BE,
            Pixel::YUVA444P9LE => AV_PIX_FMT_YUVA444P9LE,
            Pixel::YUVA420P10BE => AV_PIX_FMT_YUVA420P10BE,
            Pixel::YUVA420P10LE => AV_PIX_FMT_YUVA420P10LE,
            Pixel::YUVA422P10BE => AV_PIX_FMT_YUVA422P10BE,
            Pixel::YUVA422P10LE => AV_PIX_FMT_YUVA422P10LE,
            Pixel::YUVA444P10BE => AV_PIX_FMT_YUVA444P10BE,
            Pixel::YUVA444P10LE => AV_PIX_FMT_YUVA444P10LE,
            Pixel::YUVA420P16BE => AV_PIX_FMT_YUVA420P16BE,
            Pixel::YUVA420P16LE => AV_PIX_FMT_YUVA420P16LE,
            Pixel::YUVA422P16BE => AV_PIX_FMT_YUVA422P16BE,
            Pixel::YUVA422P16LE => AV_PIX_FMT_YUVA422P16LE,
            Pixel::YUVA444P16BE => AV_PIX_FMT_YUVA444P16BE,
            Pixel::YUVA444P16LE => AV_PIX_FMT_YUVA444P16LE,

            Pixel::VDPAU => AV_PIX_FMT_VDPAU,

            Pixel::XYZ12LE => AV_PIX_FMT_XYZ12LE,
            Pixel::XYZ12BE => AV_PIX_FMT_XYZ12BE,
            Pixel::NV16 => AV_PIX_FMT_NV16,
            Pixel::NV20LE => AV_PIX_FMT_NV20LE,
            Pixel::NV20BE => AV_PIX_FMT_NV20BE,

            Pixel::RGBA64BE => AV_PIX_FMT_RGBA64BE,
            Pixel::RGBA64LE => AV_PIX_FMT_RGBA64LE,
            Pixel::BGRA64BE => AV_PIX_FMT_BGRA64BE,
            Pixel::BGRA64LE => AV_PIX_FMT_BGRA64LE,

            Pixel::YVYU422 => AV_PIX_FMT_YVYU422,

            #[cfg(not(feature = "ffmpeg_4_0"))]
            Pixel::VDA => AV_PIX_FMT_VDA,

            Pixel::YA16BE => AV_PIX_FMT_YA16BE,
            Pixel::YA16LE => AV_PIX_FMT_YA16LE,

            Pixel::QSV => AV_PIX_FMT_QSV,
            Pixel::MMAL => AV_PIX_FMT_MMAL,

            Pixel::D3D11VA_VLD => AV_PIX_FMT_D3D11VA_VLD,

            Pixel::CUDA => AV_PIX_FMT_CUDA,

            Pixel::ZRGB => AV_PIX_FMT_0RGB,
            Pixel::RGBZ => AV_PIX_FMT_RGB0,
            Pixel::ZBGR => AV_PIX_FMT_0BGR,
            Pixel::BGRZ => AV_PIX_FMT_BGR0,
            Pixel::YUVA444P => AV_PIX_FMT_YUVA444P,
            Pixel::YUVA422P => AV_PIX_FMT_YUVA422P,

            Pixel::YUV420P12BE => AV_PIX_FMT_YUV420P12BE,
            Pixel::YUV420P12LE => AV_PIX_FMT_YUV420P12LE,
            Pixel::YUV420P14BE => AV_PIX_FMT_YUV420P14BE,
            Pixel::YUV420P14LE => AV_PIX_FMT_YUV420P14LE,
            Pixel::YUV422P12BE => AV_PIX_FMT_YUV422P12BE,
            Pixel::YUV422P12LE => AV_PIX_FMT_YUV422P12LE,
            Pixel::YUV422P14BE => AV_PIX_FMT_YUV422P14BE,
            Pixel::YUV422P14LE => AV_PIX_FMT_YUV422P14LE,
            Pixel::YUV444P12BE => AV_PIX_FMT_YUV444P12BE,
            Pixel::YUV444P12LE => AV_PIX_FMT_YUV444P12LE,
            Pixel::YUV444P14BE => AV_PIX_FMT_YUV444P14BE,
            Pixel::YUV444P14LE => AV_PIX_FMT_YUV444P14LE,
            Pixel::GBRP12BE => AV_PIX_FMT_GBRP12BE,
            Pixel::GBRP12LE => AV_PIX_FMT_GBRP12LE,
            Pixel::GBRP14BE => AV_PIX_FMT_GBRP14BE,
            Pixel::GBRP14LE => AV_PIX_FMT_GBRP14LE,
            Pixel::GBRAP => AV_PIX_FMT_GBRAP,
            Pixel::GBRAP16BE => AV_PIX_FMT_GBRAP16BE,
            Pixel::GBRAP16LE => AV_PIX_FMT_GBRAP16LE,
            Pixel::YUVJ411P => AV_PIX_FMT_YUVJ411P,

            Pixel::BAYER_BGGR8 => AV_PIX_FMT_BAYER_BGGR8,
            Pixel::BAYER_RGGB8 => AV_PIX_FMT_BAYER_RGGB8,
            Pixel::BAYER_GBRG8 => AV_PIX_FMT_BAYER_GBRG8,
            Pixel::BAYER_GRBG8 => AV_PIX_FMT_BAYER_GRBG8,
            Pixel::BAYER_BGGR16LE => AV_PIX_FMT_BAYER_BGGR16LE,
            Pixel::BAYER_BGGR16BE => AV_PIX_FMT_BAYER_BGGR16BE,
            Pixel::BAYER_RGGB16LE => AV_PIX_FMT_BAYER_RGGB16LE,
            Pixel::BAYER_RGGB16BE => AV_PIX_FMT_BAYER_RGGB16BE,
            Pixel::BAYER_GBRG16LE => AV_PIX_FMT_BAYER_GBRG16LE,
            Pixel::BAYER_GBRG16BE => AV_PIX_FMT_BAYER_GBRG16BE,
            Pixel::BAYER_GRBG16LE => AV_PIX_FMT_BAYER_GRBG16LE,
            Pixel::BAYER_GRBG16BE => AV_PIX_FMT_BAYER_GRBG16BE,

            Pixel::YUV440P10LE => AV_PIX_FMT_YUV440P10LE,
            Pixel::YUV440P10BE => AV_PIX_FMT_YUV440P10BE,
            Pixel::YUV440P12LE => AV_PIX_FMT_YUV440P12LE,
            Pixel::YUV440P12BE => AV_PIX_FMT_YUV440P12BE,
            Pixel::AYUV64LE => AV_PIX_FMT_AYUV64LE,
            Pixel::AYUV64BE => AV_PIX_FMT_AYUV64BE,

            Pixel::VIDEOTOOLBOX => AV_PIX_FMT_VIDEOTOOLBOX,

            // --- defaults
            #[cfg(not(feature = "ff_api_xvmc"))]
            Pixel::XVMC => AV_PIX_FMT_XVMC,

            Pixel::RGB32 => AV_PIX_FMT_RGB32,
            Pixel::RGB32_1 => AV_PIX_FMT_RGB32_1,
            Pixel::BGR32 => AV_PIX_FMT_BGR32,
            Pixel::BGR32_1 => AV_PIX_FMT_BGR32_1,
            Pixel::ZRGB32 => AV_PIX_FMT_0RGB32,
            Pixel::ZBGR32 => AV_PIX_FMT_0BGR32,

            Pixel::GRAY16 => AV_PIX_FMT_GRAY16,
            Pixel::YA16 => AV_PIX_FMT_YA16,
            Pixel::RGB48 => AV_PIX_FMT_RGB48,
            Pixel::RGB565 => AV_PIX_FMT_RGB565,
            Pixel::RGB555 => AV_PIX_FMT_RGB555,
            Pixel::RGB444 => AV_PIX_FMT_RGB444,
            Pixel::BGR48 => AV_PIX_FMT_BGR48,
            Pixel::BGR565 => AV_PIX_FMT_BGR565,
            Pixel::BGR555 => AV_PIX_FMT_BGR555,
            Pixel::BGR444 => AV_PIX_FMT_BGR444,

            Pixel::YUV420P9 => AV_PIX_FMT_YUV420P9,
            Pixel::YUV422P9 => AV_PIX_FMT_YUV422P9,
            Pixel::YUV444P9 => AV_PIX_FMT_YUV444P9,
            Pixel::YUV420P10 => AV_PIX_FMT_YUV420P10,
            Pixel::YUV422P10 => AV_PIX_FMT_YUV422P10,
            Pixel::YUV440P10 => AV_PIX_FMT_YUV440P10,
            Pixel::YUV444P10 => AV_PIX_FMT_YUV444P10,
            Pixel::YUV420P12 => AV_PIX_FMT_YUV420P12,
            Pixel::YUV422P12 => AV_PIX_FMT_YUV422P12,
            Pixel::YUV440P12 => AV_PIX_FMT_YUV440P12,
            Pixel::YUV444P12 => AV_PIX_FMT_YUV444P12,
            Pixel::YUV420P14 => AV_PIX_FMT_YUV420P14,
            Pixel::YUV422P14 => AV_PIX_FMT_YUV422P14,
            Pixel::YUV444P14 => AV_PIX_FMT_YUV444P14,
            Pixel::YUV420P16 => AV_PIX_FMT_YUV420P16,
            Pixel::YUV422P16 => AV_PIX_FMT_YUV422P16,
            Pixel::YUV444P16 => AV_PIX_FMT_YUV444P16,

            Pixel::GBRP9 => AV_PIX_FMT_GBRP9,
            Pixel::GBRP10 => AV_PIX_FMT_GBRP10,
            Pixel::GBRP12 => AV_PIX_FMT_GBRP12,
            Pixel::GBRP14 => AV_PIX_FMT_GBRP14,
            Pixel::GBRP16 => AV_PIX_FMT_GBRP16,
            Pixel::GBRAP16 => AV_PIX_FMT_GBRAP16,

            Pixel::BAYER_BGGR16 => AV_PIX_FMT_BAYER_BGGR16,
            Pixel::BAYER_RGGB16 => AV_PIX_FMT_BAYER_RGGB16,
            Pixel::BAYER_GBRG16 => AV_PIX_FMT_BAYER_GBRG16,
            Pixel::BAYER_GRBG16 => AV_PIX_FMT_BAYER_GRBG16,

            Pixel::YUVA420P9 => AV_PIX_FMT_YUVA420P9,
            Pixel::YUVA422P9 => AV_PIX_FMT_YUVA422P9,
            Pixel::YUVA444P9 => AV_PIX_FMT_YUVA444P9,
            Pixel::YUVA420P10 => AV_PIX_FMT_YUVA420P10,
            Pixel::YUVA422P10 => AV_PIX_FMT_YUVA422P10,
            Pixel::YUVA444P10 => AV_PIX_FMT_YUVA444P10,
            Pixel::YUVA420P16 => AV_PIX_FMT_YUVA420P16,
            Pixel::YUVA422P16 => AV_PIX_FMT_YUVA422P16,
            Pixel::YUVA444P16 => AV_PIX_FMT_YUVA444P16,

            Pixel::XYZ12 => AV_PIX_FMT_XYZ12,
            Pixel::NV20 => AV_PIX_FMT_NV20,
            Pixel::AYUV64 => AV_PIX_FMT_AYUV64,

            Pixel::P010LE => AV_PIX_FMT_P010LE,
            Pixel::P010BE => AV_PIX_FMT_P010BE,
            Pixel::GBRAP12BE => AV_PIX_FMT_GBRAP12BE,
            Pixel::GBRAP12LE => AV_PIX_FMT_GBRAP12LE,
            Pixel::GBRAP10LE => AV_PIX_FMT_GBRAP10LE,
            Pixel::GBRAP10BE => AV_PIX_FMT_GBRAP10BE,
            Pixel::MEDIACODEC => AV_PIX_FMT_MEDIACODEC,
            Pixel::GRAY12BE => AV_PIX_FMT_GRAY12BE,
            Pixel::GRAY12LE => AV_PIX_FMT_GRAY12LE,
            Pixel::GRAY10BE => AV_PIX_FMT_GRAY10BE,
            Pixel::GRAY10LE => AV_PIX_FMT_GRAY10LE,
            Pixel::P016LE => AV_PIX_FMT_P016LE,
            Pixel::P016BE => AV_PIX_FMT_P016BE,

            Pixel::D3D11 => AV_PIX_FMT_D3D11,
            Pixel::GRAY9BE => AV_PIX_FMT_GRAY9BE,
            Pixel::GRAY9LE => AV_PIX_FMT_GRAY9LE,
            Pixel::GBRPF32BE => AV_PIX_FMT_GBRPF32BE,
            Pixel::GBRPF32LE => AV_PIX_FMT_GBRPF32LE,
            Pixel::GBRAPF32BE => AV_PIX_FMT_GBRAPF32BE,
            Pixel::GBRAPF32LE => AV_PIX_FMT_GBRAPF32LE,
            Pixel::DRM_PRIME => AV_PIX_FMT_DRM_PRIME,

            #[cfg(feature = "ffmpeg_4_0")]
            Pixel::OPENCL => AV_PIX_FMT_OPENCL,

            #[cfg(feature = "ffmpeg_4_1")]
            Pixel::GRAY14BE => AV_PIX_FMT_GRAY14BE,
            #[cfg(feature = "ffmpeg_4_1")]
            Pixel::GRAY14LE => AV_PIX_FMT_GRAY14LE,
            #[cfg(feature = "ffmpeg_4_1")]
            Pixel::GRAYF32BE => AV_PIX_FMT_GRAYF32BE,
            #[cfg(feature = "ffmpeg_4_1")]
            Pixel::GRAYF32LE => AV_PIX_FMT_GRAYF32LE,

            #[cfg(feature = "ffmpeg_4_2")]
            Pixel::YUVA422P12BE => AV_PIX_FMT_YUVA422P12BE,
            #[cfg(feature = "ffmpeg_4_2")]
            Pixel::YUVA422P12LE => AV_PIX_FMT_YUVA422P12LE,
            #[cfg(feature = "ffmpeg_4_2")]
            Pixel::YUVA444P12BE => AV_PIX_FMT_YUVA444P12BE,
            #[cfg(feature = "ffmpeg_4_2")]
            Pixel::YUVA444P12LE => AV_PIX_FMT_YUVA444P12LE,
            #[cfg(feature = "ffmpeg_4_2")]
            Pixel::NV24 => AV_PIX_FMT_NV24,
            #[cfg(feature = "ffmpeg_4_2")]
            Pixel::NV42 => AV_PIX_FMT_NV42,

            #[cfg(feature = "ffmpeg_4_3")]
            Pixel::VULKAN => AV_PIX_FMT_VULKAN,
            #[cfg(feature = "ffmpeg_4_3")]
            Pixel::Y210BE => AV_PIX_FMT_Y210BE,
            #[cfg(feature = "ffmpeg_4_3")]
            Pixel::Y210LE => AV_PIX_FMT_Y210LE,

            #[cfg(feature = "ffmpeg_4_4")]
            Pixel::X2RGB10LE => AV_PIX_FMT_X2RGB10LE,
            #[cfg(feature = "ffmpeg_4_4")]
            Pixel::X2RGB10BE => AV_PIX_FMT_X2RGB10BE,

            #[cfg(feature = "rpi")]
            Pixel::RPI => AV_PIX_FMT_RPI,
            #[cfg(feature = "rpi")]
            Pixel::SAND128 => AV_PIX_FMT_SAND128,
            #[cfg(feature = "rpi")]
            Pixel::SAND64_10 => AV_PIX_FMT_SAND64_10,
            #[cfg(feature = "rpi")]
            Pixel::SAND64_16 => AV_PIX_FMT_SAND64_16,
            #[cfg(feature = "rpi")]
            Pixel::RPI4_8 => AV_PIX_FMT_RPI4_8,
            #[cfg(feature = "rpi")]
            Pixel::RPI4_10 => AV_PIX_FMT_RPI4_10,
        }
    }
}

#[derive(Debug)]
pub enum ParsePixelError {
    NulError(NulError),
    UnknownFormat,
}

impl fmt::Display for ParsePixelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParsePixelError::NulError(ref e) => e.fmt(f),
            ParsePixelError::UnknownFormat => write!(f, "unknown pixel format"),
        }
    }
}

impl error::Error for ParsePixelError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            ParsePixelError::NulError(ref e) => Some(e),
            ParsePixelError::UnknownFormat => None,
        }
    }
}

impl From<NulError> for ParsePixelError {
    fn from(x: NulError) -> ParsePixelError {
        ParsePixelError::NulError(x)
    }
}

impl FromStr for Pixel {
    type Err = ParsePixelError;

    #[inline(always)]
    fn from_str(s: &str) -> Result<Pixel, ParsePixelError> {
        let cstring = CString::new(s)?;
        let format = unsafe { av_get_pix_fmt(cstring.as_ptr()) }.into();

        if format == Pixel::None {
            Err(ParsePixelError::UnknownFormat)
        } else {
            Ok(format)
        }
    }
}
