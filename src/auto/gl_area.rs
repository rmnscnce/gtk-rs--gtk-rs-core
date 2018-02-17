// This file was generated by gir (https://github.com/gtk-rs/gir @ 8b9d0bb)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use Buildable;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use Error;
use Widget;
use ffi;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use gdk;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use gdk_ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use libc;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use signal::Inhibit;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct GLArea(Object<ffi::GtkGLArea, ffi::GtkGLAreaClass>): Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_gl_area_get_type(),
    }
}

impl GLArea {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn new() -> GLArea {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_full(ffi::gtk_gl_area_new()).downcast_unchecked()
        }
    }
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
impl Default for GLArea {
    fn default() -> Self {
        Self::new()
    }
}

pub trait GLAreaExt {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn attach_buffers(&self);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_auto_render(&self) -> bool;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_context(&self) -> Option<gdk::GLContext>;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_error(&self) -> Option<Error>;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_has_alpha(&self) -> bool;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_has_depth_buffer(&self) -> bool;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_has_stencil_buffer(&self) -> bool;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_required_version(&self) -> (i32, i32);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_use_es(&self) -> bool;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn make_current(&self);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn queue_render(&self);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_auto_render(&self, auto_render: bool);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_error<'a, P: Into<Option<&'a Error>>>(&self, error: P);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_has_alpha(&self, has_alpha: bool);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_has_depth_buffer(&self, has_depth_buffer: bool);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_has_stencil_buffer(&self, has_stencil_buffer: bool);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_required_version(&self, major: i32, minor: i32);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_use_es(&self, use_es: bool);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_create_context<F: Fn(&Self) -> gdk::GLContext + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_render<F: Fn(&Self, &gdk::GLContext) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_resize<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_auto_render_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_context_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_has_alpha_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_has_depth_buffer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_has_stencil_buffer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_use_es_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GLArea> + IsA<glib::object::Object>> GLAreaExt for O {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn attach_buffers(&self) {
        unsafe {
            ffi::gtk_gl_area_attach_buffers(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_auto_render(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gl_area_get_auto_render(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_context(&self) -> Option<gdk::GLContext> {
        unsafe {
            from_glib_none(ffi::gtk_gl_area_get_context(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_error(&self) -> Option<Error> {
        unsafe {
            from_glib_none(ffi::gtk_gl_area_get_error(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_has_alpha(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gl_area_get_has_alpha(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_has_depth_buffer(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gl_area_get_has_depth_buffer(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_has_stencil_buffer(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gl_area_get_has_stencil_buffer(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_required_version(&self) -> (i32, i32) {
        unsafe {
            let mut major = mem::uninitialized();
            let mut minor = mem::uninitialized();
            ffi::gtk_gl_area_get_required_version(self.to_glib_none().0, &mut major, &mut minor);
            (major, minor)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_use_es(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gl_area_get_use_es(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn make_current(&self) {
        unsafe {
            ffi::gtk_gl_area_make_current(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn queue_render(&self) {
        unsafe {
            ffi::gtk_gl_area_queue_render(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_auto_render(&self, auto_render: bool) {
        unsafe {
            ffi::gtk_gl_area_set_auto_render(self.to_glib_none().0, auto_render.to_glib());
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_error<'a, P: Into<Option<&'a Error>>>(&self, error: P) {
        let error = error.into();
        let error = error.to_glib_none();
        unsafe {
            ffi::gtk_gl_area_set_error(self.to_glib_none().0, error.0);
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_has_alpha(&self, has_alpha: bool) {
        unsafe {
            ffi::gtk_gl_area_set_has_alpha(self.to_glib_none().0, has_alpha.to_glib());
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_has_depth_buffer(&self, has_depth_buffer: bool) {
        unsafe {
            ffi::gtk_gl_area_set_has_depth_buffer(self.to_glib_none().0, has_depth_buffer.to_glib());
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_has_stencil_buffer(&self, has_stencil_buffer: bool) {
        unsafe {
            ffi::gtk_gl_area_set_has_stencil_buffer(self.to_glib_none().0, has_stencil_buffer.to_glib());
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_required_version(&self, major: i32, minor: i32) {
        unsafe {
            ffi::gtk_gl_area_set_required_version(self.to_glib_none().0, major, minor);
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_use_es(&self, use_es: bool) {
        unsafe {
            ffi::gtk_gl_area_set_use_es(self.to_glib_none().0, use_es.to_glib());
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_create_context<F: Fn(&Self) -> gdk::GLContext + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) -> gdk::GLContext + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "create-context",
                transmute(create_context_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_render<F: Fn(&Self, &gdk::GLContext) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &gdk::GLContext) -> Inhibit + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "render",
                transmute(render_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_resize<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "resize",
                transmute(resize_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_auto_render_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::auto-render",
                transmute(notify_auto_render_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_context_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::context",
                transmute(notify_context_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_has_alpha_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::has-alpha",
                transmute(notify_has_alpha_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_has_depth_buffer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::has-depth-buffer",
                transmute(notify_has_depth_buffer_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_has_stencil_buffer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::has-stencil-buffer",
                transmute(notify_has_stencil_buffer_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_use_es_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::use-es",
                transmute(notify_use_es_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn create_context_trampoline<P>(this: *mut ffi::GtkGLArea, f: glib_ffi::gpointer) -> *mut gdk_ffi::GdkGLContext
where P: IsA<GLArea> {
    let f: &&(Fn(&P) -> gdk::GLContext + 'static) = transmute(f);
    f(&GLArea::from_glib_borrow(this).downcast_unchecked()).to_glib_full()
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn render_trampoline<P>(this: *mut ffi::GtkGLArea, context: *mut gdk_ffi::GdkGLContext, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<GLArea> {
    let f: &&(Fn(&P, &gdk::GLContext) -> Inhibit + 'static) = transmute(f);
    f(&GLArea::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(context)).to_glib()
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn resize_trampoline<P>(this: *mut ffi::GtkGLArea, width: libc::c_int, height: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<GLArea> {
    let f: &&(Fn(&P, i32, i32) + 'static) = transmute(f);
    f(&GLArea::from_glib_borrow(this).downcast_unchecked(), width, height)
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn notify_auto_render_trampoline<P>(this: *mut ffi::GtkGLArea, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GLArea> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GLArea::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn notify_context_trampoline<P>(this: *mut ffi::GtkGLArea, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GLArea> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GLArea::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn notify_has_alpha_trampoline<P>(this: *mut ffi::GtkGLArea, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GLArea> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GLArea::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn notify_has_depth_buffer_trampoline<P>(this: *mut ffi::GtkGLArea, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GLArea> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GLArea::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn notify_has_stencil_buffer_trampoline<P>(this: *mut ffi::GtkGLArea, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GLArea> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GLArea::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
unsafe extern "C" fn notify_use_es_trampoline<P>(this: *mut ffi::GtkGLArea, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GLArea> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GLArea::from_glib_borrow(this).downcast_unchecked())
}
