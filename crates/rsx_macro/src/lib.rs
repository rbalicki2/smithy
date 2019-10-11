extern crate proc_macro;

#[proc_macro]
pub fn rsx(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  rsx_inner(input, true)
}

#[proc_macro]
pub fn rsx_borrowed(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  rsx_inner(input, false)
}

fn rsx_inner(input: proc_macro::TokenStream, should_move: bool) -> proc_macro::TokenStream {
  quote::quote!(3).into()
}
