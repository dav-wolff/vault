pub mod app;
mod account;
mod app_error_view;
mod style_utils;

#[cfg(feature = "ssr")]
pub mod serve_file;


#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
	use app::App;
	
	console_error_panic_hook::set_once();
	leptos::mount_to_body(App);
}
