extern crate proc_macro;

mod parsers;
mod prelude;
mod utils;

#[proc_macro]
pub fn rsx(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  rsx_inner(input, true)
}

#[proc_macro]
pub fn rsx_borrowed(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  rsx_inner(input, false)
}

fn rsx_inner(input: proc_macro::TokenStream, should_move: bool) -> proc_macro::TokenStream {
  let input: proc_macro2::TokenStream = input.into();

  let (rest, parsed) = parsers::parse_items(input).unwrap();
  let (rest, _) = crate::utils::ensure_consumed(rest).unwrap();

  println!("parsed {:?}", parsed);
  // input.into()
  quote::quote!(3).into()
}
