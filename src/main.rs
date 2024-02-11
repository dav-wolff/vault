#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
	use axum::{Router, routing::post};
	use leptos::*;
	use leptos_axum::{generate_route_list, handle_server_fns, LeptosRoutes};
	use tokio::net::TcpListener;
	
	use vault::app::App;
	use vault::serve_file::serve_file;
	
	// <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
	let config = get_configuration(None).await.unwrap();
	let leptos_options = config.leptos_options;
	let addr = leptos_options.site_addr;
	let routes = generate_route_list(App);
	
	let app = Router::new()
		.leptos_routes(&leptos_options, routes, App)
		.route("/api/*fn_name", post(handle_server_fns))
		.fallback(serve_file)
		.with_state(leptos_options);
	
	let listener = TcpListener::bind(&addr).await.unwrap();
	logging::log!("Server running on http://{}", &addr);
	axum::serve(listener, app.into_make_service()).await
		.unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
	eprintln!("Can't execute the client directly. To run the server, compile with feature flag `ssr` instead.");
}
