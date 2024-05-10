use crate::{account::Auth, app::file_area::FileArea, app_error_view::{AppError, AppErrorView}, file_store::FileStore, vault::{Cipher, FolderName, Vault}};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use stylance::import_style;

mod input;
mod local_image;
mod login;
mod folders;
mod file_area;

use login::Login;
use folders::Folders;

import_style!(style, "app.css");

#[derive(Clone, Debug)]
struct UserData {
	vault: Vault,
	auth: Auth,
	initial_folders: Vec<Cipher<FolderName>>,
}

#[derive(Params, Clone, PartialEq, Eq, Debug)]
struct FolderParams {
	index: usize,
}

#[component]
pub fn App() -> impl IntoView {
	provide_meta_context();
	
	let (user_data, set_user_data) = create_signal::<Option<UserData>>(None);
	let file_store = create_owning_memo(move |_| (with!(|user_data| user_data.as_ref().map(|user_data|
		FileStore::new(user_data.vault.clone(), user_data.auth.clone())
	)), true));
	
	view! {
		<Title text="Vault" />
		<Stylesheet id="leptos" href="/pkg/vault.css" />
		
		<Router fallback=|| {
			let mut outside_errors = Errors::default();
			outside_errors.insert_with_default_key(AppError::NotFound);
			view! {
				<AppErrorView outside_errors/>
			}
			.into_view()
		}>
			<header class=style::title>Vault</header>
			<main class=style::content>
				<Routes>
					<Route path="/" view=move || view! {
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
						<Route path="" view=|| "" />
						<Route path="/folder/:index" view=move || {
							file_store().map(|file_store| view! {
								<FileArea file_store />
							})
						} />
					</Route>
				</Routes>
			</main>
		</Router>
	}
}
