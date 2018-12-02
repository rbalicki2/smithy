#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate smd_macro;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    // let mut inner = smd!(<inner on_test={|_| println!("inner")} />);
    // let attr = "attr";
    // let mut a = smd!(<outer foo bar="baz" qux={attr} on_test={|b| println!("this is being handled {}", b) }>{ &mut inner } akka { "next" }</outer>);
    let mut a = smd!(<outer on_test={|_| println!("on test outer")}>
      <inner on_test={|_| println!("on test inner")} />
    </outer>);
    for x in &mut a {
      println!("token: {:?}", x.render());
      let response = x.handle_event(smithy_types::Event::OnTest(false), &[]);
      let response = x.handle_event(smithy_types::Event::OnTest(false), &[1, 2, 3]);
      println!("did we handle the event -> {:?}", response);
    }
  }
}
