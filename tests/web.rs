//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate wasm_game_of_life;
use wasm_game_of_life::*;

wasm_bindgen_test_configure!(run_in_browser);

macro_rules! test_rules {
    ($([$expected:expr, $value:expr, $count:expr]),*) => {
       $(
           assert_eq!(
               $expected, Universe::rules($value, $count),
               "{:?}:{}", $value, $count);
       )*
    };
}

#[wasm_bindgen_test]
fn test_rules() {
    test_rules!(
        [Cell::Dead, Cell::Alive, 0],
        [Cell::Dead, Cell::Alive, 1],
        [Cell::Alive, Cell::Alive, 2],
        [Cell::Alive, Cell::Alive, 3],
        [Cell::Dead, Cell::Alive, 4],
        [Cell::Alive, Cell::Dead, 3],
        [Cell::Dead, Cell::Dead, 4]
    );
}

#[wasm_bindgen_test]
fn test_collect() {
    let arr: Vec<Cell> = (0..2).map(|_i| Cell::Alive).collect();
    assert_eq!(vec![Cell::Alive, Cell::Alive], arr);
}

macro_rules! test_cells {
    ($([$exp:expr, $u:expr, $row:expr, $col:expr]),*) => {
        $(
            let idx = $u.get_index($row,$col);
            assert_eq!(
                $exp, $u.get_cells()[idx],
                "{}:{}", $row, $col
            );
        )*
    };
}

#[wasm_bindgen_test]
fn test_new_universe() {
    let u = Universe::new();
    test_cells!(
        [Cell::Alive, u, 0, 0],
        [Cell::Dead, u, 0, 1],
        [Cell::Alive, u, 0, 2],
        [Cell::Dead, u, 0, 3],
        [Cell::Alive, u, 0, 4],
        [Cell::Dead, u, 0, 5],
        [Cell::Alive, u, 0, 6],
        [Cell::Alive, u, 0, 7]
    );
}

#[wasm_bindgen_test]
fn test_unverse_index() {
    let u = Universe::new();

    assert_eq!(0, u.get_index(0, 0));
    assert_eq!(1, u.get_index(0, 1));
    assert_eq!(u.get_width() + 1, u.get_index(1, 1));
}

#[wasm_bindgen_test]
fn test_fmt() {
    let u = Universe::new();
    let s = u.to_string();
    let mut cs = s.chars();

    assert_eq!(Some('◼'), cs.next());
    assert_eq!(Some('◻'), cs.next());
    assert_eq!(Some('◼'), cs.next());
    assert_eq!(Some('◻'), cs.next());
    assert_eq!(Some('◼'), cs.next());
    assert_eq!(Some('◻'), cs.next());
    assert_eq!(Some('◼'), cs.next());
    assert_eq!(Some('◼'), cs.next());
}

macro_rules! test_neighbors {
    ($($name:ident : $exp:expr, $c:expr),*) => {

        $(
            #[wasm_bindgen_test]
            fn $name(){
                let u = Universe::new();
                let count = u.live_neighbor_count($c.0,$c.1);
                assert_eq!(
                    $exp,
                    count
                    );
            }
        )*

    }
}

#[wasm_bindgen_test]
fn test_live_neighbors() {
    test_neighbors!(
        cell_0_1: 6, (0, 1),
        cell_0_0: 4, (0, 0),
        cell_2_4: 3, (2, 4)
    );
}

#[wasm_bindgen_test]
fn test_document() {
    let u = Universe::new();
    let doc = u.get_document();
    assert_eq!(0, doc.url().unwrap().find("http").unwrap());
}
