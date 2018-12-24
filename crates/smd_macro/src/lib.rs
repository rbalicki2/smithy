#![feature(proc_macro_span, proc_macro_raw_ident, slice_patterns)]
#![recursion_limit = "128"]

extern crate proc_macro;

mod parsers;
mod types;

#[proc_macro]
pub fn smd(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input_2: proc_macro2::TokenStream = input.into();
  // println!("input {:?}", input_2);
  let vec_of_trees: Vec<proc_macro2::TokenTree> = input_2.into_iter().collect();

  let parsed = parsers::match_smd(&vec_of_trees);
  // println!("\nin smd parsed - {:?}", parsed);

  let unwrapped = parsed.unwrap();
  // println!("\nlet mut a = {};\n", unwrapped.1);
  let remaining = unwrapped.0;

  // TODO handle this at the nom level
  if remaining.len() > 0 {
    panic!("the smd! macro had left over characters. Make sure you only pass one html node.");
  }

  unwrapped.1.into()
}
