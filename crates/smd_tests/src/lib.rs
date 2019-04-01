#![feature(proc_macro_hygiene, slice_patterns)]

mod basic_event_handler_tests;
mod basic_post_rendering_tests;
mod basic_rendering_tests;

// TODO figure out how to test handle_ref_assignment without erroring:
// thread 'basic_ref_assignment_tests::tests::stuff' panicked at
// 'cannot call wasm-bindgen imported functions on non-wasm targets',
// test basic_post_rendering_tests::tests::basic_post_render ... ok
