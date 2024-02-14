#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
	vault::server::serve().await;
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
	eprintln!("Can't execute the client directly. To run the server, compile with feature flag `ssr` instead.");
}
