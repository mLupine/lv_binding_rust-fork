//! LVGL 9 generic object wrapper (`lv_obj_t`).

use core::ptr::NonNull;

/// A non-owning wrapper around `*mut lv_obj_t`. Most LVGL objects are
/// owned by their parent screen; `Obj` is just a typed handle.
#[derive(Copy, Clone)]
pub struct Obj {
    raw: NonNull<lvgl_sys::lv_obj_t>,
}

impl Obj {
    /// Wrap a non-null `lv_obj_t` pointer.
    ///
    /// # Safety
    /// `raw` must point at a live `lv_obj_t` for the lifetime of this
    /// handle.
    pub unsafe fn from_raw(raw: NonNull<lvgl_sys::lv_obj_t>) -> Self {
        Self { raw }
    }

    /// Borrow the raw pointer.
    pub fn as_ptr(&self) -> *mut lvgl_sys::lv_obj_t {
        self.raw.as_ptr()
    }

    /// Delete the underlying `lv_obj_t` (`lv_obj_delete`). The handle
    /// is consumed; the C-side memory is freed.
    pub fn delete(self) {
        unsafe { lvgl_sys::lv_obj_delete(self.raw.as_ptr()) }
    }
}

/// The currently active screen of the default display
/// (`lv_screen_active`). Returns `None` if no display is registered.
pub fn screen_active() -> Option<Obj> {
    let raw = unsafe { lvgl_sys::lv_screen_active() };
    NonNull::new(raw).map(|raw| Obj { raw })
}
