#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate smd_macro;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let attr = "Foo";
    let mut a = smd!(<div foo="FI" bar={attr} fo />);
    for x in &mut a {
      println!("token: {:?}", x.render());
    }
  }
}
