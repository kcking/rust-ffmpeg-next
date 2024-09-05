use std::ffi::CStr;
use std::marker::PhantomData;
use std::str::from_utf8_unchecked;

use super::{Flags, Type};
use ffi::*;
use {format, Picture};

pub enum Rect<'a> {
    None(*const AVSubtitleRect),
    Bitmap(Bitmap<'a>),
    /** 0 terminated plain UTF-8 text. */
    Text(Text<'a>),
    /** 0 terminated ASS/SSA compatible event line.
     *
     * The presentation of this is unaffected by the other values in
     * this struct. */
    Ass(Ass<'a>),
}

impl<'a> Rect<'a> {
    pub unsafe fn wrap(ptr: *const AVSubtitleRect) -> Self {
        match Type::from((*ptr).type_) {
            Type::None => Rect::None(ptr),
            Type::Bitmap => Rect::Bitmap(Bitmap::wrap(ptr)),
            Type::Text => Rect::Text(Text::wrap(ptr)),
            Type::Ass => Rect::Ass(Ass::wrap(ptr)),
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVSubtitleRect {
        match *self {
            Rect::None(ptr) => ptr,
            Rect::Bitmap(ref b) => b.as_ptr(),
            Rect::Text(ref t) => t.as_ptr(),
            Rect::Ass(ref a) => a.as_ptr(),
        }
    }
}

impl<'a> Rect<'a> {
    pub fn flags(&self) -> Flags {
        unsafe {
            Flags::from_bits_truncate(match *self {
                Rect::None(ptr) => (*ptr).flags,
                Rect::Bitmap(ref b) => (*b.as_ptr()).flags,
                Rect::Text(ref t) => (*t.as_ptr()).flags,
                Rect::Ass(ref a) => (*a.as_ptr()).flags,
            })
        }
    }
}

/** Wrapper around a Bitmap `AVSubtitleRect` ptr. */
pub struct Bitmap<'a> {
    ptr: *const AVSubtitleRect,

    _marker: PhantomData<&'a ()>,
}

impl<'a> Bitmap<'a> {
    pub unsafe fn wrap(ptr: *const AVSubtitleRect) -> Self {
        Bitmap {
            ptr,
            _marker: PhantomData,
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVSubtitleRect {
        self.ptr
    }
}

impl<'a> Bitmap<'a> {
    /** Top left corner of pict, undefined when pict not set. */
    pub fn x(&self) -> usize {
        unsafe { (*self.as_ptr()).x as usize }
    }

    /** Top left corner of pict, undefined when pict not set. */
    pub fn y(&self) -> usize {
        unsafe { (*self.as_ptr()).y as usize }
    }

    /** Width of pict. Undefined when pict not set. */
    pub fn width(&self) -> u32 {
        unsafe { (*self.as_ptr()).w as u32 }
    }

    /** Height of pict. Undefined when pict not set. */
    pub fn height(&self) -> u32 {
        unsafe { (*self.as_ptr()).h as u32 }
    }

    /** Number of colors in pict. Undefined when pict not set. */
    pub fn colors(&self) -> usize {
        unsafe { (*self.as_ptr()).nb_colors as usize }
    }

    // XXX: must split Picture and PictureMut
    pub fn picture(&self, format: format::Pixel) -> Picture<'a> {
        unsafe {
            Picture::wrap(
                &(*self.as_ptr()).pict as *const _ as *mut _,
                format,
                (*self.as_ptr()).w as u32,
                (*self.as_ptr()).h as u32,
            )
        }
    }
}

/** Wrapper around a Text `AVSubtitleRect` ptr. */
pub struct Text<'a> {
    ptr: *const AVSubtitleRect,

    _marker: PhantomData<&'a ()>,
}

impl<'a> Text<'a> {
    pub unsafe fn wrap(ptr: *const AVSubtitleRect) -> Self {
        Text {
            ptr,
            _marker: PhantomData,
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVSubtitleRect {
        self.ptr
    }
}

impl<'a> Text<'a> {
    pub fn get(&self) -> &str {
        unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).text).to_bytes()) }
    }
}

/** Wrapper around an ASS `AVSubtitleRect` ptr. */
pub struct Ass<'a> {
    ptr: *const AVSubtitleRect,

    _marker: PhantomData<&'a ()>,
}

impl<'a> Ass<'a> {
    pub unsafe fn wrap(ptr: *const AVSubtitleRect) -> Self {
        Ass {
            ptr,
            _marker: PhantomData,
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVSubtitleRect {
        self.ptr
    }
}

impl<'a> Ass<'a> {
    pub fn get(&self) -> &str {
        unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).ass).to_bytes()) }
    }
}
