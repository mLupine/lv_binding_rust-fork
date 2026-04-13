//! LVGL 9 screen wrapper.
//!
//! A "screen" in LVGL is just a top-level `lv_obj_t` with no parent. This
//! module is a thin convenience layer over [`crate::obj::Obj`] for that
//! pattern.

use core::ptr::NonNull;

use crate::obj::Obj;

/// LVGL screen handle. A non-owning view over the underlying `lv_obj_t`
/// (the display owns the object).
#[derive(Copy, Clone)]
pub struct Screen {
    obj: Obj,
}

impl Screen {
    /// Wrap a non-null `lv_obj_t` (returned by e.g.
    /// `lv_display_get_screen_active`).
    pub(crate) fn from_raw_unowned(raw: NonNull<lvgl_sys::lv_obj_t>) -> Self {
        Self {
            obj: unsafe { Obj::from_raw(raw) },
        }
    }

    /// Returns the underlying [`Obj`].
    pub fn as_obj(&self) -> Obj {
        self.obj
    }

    /// Borrow the raw pointer.
    pub fn as_ptr(&self) -> *mut lvgl_sys::lv_obj_t {
        self.obj.as_ptr()
    }
}
