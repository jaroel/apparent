mod app;
mod ws;

use crate::app::App;
use mogwai::{
    prelude::{Gizmo, View},
    utils,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main(parent_id: Option<String>) -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // panic::set_hook(Box::new(console_error_panic_hook::hook));
    // console_log::init_with_level(Level::Trace).unwrap();

    let view = View::from(Gizmo::from(App::default()));

    if let Some(id) = parent_id {
        let parent = utils::document().get_element_by_id(&id).unwrap();
        view.run_in_container(&parent)
    } else {
        view.run()
    }
}
