use leptos::*;
use stylance::import_style;

use crate::{account::{self, CreateAccountError, LoginError}, utils::classes, vault::{Password, Salt, Vault}};

use super::{input::{TextInput, InputType}, UserData};

import_style!(style, "login.css");

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Form {
	Login,
	CreateAccount,
}

#[component]
pub fn Login(
	set_user_data: WriteSignal<Option<UserData>>,
) -> impl IntoView {
	let (form, set_form) = create_signal(Form::Login);
	
	view! {
		<div class={style::login_box}>
			<LoginForm set_form set_user_data shown=move || form() == Form::Login />
			<CreateAccountForm set_form set_user_data shown=move || form() == Form::CreateAccount />
		</div>
	}
}

#[component]
fn LoginForm<F>(
	set_form: WriteSignal<Form>,
	set_user_data: WriteSignal<Option<UserData>>,
	shown: F,
) -> impl IntoView
where
	F: Fn() -> bool + 'static
{
	let username = create_rw_signal(String::new());
	let password = create_rw_signal(String::new());
	let username_error = create_rw_signal(None);
	let password_error = create_rw_signal(None);
	
	let clear_errors = move || {
		if username_error.with_untracked(Option::is_some) {
			username_error.set(None);
		}
		
		if password_error.with_untracked(Option::is_some) {
			password_error.set(None);
		}
	};
	
	let login = move |()| {
		clear_errors();
		
		let Some((username, password)) = with!(|username, password| {
			if username.is_empty() {
				username_error.set(Some("Please enter a username"));
			}
			
			if password.is_empty() {
				password_error.set(Some("Please enter a password"));
			}
			
			if username.is_empty() || password.is_empty() {
				return None;
			}
			
			Some((username.clone(), Password::new(password.clone())))
		}) else {
			return;
		};
		
		spawn_local(async move {
			let salt = match account::get_user_salt(username.clone()).await {
				Err(err) => {
					// TODO: handle error
					leptos_dom::error!("Error retrieving salt: {err}");
					return;
				},
				Ok(None) => {
					username_error.set(Some("Unknown user"));
					return;
				},
				Ok(Some(salt)) => salt,
			};
			
			let hash = password.hash(&salt);
			
			match account::login(username, hash).await {
				Err(err) => {
					// TODO: handle error
					leptos_dom::error!("Error logging in: {err}");
				},
				Ok(Err(LoginError::UnknownUser)) => {
					username_error.set(Some("Unknown user"));
				},
				Ok(Err(LoginError::IncorrectPassword)) => {
					password_error.set(Some("Incorrect password"));
				},
				Ok(Ok(login_data)) => {
					let vault = Vault::new(password, salt);
					
					set_user_data(Some(UserData {
						vault,
						auth: login_data.auth,
						initial_folders: login_data.folder_names,
					}));
				},
			}
		});
	};
	
	view! {
		<div hidden=move || !shown()>
			<p class={style::prompt}>Login</p>
			<p class={style::label}>Username</p>
			<TextInput value={username} error={username_error} on_submit={login} />
			<p class={style::label}>Password</p>
			<TextInput value={password} error={password_error} input_type={InputType::Password} on_submit={login} />
			<button class={style::button} on:click=move |_| login(())>Login</button>
			<hr class={style::hr} />
			<button
				class={classes([style::button, style::switch_button])}
				on:click=move |_| set_form(Form::CreateAccount)
			>
				Create account
			</button>
		</div>
	}
}

#[component]
fn CreateAccountForm<F>(
	set_form: WriteSignal<Form>,
	set_user_data: WriteSignal<Option<UserData>>,
	shown: F,
) -> impl IntoView
where
	F: Fn() -> bool + 'static
{
	let username = create_rw_signal(String::new());
	let password = create_rw_signal(String::new());
	let password_confirm = create_rw_signal(String::new());
	let username_error = create_rw_signal(None);
	let password_error = create_rw_signal(None);
	let password_confirm_error = create_rw_signal(None);
	
	let clear_errors = move || {
		if username_error.with_untracked(Option::is_some) {
			username_error.set(None);
		}
		
		if password_error.with_untracked(Option::is_some) {
			password_error.set(None);
		}
		
		if password_confirm_error.with_untracked(Option::is_some) {
			password_confirm_error.set(None);
		}
	};
	
	let create_account = move |()| {
		clear_errors();
		
		let Some((username, password)) = with!(|username, password, password_confirm| {
			if username.is_empty() {
				username_error.set(Some("Please enter a username"));
			}
			
			if password.is_empty() {
				password_error.set(Some("Please enter a password"));
			}
			
			if password_confirm.is_empty() {
				password_confirm_error.set(Some("Please confirm your password"));
			} else if password != password_confirm {
				password_confirm_error.set(Some("Passwords don't match"));
			}
			
			if username.is_empty() || password.is_empty() || password != password_confirm {
				return None;
			}
			
			Some((username.clone(), Password::new(password.clone())))
		}) else {
			return;
		};
		
		let salt = Salt::generate().unwrap(); // TODO: handle error
		let hash = password.hash(&salt);
		
		spawn_local(async move {
			match account::create_account(username, salt.clone(), hash).await {
				Err(err) => {
					//TODO: handle error
					leptos_dom::error!("Error creating account: {err}");
				},
				Ok(Err(CreateAccountError::UsernameTaken)) => {
					username_error.set(Some("Username is already taken"));
				},
				Ok(Ok(login_data)) => {
					let vault = Vault::new(password, salt);
					
					set_user_data(Some(UserData {
						vault,
						auth: login_data.auth,
						initial_folders: login_data.folder_names,
					}));
				},
			}
		});
	};
	
	view! {
		<div hidden=move || !shown()>
			<button
				class={classes([style::button, style::back_button])}
				on:click=move |_| set_form(Form::Login)
			>
				<img src="/back_arrow.svg" alt="Back" />
			</button>
			<p class={style::prompt}>Create account</p>
			<p class={style::label}>Username</p>
			<TextInput value={username} error={username_error} on_submit={create_account} />
			<p class={style::label}>Password</p>
			<TextInput value={password} error={password_error} input_type={InputType::Password} on_submit={create_account} />
			<p class={style::label}>Confirm password</p>
			<TextInput value={password_confirm} error={password_confirm_error} input_type={InputType::Password} on_submit={create_account} />
			<button class={style::button} on:click=move |_| create_account(())>Create account</button>
		</div>
	}
}
