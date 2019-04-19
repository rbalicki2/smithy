#![feature(proc_macro_span, proc_macro_raw_ident, slice_patterns)]
#![recursion_limit = "128"]
#![feature(drain_filter)]

extern crate proc_macro;

mod parsers;
mod types;

// TODO can we cache this?
// e.g. in a thread local or something?
// groups are duplicated in many places; if they include macros,
// that will cause those child macros to be compiled 5 times. (At least...?)
// doubly-nested macros will be compiled 5^2 times. Yikes!

#[proc_macro]
pub fn smd(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  smd_inner(input, true)
}

#[proc_macro]
pub fn smd_no_move(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  smd_inner(input, false)
}

fn smd_inner(input: proc_macro::TokenStream, should_move: bool) -> proc_macro::TokenStream {
  let input_2: proc_macro2::TokenStream = input.into();
  let vec_of_trees: Vec<proc_macro2::TokenTree> = input_2.into_iter().collect();
  let parsed = parsers::match_html_component(&vec_of_trees, should_move);

  let unwrapped = parsed.unwrap();
  #[cfg(feature = "smd-logs")]
  println!("\nlet mut a = {};\n", unwrapped.1);

  unwrapped.1.into()
}
