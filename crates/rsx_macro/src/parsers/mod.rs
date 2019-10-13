use crate::prelude::*;

use proc_macro2::Delimiter;

#[derive(Debug, Clone)]
pub enum RsxItem {
  Literal(TokenStream),
  MacroItem(MacroItem),
}

#[derive(Debug, Clone)]
pub struct MacroItem {
  node_type: TokenStreamOrString,
}

#[derive(Debug, Clone)]
pub enum TokenStreamOrString {
  String(String),
  TokenStream(TokenStream),
}

// match:
// - name or interpolation
// - optional attributes
// - optional event handlers
// - optional square bracket of children

fn parse_node_type(input: TokenStream) -> TokenStreamIResult<TokenStreamOrString> {
  nom::branch::alt((
    |input| {
      crate::utils::match_ident(input)
        .map(|(rest, ident)| (rest, TokenStreamOrString::String(ident.to_string())))
    },
    |input| {
      crate::utils::match_group_with_delimiter(Delimiter::Brace)(input)
        .map(|(rest, content)| (rest, TokenStreamOrString::TokenStream(content)))
    },
  ))(input)
}

fn parse_macro_item_contents(input: TokenStream) -> TokenStreamIResult<MacroItem> {
  let (rest, result) = nom::sequence::tuple((parse_node_type, parse_node_type))(input)?;
  Ok((
    rest,
    MacroItem {
      node_type: result.0,
    },
  ))
}

fn parse_macro_item(input: TokenStream) -> TokenStreamIResult<RsxItem> {
  let (rest, group_contents) =
    crate::utils::match_group_with_delimiter(proc_macro2::Delimiter::Parenthesis)(input)?;
  let (inner_rest, macro_item) = parse_macro_item_contents(group_contents)?;
  crate::utils::ensure_consumed(inner_rest)?;
  Ok((rest, RsxItem::MacroItem(macro_item)))
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
