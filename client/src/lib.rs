mod app;
// mod counter;
// mod header;
// mod nav;
// mod progressbar;
// mod queue;
// mod table;
mod wsding;

use wasm_bindgen::prelude::*;

use crate::app::start;

#[wasm_bindgen(start)]
pub fn run_app() {
    start()
}
