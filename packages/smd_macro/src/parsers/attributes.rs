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
  Spacing,
  TokenStream,
};
use std::collections::HashMap;

named!(
  pub match_attribute <TokenTreeSlice, (String, bool)>,
  map!(
    tuple!(
      apply!(util::match_ident, None, false),
      apply!(util::match_punct, Some('='), None, vec![]),
      alt!(
        map!(apply!(util::match_group, Some(Delimiter::Brace)), super::util::enquote)
          | map!(
            call!(util::match_literal_as_string),
            super::util::enquote
          )
      )
    ),
    |val| {
      (val.0, true)
    }
  )
);
