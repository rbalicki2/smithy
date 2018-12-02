use crate::types::TokenTreeSlice;
use nom::{
  call,
  map,
  named,
};
use proc_macro2::TokenStream;
use quote::quote;

#[macro_use]
mod many_custom;
mod attributes;
mod core;
mod event_names;
mod make_smithy_tokens;
mod util;

named!(
  pub match_smd <TokenTreeSlice, TokenStream>,
  map!(
    // TODO
    // * figure out why many_0_custom does not consume the remaining vector
    // * consider using many_0_custom here
    // many_1_custom!(self::core::match_html_component),
    many_1_custom!(self::core::match_node),
    |vec| {
      let vec = vec.into_iter().enumerate().map(|(i, mut val)| {
        let component = val.0;
        let mut event_handling_infos = val.1;
        for mut event_handling_info in event_handling_infos.iter_mut() {
          event_handling_info.reversed_path.push(i);
        }
        make_smithy_tokens::make_component(component, event_handling_infos)
      })
      .collect();
      let as_token = util::reduce_vec_to_tokens(&vec);
      let quoted = quote!(#as_token);
      quoted
    }
  )
);
