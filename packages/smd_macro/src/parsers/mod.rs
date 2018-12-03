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
    self::core::match_html_component,
    |as_token| {
      // let as_token = util::reduce_vec_to_tokens(&vec);
      // let quoted = quote!(#as_token);
      // quoted
      as_token
    }
  )
);
