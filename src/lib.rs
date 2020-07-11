
extern crate js_sys;
extern crate wasm_bindgen;

mod macros;
mod utils;
mod game;

use wasm_bindgen::prelude::*;
use game::manager::Manager;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    utils::set_panic_hook();
    Ok(())
}

#[wasm_bindgen]
pub fn new_game() {
    let mut manager = Manager::new(String::from("Alex"), String::from("Angelica"));
    let current_player = manager.get_current_player();
    current_player.remove_piece_from_hand(1);
    current_player.remove_piece_from_hand(1);
    current_player.remove_piece_from_hand(1);
    current_player.remove_piece_from_hand(1);
    current_player.remove_piece_from_hand(1);
    current_player.remove_piece_from_hand(1);
    current_player.remove_piece_from_hand(1);
    let d = current_player.remove_piece_from_hand(1);
    log!("{:?}", d);

    
}
