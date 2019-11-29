extern crate proc_macro;
use crate::parsers::RsxItemOrLiteral;

mod convert_to_component;
mod parsers;
mod prelude;
mod utils;
mod with_ref;

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

  // println!("\n\nparsing input: {}", input);
  let (rest, parsed) = parsers::parse_items(input).unwrap();
  // println!("final rest (should be empty to not panic) '{}'", rest);
  let _ = crate::utils::ensure_consumed(rest).unwrap();

  let converted = convert_to_component::convert_to_component(parsed, should_move);
  // println!("converted to\n{}", converted);
  // input.into()
  converted.into()
}

// TODO this could probably be a macro_rules!
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

#[proc_macro]
pub fn with_ref(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input: proc_macro2::TokenStream = input.into();
  let (rest, (ref_variable, rsx_item)) = with_ref::parse_with_ref(input).unwrap();
  let _ = crate::utils::ensure_consumed(rest).unwrap();

  if let RsxItemOrLiteral::Node(_) = rsx_item {
  } else {
    // TODO re-write this error message to be more helpful.
    panic!("You must pass an rsx token as the second parameter to with_ref!, e.g. (div)");
  }

  let match_arms = convert_to_component::match_statement::get_match_arms(&vec![rsx_item]);
  let match_arms = quote::quote!(
    #match_arms
    ::smithy::types::Phase::RefAssignment(ref path) => {
      let mut path = path.clone();
      path.push(0);
      let selector = path
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

      let document = web_sys::window().unwrap().document().unwrap();
      let el_opt: Option<::web_sys::HtmlElement> = document
        .query_selector(&format!("[data-smithy-path=\"{}\"]", selector))
        .unwrap()
        .map(::wasm_bindgen::JsCast::unchecked_into);

      #ref_variable = el_opt;
      ::smithy::types::PhaseResult::RefAssignment
    },
  );
  // TODO outer_wrap(match_arms, false, true), which doesn't work
  // for simple testing ATM
  convert_to_component::outer_wrap(match_arms, true, true).into()

  // convert_to_component::convert_to_component(vec![rsx_item], false).into()
}
