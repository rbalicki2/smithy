use crate::types::*;
use proc_macro2::{
  Delimiter,
  Group,
  Literal,
  Spacing,
  TokenStream,
};
use quote::quote;
use quote::ToTokens;
use std::iter;

pub type TtsIResult<'a, T> = nom::IResult<TokenTreeSlice<'a>, T>;
pub type StringResult<'a> = TtsIResult<'a, String>;

pub fn match_punct(
  input: TokenTreeSlice,
  c_opt: Option<char>,
  spacing_opt: Option<Spacing>,
  excluded_chars: Vec<char>,
) -> StringResult {
  let get_err = || {
    Err(nom::Err::Error(nom::error_position!(
      input,
      nom::ErrorKind::Custom(42)
    )))
  };
  let filler_spaces = get_filler_spaces(input);

  match input.split_first() {
    Some((first, rest)) => match first {
      TokenTree::Punct(ref punct) => {
        let wrong_char = c_opt.map(|c| punct.as_char() != c).unwrap_or(false);
        let wrong_spacing = spacing_opt
          .map(|spacing| punct.spacing() != spacing)
          .unwrap_or(false);
        let contains_excluded_char = excluded_chars.contains(&punct.as_char());

        if wrong_char || wrong_spacing || contains_excluded_char {
          get_err()
        } else {
          Ok((rest, format!("{}{}", punct.as_char(), filler_spaces)))
        }
      }
      _ => get_err(),
    },
    None => get_err(),
  }
}

pub fn match_ident(
  input: TokenTreeSlice,
  sym_opt: Option<String>,
  include_filler: bool,
) -> StringResult {
  let get_err = || {
    Err(nom::Err::Error(nom::error_position!(
      input,
      nom::ErrorKind::Custom(42)
    )))
  };

  let filler_spaces = if include_filler {
    get_filler_spaces(input)
  } else {
    "".into()
  };
  match input.split_first() {
    Some((first, rest)) => match first {
      TokenTree::Ident(ref ident) => {
        let get_success = || Ok((rest, format!("{}{}", ident, filler_spaces)));
        match sym_opt {
          Some(s) => {
            if s == format!("{}", ident) {
              get_success()
            } else {
              get_err()
            }
          }
          None => get_success(),
        }
      }
      _ => get_err(),
    },
    None => get_err(),
  }
}

pub type GroupResult<'a> = TtsIResult<'a, Group>;

pub fn match_group(input: TokenTreeSlice, delimiter_opt: Option<Delimiter>) -> GroupResult {
  let get_err = || {
    Err(nom::Err::Error(nom::error_position!(
      input,
      nom::ErrorKind::Custom(42)
    )))
  };

  match input.split_first() {
    Some((first, rest)) => match first {
      TokenTree::Group(ref group) => {
        let get_success = || Ok((rest, group.clone()));
        match delimiter_opt {
          Some(delimiter) => {
            if group.delimiter() == delimiter {
              get_success()
            } else {
              get_err()
            }
          }
          None => get_success(),
        }
      }
      _ => get_err(),
    },
    None => get_err(),
  }
}

pub fn match_literal(input: TokenTreeSlice) -> TtsIResult<Literal> {
  let get_err = || {
    Err(nom::Err::Error(nom::error_position!(
      input,
      nom::ErrorKind::Custom(42)
    )))
  };

  match input.split_first() {
    Some((first, rest)) => match first {
      TokenTree::Literal(literal) => Ok((rest, literal.clone())),
      _ => get_err(),
    },
    None => get_err(),
  }
}

pub fn match_literal_as_string(input: TokenTreeSlice) -> TtsIResult<String> {
  let filler_spaces = get_filler_spaces(input);
  match_literal(input).map(|(rest, lit)| (rest, format!("{}{}", lit.to_string(), filler_spaces)))
}

pub fn get_filler_spaces(input: TokenTreeSlice) -> String {
  let first_opt = input.get(0).map(|i| i.span().end());
  let second_opt = input.get(1).map(|i| i.span().start());
  match (first_opt, second_opt) {
    (Some(first), Some(second)) => {
      if first.line != second.line {
        "".into()
      } else {
        iter::repeat(" ")
          .take(second.column - first.column)
          .collect::<String>()
      }
    }
    _ => "".into(),
  }
}

pub fn reduce_vec_to_tokens(v: &Vec<proc_macro2::TokenStream>) -> proc_macro2::TokenStream {
  let vec_contents = v
    .iter()
    .fold(quote!(), |existing, token| quote!(#existing #token,));
  quote!(vec![
    #vec_contents
  ])
}

pub fn enquote<T: ToTokens>(t: T) -> TokenStream {
  quote!(#t)
}
