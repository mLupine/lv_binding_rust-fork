//! LVGL 9 display object wrapper (`lv_display_t`).
//!
//! In LVGL 8 a display was registered through a "driver-then-register"
//! flow (fill the legacy display-driver struct, then register it). LVGL 9
//! reverses that: [`Display::create`] allocates a display object directly
//! via `lv_display_create(hres, vres)`, then per-attribute setters
//! configure flush, buffers, color format, etc.
//!
//! Most consumers in this workspace will instead obtain a display via
//! [`Display::from_raw`] — `esp_lvgl_port`'s `lvgl_port_add_disp` returns
//! a `*mut lv_display_t` we wrap rather than re-creating ourselves. See
//! Phase 4 CONTEXT.md decision D-29-a.

use core::ptr::NonNull;

use crate::color::ColorFormat;

/// Render mode for [`Display::set_buffers`]. Mirrors LVGL 9's
/// `lv_display_render_mode_t`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DisplayRenderMode {
    /// `LV_DISPLAY_RENDER_MODE_PARTIAL` — buffer holds a slice of the
    /// screen; driver flushes incrementally. This is what `esp_lvgl_port`
    /// uses by default.
    Partial,
    /// `LV_DISPLAY_RENDER_MODE_DIRECT` — buffer is the full screen and
    /// drawn into directly.
    Direct,
    /// `LV_DISPLAY_RENDER_MODE_FULL` — like `Direct` but always re-renders
    /// the entire screen each frame.
    Full,
}

impl DisplayRenderMode {
    pub fn as_raw(self) -> lvgl_sys::lv_display_render_mode_t {
        use lvgl_sys as s;
        match self {
            DisplayRenderMode::Partial => s::lv_display_render_mode_t_LV_DISPLAY_RENDER_MODE_PARTIAL,
            DisplayRenderMode::Direct => s::lv_display_render_mode_t_LV_DISPLAY_RENDER_MODE_DIRECT,
            DisplayRenderMode::Full => s::lv_display_render_mode_t_LV_DISPLAY_RENDER_MODE_FULL,
        }
    }
}

/// An LVGL 9 display handle. Wraps `*mut lv_display_t`.
///
/// **Ownership model:** A `Display` constructed via [`Display::create`]
/// owns its `lv_display_t` and will call `lv_display_delete` on drop. A
/// `Display` constructed via [`Display::from_raw`] does *not* take
/// ownership — `esp_lvgl_port` (or whoever created the underlying object)
/// retains responsibility for tearing it down. This matches CONTEXT.md
/// D-29-a's "wrap pointers from `lvgl_port_add_disp`" pattern.
pub struct Display {
    raw: NonNull<lvgl_sys::lv_display_t>,
    owned: bool,
}

impl Display {
    /// Create a new display via `lv_display_create`. Returns `None` if LVGL
    /// is out of memory.
    pub fn create(hor_res: i32, ver_res: i32) -> Option<Self> {
        let raw = unsafe { lvgl_sys::lv_display_create(hor_res, ver_res) };
        NonNull::new(raw).map(|raw| Self { raw, owned: true })
    }

    /// Wrap a `*mut lv_display_t` returned by C code (e.g.
    /// `lvgl_port_add_disp`). The returned `Display` does **not** take
    /// ownership; the caller is responsible for the underlying lifetime.
    ///
    /// Returns `None` if `raw` is null.
    ///
    /// # Safety
    /// `raw` must be a valid `lv_display_t*` for the lifetime of this
    /// `Display`. The caller must not concurrently free it.
    pub unsafe fn from_raw(raw: *mut lvgl_sys::lv_display_t) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw, owned: false })
    }

    /// Default display (returned by `lv_display_get_default`). `None` if
    /// no display has been registered.
    pub fn default() -> Option<Self> {
        let raw = unsafe { lvgl_sys::lv_display_get_default() };
        // Default-display does NOT confer ownership.
        NonNull::new(raw).map(|raw| Self { raw, owned: false })
    }

    /// Borrow the raw pointer. Useful for passing through FFI boundaries
    /// to other LVGL 9 entry points not yet wrapped here.
    pub fn as_ptr(&self) -> *mut lvgl_sys::lv_display_t {
        self.raw.as_ptr()
    }

    /// Set the per-display color format (`lv_display_set_color_format`).
    pub fn set_color_format(&mut self, format: ColorFormat) {
        unsafe { lvgl_sys::lv_display_set_color_format(self.raw.as_ptr(), format.as_raw()) }
    }

    /// Get the per-display color format.
    pub fn color_format(&self) -> lvgl_sys::lv_color_format_t {
        unsafe { lvgl_sys::lv_display_get_color_format(self.raw.as_ptr()) }
    }

    /// Set the rendering buffers (`lv_display_set_buffers`). LVGL 9's
    /// buffer is an *untyped* byte buffer; the per-display color format
    /// (set via [`set_color_format`]) tells LVGL how to interpret it.
    ///
    /// # Safety
    /// `buf1` (and `buf2` if non-null) must each remain valid for at least
    /// `buf_size` bytes for the lifetime of the display.
    pub unsafe fn set_buffers(
        &mut self,
        buf1: *mut u8,
        buf2: *mut u8,
        buf_size: u32,
        render_mode: DisplayRenderMode,
    ) {
        lvgl_sys::lv_display_set_buffers(
            self.raw.as_ptr(),
            buf1.cast(),
            buf2.cast(),
            buf_size,
            render_mode.as_raw(),
        )
    }

    /// Set the flush callback (`lv_display_set_flush_cb`). LVGL 9's
    /// signature is `fn(*mut lv_display_t, *const lv_area_t, *mut u8)` —
    /// the buffer is **untyped bytes** (per the per-display color format),
    /// not the LVGL-8 `lv_color_t*`.
    ///
    /// `esp_lvgl_port` provides its own panel-IO-aware default flush, so
    /// most consumers will never call this directly. It exists for the
    /// rare case where a consumer wants a custom flush in pure Rust.
    pub fn set_flush_cb(&mut self, flush_cb: lvgl_sys::lv_display_flush_cb_t) {
        unsafe { lvgl_sys::lv_display_set_flush_cb(self.raw.as_ptr(), flush_cb) }
    }

    /// Mark this display as the default (`lv_display_set_default`).
    pub fn set_as_default(&mut self) {
        unsafe { lvgl_sys::lv_display_set_default(self.raw.as_ptr()) }
    }

    /// Active screen of this display (`lv_display_get_screen_active`).
    pub fn screen_active(&self) -> Option<crate::screen::Screen> {
        let raw = unsafe { lvgl_sys::lv_display_get_screen_active(self.raw.as_ptr()) };
        NonNull::new(raw).map(crate::screen::Screen::from_raw_unowned)
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        if self.owned {
            unsafe { lvgl_sys::lv_display_delete(self.raw.as_ptr()) }
        }
    }
}

// Display is not Send / Sync — LVGL 9 requires all access to happen from
// the LVGL task or under `lvgl_port_lock`.
