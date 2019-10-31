#[test]
fn basic_rsx_tests() {
  use rsx_macro::rsx;

  // let app = rsx![(div { readonly, ...foo } {})];
  // let app = rsx![({ div })];
  rsx![
    ({ "div" }
      { foo: "bar", another_attribute, {attribute_name}: value }
      [
        (div),
        {component}
      ]
    )
  ];
  rsx![(div {} {})];
  rsx![(div { {attr}})];
  rsx![(div { {attr}: true })];
}
