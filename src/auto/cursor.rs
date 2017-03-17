// This file was generated by gir (310a2da) from gir-files (71d73f0)
// DO NOT EDIT

use CursorType;
use Display;
#[cfg(feature = "v3_10")]
use cairo;
use ffi;
use gdk_pixbuf;
use glib::Value;
use glib::translate::*;
use gobject_ffi;
#[cfg(feature = "v3_10")]
use std::mem;
use std::mem::transmute;

glib_wrapper! {
    pub struct Cursor(Object<ffi::GdkCursor>);

    match fn {
        get_type => || ffi::gdk_cursor_get_type(),
    }
}

impl Cursor {
    pub fn new(cursor_type: CursorType) -> Cursor {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gdk_cursor_new(cursor_type.to_glib()))
        }
    }

    pub fn new_for_display(display: &Display, cursor_type: CursorType) -> Cursor {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gdk_cursor_new_for_display(display.to_glib_none().0, cursor_type.to_glib()))
        }
    }

    pub fn new_from_name(display: &Display, name: &str) -> Cursor {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gdk_cursor_new_from_name(display.to_glib_none().0, name.to_glib_none().0))
        }
    }

    pub fn new_from_pixbuf(display: &Display, pixbuf: &gdk_pixbuf::Pixbuf, x: i32, y: i32) -> Cursor {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gdk_cursor_new_from_pixbuf(display.to_glib_none().0, pixbuf.to_glib_none().0, x, y))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn new_from_surface(display: &Display, surface: &cairo::Surface, x: f64, y: f64) -> Cursor {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gdk_cursor_new_from_surface(display.to_glib_none().0, mut_override(surface.to_glib_none().0), x, y))
        }
    }

    pub fn get_cursor_type(&self) -> CursorType {
        unsafe {
            from_glib(ffi::gdk_cursor_get_cursor_type(self.to_glib_none().0))
        }
    }

    pub fn get_display(&self) -> Display {
        unsafe {
            from_glib_none(ffi::gdk_cursor_get_display(self.to_glib_none().0))
        }
    }

    pub fn get_image(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_full(ffi::gdk_cursor_get_image(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_surface(&self) -> (Option<cairo::Surface>, f64, f64) {
        unsafe {
            let mut x_hot = mem::uninitialized();
            let mut y_hot = mem::uninitialized();
            let ret = from_glib_full(ffi::gdk_cursor_get_surface(self.to_glib_none().0, &mut x_hot, &mut y_hot));
            (ret, x_hot, y_hot)
        }
    }

    pub fn get_property_cursor_type(&self) -> CursorType {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "cursor-type".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    pub fn set_property_cursor_type(&self, cursor_type: CursorType) {
        let cursor_type = cursor_type.to_glib() as i32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "cursor-type".to_glib_none().0, Value::from(&cursor_type).to_glib_none().0);
        }
    }

    pub fn get_property_display(&self) -> Option<Display> {
        let mut value = Value::from(None::<&Display>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "display".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn set_property_display(&self, display: Option<&Display>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "display".to_glib_none().0, Value::from(display).to_glib_none().0);
        }
    }
}
