use crate::{
  parsers::{
    parse_macro_item,
    take_until_comma,
    RsxItemOrLiteral,
  },
  prelude::*,
  utils::{
    ensure_consumed,
    many_0_delimited,
    match_group_with_delimiter,
    match_ident,
    match_punct,
  },
};
use nom::{
  branch::alt,
  combinator::opt,
  sequence::tuple,
};
use quote::quote;

pub fn parse_with_ref(input: TokenStream) -> TokenStreamIResult<(TokenStream, RsxItemOrLiteral)> {
  let (rest, parsed) = tuple((
    // N.B. take_until_comma does not match the comma
    take_until_comma,
    match_punct(Some(','), None),
    parse_macro_item,
  ))(input)?;

  Ok((rest, (parsed.0.into_iter().collect(), parsed.2)))
}
