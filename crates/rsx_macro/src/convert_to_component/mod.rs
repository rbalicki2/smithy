pub mod match_statement;

use crate::prelude::*;
use quote::quote;

use crate::parsers::RsxItemOrLiteral;

type ParsedOutput = Vec<RsxItemOrLiteral>;

pub fn convert_to_component(parsed_output: ParsedOutput, should_move: bool) -> TokenStream {
  // TODO use Span::def_site() for phase_variable_name
  // as well as for ::smithy
  let match_arms = match_statement::get_match_arms(&parsed_output);
  outer_wrap(match_arms, should_move, true)
}

pub fn outer_wrap(
  match_arms: TokenStream,
  should_move: bool,
  include_catch_all_case: bool,
) -> TokenStream {
  let phase_variable_name = quote!(phase);
  let move_keyword = if should_move { quote!(move) } else { quote!() };
  let catch_all_case = if include_catch_all_case {
    // TODO PhaseResult::Ignored or the like
    quote!(_ => ::smithy::types::PhaseResult::UiEventHandling(false))
  } else {
    quote!()
  };
  quote!({
    use ::smithy::types::Component as _;
    ::smithy::types::SmithyComponent(::std::boxed::Box::new(#move_keyword |#phase_variable_name| {
      match #phase_variable_name {
        #match_arms
        #catch_all_case
      }
    }))
  })
}
