//! LVGL 9.2 safe wrapper crate (mLupine fork — `mLupine-lvgl-9` branch).
//!
//! This crate is a **minimum-viable port** of `lv_binding_rust` from LVGL 8.x
//! to LVGL 9.2.x, scoped to the surface that
//! [`lupin-display`](https://github.com/mLupine/lupin-esp32-fw) needs to drive
//! the M5Stack Atom S3R LCD via `esp_lvgl_port` on ESP-IDF v6.
//!
//! Per Phase 4 plan 04-02 (and CONTEXT.md decision D-29):
//!
//! * Display creation goes through `lvgl_port_add_disp` on the C side; Rust
//!   wraps the returned `*mut lv_display_t` via [`Display::from_raw`].
//! * The flush callback signature is the LVGL 9 one
//!   (`fn(*mut lv_display_t, *const lv_area_t, *mut u8)`) — see
//!   [`Display::set_flush_cb`].
//! * The LVGL 9 image descriptor (renamed from the LVGL 8 spelling with the
//!   `_img_` infix to one with `_image_`) is the descriptor type for image
//!   widgets — see [`Image::set_src_dsc`].
//! * `lv_snapshot_take` is exposed by [`snapshot::snapshot_take`] for the
//!   parity-byte capture flow Plan 04-09 will execute.
//!
//! **Out of scope (vs the upstream 0.6.2 LVGL 8 crate):** the
//! `embedded_graphics` integration, the `lv_drivers` SDL/Linux backends, the
//! `input_device` family, the `lv_core::style` builder, and the full widget
//! catalog were all removed during the port. They were tied to LVGL 8 type
//! shapes that no longer exist (the LVGL 8 display-driver object, input-
//! device driver object, draw-buffer struct, and the compile-time-color-
//! depth `lv_color_t`). They will return in a
//! follow-up plan if `tesla-can-thief` (the consumer) ever needs them.

#![cfg_attr(not(test), no_std)]

pub use lvgl_sys as sys;

pub mod color;
pub mod display;
pub mod image_dsc;
pub mod obj;
pub mod screen;
pub mod snapshot;
pub mod widgets;

pub use color::{Color, ColorFormat};
pub use display::{Display, DisplayRenderMode};
pub use image_dsc::{ImageDsc, ImageHeader};
pub use obj::Obj;
pub use screen::Screen;
pub use snapshot::{snapshot_take, Snapshot};

/// Initialize LVGL. Safe to call multiple times — `lv_is_initialized` is
/// checked internally.
///
/// On the ESP-IDF v6 target this is normally driven by `lvgl_port_init` from
/// the `esp_lvgl_port` C component; calling this from Rust is only useful for
/// host-side `cargo test` runs.
pub fn init() {
    unsafe {
        if !lvgl_sys::lv_is_initialized() {
            lvgl_sys::lv_init();
        }
    }
}

/// Tear down LVGL state. Mirror of [`init`]. Mostly useful for tests that
/// want a clean slate between iterations.
///
/// # Safety
/// All `Display`, `Obj`, `Screen`, etc. handles obtained from a prior `init`
/// must be dropped before this is called.
pub unsafe fn deinit() {
    if lvgl_sys::lv_is_initialized() {
        lvgl_sys::lv_deinit();
    }
}
