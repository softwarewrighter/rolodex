mod card;
mod components;
mod storage;
mod test_data;
mod three_js;

use wasm_bindgen::prelude::*;

use components::app::App;

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Starting Rolodex application");
    yew::Renderer::<App>::new().render();
}
