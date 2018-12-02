use super::util;
use crate::types::{
  AttributeOrEventHandler,
  TokenTreeSlice,
};
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

named!(
  pub match_attribute <TokenTreeSlice, AttributeOrEventHandler>,
  alt!(
    alt!(
      map!(
        tuple!(
          apply!(util::match_ident, None, false),
          apply!(util::match_punct, Some('='), None, vec![]),
          map!(apply!(util::match_group, Some(Delimiter::Brace)), super::util::enquote)
        ),
        |val| AttributeOrEventHandler::Attribute((val.0, val.2))
      )
        | map!(
          tuple!(
            apply!(util::match_ident, None, false),
            apply!(util::match_punct, Some('='), None, vec![]),
            map!(call!(util::match_literal), super::util::enquote)
          ),
          |val| AttributeOrEventHandler::Attribute((val.0, val.2))
        )
        | map!(
          apply!(util::match_ident, None, false),
          |attr_name| AttributeOrEventHandler::Attribute((attr_name, quote::quote!("")))
        )
    )
  )
);
