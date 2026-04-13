//! LVGL 9 image widget wrapper.
//!
//! This is the LVGL 9 surface that replaces LVGL 8's image-widget
//! family. In LVGL 9 the C API renamed every `_img_` infix to `_image_`
//! (rename only, no behavioural change); this module exposes the new
//! names directly. A back-compat macro layer exists in the LVGL C
//! headers but Rust call sites should prefer the new spelling.

use core::ptr::NonNull;

use crate::image_dsc::ImageDsc;
use crate::obj::Obj;

/// LVGL image widget (`lv_image_*`).
#[derive(Copy, Clone)]
pub struct Image {
    obj: Obj,
}

impl Image {
    /// Create a new image widget as a child of `parent`
    /// (`lv_image_create`). Returns `None` on OOM.
    pub fn new(parent: &Obj) -> Option<Self> {
        let raw = unsafe { lvgl_sys::lv_image_create(parent.as_ptr()) };
        NonNull::new(raw).map(|raw| Self {
            obj: unsafe { Obj::from_raw(raw) },
        })
    }

    /// Set the image source from an LVGL 9 `lv_image_dsc_t` (renamed
    /// from the LVGL 8 image-descriptor with the `_img_` infix).
    ///
    /// # Safety
    /// `dsc` must remain valid for as long as the widget is being drawn.
    /// Typical usage is to pass a `&'static lv_image_dsc_t` baked into
    /// firmware via `extern "C"` from a C source file (the pepe image
    /// path used by `lupin-display`).
    pub unsafe fn set_src_dsc(&mut self, dsc: &ImageDsc) {
        lvgl_sys::lv_image_set_src(self.obj.as_ptr(), dsc as *const _ as *const _);
    }

    /// Set the image source from a raw pointer. Use this when the source
    /// is a string symbol or other non-descriptor value.
    ///
    /// # Safety
    /// `src` must be one of the source kinds LVGL accepts and live for
    /// the lifetime the widget renders it.
    pub unsafe fn set_src_raw(&mut self, src: *const cty::c_void) {
        lvgl_sys::lv_image_set_src(self.obj.as_ptr(), src);
    }

    /// The wrapped object.
    pub fn as_obj(&self) -> Obj {
        self.obj
    }
}
