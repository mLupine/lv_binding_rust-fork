//! Minimal LVGL 9 widget wrappers used by `lupin-display`.
//!
//! Only `label` and `image` survive the 8→9 port. The full upstream
//! widget set (arc, bar, slider, table, meter, keyboard) was tied to the
//! LVGL 8 codegen pipeline and is out of scope for the Phase 4 deliverable.

pub mod image;
pub mod label;

pub use image::Image;
pub use label::Label;
