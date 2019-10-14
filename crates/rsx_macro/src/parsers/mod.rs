use crate::{
  prelude::*,
  utils::{
    ensure_consumed,
    match_group_with_delimiter,
  },
};
use nom::combinator::opt;

use proc_macro2::Delimiter;

#[derive(Debug, Clone)]
pub enum RsxItemOrLiteral {
  Literal(TokenStream),
  Node(Node),
}

#[derive(Debug, Clone)]
pub struct Node {
  node_type: TokenStreamOrString,
  children: Vec<RsxItemOrLiteral>,
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
    // TODO don't just match a single ident. Instead, match adjacent idents and puncts.
    |input| {
      crate::utils::match_ident(input)
        .map(|(rest, ident)| (rest, TokenStreamOrString::String(ident.to_string())))
    },
    |input| {
      match_group_with_delimiter(Delimiter::Brace)(input)
        .map(|(rest, content)| (rest, TokenStreamOrString::TokenStream(content)))
    },
  ))(input)
}

fn parse_attribute_group(input: TokenStream) -> TokenStreamIResult<TokenStream> {
  match_group_with_delimiter(Delimiter::Brace)(input)
}

fn parse_event_handler_group(input: TokenStream) -> TokenStreamIResult<TokenStream> {
  match_group_with_delimiter(Delimiter::Brace)(input)
}

fn parse_children_group(input: TokenStream) -> TokenStreamIResult<Vec<RsxItemOrLiteral>> {
  let (rest, group_contents) = match_group_with_delimiter(Delimiter::Bracket)(input)?;
  let (inner_rest, vec) = parse_items(group_contents)?;
  ensure_consumed(inner_rest)?;
  Ok((rest, vec))
}

fn parse_macro_item_contents(input: TokenStream) -> TokenStreamIResult<Node> {
  let (rest, (node_type, attributes, event_handlers, children)) = nom::sequence::tuple((
    parse_node_type,
    opt(parse_attribute_group),
    opt(parse_event_handler_group),
    opt(parse_children_group),
  ))(input)?;
  Ok((
    rest,
    Node {
      node_type,
      children: children.unwrap_or(vec![]),
    },
  ))
}

fn parse_macro_item(input: TokenStream) -> TokenStreamIResult<RsxItemOrLiteral> {
  let (rest, group_contents) =
    match_group_with_delimiter(proc_macro2::Delimiter::Parenthesis)(input)?;
  let (inner_rest, macro_item) = parse_macro_item_contents(group_contents)?;
  ensure_consumed(inner_rest)?;
  Ok((rest, RsxItemOrLiteral::Node(macro_item)))
}

fn parse_literal(input: proc_macro2::TokenStream) -> TokenStreamIResult<RsxItemOrLiteral> {
  crate::utils::take_until(crate::utils::match_punct(Some(','), None))(input).map(
    |(rest, parsed)| {
      (
        rest,
        RsxItemOrLiteral::Literal(parsed.into_iter().collect()),
      )
    },
  )
}

pub fn parse_items(input: proc_macro2::TokenStream) -> TokenStreamIResult<Vec<RsxItemOrLiteral>> {
  let (rest, parsed) = crate::utils::many_0_delimited(
    nom::branch::alt((parse_macro_item, parse_literal)),
    crate::utils::match_punct(Some(','), None),
  )(input)?;
  Ok((rest, parsed))
}
