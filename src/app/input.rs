use leptos::*;
use stylance::import_style;

import_style!(style, "input.css");

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InputType {
	Plain,
	Password,
}

impl InputType {
	fn identifier(self) -> &'static str {
		match self {
			InputType::Plain => "text",
			InputType::Password => "password",
		}
	}
}

#[component]
pub fn TextInput(
	value: RwSignal<String>,
	error: RwSignal<Option<&'static str>>,
	#[prop(into)] on_submit: Callback<()>,
	#[prop(default = InputType::Plain)] input_type: InputType,
) -> impl IntoView {
	let input_class = move || {
		if error().is_some() {
			style::error_input
		} else {
			style::input
		}
	};
	
	let value_changed = move |new_value| {
		// if new_value == value.get_untracked() {
		// 	return;
		// }
		
		value.set(new_value);
		
		if error().is_some() {
			error.set(None);
		}
	};
	
	let on_keydown = move |event: ev::KeyboardEvent| {
		if event.key_code() == 13 { // enter
			on_submit(());
		}
	};
	
	view! {
		<input
			type=move || input_type.identifier()
			class=input_class
			prop:value=value
			on:input=move |event| value_changed(event_target_value(&event))
			on:keydown=on_keydown
		/>
		{move || error().map(|error| view! {
			<p class={style::error_message}>{error}</p>
		})}
	}
}
