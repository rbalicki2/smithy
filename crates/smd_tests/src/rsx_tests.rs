#[test]
fn basic_rsx_tests() {
  use rsx_macro::rsx;

  let _app = rsx![(asdafa { a: 123 })];
}
