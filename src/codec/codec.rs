use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use super::{Audio, Capabilities, Id, Profile, Video};
use ffi::*;
use {media, Error};

/** A wrapper around an `AVCodec` pointer. */
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Codec {
    ptr: *mut AVCodec,
}

unsafe impl Send for Codec {}
unsafe impl Sync for Codec {}

impl Codec {
    pub unsafe fn wrap(ptr: *mut AVCodec) -> Self {
        Codec { ptr }
    }

    pub unsafe fn as_ptr(&self) -> *const AVCodec {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVCodec {
        self.ptr
    }
}

impl Codec {
    pub fn is_encoder(&self) -> bool {
        unsafe { av_codec_is_encoder(self.as_ptr()) != 0 }
    }

    pub fn is_decoder(&self) -> bool {
        unsafe { av_codec_is_decoder(self.as_ptr()) != 0 }
    }

    /** Name of the codec implementation.
     *
     * The name is globally unique among encoders and among decoders (but an encoder and a decoder
     * can share the same name). This is the primary way to find a codec from the user perspective.
     */
    pub fn name(&self) -> &str {
        unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).name).to_bytes()) }
    }

    /** Description aka long_name. Meant to be more human readable
     * than name. */
    pub fn description(&self) -> &str {
        unsafe {
            let long_name = (*self.as_ptr()).long_name;
            if long_name.is_null() {
                ""
            } else {
                from_utf8_unchecked(CStr::from_ptr(long_name).to_bytes())
            }
        }
    }

    /** Get the media type of the codec. */
    pub fn medium(&self) -> media::Type {
        unsafe { media::Type::from((*self.as_ptr()).type_) }
    }

    pub fn id(&self) -> Id {
        unsafe { Id::from((*self.as_ptr()).id) }
    }

    pub fn is_video(&self) -> bool {
        self.medium() == media::Type::Video
    }

    /** Get the video data if it exists, otherwise error. */
    pub fn video(self) -> Result<Video, Error> {
        unsafe {
            if self.medium() == media::Type::Video {
                Ok(Video::new(self))
            } else {
                Err(Error::InvalidData)
            }
        }
    }

    pub fn is_audio(&self) -> bool {
        self.medium() == media::Type::Audio
    }

    /** Get the audio data if it exists, otherwise error. */
    pub fn audio(self) -> Result<Audio, Error> {
        unsafe {
            if self.medium() == media::Type::Audio {
                Ok(Audio::new(self))
            } else {
                Err(Error::InvalidData)
            }
        }
    }

    pub fn max_lowres(&self) -> i32 {
        unsafe { av_codec_get_max_lowres(self.as_ptr()) }
    }

    /** Codec capabilities.
     *
     * See `Capabilities`. */
    pub fn capabilities(&self) -> Capabilities {
        unsafe { Capabilities::from_bits_truncate((*self.as_ptr()).capabilities as u32) }
    }

    pub fn profiles(&self) -> Option<ProfileIter> {
        unsafe {
            if (*self.as_ptr()).profiles.is_null() {
                None
            } else {
                Some(ProfileIter::new(self.id(), (*self.as_ptr()).profiles))
            }
        }
    }
}

pub struct ProfileIter {
    id: Id,
    ptr: *const AVProfile,
}

impl ProfileIter {
    pub fn new(id: Id, ptr: *const AVProfile) -> Self {
        ProfileIter { id, ptr }
    }
}

impl Iterator for ProfileIter {
    type Item = Profile;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if (*self.ptr).profile == FF_PROFILE_UNKNOWN {
                return None;
            }

            let profile = Profile::from((self.id, (*self.ptr).profile));
            self.ptr = self.ptr.offset(1);

            Some(profile)
        }
    }
}
