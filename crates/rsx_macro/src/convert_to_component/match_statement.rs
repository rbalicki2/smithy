use crate::prelude::*;
use quote::quote;

use crate::{
  parsers::{
    AttributeInstruction,
    RsxItemOrLiteral,
  },
  utils::event_names,
};
use proc_macro2::{
  Ident,
  Span,
};

type ParsedOutput = Vec<RsxItemOrLiteral>;

/// N.B. must return a TokenStream that evaluates to a ::smithy::types::PhaseResult
pub fn get_match_statement(
  parsed_output: &ParsedOutput,
  phase_variable_name: &TokenStream,
) -> TokenStream {
  let rendering_result = get_rendering_result(&parsed_output);
  let ui_event_handling_result = get_ui_event_handling_result(&parsed_output);
  quote!(
    match #phase_variable_name {
      ::smithy::types::Phase::Rendering => #rendering_result,
      ::smithy::types::Phase::UiEventHandling(ui_event_handling) => {
        #ui_event_handling_result
      },
      // this is for post-render, window event handling and ref assignment phases
      // TODO PhaseResult::Ignored
      _ => ::smithy::types::PhaseResult::UiEventHandling(false)
    }
  )
}

fn get_ui_event_handling_result(parsed_output: &ParsedOutput) -> TokenStream {
  let result = get_ui_event_handling_result_inner(parsed_output, &vec![]);

  quote!(match ui_event_handling {
    #result
    _ => ::smithy::types::PhaseResult::UiEventHandling(false),
  })
}

fn join(path: &Vec<usize>, next: usize) -> Vec<usize> {
  let mut path = path.clone();
  path.push(next);
  path
}

fn quotify_path(path: &Vec<usize>, include_rest_param: bool) -> TokenStream {
  let inner = path.iter().fold(quote! {}, |accum, path_item| {
    quote! { #accum #path_item, }
  });
  let additional_dot_dot = if include_rest_param {
    quote! { rest @ .. }
  } else {
    quote! {}
  };
  quote! {
    [ #inner #additional_dot_dot ]
  }
}

fn get_ui_event_handling_result_inner(
  parsed_output: &ParsedOutput,
  path_so_far: &Vec<usize>,
) -> TokenStream {
  parsed_output
    .iter()
    .enumerate()
    .fold(quote!(), |match_arms, (i, current)| {
      let joined_path = join(path_so_far, i);
      let result = match current {
        RsxItemOrLiteral::Literal(token_stream) => {
          let path = quotify_path(&joined_path, true);
          quote!(
            (event, #path) => smithy::types::PhaseResult::UiEventHandling(
              #token_stream.handle_ui_event(event, rest)
            ),
          )
        },
        RsxItemOrLiteral::Node(node) => {
          let current_match_arms = node.event_handler_instructions.iter().fold(
            quote!(),
            |match_arms, (event_name, callback)| {
              // TODO what is should_include_rest? Should it be include_rest_param?
              let (enum_event_name, _should_include_rest) =
                event_names::UI_EVENT_NAMES.get(event_name).expect(&format!(
                  "rsx compilation error: event handler not recognized: {}",
                  event_name
                ));
              let event = Ident::new(&enum_event_name, Span::call_site());
              let path = quotify_path(&joined_path, false);

              // TODO use Span::def_site() for js_event, ui_event_handling, etc.
              quote!(
                #match_arms
                (::smithy::types::UiEvent::#event(js_event), #path) => {
                  // TODO explore why the following doesn't work:
                  // let cb = |_| {};
                  // rsx![(div {} { on_click: cb })]
                  // N.B. annotating the parameter type to cb fixes it.
                  (#callback)(js_event);
                  ::smithy::types::PhaseResult::UiEventHandling(true)
                },
              )
            },
          );
          let children_match_arms =
            get_ui_event_handling_result_inner(&node.children, &joined_path);
          quote!(
            #current_match_arms
            #children_match_arms
          )
        },
      };
      quote!(#match_arms #result)
    })
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
