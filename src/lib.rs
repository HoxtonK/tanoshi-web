#![recursion_limit = "1024"]
#[macro_use]
extern crate log;
extern crate chrono;
extern crate web_logger;

mod utils;

use wasm_bindgen::prelude::*;

mod app;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    utils::set_panic_hook();
    web_logger::custom_init(web_logger::Config {
        level: log::Level::Info,
    });
    yew::start_app::<app::App>();
    Ok(())
}
