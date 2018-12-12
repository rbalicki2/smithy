#![feature(proc_macro_hygiene, slice_patterns)]

#[allow(unused_imports)]
#[macro_use]
extern crate smd_macro;

mod basic_event_handler_tests;
mod basic_rendering_tests;

// #[test]
// fn name() {
//   use smithy_types::{
//     AsInnerHtml,
//     Component,
//   };
//   let mut previous = smd!(<div on_click={|_| println!("previous") } /><h3 />);
//   let mut a = smd!(<div
//     on_click={|_| println!("outer")}
//   >
//     <h1 on_click={|_| println!("inner")} />
//     <h2 on_click={|_| println!("inner2")} />
//     { &mut previous }
//   </div>);
//   let b = a.render();

//   println!("{}", b.as_inner_html(&[]));
// }
