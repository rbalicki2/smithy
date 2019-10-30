use crate::prelude::*;

pub fn ensure_consumed(rest: TokenStream) -> TokenStreamIResult<()> {
  if !rest.is_empty() {
    Err(Err::Error((rest, ErrorKind::TakeTill1)))
  } else {
    Ok((rest, ()))
  }
}

// TODO rename
pub fn ensure2<T>(rest: TokenStream, real_rest: TokenStream, t: T) -> TokenStreamIResult<T> {
  if !rest.is_empty() {
    Err(Err::Error((real_rest, ErrorKind::TakeTill1)))
  } else {
    Ok((real_rest, t))
  }
}
