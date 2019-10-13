use crate::prelude::*;
use proc_macro2::Delimiter;

pub fn match_group(input: TokenStream) -> TokenStreamIResult<TokenStream> {
  match_group_with_delimiter_opt(None)(input)
}

pub fn match_group_with_delimiter(
  delim: Delimiter,
) -> impl Fn(TokenStream) -> TokenStreamIResult<TokenStream> {
  match_group_with_delimiter_opt(Some(delim))
}

fn match_group_with_delimiter_opt(
  delimiter: Option<Delimiter>,
) -> impl Fn(TokenStream) -> TokenStreamIResult<TokenStream> {
  move |input: TokenStream| {
    let vec = input.to_tree_vec();
    match vec.split_first() {
      Some((first, rest)) => match first {
        TokenTree::Group(ref g) => {
          if let Some(target_delimiter) = delimiter {
            if g.delimiter() == target_delimiter {
              Ok((rest.to_token_stream(), g.stream()))
            } else {
              Err(Err::Error((input, ErrorKind::TakeTill1)))
            }
          } else {
            Ok((rest.to_token_stream(), g.stream()))
          }
        },
        _ => Err(Err::Error((input, ErrorKind::TakeTill1))),
      },
      // None => Err(Err::Incomplete(Needed::Size(1))),
      None => Err(Err::Error((input, ErrorKind::TakeTill1))),
    }
  }
}
