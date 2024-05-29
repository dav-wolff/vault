use axum::{body::Body, extract::State};
use axum::response::{IntoResponse, Response};
use http::{header, HeaderValue, Request, StatusCode, Uri};
use leptos::LeptosOptions;
use leptos_axum::render_app_to_stream;
use tower::ServiceExt;
use tower_http::services::ServeDir;

use crate::app::App;

const CACHE_CONTROL_STATIC: HeaderValue = HeaderValue::from_static(if option_env!("CACHE_BUST_SKIP_HASHING").is_none() {
	"public, max-age=604800, immutable"
} else {
	"public, no-cache"
});

pub async fn serve_file(uri: Uri, State(options): State<LeptosOptions>, request: Request<Body>) -> Response {
	let file_request = Request::builder()
		.uri(uri)
		.body(Body::empty())
		.unwrap();
	
	let file_response = ServeDir::new(&options.site_root)
		.oneshot(file_request).await
		.expect("Error serving file");
	
	if file_response.status() == StatusCode::OK {
		let mut response = file_response.into_response();
		response.headers_mut().append(header::CACHE_CONTROL, CACHE_CONTROL_STATIC);
		response
	} else {
		// render error page
		let handler = render_app_to_stream(options.to_owned(), App);
		handler(request).await
	}
}
