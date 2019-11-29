mod match_statement;

use crate::prelude::*;
use quote::quote;

use crate::parsers::RsxItemOrLiteral;

type ParsedOutput = Vec<RsxItemOrLiteral>;

pub fn convert_to_component(parsed_output: ParsedOutput, should_move: bool) -> TokenStream {
  let parsed_output = dbg!(parsed_output);
  // TODO use Span::def_site() for phase_variable_name
  let phase_variable_name = quote!(phase);
  let match_statement = match_statement::get_match_statement(&parsed_output, &phase_variable_name);
  outer_wrap(match_statement, should_move, &phase_variable_name)
}

fn outer_wrap(
  match_statement: TokenStream,
  should_move: bool,
  phase_variable_name: &TokenStream,
) -> TokenStream {
  let move_keyword = if should_move { quote!(move) } else { quote!() };
  quote!({
    use ::smithy::types::Component as _;
    ::smithy::types::SmithyComponent(::std::boxed::Box::new(#move_keyword |#phase_variable_name| {
      #match_statement
    }))
  })
}
