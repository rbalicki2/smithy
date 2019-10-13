use crate::types::*;
use nom::{
  error::ErrorKind,
  Err,
  IResult,
};
use proc_macro2::{
  Delimiter,
  TokenStream,
  TokenTree,
};

pub fn parse_group_with_delimiter(
  input: TokenStream,
  delimiter: Option<Delimiter>,
) -> TokenStreamIResult<TokenStream> {
  let vec = stream_to_tree_vec(&input);
  match vec.split_first() {
    Some((first, rest)) => match first {
      TokenTree::Group(ref g) => {
        if let Some(target_delimiter) = delimiter {
          if g.delimiter() == target_delimiter {
            Ok((slice_to_stream(rest), g.stream()))
          } else {
            Err(Err::Error((input, ErrorKind::TakeTill1)))
          }
        } else {
          Ok((slice_to_stream(rest), g.stream()))
        }
      },
      _ => Err(Err::Error((input, ErrorKind::TakeTill1))),
    },
    // None => Err(Err::Incomplete(Needed::Size(1))),
    None => Err(Err::Error((input, ErrorKind::TakeTill1))),
  }
}

pub fn stream_to_tree_vec(input: &TokenStream) -> TokenTreeVec {
  input.clone().into_iter().collect::<TokenTreeVec>()
}

pub fn slice_to_stream(input: TokenTreeSlice) -> TokenStream {
  input.iter().map(|x| x.clone()).collect()
}

pub fn many_0<T>(
  f: impl Fn(TokenStream) -> TokenStreamIResult<T>,
) -> impl Fn(TokenStream) -> TokenStreamIResult<Vec<T>> {
  move |mut i: TokenStream| {
    let mut acc = vec![];
    let mut last_len = stream_to_tree_vec(&i).len();
    loop {
      match f(i.clone()) {
        Err(Err::Error(_)) => return Ok((i, acc)),
        Err(e) => return Err(e),
        Ok((i1, o)) => {
          // TODO I'm not sure if this block is necessary, but there was a similar
          // block in the original (nom) source code.
          let new_len = stream_to_tree_vec(&i1).len();
          if last_len == new_len {
            if acc.len() > 0 {
              return Ok((i, acc));
            }
            return Err(Err::Error((i, ErrorKind::Many0)));
          }
          last_len = new_len;

          i = i1;
          acc.push(o);
        },
      }
    }
  }
}

// pub fn many_0_delimited<T>(
//   f: impl Fn(TokenStream) -> TokenStreamIResult<T>,
//   delim_f: impl Fn(TokenStream) -> TokenStreamIResult<T>,
// ) -> impl Fn(TokenStream) -> TokenStreamIResult<Vec<T>> {
//   move |mut i: TokenStream| {
//     let mut acc = vec![];
//     let mut last_len = stream_to_tree_vec(&i).len();
//     loop {
//       match f(i.clone()) {
//         Err(Err::Error(_)) => return Ok((i, acc)),
//         Err(e) => return Err(e),
//         Ok((i1, o)) => {
//           // TODO I'm not sure if this block is necessary, but there was a similar
//           // block in the original (nom) source code.
//           let new_len = stream_to_tree_vec(&i1).len();
//           if last_len == new_len {
//             if acc.len() > 0 {
//               return Ok((i, acc));
//             }
//             return Err(Err::Error((i, ErrorKind::Many0)));
//           }
//           last_len = new_len;

//           i = i1;
//           acc.push(o);

//           // match delim_f(i1)
//         },
//       }
//     }
//   }
// }

pub fn ensure_consumed(rest: TokenStream) -> TokenStreamIResult<()> {
  if !rest.is_empty() {
    Err(Err::Error((rest, ErrorKind::TakeTill1)))
  } else {
    Ok((rest, ()))
  }
}
