mod app;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::app::app as app_root;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<app_root::App>::new().mount_to_body();
}
