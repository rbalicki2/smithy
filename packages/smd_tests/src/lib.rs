#![feature(proc_macro_hygiene, slice_patterns)]

#[allow(unused_imports)]
#[macro_use]
extern crate smd_macro;

// mod basic_event_handler_tests;
// mod basic_rendering_tests;

#[test]
fn name() {
  use smithy_types::{
    AsInnerHtml,
    Component,
  };
  struct State {
    count: i32,
  }
  let mut state = State { count: 0 };
  // let mut previous = smd!(<div on_click={|_| println!("previous") } /><h3 />);
  let mut a = smd!(
    on_hash_change={|_| state.count = 0};
    <div>
      <h1 on_click={|_| state.count = state.count - 1}>-</h1>
      <h2 on_click={|_| state.count = state.count + 1}>+</h2>
      // { &mut previous }
    </div>
  );
  let b = a.render();

  println!("{}", b.as_inner_html(&[]));
}
