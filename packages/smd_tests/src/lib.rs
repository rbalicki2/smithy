#![feature(proc_macro_hygiene, slice_patterns)]
#[macro_use]
extern crate smd_macro;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    use smithy_types::EventHandler;
    let mut inner0 = smd!(
      <inner0 on_test={|_| println!("inner00 event handled")} />
      <inner1 on_test={|_| println!("inner01 event handled")} />
    );
    let mut inner1 = smd!(
      <inner0 on_test={|_| println!("inner10 event handled")} />
      <inner1 on_test={|_| println!("inner11 event handled")} />
    );
    // let attr = "attr";
    // let mut a = smd!(<outer foo bar="baz" qux={attr} on_test={|b| println!("this is being handled {}", b) }>{ &mut inner } akka { "next" }</outer>);
    // { &mut inner2 }
    //  on_test={|_| println!("should be [0]")}
    let mut a = smd!(
      { &mut inner0 }
      <inner on_test={|_| println!("inner1")} />
    );

    let response = a.handle_event(&smithy_types::Event::OnTest(false), &[1]);
    println!("did we handle the event -> {:?}", response);
    // for x in &mut a {
    //   // println!("token: {:?}", x.render());
    //   // let response = x.handle_event(&smithy_types::Event::OnTest(false), &[1, 2, 3]);
    // }
  }
}
