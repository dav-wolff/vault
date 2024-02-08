use leptos::{leptos_dom::logging::console_log, *};
use stylance::import_style;

use crate::style_utils::classes;

use super::input::{TextInput, InputType};

import_style!(style, "login.css");

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Form {
	Login,
	CreateAccount,
}

#[component]
pub fn Login(
	set_logged_in: WriteSignal<bool>,
) -> impl IntoView {
	let (form, set_form) = create_signal(Form::Login);
	
	view! {
		<div class={style::login_box}>
			<Show
				when=move || form() == Form::CreateAccount
				fallback=move || view! {<LoginForm set_form set_logged_in />}
			>
				<CreateAccountForm set_form set_logged_in />
			</Show>
		</div>
	}
}

#[component]
pub fn LoginForm(
	set_form: WriteSignal<Form>,
	set_logged_in: WriteSignal<bool>,
) -> impl IntoView {
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
		
		with!(|username, password| {
			if username.is_empty() {
				username_error.set(Some("Please enter a username"));
			}
			
			if password.is_empty() {
				password_error.set(Some("Please enter a password"));
			}
			
			if username.is_empty() || password.is_empty() {
				return;
			}
			
			if username != "Test" {
				username_error.set(Some("Unknown user"));
				return;
			}
			
			if password != "1234" {
				password_error.set(Some("Incorrect password"));
				return;
			}
			
			set_logged_in(true);
		});
	};
	
	create_effect(move |_| {
		console_log(&username());
	});
	
	view! {
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
	}
}

#[component]
pub fn CreateAccountForm(
	set_form: WriteSignal<Form>,
	set_logged_in: WriteSignal<bool>,
) -> impl IntoView {
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
		
		with!(|username, password, password_confirm| {
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
				return;
			}
			
			set_logged_in(true);
		})
	};
	
	view! {
		<button
			class={classes([style::button, style::back_button])}
			on:click=move |_| set_form(Form::Login)
		>
			<img src={"back_arrow.svg"} alt="Back" />
		</button>
		<p class={style::prompt}>Create account</p>
		<p class={style::label}>Username</p>
		<TextInput value={username} error={username_error} on_submit={create_account} />
		<p class={style::label}>Password</p>
		<TextInput value={password} error={password_error} input_type={InputType::Password} on_submit={create_account} />
		<p class={style::label}>Confirm password</p>
		<TextInput value={password_confirm} error={password_confirm_error} input_type={InputType::Password} on_submit={create_account} />
		<button class={style::button} on:click=move |_| create_account(())>Create account</button>
	}
}
