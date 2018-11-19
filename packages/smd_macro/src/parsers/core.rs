use crate::types::TokenTreeSlice;
use nom::{
  alt,
  apply,
  call,
  delimited,
  error_position,
  map,
  named,
  tuple,
  tuple_parser,
};
use proc_macro2::{
  Spacing,
  TokenStream,
};

use super::make_smithy_tokens::make_html_tokens;
use super::util;

named!(
  match_self_closing_token <TokenTreeSlice, TokenStream>,
  map!(
    delimited!(
      apply!(util::match_punct, Some('<'), Some(Spacing::Alone), vec![]),
      tuple!(
        apply!(util::match_ident, None, false),
        many_0_custom!(super::attributes::match_attribute)
      ),
      tuple!(
        apply!(util::match_punct, Some('/'), Some(Spacing::Joint), vec![]),
        apply!(util::match_punct, Some('>'), None, vec![])
      )
    ),
    |(name, attributes)| make_html_tokens(name, attributes, vec![])
  )
);

named!(
  match_opening_tag <TokenTreeSlice, (String, Vec<(String, TokenStream)>)>,
  delimited!(
    apply!(util::match_punct, Some('<'), Some(Spacing::Alone), vec![]),
    tuple!(
      apply!(util::match_ident, None, false),
      many_0_custom!(super::attributes::match_attribute)
    ),
    apply!(util::match_punct, Some('>'), None, vec![])
  )
);

named!(
  match_closing_tag <TokenTreeSlice, String>,
  delimited!(
    tuple!(
      apply!(util::match_punct, Some('<'), Some(Spacing::Joint), vec![]),
      apply!(util::match_punct, Some('/'), Some(Spacing::Alone), vec![])
    ),
    apply!(util::match_ident, None, false),
    apply!(util::match_punct, Some('>'), Some(Spacing::Alone), vec![])
  )
);

named!(
  match_regular_token <TokenTreeSlice, TokenStream>,
  map!(
    tuple!(
      match_opening_tag,
      many_0_custom!(match_html_token),
      match_closing_tag
    ),
    |((name, attributes), children, closing_tag_name)| {
      assert_eq!(name, closing_tag_name);
      make_html_tokens(name, attributes, children)
    }
  )
);

named!(
  match_html_token <TokenTreeSlice, TokenStream>,
  alt!(
    match_self_closing_token
      | match_regular_token
  )
);

named!(
  pub match_html_component <TokenTreeSlice, TokenStream>,
  map!(
    match_html_token,
    |token| super::make_smithy_tokens::make_component(token)
  )
);
