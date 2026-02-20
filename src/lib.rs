pub(crate) mod utils;

mod xlsx_reader;

pub use xlsx_reader::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn start() {
    utils::set_panic_hook();
}
