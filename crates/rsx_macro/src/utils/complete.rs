use crate::prelude::*;

pub fn ensure_consumed(rest: TokenStream) -> TokenStreamIResult<()> {
  if !rest.is_empty() {
    Err(Err::Error((rest, ErrorKind::TakeTill1)))
  } else {
    Ok((rest, ()))
  }
}
