#[test]
fn basic_rsx_tests() {
  use rsx_macro::rsx;

  let _app = rsx![asdf, (div[asdf2, (poop)])];
}
