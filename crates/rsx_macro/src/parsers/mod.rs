use crate::{
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
use proc_macro2::{
  Delimiter,
  Spacing,
};

#[derive(Debug, Clone)]
pub enum RsxItemOrLiteral {
  Literal(TokenStream),
  Node(NodeConstructionInstructions),
}

#[derive(Debug, Clone)]
pub struct NodeConstructionInstructions {
  pub node_type: TokenStreamOrString,
  pub children: Vec<RsxItemOrLiteral>,
  pub attribute_instructions: Vec<AttributeInstruction>,
  pub event_handler_instructions: Vec<(String, TokenStream)>,
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

impl TokenStreamOrString {
  pub fn to_token_stream(&self) -> TokenStream {
    match self {
      TokenStreamOrString::String(s) => quote::quote!(#s),
      // TODO this seems fishy that we need to clone
      TokenStreamOrString::TokenStream(token_stream) => token_stream.clone(),
    }
  }
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
    // e.g. data-name
    |input| {
      match_ident(input).map(|(rest, ident)| (rest, TokenStreamOrString::String(ident.to_string())))
    },
    |input| {
      match_group_with_delimiter(Delimiter::Brace)(input)
        .map(|(rest, content)| (rest, TokenStreamOrString::TokenStream(content)))
    },
  ))(input)
}

fn match_attribute_assignment(input: TokenStream) -> TokenStreamIResult<AttributeInstruction> {
  let (rest, items) = tuple((
    parse_token_stream_or_string,
    match_punct(Some(':'), None),
    take_until_comma,
  ))(input)?;
  Ok((
    rest,
    AttributeInstruction::Assign(items.0, items.2.into_iter().collect()),
  ))
}

fn match_solo_attribute(input: TokenStream) -> TokenStreamIResult<AttributeInstruction> {
  let (rest, solo_attribute) = parse_token_stream_or_string(input)?;
  Ok((
    rest,
    AttributeInstruction::Assign(solo_attribute, quote::quote!(true)),
  ))
}

fn match_attribute_explosion(input: TokenStream) -> TokenStreamIResult<AttributeInstruction> {
  let (rest, parsed) = tuple((
    match_punct(Some('.'), Some(Spacing::Joint)),
    match_punct(Some('.'), Some(Spacing::Joint)),
    match_punct(Some('.'), Some(Spacing::Alone)),
    take_until_comma,
  ))(input)?;

  Ok((
    rest,
    AttributeInstruction::Explode(parsed.3.into_iter().collect()),
  ))
}

fn parse_attribute_group_contents(
  input: TokenStream,
) -> TokenStreamIResult<Vec<AttributeInstruction>> {
  many_0_delimited(
    alt((
      match_attribute_assignment,
      match_solo_attribute,
      match_attribute_explosion,
    )),
    match_punct(Some(','), None),
  )(input)
}

fn parse_attribute_group(input: TokenStream) -> TokenStreamIResult<Vec<AttributeInstruction>> {
  let (rest, contents) = match_group_with_delimiter(Delimiter::Brace)(input)?;
  let (inner_rest, attribute_vec) = parse_attribute_group_contents(contents)?;
  crate::utils::ensure2(inner_rest, rest, attribute_vec)
}

fn match_event_handler_assignment(input: TokenStream) -> TokenStreamIResult<(String, TokenStream)> {
  let (rest, val) = tuple((match_ident, match_punct(Some(':'), None), take_until_comma))(input)?;
  Ok((rest, (val.0.to_string(), val.2.into_iter().collect())))
}

fn parse_event_handler_group_contents(
  input: TokenStream,
) -> TokenStreamIResult<Vec<(String, TokenStream)>> {
  many_0_delimited(match_event_handler_assignment, match_punct(Some(','), None))(input)
}

fn parse_event_handler_group(input: TokenStream) -> TokenStreamIResult<Vec<(String, TokenStream)>> {
  let (rest, contents) = match_group_with_delimiter(Delimiter::Brace)(input)?;
  let (inner_rest, event_handler_assignment_instruction_vec) =
    parse_event_handler_group_contents(contents)?;
  crate::utils::ensure2(inner_rest, rest, event_handler_assignment_instruction_vec)
}

fn parse_children_group(input: TokenStream) -> TokenStreamIResult<Vec<RsxItemOrLiteral>> {
  let (rest, group_contents) = match_group_with_delimiter(Delimiter::Bracket)(input)?;
  let (inner_rest, vec) = parse_items(group_contents)?;
  ensure_consumed(inner_rest)?;
  Ok((rest, vec))
}

fn parse_macro_item_contents(
  input: TokenStream,
) -> TokenStreamIResult<NodeConstructionInstructions> {
  let (rest, (node_type, attribute_instructions, event_handlers, children)) = tuple((
    parse_node_type,
    opt(parse_attribute_group),
    opt(parse_event_handler_group),
    opt(parse_children_group),
  ))(input)?;
  Ok((
    rest,
    NodeConstructionInstructions {
      node_type,
      attribute_instructions: attribute_instructions.unwrap_or(vec![]),
      event_handler_instructions: event_handlers.unwrap_or(vec![]),
      children: children.unwrap_or(vec![]),
    },
  ))
}

fn parse_macro_item(input: TokenStream) -> TokenStreamIResult<RsxItemOrLiteral> {
  let (rest, group_contents) = match_group_with_delimiter(Delimiter::Parenthesis)(input)?;
  let (inner_rest, macro_item) = parse_macro_item_contents(group_contents)?;
  crate::utils::ensure2(inner_rest, rest, RsxItemOrLiteral::Node(macro_item))
}

fn take_until_comma(input: TokenStream) -> TokenStreamIResult<Vec<TokenTree>> {
  let (rest, parsed) = crate::utils::take_until(match_punct(Some(','), None))(input)?;
  Ok((rest, parsed))
}

fn is_group(item: &TokenTree, delimiter: Option<Delimiter>) -> bool {
  match item {
    TokenTree::Group(g) => delimiter.map(|d| g.delimiter() == d).unwrap_or(true),
    _ => false,
  }
}

fn parse_literal(input: TokenStream) -> TokenStreamIResult<RsxItemOrLiteral> {
  let (rest, val) = take_until_comma(input.clone())?;
  if val.len() == 1 && is_group(val.get(0).unwrap(), Some(Delimiter::Parenthesis)) {
    Err(Err::Error((input, ErrorKind::TakeTill1)))
  } else {
    Ok((rest, RsxItemOrLiteral::Literal(val.into_iter().collect())))
  }
}

pub fn parse_items(input: TokenStream) -> TokenStreamIResult<Vec<RsxItemOrLiteral>> {
  let (rest, parsed) = many_0_delimited(
    alt((parse_macro_item, parse_literal)),
    match_punct(Some(','), None),
  )(input)?;
  Ok((rest, parsed))
}
