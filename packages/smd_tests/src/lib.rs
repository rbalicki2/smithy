#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate smd_macro;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let mut inner = smd!(<inner />);
    let attr = "attr";
    let mut a = smd!(<outer foo bar="baz" qux={attr} on_test={|b| println!("this is being handled {}", b) }>{ &mut inner } akka { "next" }</outer>);
    for x in &mut a {
      println!("token: {:?}", x.render());
      let response = x.handle_event(smithy_types::Event::OnTest(false), &[]);
      println!("did we handle the event -> {:?}", response);
    }
  }
}
