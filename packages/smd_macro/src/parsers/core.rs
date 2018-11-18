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
      tuple!(
        apply!(util::match_ident, None, false),
        many_0_custom!(super::attributes::match_attribute)
      ),
      tuple!(
        apply!(util::match_punct, Some('/'), Some(Spacing::Joint), vec![]),
        apply!(util::match_punct, Some('>'), None, vec![])
      )
    ),
    |a| { println!("{:?}", a); let name = a.0; quote!(#name) }
  )
);

named!(
  pub match_html_token <TokenTreeSlice, TokenStream>,
  alt!(
    match_self_closing_token
      | match_self_closing_token
  )
);
