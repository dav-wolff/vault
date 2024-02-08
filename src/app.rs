use crate::app_error_view::{AppError, AppErrorView};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use stylance::import_style;

mod input;
mod login;

use login::Login;

import_style!(style, "app.css");

#[component]
pub fn App() -> impl IntoView {
	provide_meta_context();
	
	let (is_logged_in, set_logged_in) = create_signal(false);
	
	view! {
		<Title text="Vault"/>
		<Stylesheet id="leptos" href="/pkg/vault.css"/>
		
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
					<Route path="" view=move || view! {
						<Show
							when=move || is_logged_in()
							fallback=move || view! {<Login set_logged_in />}
						>
							<h1>Welcome!</h1>
						</Show>
					} />
				</Routes>
			</main>
		</Router>
	}
}
