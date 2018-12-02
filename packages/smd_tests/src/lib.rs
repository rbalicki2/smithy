#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate smd_macro;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    use smithy_types::EventHandler;
    // let mut inner2 = smd!(<inner2 on_test={|_| println!("inner2")} />);
    // let attr = "attr";
    // let mut a = smd!(<outer foo bar="baz" qux={attr} on_test={|b| println!("this is being handled {}", b) }>{ &mut inner } akka { "next" }</outer>);
    // { &mut inner2 }
    let mut a = smd!(<outer on_test={|_| println!("should be [0]")}>
      <inner on_test={|_| println!("should be [0, 0]")} />
    </outer>
    );
    for x in &mut a {
      println!("token: {:?}", x.render());
      let response = x.handle_event(smithy_types::Event::OnTest(false), &[]);
      let response = x.handle_event(smithy_types::Event::OnTest(false), &[1, 2, 3]);
      println!("did we handle the event -> {:?}", response);
    }
  }
}
