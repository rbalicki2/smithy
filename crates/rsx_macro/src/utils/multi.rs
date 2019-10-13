use crate::prelude::*;

pub fn many_0<T>(
  f: impl Fn(TokenStream) -> TokenStreamIResult<T>,
) -> impl Fn(TokenStream) -> TokenStreamIResult<Vec<T>> {
  move |mut i: TokenStream| {
    let mut acc = vec![];
    // TODO this seems wrong. Cloning in order to get the length.
    let mut last_len = i.to_tree_vec().len();
    loop {
      match f(i.clone()) {
        Err(Err::Error(_)) => return Ok((i, acc)),
        Err(e) => return Err(e),
        Ok((i1, o)) => {
          // TODO I'm not sure if this block is necessary, but there was a similar
          // block in the original (nom) source code.
          let new_len = i1.to_tree_vec().len();
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

pub fn many_0_delimited<T>(
  f: impl Fn(TokenStream) -> TokenStreamIResult<T>,
  delim_f: impl Fn(TokenStream) -> TokenStreamIResult<T>,
) -> impl Fn(TokenStream) -> TokenStreamIResult<Vec<T>> {
  move |mut i: TokenStream| {
    let mut acc = vec![];
    let mut last_len = i.to_tree_vec().len();
    loop {
      match f(i.clone()) {
        Err(Err::Error(_)) => return Ok((i, acc)),
        Err(e) => return Err(e),
        Ok((i1, o)) => {
          // TODO I'm not sure if this block is necessary, but there was a similar
          // block in the original (nom) source code.
          let new_len = i1.to_tree_vec().len();
          if last_len == new_len {
            if acc.len() > 0 {
              return Ok((i, acc));
            }
            return Err(Err::Error((i, ErrorKind::Many0)));
          }
          last_len = new_len;

          acc.push(o);

          match delim_f(i1) {
            Err(Err::Error(_)) => return Ok((i, acc)),
            Err(e) => return Err(e),
            Ok((rest, _delim)) => {
              i = rest;
            },
          };
        },
      }
    }
  }
}
