mod app;

use crate::app::State;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main_js(parent_id: String) -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let state = State::new();
    dominator::append_dom(&dominator::get_id(&parent_id), State::render(state));

    Ok(())
}
