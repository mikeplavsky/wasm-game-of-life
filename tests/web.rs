//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate wasm_game_of_life;
use wasm_game_of_life::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_unverse_index() {
    let u = Universe::new();

    assert_eq!(0, u.get_index(0, 0));
    assert_eq!(1, u.get_index(0, 1));
    assert_eq!(5, u.get_index(1, 1));
}
