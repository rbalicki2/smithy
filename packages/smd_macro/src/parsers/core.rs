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

named!(
  match_self_closing_token <TokenTreeSlice, TokenStream>,
  map!(
    delimited!(
      apply!(util::match_punct, Some('<'), Some(Spacing::Alone), vec![]),
      apply!(util::match_ident, None, false),
      tuple!(
        apply!(util::match_punct, Some('/'), Some(Spacing::Joint), vec![]),
        apply!(util::match_punct, Some('>'), None, vec![])
      )
    ),
    |a| { quote!(#a) }
  )
);

named!(
  pub match_html_token <TokenTreeSlice, TokenStream>,
  alt!(
    match_self_closing_token
      | match_self_closing_token
  )
);
