#![recursion_limit = "512"]

mod app;
mod counter;
mod header;
mod nav;
mod progressbar;
mod table;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::app::Model;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
