use crate::types::TokenTreeSlice;
use nom::{
  alt,
  apply,
  call,
  delimited,
  error_position,
  map,
  named,
  tuple,
  tuple_parser,
};
use proc_macro2::{
  Spacing,
  TokenStream,
};
use quote::quote;

use super::util;

// TODO handle children and event handling attributes
fn make_smithy_tokens(
  name: String,
  attributes: Vec<(String, TokenStream)>,
  children: Vec<TokenStream>,
) -> TokenStream {
  let attribute_initialization = if attributes.len() > 0 {
    let attribute_insertion = attributes.into_iter().fold(quote!(), |accum, (key, val)| {
      quote!(
        #accum
        map.insert(#key.into(), #val.into());
      )
    });
    quote!({
      let mut map = std::collections::HashMap::new();
      #attribute_insertion
      map
    })
  } else {
    quote!(std::collections::HashMap::new())
  };
  let child_initialization = quote!(vec![]);

  quote!({
    let component: smithy_types::Component = smithy_types::Component(Box::new(move |phase| {
      match phase {
        smithy_types::Phase::Rendering => {
          smithy_types::PhaseResult::Rendering(smithy_types::HtmlToken {
            node_type: #name.into(),
            attributes: #attribute_initialization,
            children: #child_initialization,
          })
        },
        smithy_types::Phase::EventHandling(_) =>
          smithy_types::PhaseResult::EventHandling(false)
      }
    }));
    component
  })
}

named!(
  match_self_closing_token <TokenTreeSlice, TokenStream>,
  map!(
    delimited!(
      apply!(util::match_punct, Some('<'), Some(Spacing::Alone), vec![]),
      tuple!(
        apply!(util::match_ident, None, false),
        many_0_custom!(super::attributes::match_attribute)
      ),
      tuple!(
        apply!(util::match_punct, Some('/'), Some(Spacing::Joint), vec![]),
        apply!(util::match_punct, Some('>'), None, vec![])
      )
    ),
    |(name, attributes)| make_smithy_tokens(name, attributes, vec![])
  )
);

named!(
  pub match_html_token <TokenTreeSlice, TokenStream>,
  alt!(
    match_self_closing_token
      | match_self_closing_token
  )
);
