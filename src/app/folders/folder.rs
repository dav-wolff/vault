use std::time::Duration;

use leptos::*;
use leptos_router::A;
use stylance::{classes, import_style};
use cache_bust::asset;

use crate::{app::folders::CurrentFolder, vault::{Cipher, FolderName, Secret}};

import_style!(style, "folder.scss");

#[derive(Clone, Debug)]
pub struct FolderData {
	pub id: Cipher<FolderName>,
	pub index: RwSignal<usize>,
	pub name: RwSignal<Secret<FolderName>>,
}

impl FolderData {
	pub fn dispose(&self) {
		self.index.dispose();
		self.name.dispose();
	}
}

#[component]
pub fn Folder<D>(
	data: FolderData,
	delete_folder: D,
) -> impl IntoView
where
	D: Fn(bool) + 'static,
{
	let FolderData {id, index, name} = data;
	
	let (is_editing, set_editing) = create_signal(false);
	let (new_folder_name, set_new_folder_name) = create_signal(name.get_untracked());
	
	let CurrentFolder(selected_folder) = use_context().unwrap();
	let is_selected = Signal::derive(move || selected_folder().is_some_and(|selected_id| selected_id == id));
	
	let input_ref: NodeRef<html::Input> = create_node_ref();
	
	create_effect(move |_| {
		if let Some(input) = input_ref() {
			// workaround, because without a timeout the focus doesn't work
			set_timeout(move || {
				let _ = input.focus();
			}, Duration::ZERO);
		}
	});
	
	let cancel_name_edit = move || {
		set_new_folder_name(name.get_untracked());
		set_editing(false);
	};
	
	let on_input = move |ev: ev::Event| {
		let folder_name = Secret::hide(FolderName {
			name: event_target_value(&ev),
		});
		
		set_new_folder_name(folder_name);
	};
	
	let on_keydown = move |ev: ev::KeyboardEvent| {
		if ev.key_code() == 13 { // enter
			name.set(new_folder_name.get_untracked());
			set_editing(false);
		} else if ev.key_code() == 27 { // escape
			cancel_name_edit();
		}
	};
	
	let href = move || {
		if !is_selected() {
			format!("/folder/{}", index())
		} else {
			"/".to_owned()
		}
	};
	
	view! {
		<div class=move || classes!(style::folder, is_selected().then_some(style::selected))>
			<A class=style::folder_button href>
				<Show
					when=move || is_editing()
					fallback=move || view! {<p class=style::folder_name>{name().into_revealed_secret().name}</p>}
				>
					<input
						node_ref=input_ref
						class=style::folder_input
						prop:value=move || new_folder_name().into_revealed_secret().name
						on:blur=move |_| cancel_name_edit()
						on:input=on_input
						on:keydown=on_keydown
					/>
				</Show>
			</A>
			<div class=style::background />
			<button class=style::icon_button on:click=move |_| set_editing(true)>
				<img class=style::icon src=asset!("/edit.svg") alt="Edit" />
			</button>
			<button class={classes!(style::icon_button, style::delete_button)} on:click=move |_| delete_folder(is_selected.get_untracked())>
				<img class=style::icon src=asset!("/cross.svg") alt="Delete" />
			</button>
		</div>
	}
}
