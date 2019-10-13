use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum RsxItem {
  Literal(TokenStream),
  MacroItem(TokenStream),
}

fn parse_macro_item(input: proc_macro2::TokenStream) -> TokenStreamIResult<RsxItem> {
  let (rest, group_contents) =
    crate::utils::match_group_with_delimiter(proc_macro2::Delimiter::Parenthesis)(input)?;
  Ok((rest, RsxItem::MacroItem(group_contents)))
}

fn parse_literal(input: proc_macro2::TokenStream) -> TokenStreamIResult<RsxItem> {
  crate::utils::take_until(crate::utils::match_punct(Some(','), None))(input)
    .map(|(rest, parsed)| (rest, RsxItem::Literal(parsed.into_iter().collect())))
}

pub fn parse_items(input: proc_macro2::TokenStream) -> TokenStreamIResult<Vec<RsxItem>> {
  let (rest, parsed) = crate::utils::many_0_delimited(
    nom::branch::alt((parse_macro_item, parse_literal)),
    crate::utils::match_punct(Some(','), None),
  )(input)?;
  Ok((rest, parsed))
}
