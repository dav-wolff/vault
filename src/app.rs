use crate::{account::Auth, app::file_area::FileArea, app_error_view::{AppError, AppErrorView}, vault::{CipherFolderName, Vault}};
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
	vault: StoredValue<Vault>,
	auth: StoredValue<Auth>,
	initial_folder_names: Vec<CipherFolderName>,
}

#[derive(Params, Clone, PartialEq, Eq, Debug)]
struct FolderParams {
	index: usize,
}

#[component]
pub fn App() -> impl IntoView {
	provide_meta_context();
	
	let (user_data, set_user_data) = create_signal(None);
	
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
			<header class={style::title}>Vault</header>
			<main class={style::content}>
				<Routes>
					<Route path="/" view=move || view! {
						<Show
							when=move || user_data.with(Option::is_none)
						>
							<Login set_user_data />
						</Show>
						{move || user_data().map(|user_data| view! {
							<Folders user_data />
							<div class=style::file_area>
								<Outlet />
							</div>
						})}
					}>
						<Route path="" view=|| "" />
						<Route path="/folder/:index" view=move || {
							let params = use_params::<FolderParams>();
							
							view! {
								<FileArea />
							}
						} />
					</Route>
				</Routes>
			</main>
		</Router>
	}
}
