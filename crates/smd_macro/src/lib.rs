//! A crate containing the `smd` and `smd_borrowed` macros, which are the
//! workhorses that generate `SmithyComponent`s.

#![feature(proc_macro_span, proc_macro_raw_ident, slice_patterns)]
#![recursion_limit = "128"]
#![feature(drain_filter)]

use serde_derive::{
  Deserialize,
  Serialize,
};
use std::{
  cell::RefCell,
  collections::HashMap,
  fs::{
    create_dir_all,
    read_to_string,
    write,
  },
  path::Path,
};

extern crate proc_macro;

mod parsers;
mod types;

type ProcMacroMap = HashMap<String, proc_macro::TokenStream>;
thread_local! {
  static SMD_CACHE: RefCell<ProcMacroMap> = RefCell::new(HashMap::new());
  static SMD_BORROWED_CACHE: RefCell<ProcMacroMap> = RefCell::new(HashMap::new());
}

/// proc-macro to take a `SmithyComponent`, capturing referenced variables.
#[proc_macro]
pub fn smd(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  smd_inner(input, true)
}

/// proc-macro to take a `SmithyComponent`, not capturing referenced variables.
///
/// A call to `smd_borrowed!` should usually be inside of a call to `smd!`.
#[proc_macro]
pub fn smd_borrowed(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  smd_inner(input, false)
}

fn get_file_path() -> String {
  format!(
    "{}/.smd/{}",
    std::env::var("HOME").unwrap(),
    env!("CARGO_PKG_VERSION")
  )
}

#[derive(Serialize, Deserialize, Debug)]
struct StringMap(HashMap<String, String>);

fn read_hash_map() -> Result<StringMap, ()> {
  let path = get_file_path();
  read_to_string(path)
    .map_err(|_| ())
    .and_then(|s| serde_json::from_str(&s).map_err(|_| ()))
}

fn write_hash_map(map: &StringMap) -> Result<(), ()> {
  let path = get_file_path();
  let parent = Path::new(&path).parent().unwrap();

  create_dir_all(parent);

  write(path, serde_json::to_string(map).unwrap()).map_err(|_| ())
}

fn smd_inner(input: proc_macro::TokenStream, should_move: bool) -> proc_macro::TokenStream {
  let input_as_str = input.to_string();
  let parse_input = || {
    let input: proc_macro2::TokenStream = input.into();
    let vec_of_trees: Vec<proc_macro2::TokenTree> = input.into_iter().collect();
    let parsed = parsers::match_html_component(&vec_of_trees, should_move);

    let unwrapped = parsed.unwrap();
    #[cfg(feature = "smd-logs")]
    println!("\nlet mut app = {};\n", unwrapped.1);

    let proc_macro_result: proc_macro::TokenStream = unwrapped.1.into();
    proc_macro_result
  };

  match read_hash_map() {
    Ok(mut map) => match map.0.get(&input_as_str) {
      Some(cached_item) => cached_item.parse().unwrap(),
      None => {
        let proc_macro_result = parse_input();
        map.0.insert(input_as_str, proc_macro_result.to_string());
        write_hash_map(&map);
        proc_macro_result
      },
    },
    Err(_) => {
      let proc_macro_result = parse_input();
      let map = StringMap({
        let mut map = HashMap::new();
        map.insert(input_as_str, proc_macro_result.to_string());
        map
      });
      write_hash_map(&map);
      proc_macro_result
    },
  }
}
