use std::time::Duration;

use leptos::*;
use stylance::{classes, import_style};

import_style!(style, "folder.scss");

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FolderID(pub u32);

#[derive(Clone, Debug)]
pub struct FolderData {
	pub id: FolderID,
	pub name: RwSignal<String>,
}

#[component]
pub fn Folder<FS, FD>(
	data: FolderData,
	selected_folder: ReadSignal<Option<FolderID>>,
	mut select_folder: FS,
	mut delete_folder: FD,
) -> impl IntoView
where
	FS: FnMut() + 'static,
	FD: FnMut() + 'static,
{
	let FolderData {id, name} = data;
	
	let is_selected = move || selected_folder() == Some(id);
	
	let (is_editing, set_editing) = create_signal(false);
	let (new_folder_name, set_new_folder_name) = create_signal(name.get_untracked());
	
	let input_ref: NodeRef<html::Input> = create_node_ref();
	
	create_effect(move |_| {
		if let Some(input) = input_ref() {
			// workaround, because without a timeout the focus doesn't work
			set_timeout(move || {
				let _ = input.focus();
			}, Duration::ZERO);
		}
	});
	
	let cancel_name_edit = move |_| {
		set_new_folder_name(name.get_untracked());
		set_editing(false);
	};
	
	let on_keydown = move |ev: ev::KeyboardEvent| {
		if ev.key_code() == 13 { // enter
			name.set(new_folder_name.get_untracked());
			set_editing(false);
		}
	};
	
	let folder_classes = move || classes!(
		style::folder,
		is_selected().then_some(style::selected)
	);
	
	view! {
		<div class=folder_classes>
			<button class=style::folder_button on:click=move |_| select_folder()>
				<Show
					when=move || is_editing()
					fallback=move || view!{<p class=style::folder_name>{name}</p>}
				>
					<input
						node_ref=input_ref
						class=style::folder_input
						prop:value=new_folder_name
						on:blur=cancel_name_edit
						on:input=move |ev| set_new_folder_name(event_target_value(&ev))
						on:keydown=on_keydown
					/>
				</Show>
			</button>
			<div class=style::background />
			<button class=style::icon_button on:click=move |_| set_editing(true)>
				<img class=style::icon src="edit.svg" alt="Edit" />
			</button>
			<button class={classes!(style::icon_button, style::delete_button)} on:click=move |_| delete_folder()>
				<img class=style::icon src="cross.svg" alt="Delete" />
			</button>
		</div>
	}
}
