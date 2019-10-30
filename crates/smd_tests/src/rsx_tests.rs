#[test]
fn basic_rsx_tests() {
  use rsx_macro::rsx;

  let app = rsx![(div { readonly, ...foo } {})];
}
