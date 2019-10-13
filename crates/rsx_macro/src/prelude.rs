pub use nom::{
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

pub trait ToTreeVec {
  fn to_tree_vec(&self) -> TokenTreeVec;
}
impl ToTreeVec for TokenStream {
  fn to_tree_vec(&self) -> TokenTreeVec {
    self.clone().into_iter().collect::<TokenTreeVec>()
  }
}

pub trait ToStream {
  fn to_token_stream(&self) -> TokenStream;
}
impl ToStream for TokenTreeSlice<'_> {
  fn to_token_stream(&self) -> TokenStream {
    // this converts from a Vec of references to a Vec...
    // there should be a simpler way to do this
    self.iter().map(|x| x.clone()).collect()
  }
}
