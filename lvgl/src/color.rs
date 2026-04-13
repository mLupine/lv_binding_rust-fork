//! LVGL 9 color types.
//!
//! In LVGL 9 the basic `lv_color_t` is **always** native RGB888 (struct
//! layout `{ blue, green, red }` — see `lvgl-sys/shims/lvgl_sys.c`). The
//! per-display color *format* is a separate enum (`lv_color_format_t`) so
//! displays can render into RGB565, RGB888, RGB565+A8, etc., independently
//! of how `lv_color_t` is laid out in memory. Driver code chooses the
//! framebuffer format via [`Display::set_color_format`].

/// LVGL color value (always native RGB888 in 9.x).
#[derive(Copy, Clone, Default)]
pub struct Color {
    pub(crate) raw: lvgl_sys::lv_color_t,
}

impl core::fmt::Debug for Color {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Color")
            .field("r", &self.r())
            .field("g", &self.g())
            .field("b", &self.b())
            .finish()
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r() == other.r() && self.g() == other.g() && self.b() == other.b()
    }
}

impl Eq for Color {}

impl Color {
    /// Build a `Color` from RGB triple components.
    pub fn from_rgb((r, g, b): (u8, u8, u8)) -> Self {
        let raw = unsafe { lvgl_sys::_LV_COLOR_MAKE(r, g, b) };
        Self { raw }
    }

    /// Build a `Color` from a raw `lv_color_t`.
    pub fn from_raw(raw: lvgl_sys::lv_color_t) -> Self {
        Self { raw }
    }

    /// Red channel (0..=255).
    pub fn r(&self) -> u8 {
        unsafe { lvgl_sys::_LV_COLOR_GET_R(self.raw) as u8 }
    }

    /// Green channel (0..=255).
    pub fn g(&self) -> u8 {
        unsafe { lvgl_sys::_LV_COLOR_GET_G(self.raw) as u8 }
    }

    /// Blue channel (0..=255).
    pub fn b(&self) -> u8 {
        unsafe { lvgl_sys::_LV_COLOR_GET_B(self.raw) as u8 }
    }

    /// Borrow the raw `lv_color_t`.
    pub fn raw(&self) -> lvgl_sys::lv_color_t {
        self.raw
    }
}

/// LVGL 9 per-display color format. Mirrors the `LV_COLOR_FORMAT_*` enum
/// values from `lv_color_format_t`.
///
/// The set is intentionally narrow: just the formats `lupin-display` cares
/// about (RGB565 for the Atom S3R LCD, RGB888 for snapshots, I1 for the
/// Mini OLED in Phase 7). Add variants when consumers need them.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ColorFormat {
    /// `LV_COLOR_FORMAT_NATIVE` — driver-default native format. Equivalent
    /// to RGB565 with `LV_COLOR_DEPTH=16` (our `lv_conf.h`).
    Native,
    /// `LV_COLOR_FORMAT_RGB565`.
    Rgb565,
    /// `LV_COLOR_FORMAT_RGB888`.
    Rgb888,
    /// `LV_COLOR_FORMAT_ARGB8888`.
    Argb8888,
    /// `LV_COLOR_FORMAT_I1` — 1-bit indexed (monochrome).
    I1,
}

impl ColorFormat {
    /// Cast back to the raw `lv_color_format_t` value.
    pub fn as_raw(self) -> lvgl_sys::lv_color_format_t {
        use lvgl_sys as s;
        match self {
            ColorFormat::Native => s::lv_color_format_t_LV_COLOR_FORMAT_NATIVE,
            ColorFormat::Rgb565 => s::lv_color_format_t_LV_COLOR_FORMAT_RGB565,
            ColorFormat::Rgb888 => s::lv_color_format_t_LV_COLOR_FORMAT_RGB888,
            ColorFormat::Argb8888 => s::lv_color_format_t_LV_COLOR_FORMAT_ARGB8888,
            ColorFormat::I1 => s::lv_color_format_t_LV_COLOR_FORMAT_I1,
        }
    }
}
