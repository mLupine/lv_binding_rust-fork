//! LVGL 9 label widget wrapper.

use core::ptr::NonNull;

use crate::obj::Obj;

/// LVGL label widget. Non-owning view of an `lv_obj_t` created via
/// `lv_label_create(parent)`.
#[derive(Copy, Clone)]
pub struct Label {
    obj: Obj,
}

impl Label {
    /// Create a new label as a child of `parent`. Returns `None` on OOM.
    pub fn new(parent: &Obj) -> Option<Self> {
        let raw = unsafe { lvgl_sys::lv_label_create(parent.as_ptr()) };
        NonNull::new(raw).map(|raw| Self {
            obj: unsafe { Obj::from_raw(raw) },
        })
    }

    /// Set the label's text. The bytes are copied LVGL-side
    /// (`lv_label_set_text` allocates an internal copy), so `text` does
    /// not need to outlive this call.
    pub fn set_text(&mut self, text: &core::ffi::CStr) {
        unsafe {
            lvgl_sys::lv_label_set_text(self.obj.as_ptr(), text.as_ptr() as *const _);
        }
    }

    /// Set static text (`lv_label_set_text_static` — no LVGL-side copy).
    ///
    /// # Safety
    /// `text` must remain valid for the lifetime of the label.
    pub unsafe fn set_text_static(&mut self, text: &'static core::ffi::CStr) {
        lvgl_sys::lv_label_set_text_static(self.obj.as_ptr(), text.as_ptr() as *const _);
    }

    /// The wrapped object.
    pub fn as_obj(&self) -> Obj {
        self.obj
    }
}
