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

pub fn many_0_delimited<T, U>(
  f: impl Fn(TokenStream) -> TokenStreamIResult<T>,
  delim_f: impl Fn(TokenStream) -> TokenStreamIResult<U>,
) -> impl Fn(TokenStream) -> TokenStreamIResult<Vec<T>> {
  // Last delimiter is optional! x,y and x,y, will both match
  // and the final comma will be consumed
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
            Err(Err::Error((rest, _e))) => {
              return Ok((rest, acc));
            },
            Err(e) => {
              return Err(e);
            },
            Ok((rest, _delim)) => {
              i = rest;
            },
          };
        },
      }
    }
  }
}

pub fn take_until<T>(
  until: impl Fn(TokenStream) -> TokenStreamIResult<T>,
) -> impl Fn(TokenStream) -> TokenStreamIResult<Vec<TokenTree>> {
  move |mut i: TokenStream| {
    let mut acc = vec![];
    loop {
      match until(i.clone()) {
        Err(Err::Error((rest, err))) => {
          // We encountered an error, push the item onto acc and continue.
          match rest.to_tree_vec().split_first() {
            Some((first, rest)) => {
              acc.push(first.clone());
              i = rest.to_token_stream();
            },
            None => {
              // This parser never encountered a failing case, so we succeed with
              // all of the TokenTree's we've accumulated
              return Ok((i, acc));
            },
          }
        },
        Err(e) => return Err(e),
        Ok((_rest, _parsed)) => {
          // We matched the "until" case, so we succeed with acc as it currently
          // is. What we matched on the until case remains in the queue.
          // (i.e. this allows us to match "anything until a comma" in the phrase
          // match many ("anything until a comma" + "a comma"))
          return Ok((i, acc));
        },
      }
    }
  }
}
