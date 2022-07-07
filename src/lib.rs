mod app;
mod simulation;
mod view;

use seed::prelude::{wasm_bindgen, App};

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", app::init, app::update, view::view);
}
