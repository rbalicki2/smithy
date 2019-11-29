extern crate proc_macro;

mod convert_to_component;
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

  println!("\n\nparsing input: {}", input);
  let (rest, parsed) = parsers::parse_items(input).unwrap();
  println!("final rest (should be empty to not panic) '{}'", rest);
  let _ = crate::utils::ensure_consumed(rest).unwrap();

  // println!("final rsx parsed into\n{:#?}", parsed);

  let converted = convert_to_component::convert_to_component(parsed, should_move);
  println!("converted to\n{}", converted);
  // input.into()
  converted.into()
}

#[proc_macro]
pub fn post_render(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input: proc_macro2::TokenStream = input.into();
  convert_to_component::outer_wrap(
    quote::quote!(
      // TODO remove this hack!
      ::smithy::types::Phase::Rendering =>
        ::smithy::types::PhaseResult::Rendering(::smithy::types::Node::Vec(vec![])),
      ::smithy::types::Phase::PostRendering => {
        (#input)();
        ::smithy::types::PhaseResult::PostRendering
      },
    ),
    false,
    true,
  )
  .into()
}
