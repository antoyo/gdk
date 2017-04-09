// This file was generated by gir (bf7bd49) from gir-files (71d73f0)
// DO NOT EDIT

#[cfg(feature = "v3_16")]
use Display;
#[cfg(feature = "v3_16")]
use Error;
#[cfg(feature = "v3_16")]
use Window;
use ffi;
use glib::translate::*;
#[cfg(feature = "v3_16")]
use std::mem;
#[cfg(feature = "v3_16")]
use std::ptr;

glib_wrapper! {
    pub struct GLContext(Object<ffi::GdkGLContext>);

    match fn {
        get_type => || ffi::gdk_gl_context_get_type(),
    }
}

impl GLContext {
    #[cfg(feature = "v3_16")]
    pub fn get_debug_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_gl_context_get_debug_enabled(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_display(&self) -> Option<Display> {
        unsafe {
            from_glib_none(ffi::gdk_gl_context_get_display(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_forward_compatible(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_gl_context_get_forward_compatible(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_required_version(&self) -> (i32, i32) {
        unsafe {
            let mut major = mem::uninitialized();
            let mut minor = mem::uninitialized();
            ffi::gdk_gl_context_get_required_version(self.to_glib_none().0, &mut major, &mut minor);
            (major, minor)
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_shared_context(&self) -> Option<GLContext> {
        unsafe {
            from_glib_none(ffi::gdk_gl_context_get_shared_context(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_22")]
    pub fn get_use_es(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_gl_context_get_use_es(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_version(&self) -> (i32, i32) {
        unsafe {
            let mut major = mem::uninitialized();
            let mut minor = mem::uninitialized();
            ffi::gdk_gl_context_get_version(self.to_glib_none().0, &mut major, &mut minor);
            (major, minor)
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_window(&self) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gdk_gl_context_get_window(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_20")]
    pub fn is_legacy(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_gl_context_is_legacy(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn make_current(&self) {
        unsafe {
            ffi::gdk_gl_context_make_current(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn realize(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gdk_gl_context_realize(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn set_debug_enabled(&self, enabled: bool) {
        unsafe {
            ffi::gdk_gl_context_set_debug_enabled(self.to_glib_none().0, enabled.to_glib());
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn set_forward_compatible(&self, compatible: bool) {
        unsafe {
            ffi::gdk_gl_context_set_forward_compatible(self.to_glib_none().0, compatible.to_glib());
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn set_required_version(&self, major: i32, minor: i32) {
        unsafe {
            ffi::gdk_gl_context_set_required_version(self.to_glib_none().0, major, minor);
        }
    }

    #[cfg(feature = "v3_22")]
    pub fn set_use_es(&self, use_es: i32) {
        unsafe {
            ffi::gdk_gl_context_set_use_es(self.to_glib_none().0, use_es);
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn clear_current() {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gdk_gl_context_clear_current();
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_current() -> Option<GLContext> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gdk_gl_context_get_current())
        }
    }
}
