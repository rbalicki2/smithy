//! A crate containing the `smd` and `smd_borrowed` macros, which are the
//! workhorses that generate `SmithyComponent`s.

#![feature(proc_macro_span, proc_macro_raw_ident, slice_patterns)]
#![recursion_limit = "128"]
#![feature(drain_filter)]

use std::{
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

// TODO rename should_move to is_borrowed and invert the value
// since that makes more sense.
fn get_file_path(should_move: bool) -> String {
  format!(
    "{}/.smd/{}{}",
    std::env::var("HOME").unwrap(),
    env!("CARGO_PKG_VERSION"),
    if should_move { "" } else { "-borrowed" }
  )
}

type StringMap = HashMap<String, String>;

fn read_hash_map(should_move: bool) -> Result<StringMap, ()> {
  let path = get_file_path(should_move);
  read_to_string(path)
    .map_err(|_| ())
    .and_then(|s| serde_json::from_str(&s).map_err(|_| ()))
}

/// Attempts to write a hash map to the appropriate location.
/// May fail silently, we don't really care.
fn write_hash_map(map: &StringMap, should_move: bool) {
  let path = get_file_path(should_move);
  let parent = Path::new(&path).parent().unwrap();

  let _ = create_dir_all(parent);
  let _ = write(path, serde_json::to_string(map).unwrap());
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

  match read_hash_map(should_move) {
    Ok(mut map) => match map.get(&input_as_str) {
      Some(cached_item) => match cached_item.parse() {
        Ok(item) => {
          #[cfg(feature = "cache-logs")]
          println!("Item found in hashmap");
          item
        },
        Err(_) => {
          #[cfg(feature = "cache-logs")]
          println!("Error parsing item from hashmap");
          // We encountered an item that was improperly written. We need to
          // overwrite that item and re-write the hashmap to disk.
          let proc_macro_result = parse_input();
          map.insert(input_as_str, proc_macro_result.to_string());
          write_hash_map(&map, should_move);
          proc_macro_result
        },
      },
      None => {
        #[cfg(feature = "cache-logs")]
        println!("Cached item not found");
        let proc_macro_result = parse_input();
        map.insert(input_as_str, proc_macro_result.to_string());
        write_hash_map(&map, should_move);
        proc_macro_result
      },
    },
    Err(_) => {
      #[cfg(feature = "cache-logs")]
      println!("Could not deserialize hashmap!!!");
      let proc_macro_result = parse_input();
      let map = {
        let mut map = HashMap::new();
        map.insert(input_as_str, proc_macro_result.to_string());
        map
      };
      write_hash_map(&map, should_move);
      proc_macro_result
    },
  }
}
