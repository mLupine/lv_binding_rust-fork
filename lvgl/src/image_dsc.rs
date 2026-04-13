//! LVGL 9 image descriptor wrapper.
//!
//! In LVGL 8 the descriptor type was spelled with an `_img_` infix; LVGL
//! 9 renamed it (rename only, layout-compatible) to use `_image_`. This
//! module re-exports the LVGL 9 name and provides a thin Rust facade.

/// Re-export of the LVGL 9 image header.
pub type ImageHeader = lvgl_sys::lv_image_header_t;

/// LVGL 9 image descriptor. Owns no memory itself — the `data` pointer
/// references caller-owned bytes (typically a `static` array baked into
/// the firmware image, e.g. `lupin-display`'s `pepe.h`).
pub type ImageDsc = lvgl_sys::lv_image_dsc_t; // LVGL 9 type name

