#![feature(proc_macro_hygiene, slice_patterns)]
#[macro_use]
extern crate smd_macro;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    use smithy_types::Component;
    struct AppState {
      pub is_transitioned: bool,
    }
    let mut state: AppState = AppState {
      is_transitioned: false,
    };
    let mut a = smd!(
      <div on_test={|_| state.is_transitioned = true}>
        { if state.is_transitioned { "true" } else { "false" } }
      </div>
    );

    // let nodez = a.render();
    // println!("we rendered {:#?}", nodez);
    let response = a.handle_event(&smithy_types::Event::OnTest(false), &[0]);
    println!("did we handle the event -> {:?}", response);
    // let nodez = a.render();
    // println!("we rendered {:#?}", nodez);
  }
}
