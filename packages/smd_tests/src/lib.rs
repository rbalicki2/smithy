#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate smd_macro;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let attr = "Foo";
    let mut a = smd!(<outer attr={attr}>foo bar  baaak.!          ! asdffd</outer>);
    // let mut a = smd!(foo  bar);
    for x in &mut a {
      println!("token: {:?}", x.render());
      // println!("token: {:?}", x);
    }
  }
}
