use nom::{
  error::ErrorKind,
  Err,
  IResult,
};
pub use proc_macro2::{
  TokenStream,
  TokenTree,
};

pub type TokenStreamIResult<T> = IResult<TokenStream, T>;
pub type TokenTreeSlice<'a> = &'a [TokenTree];
pub type TokenTreeVec = Vec<TokenTree>;
