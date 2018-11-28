#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate smd_macro;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    // let attr = "Foo";
    let mut inner = smd!(<inner />);
    // let inner = "inner".to_string();
    let mut a = smd!(<outer>{ &mut inner } akka { "next" }</outer>);
    // let mut a = smd!(foo  bar);
    for x in &mut a {
      println!("token: {:?}", x.render());
      let response = x.handle_event(smithy_types::Event::OnTest(false), &[0]);
      println!("response {:?}", response);
      // println!("token: {:?}", x);
    }
  }
}
