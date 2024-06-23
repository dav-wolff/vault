mod serve_file;

use std::{fs, net::Ipv4Addr, path::{Path, PathBuf}};

use axum::{body::Body, extract::{FromRef, Request, State}, response::IntoResponse, routing::post, Router};
use http::{header, HeaderValue};
use leptos::*;
use leptos_axum::{generate_route_list, handle_server_fns_with_context, render_app_to_stream_with_context, LeptosRoutes};
use tokio::net::TcpListener;
use getrandom::getrandom;

use crate::{account::Authenticator, app::App, db::Database};
use serve_file::serve_file;

#[derive(Clone, Debug)]
pub struct AppState {
	leptos_options: LeptosOptions,
	authenticator: Authenticator,
	database: Database,
	files_location: PathBuf,
}

impl FromRef<AppState> for LeptosOptions {
	fn from_ref(input: &AppState) -> Self {
		input.leptos_options.clone()
	}
}

macro_rules! get_env {
	($key: literal, $map: expr) => {
		std::env::var($key)
			.ok()
			.map($map)
			.or(option_env!($key).map($map))
			.expect(concat!($key, " variable must be provided"))
	};
	
	($key: literal) => {
		get_env!($key, Into::into)
	};
}

fn get_auth_key(path: &Path) -> [u8; 64] {
	if path.exists() {
		fs::read(&path).expect(&format!("Could not read auth key file at: {path:?}"))
			.try_into().expect("Auth key should be 64 bytes long")
	} else {
		let mut key = [0; 64];
		getrandom(&mut key).expect("Could not generate random key");
		fs::write(path, &key).expect(&format!("Could not write auth key file at: {path:?}"));
		key
	}
}

pub async fn serve() {
	// <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
	let config = get_configuration(None).await.unwrap();
	let leptos_options = config.leptos_options;
	let routes = generate_route_list(App);
	
	let port: u16 = get_env!("VAULT_PORT", |str| str.parse()).expect("VAULT_PORT must be a number");
	let auth_key_file: PathBuf = get_env!("VAULT_AUTH_KEY");
	let db_file: PathBuf = get_env!("VAULT_DB_FILE");
	let files_location: PathBuf = get_env!("VAULT_FILES_LOCATION");
	
	let auth_key = get_auth_key(&auth_key_file);
	
	if !files_location.exists() {
		fs::create_dir_all(&files_location).expect("Could not create folder for files at: {files_location}");
	}
	
	let context = AppState {
		leptos_options,
		authenticator: Authenticator::new(auth_key),
		database: Database::open(db_file).unwrap(),
		files_location,
	};
	
	let app = Router::<AppState>::new()
		.leptos_routes_with_handler(routes, handle_leptos_routes)
		.route("/api/*fn_name", post(handle_server_fns))
		.fallback(serve_file)
		.with_state(context);
	
	let listener = TcpListener::bind((Ipv4Addr::new(127, 0, 0, 1), port)).await.unwrap();
	logging::log!("Server running on port {port}");
	axum::serve(listener, app.into_make_service()).await
		.unwrap();
}

const CACHE_CONTROL_HTML: HeaderValue = HeaderValue::from_static("no-cache");

async fn handle_leptos_routes(State(app_state): State<AppState>, request: Request<Body>) -> impl IntoResponse {
	let handler = render_app_to_stream_with_context(
		app_state.leptos_options.clone(),
		move || {
			provide_context(app_state.database.clone());
			provide_context(app_state.files_location.clone());
		},
		App
	);
	
	let mut response = handler(request).await;
	response.headers_mut().append(header::CACHE_CONTROL, CACHE_CONTROL_HTML);
	response
}

async fn handle_server_fns(State(app_state): State<AppState>, request: Request<Body>) -> impl IntoResponse {
	handle_server_fns_with_context(
		move || {
			// TODO isn't there a better way?
			provide_context(app_state.authenticator.clone());
			provide_context(app_state.database.clone());
			provide_context(app_state.files_location.clone());
		},
		request
	).await
}
