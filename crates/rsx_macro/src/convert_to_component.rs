use crate::prelude::*;
use quote::quote;

use crate::parsers::{
  AttributeInstruction,
  RsxItemOrLiteral,
};
type ParsedOutput = Vec<RsxItemOrLiteral>;

pub fn convert_to_component(parsed_output: ParsedOutput, should_move: bool) -> TokenStream {
  let parsed_output = dbg!(parsed_output);
  // TODO figure out how to get this to be an opaque variable name
  let phase_variable_name = quote!(phase);
  let match_statement = get_match_statement(&parsed_output, &phase_variable_name);
  outer_wrap(match_statement, should_move, &phase_variable_name)
}

/// N.B. must return a TokenStream that evaluates to a ::smithy::types::PhaseResult
fn get_match_statement(
  parsed_output: &ParsedOutput,
  phase_variable_name: &TokenStream,
) -> TokenStream {
  let rendering_result = get_rendering_result(&parsed_output);
  quote!(
    match #phase_variable_name {
      ::smithy::types::Phase::Rendering => #rendering_result,
      _ => ::smithy::types::PhaseResult::UiEventHandling(false)
    }
  )
}

fn get_attributes_token_stream(attribute_instructions: &Vec<AttributeInstruction>) -> TokenStream {
  // Special case: If we have no attribute instructions, return an empty hashmap
  // directly.
  if attribute_instructions.len() == 0 {
    return quote!(::std::collections::HashMap::new());
  }

  // Henceforth, assume we have a map named map
  let inner =
    attribute_instructions
      .iter()
      .fold(quote!(), |attribute_instructions_so_far, instruction| {
        let result = match instruction {
          AttributeInstruction::Assign(lhs, rhs) => {
            let lhs = lhs.to_token_stream();
            quote!(map.insert(#lhs.to_string(), #rhs.to_string()))
          },
          AttributeInstruction::Explode(explosion) => quote!(
            for (key, val) in #explosion.iter() {
              // TODO is there a "copy into" method?
              map.insert(key.to_string(), val.to_string());
            }
          ),
        };
        quote!(#attribute_instructions_so_far #result;)
      });

  quote!({
    let mut map = ::std::collections::HashMap::new();
    #inner
    map
  })
}

fn get_rendering_result_vec(parsed_output: &ParsedOutput) -> TokenStream {
  let result = parsed_output
    .iter()
    .fold(quote!(), |phase_result_so_far, rsx_item_or_literal| {
      let current = match rsx_item_or_literal {
        RsxItemOrLiteral::Literal(token_stream) => quote!(#token_stream.render()),
        RsxItemOrLiteral::Node(node_construction_instructions) => {
          let node_type = node_construction_instructions.node_type.to_token_stream();
          let children = get_rendering_result_vec(&node_construction_instructions.children);
          let attributes =
            get_attributes_token_stream(&node_construction_instructions.attribute_instructions);
          quote!(
            ::smithy::types::Node::Dom(::smithy::types::HtmlToken {
              node_type: #node_type.to_string(),
              children: #children,
              attributes: #attributes,
            })
          )
        },
      };
      quote!(#phase_result_so_far #current,)
    });
  quote!(vec![#result])
}

fn get_rendering_result(parsed_output: &ParsedOutput) -> TokenStream {
  let result = get_rendering_result_vec(parsed_output);
  quote!(::smithy::types::PhaseResult::Rendering(
    ::smithy::types::Node::Vec(#result)
  ))
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
