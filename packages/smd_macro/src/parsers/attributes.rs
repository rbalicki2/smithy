use super::util;
use crate::types::TokenTreeSlice;
use nom::{
  alt,
  apply,
  call,
  error_position,
  map,
  named,
  tuple,
  tuple_parser,
};
use proc_macro2::{
  Delimiter,
  TokenStream,
};

// TODO handle event-handling attributes
named!(
  pub match_attribute <TokenTreeSlice, (String, TokenStream)>,
  alt!(
    map!(
      tuple!(
        apply!(util::match_ident, None, false),
        apply!(util::match_punct, Some('='), None, vec![]),
        alt!(
          map!(apply!(util::match_group, Some(Delimiter::Brace)), super::util::enquote)
            | map!(
              call!(util::match_literal),
              super::util::enquote
            )
        )
      ),
      |val| (val.0, val.2)
    )
    | map!(apply!(util::match_ident, None, false), |s| (s, quote::quote!("")))
  )
);
