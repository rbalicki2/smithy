use nom::IResult;
pub use proc_macro2::{
  Group,
  Literal,
  TokenStream,
  TokenTree,
};
use quote::ToTokens;

pub type TokenTreeSlice<'a> = &'a [TokenTree];
// pub type JsxIResult<'a, T> = IResult<TokenTreeSlice<'a>, T>;

// #[derive(Clone, Debug)]
// pub enum LiteralOrGroup {
//   Literal(Literal),
//   Group(Group),
// }

// impl From<Literal> for LiteralOrGroup {
//   fn from(literal: Literal) -> Self {
//     LiteralOrGroup::Literal(literal)
//   }
// }

// impl From<Group> for LiteralOrGroup {
//   fn from(group: Group) -> Self {
//     LiteralOrGroup::Group(group)
//   }
// }

// impl ToTokens for LiteralOrGroup {
//   fn to_tokens(&self, tokens: &mut TokenStream) {
//     match self {
//       LiteralOrGroup::Literal(literal) => literal.to_tokens(tokens),
//       LiteralOrGroup::Group(ref group) => {
//         group.to_tokens(tokens)
//       },
//     };
//   }
// }
