use crate::prelude::*;

fn parse_macro_item(input: proc_macro2::TokenStream) -> TokenStreamIResult<TokenStream> {
  let (rest, group_contents) =
    crate::utils::match_group_with_delimiter(proc_macro2::Delimiter::Parenthesis)(input)?;
  Ok((rest, group_contents))
}

fn parse_literal(input: proc_macro2::TokenStream) -> TokenStreamIResult<TokenStream> {
  crate::utils::take_until(crate::utils::match_punct(Some(','), None))(input)
    .map(|(rest, parsed)| (rest, parsed.into_iter().collect()))
}

pub fn parse_items(input: proc_macro2::TokenStream) -> TokenStreamIResult<Vec<TokenStream>> {
  let (rest, parsed) = crate::utils::many_0_delimited(
    nom::branch::alt((parse_macro_item, parse_literal)),
    crate::utils::match_punct(Some(','), None),
  )(input)?;
  let (rest, _) = crate::utils::ensure_consumed(rest)?;
  Ok((rest, parsed))
}
