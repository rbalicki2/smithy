use crate::{
  prelude::*,
  utils::{
    ensure_consumed,
    many_0_delimited,
    match_group_with_delimiter,
    match_punct,
  },
};
use nom::{
  branch::alt,
  combinator::opt,
  sequence::tuple,
};
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
  attribute_instructions: Vec<AttributeInstruction>,
}

#[derive(Debug, Clone)]
pub enum AttributeInstruction {
  Explode(TokenStream),
  Assign(TokenStreamOrString, TokenStream),
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
  parse_token_stream_or_string(input)
}

fn parse_token_stream_or_string(input: TokenStream) -> TokenStreamIResult<TokenStreamOrString> {
  alt((
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

fn match_attribute_assignment(input: TokenStream) -> TokenStreamIResult<AttributeInstruction> {
  // TODO here
  let (rest, items) = tuple((
    parse_token_stream_or_string,
    match_punct(Some(':'), None),
    take_until_comma,
  ))(input)?;
  Ok((
    rest,
    AttributeInstruction::Assign(items.0, items.2.iter().map(|x| x.clone()).collect()),
  ))
}

fn match_attribute_explosion(input: TokenStream) -> TokenStreamIResult<AttributeInstruction> {
  // TODO work here!
  unimplemented!()
}

fn parse_attribute_group_contents(
  input: TokenStream,
) -> TokenStreamIResult<Vec<AttributeInstruction>> {
  many_0_delimited(
    alt((match_attribute_assignment, match_attribute_explosion)),
    match_punct(Some(','), None),
  )(input)
}

fn parse_attribute_group(input: TokenStream) -> TokenStreamIResult<Vec<AttributeInstruction>> {
  let (rest, contents) = match_group_with_delimiter(Delimiter::Brace)(input)?;
  let (inner_rest, attribute_vec) = parse_attribute_group_contents(contents)?;
  ensure_consumed(inner_rest)?;

  Ok((rest, attribute_vec))
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
  let (rest, (node_type, attribute_instructions, event_handlers, children)) = tuple((
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
      attribute_instructions: attribute_instructions.unwrap_or(vec![]),
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

fn take_until_comma(input: TokenStream) -> TokenStreamIResult<Vec<TokenTree>> {
  let (rest, parsed) = crate::utils::take_until(match_punct(Some(','), None))(input)?;
  Ok((rest, parsed))
}

fn parse_literal(input: TokenStream) -> TokenStreamIResult<RsxItemOrLiteral> {
  take_until_comma(input).map(|(rest, parsed)| {
    (
      rest,
      RsxItemOrLiteral::Literal(parsed.into_iter().collect()),
    )
  })
}

pub fn parse_items(input: TokenStream) -> TokenStreamIResult<Vec<RsxItemOrLiteral>> {
  let (rest, parsed) = many_0_delimited(
    alt((parse_macro_item, parse_literal)),
    match_punct(Some(','), None),
  )(input)?;
  Ok((rest, parsed))
}
