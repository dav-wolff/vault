mod serve_file;

use axum::{body::Body, extract::{FromRef, Request, State}, response::IntoResponse, routing::post, Router};
use leptos::*;
use leptos_axum::{generate_route_list, handle_server_fns_with_context, render_app_to_stream_with_context, LeptosRoutes};
use tokio::net::TcpListener;

use crate::{app::App, db::Database};
use serve_file::serve_file;

#[derive(Clone, Debug)]
pub struct AppState {
	leptos_options: LeptosOptions,
	database: Database,
}

impl FromRef<AppState> for LeptosOptions {
	fn from_ref(input: &AppState) -> Self {
		input.leptos_options.clone()
	}
}

pub async fn serve() {
	// <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
	let config = get_configuration(None).await.unwrap();
	let leptos_options = config.leptos_options;
	let addr = leptos_options.site_addr;
	let routes = generate_route_list(App);
	
	let db_file = std::env::var("VAULT_DB_FILE")
		.expect("VAULT_DB_FILE variable must be provided");
	
	let context = AppState {
		leptos_options,
		database: Database::open(db_file).unwrap(),
	};
	
	let app = Router::<AppState>::new()
		.leptos_routes_with_handler(routes, handle_leptos_routes)
		.route("/api/*fn_name", post(handle_server_fns))
		.fallback(serve_file)
		.with_state(context);
	
	let listener = TcpListener::bind(&addr).await.unwrap();
	logging::log!("Server running on http://{}", &addr);
	axum::serve(listener, app.into_make_service()).await
		.unwrap();
}

async fn handle_leptos_routes(State(app_state): State<AppState>, request: Request<Body>) -> impl IntoResponse {
	let handler = render_app_to_stream_with_context(
		app_state.leptos_options.clone(),
		move || {
			provide_context(app_state.database.clone());
		},
		App
	);
	
	handler(request).await
}

async fn handle_server_fns(State(app_state): State<AppState>, request: Request<Body>) -> impl IntoResponse {
	handle_server_fns_with_context(
		move || {
			provide_context(app_state.database.clone());
		},
		request
	).await
}
