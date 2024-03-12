use std::time::Duration;

use leptos::*;
use leptos_router::{use_location, A};
use stylance::{classes, import_style};

use crate::vault::{CipherFolderName, SecretFolderName};

import_style!(style, "folder.scss");

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct FolderID(pub CipherFolderName);

#[derive(Clone, Debug)]
pub struct FolderData {
	pub id: FolderID,
	pub index: RwSignal<usize>,
	pub name: RwSignal<SecretFolderName>,
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
	let FolderData {index, name, ..} = data;
	
	let (is_editing, set_editing) = create_signal(false);
	let (new_folder_name, set_new_folder_name) = create_signal(name.get_untracked());
	
	let folder_href = move || format!("/folder/{}", index());
	
	let location = use_location();
	
	let is_selected = create_memo(move |_| {
		location.pathname.with(|pathname| {
			pathname.to_ascii_lowercase().starts_with(&folder_href())
		})
	});
	
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
			folder_href()
		} else {
			"/".to_owned()
		}
	};
	
	view! {
		<div class=move || classes!(style::folder, is_selected().then_some(style::selected))>
			<A class=style::folder_button href>
				<Show
					when=move || is_editing()
					fallback=move || view! {<p class=style::folder_name>{name}</p>}
				>
					<input
						node_ref=input_ref
						class=style::folder_input
						prop:value=new_folder_name
						on:blur=move |_| cancel_name_edit()
						on:input=move |ev| set_new_folder_name(SecretFolderName::new(event_target_value(&ev)))
						on:keydown=on_keydown
					/>
				</Show>
			</A>
			<div class=style::background />
			<button class=style::icon_button on:click=move |_| set_editing(true)>
				<img class=style::icon src="/edit.svg" alt="Edit" />
			</button>
			<button class={classes!(style::icon_button, style::delete_button)} on:click=move |_| delete_folder(is_selected.get_untracked())>
				<img class=style::icon src="/cross.svg" alt="Delete" />
			</button>
		</div>
	}
}
