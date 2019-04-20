//! Smithy is a framework for writing WebAssembly applications entirely
//! in Rust.
//! Its goal is to allow you to do so using ergonomic, idiomatic Rust,
//! without giving up any of the compiler’s safety guarantees.
//!
//! # Example
//! ```rs
//! #[wasm_bindgen]
//! fn start() {
//!   let app = smd!(<div>hello world</div>);
//!   let el = web_sys::window()
//!     .and_then(|w| w.document())
//!     .query_selector("#app").unwrap();
//!   smithy::mount(app, el);
//! }
//! ```
//!
//! > N.B. these docs seem to omit `smd!` and `smd_no_move!`, which are
//! > re-exported from the `smd_macro` crate.

/// A module that re-exports useful Smithy types, and some others.
pub mod types {
  pub use smithy_types::*;
}

pub use smd_macro::*;
pub use smithy_core::*;
