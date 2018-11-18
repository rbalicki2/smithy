#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate smd_macro;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let a = smd!(<div foo="FI" bar={a} />);
  }
}
