#[test]
fn basic_rsx_tests() {
  use rsx_macro::rsx;

  let _app = rsx![asdf, (div[asdf2, (poop), "Asdf", 123usize, 1u2])];
}
