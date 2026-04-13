//! LVGL 9 snapshot wrapper (`lv_snapshot_take`).
//!
//! Used by Plan 04-09 (DISP-03 pixel-byte parity gate): take an
//! RGB565 snapshot of a screen, copy the bytes out for diffing against
//! the C++-build baseline.
//!
//! Requires `LV_USE_SNAPSHOT=1` in `lv_conf.h` — set in the vendored
//! config at `lvgl-sys/vendor/include-9.2/lv_conf.h`.

use core::ptr::NonNull;
use core::slice;

use crate::color::ColorFormat;
use crate::obj::Obj;

/// An owned `lv_draw_buf_t*` returned by `lv_snapshot_take`. Drop calls
/// `lv_draw_buf_destroy` to free the LVGL-side allocation.
pub struct Snapshot {
    raw: NonNull<lvgl_sys::lv_draw_buf_t>,
}

impl Snapshot {
    /// Borrow the framebuffer bytes. Length comes from the underlying
    /// `lv_draw_buf_t.data_size`.
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            let buf = self.raw.as_ref();
            slice::from_raw_parts(buf.data as *const u8, buf.data_size as usize)
        }
    }

    /// The framebuffer's pixel size in bytes.
    pub fn len(&self) -> usize {
        unsafe { self.raw.as_ref().data_size as usize }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Borrow the raw `lv_draw_buf_t*`.
    pub fn as_ptr(&self) -> *mut lvgl_sys::lv_draw_buf_t {
        self.raw.as_ptr()
    }
}

impl Drop for Snapshot {
    fn drop(&mut self) {
        unsafe { lvgl_sys::lv_draw_buf_destroy(self.raw.as_ptr()) }
    }
}

/// Take a snapshot of an LVGL object (`lv_snapshot_take`).
///
/// Returns `None` if the snapshot allocation failed (LVGL OOM, or
/// `LV_USE_SNAPSHOT=0` in `lv_conf.h`).
///
/// **Locking:** the caller is responsible for holding `lvgl_port_lock` (or
/// otherwise serializing LVGL access) for the duration of this call.
pub fn snapshot_take(obj: &Obj, format: ColorFormat) -> Option<Snapshot> {
    let raw = unsafe { lvgl_sys::lv_snapshot_take(obj.as_ptr(), format.as_raw()) };
    NonNull::new(raw).map(|raw| Snapshot { raw })
}
