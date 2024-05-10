#[cfg(feature = "ssr")]
pub mod server;

#[cfg(feature = "ssr")]
mod db;

#[cfg(not(feature = "ssr"))]
mod db {}

mod app;
mod utils;
mod account;
mod files;
mod app_error_view;
mod vault;
mod file_store;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
	use app::App;
	
	console_error_panic_hook::set_once();
	leptos::mount_to_body(App);
}
