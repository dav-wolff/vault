pub mod app;
mod utils;
mod account;
mod app_error_view;
mod vault;

#[cfg(feature = "ssr")]
pub mod serve_file;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
	use app::App;
	
	console_error_panic_hook::set_once();
	leptos::mount_to_body(App);
}
