use crate::{account::Auth, app::file_area::FileArea, app_error_view::{AppError, AppErrorView}, file_store::FileStore, vault::{Cipher, FolderName, Vault}};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::path;
use leptos_router::components::{Outlet, ParentRoute, Route, Router, Routes};
use stylance::import_style;

pub mod notify;

mod input;
mod local_image;
mod login;
mod folders;
mod file_area;

use notify::NotifyProvider;
use login::Login;
use folders::Folders;

import_style!(style, "app.css");

#[derive(Clone, Debug)]
struct UserData {
	vault: Vault,
	auth: Auth,
	initial_folders: Vec<Cipher<FolderName>>,
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
	view! {
		<!DOCTYPE html>
		<html>
			<head>
				<title>Vault</title>
				<link rel="stylesheet" href="/pkg/vault.css" />
				<AutoReload options=options.clone() />
				<HydrationScripts options />
			</head>
			<body>
				<App />
			</body>
		</html>
	}
}

#[component]
pub fn App() -> impl IntoView {
	provide_meta_context();
	
	let (user_data, set_user_data) = signal_local::<Option<UserData>>(None);
	let file_store = Memo::new_owning(move |_| (user_data.read().as_ref().map(|user_data|
		FileStore::new(user_data.vault.clone(), user_data.auth.clone())
	), true));
	
	view! {
		<Router>
			<header class=style::title>Vault</header>
			<NotifyProvider>
				<main class=style::content>
					<Routes fallback=|| {
						let mut outside_errors = Errors::default();
						outside_errors.insert_with_default_key(AppError::NotFound);
						view! {
							<AppErrorView outside_errors/>
						}
						.into_view()
					}>
						<ParentRoute path=path!("/") view=move || view! {
							<Show
								when=move || user_data.with(Option::is_none)
							>
								<Login set_user_data />
							</Show>
							{move || user_data.with(|user_data| user_data.as_ref().map(|user_data| view! {
								<Folders vault=user_data.vault.clone() auth=user_data.auth.clone() initial_folders=user_data.initial_folders.clone()>
									<Outlet />
								</Folders>
							}))}
						}>
							<Route path=path!("") view=|| "" />
							<Route path=path!("/folder/:index") view=move || {
								file_store().map(|file_store| view! {
									<FileArea file_store />
								})
							} />
						</ParentRoute>
					</Routes>
				</main>
			</NotifyProvider>
		</Router>
	}
}
