use crate::prelude::*;
use proc_macro2::{
  Delimiter,
  Ident,
  Literal,
  Punct,
  Spacing,
};

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
  match_single_tree(move |t| match t {
    TokenTree::Group(ref g) => {
      if let Some(target_delimiter) = delimiter {
        if g.delimiter() == target_delimiter {
          Ok(((), g.stream()))
        } else {
          Err(Err::Error(((), ErrorKind::IsA)))
        }
      } else {
        Ok(((), g.stream()))
      }
    },
    _ => Err(Err::Error(((), ErrorKind::IsA))),
  })
}

pub fn match_punct(
  ch: Option<char>,
  spacing: Option<Spacing>,
) -> impl Fn(TokenStream) -> TokenStreamIResult<Punct> {
  match_single_tree(move |t| match t {
    TokenTree::Punct(ref p) => {
      let mut failed_match = false;

      if let Some(ch) = ch {
        failed_match = failed_match || p.as_char() != ch;
      }
      if let Some(spacing) = spacing {
        failed_match = failed_match || p.spacing() != spacing;
      }

      if !failed_match {
        Ok(((), p.clone()))
      } else {
        Err(Err::Error(((), ErrorKind::IsA)))
      }
    },
    _ => Err(Err::Error(((), ErrorKind::IsA))),
  })
}

pub fn match_literal(input: TokenStream) -> TokenStreamIResult<Literal> {
  match_single_tree(|t| match t {
    TokenTree::Literal(lit) => Ok(((), lit.clone())),
    _ => Err(Err::Error(((), ErrorKind::IsA))),
  })(input)
}

pub fn match_ident(input: TokenStream) -> TokenStreamIResult<Ident> {
  match_single_tree(|t| match t {
    TokenTree::Ident(ident) => Ok(((), ident.clone())),
    _ => Err(Err::Error(((), ErrorKind::IsA))),
  })(input)
}

fn match_single_tree<T>(
  f: impl Fn(&TokenTree) -> IResult<(), T>,
) -> impl Fn(TokenStream) -> TokenStreamIResult<T> {
  move |input: TokenStream| {
    let vec = input.to_tree_vec();
    match vec.split_first() {
      Some((first, rest)) => {
        let rest = rest.to_token_stream();
        match f(first) {
          Ok((_rest, matched)) => Ok((rest, matched)),
          Err(e) => match e {
            Err::Error((_rest, err)) => Err(Err::Error((rest, err))),
            Err::Failure((_rest, err)) => Err(Err::Failure((rest, err))),
            Err::Incomplete(e) => Err(Err::Incomplete(e)),
          },
        }
      },
      None => Err(Err::Error((input, ErrorKind::TakeTill1))),
    }
  }
}
