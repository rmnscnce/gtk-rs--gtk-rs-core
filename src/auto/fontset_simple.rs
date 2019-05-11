// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Font;
use Fontset;
use Language;
use glib::object::IsA;
use glib::translate::*;
use pango_sys;
use std::fmt;

glib_wrapper! {
    pub struct FontsetSimple(Object<pango_sys::PangoFontsetSimple, pango_sys::PangoFontsetSimpleClass, FontsetSimpleClass>) @extends Fontset;

    match fn {
        get_type => || pango_sys::pango_fontset_simple_get_type(),
    }
}

impl FontsetSimple {
    pub fn new(language: &mut Language) -> FontsetSimple {
        unsafe {
            from_glib_full(pango_sys::pango_fontset_simple_new(language.to_glib_none_mut().0))
        }
    }

    pub fn append<P: IsA<Font>>(&self, font: &P) {
        unsafe {
            pango_sys::pango_fontset_simple_append(self.to_glib_none().0, font.as_ref().to_glib_none().0);
        }
    }

    pub fn size(&self) -> i32 {
        unsafe {
            pango_sys::pango_fontset_simple_size(self.to_glib_none().0)
        }
    }
}

impl fmt::Display for FontsetSimple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FontsetSimple")
    }
}
