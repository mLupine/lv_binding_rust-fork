// LVGL 9 port: build.rs is a no-op.
//
// In the LVGL 8 upstream this script ran `lvgl-codegen` against the
// `lvgl-sys` bindings to emit `widgets/generated.rs` with macro-generated
// safe wrappers for every widget. The codegen pipeline depends on LVGL 8
// signature shapes (`lv_obj_t * lv_btn_create(lv_obj_t *)`, no `image_dsc`
// rename, etc.) and the widget surface we need from Rust is small enough
// (label + image + snapshot) that hand-authored wrappers under
// `src/widgets/` are clearer and cheaper to maintain than re-targeting
// `lvgl-codegen` for LVGL 9.
//
// Re-introducing codegen for LVGL 9 widgets is tracked as follow-up work
// in the upstream PR draft (Phase 4 Plan 04-09).

fn main() {}
