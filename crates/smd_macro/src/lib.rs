//! A crate containing the `smd` and `smd_borrowed` macros, which are the
//! workhorses that generate `SmithyComponent`s.

#![feature(proc_macro_span, proc_macro_raw_ident, slice_patterns)]
#![recursion_limit = "128"]
#![feature(drain_filter)]

use std::{
  cell::RefCell,
  collections::HashMap,
};

extern crate proc_macro;

mod parsers;
mod types;

type ProcMacroMap = HashMap<String, proc_macro::TokenStream>;
thread_local! {
  static SMD_CACHE: RefCell<ProcMacroMap> = RefCell::new(HashMap::new());
  static SMD_BORROWED_CACHE: RefCell<ProcMacroMap> = RefCell::new(HashMap::new());
}

/// proc-macro to take a `SmithyComponent`, capturing referenced variables.
#[proc_macro]
pub fn smd(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  smd_inner(input, true)
}

/// proc-macro to take a `SmithyComponent`, not capturing referenced variables.
///
/// A call to `smd_borrowed!` should usually be inside of a call to `smd!`.
#[proc_macro]
pub fn smd_borrowed(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  smd_inner(input, false)
}

fn with_cache<T>(should_move: bool, callback: impl FnOnce(&mut ProcMacroMap) -> T) -> T {
  let cache_local_key = if should_move {
    &SMD_CACHE
  } else {
    &SMD_BORROWED_CACHE
  };

  cache_local_key.with(|cache_cell| {
    let mut cache = cache_cell.borrow_mut();
    callback(&mut cache)
  })
}

fn smd_inner(input: proc_macro::TokenStream, should_move: bool) -> proc_macro::TokenStream {
  with_cache(should_move, |cache| {
    let as_str = input.to_string();
    if let Some(proc_macro_result) = cache.get(&as_str) {
      proc_macro_result.clone()
    } else {
      let input: proc_macro2::TokenStream = input.into();
      let vec_of_trees: Vec<proc_macro2::TokenTree> = input.into_iter().collect();
      let parsed = parsers::match_html_component(&vec_of_trees, should_move);

      let unwrapped = parsed.unwrap();
      #[cfg(feature = "smd-logs")]
      println!("\nlet mut app = {};\n", unwrapped.1);

      let proc_macro_result: proc_macro::TokenStream = unwrapped.1.into();
      cache.insert(as_str, proc_macro_result.clone());
      proc_macro_result
    }
  })
}
