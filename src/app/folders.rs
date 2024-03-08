use leptos::{leptos_dom::logging::{console_error, console_warn}, *};
use stylance::{classes, import_style};

use crate::{files, vault::SecretFolderName};

use super::{input::TextInput, UserData};

mod folder;

use folder::*;

import_style!(style, "folders.css");

#[component]
pub fn Folders(
	user_data: UserData,
) -> impl IntoView {
	let vault = user_data.vault.get_value();
	
	let initial_folders: Vec<_> = user_data.initial_folder_names.iter()
		.map(|folder_name| {
			// TODO: handle error
			let name = vault.decrypt_folder_name(folder_name).unwrap();
			
			FolderData {
				id: FolderID(folder_name.clone()),
				name: create_rw_signal(name),
			}
		})
		.collect();
	
	let new_folder_name = create_rw_signal(String::new());
	let folder_name_error = create_rw_signal(None);
	let (folders, set_folders) = create_signal(initial_folders);
	let (selected_folder, set_selected_folder) = create_signal(None);
	let (is_sidebar_open, set_sidebar_open) = create_signal(true);
	
	let sidebar_classes = move || classes!(
		style::sidebar,
		is_sidebar_open().then_some(style::sidebar_open)
	);
	
	let create_folder = move |()| {
		if new_folder_name.with(String::is_empty) {
			folder_name_error.set(Some("Please enter a folder name"));
		}
		
		let folder_name = SecretFolderName::new(new_folder_name.get_untracked());
		
		// TODO: handle error
		let cipher_folder_name = user_data.vault.get_value().encrypt_folder_name(&folder_name).unwrap();
		
		spawn_local(async move {
			if let Err(_) = files::create_folder(user_data.auth.get_value(), cipher_folder_name.clone()).await {
				// TODO: handle error
				console_error("Error creating folder");
				return;
			};
			
			set_folders.update(|folders| {
				folders.push(
					FolderData {
						id: FolderID(cipher_folder_name),
						name: create_rw_signal(folder_name),
					}
				);
			});
			
			new_folder_name.set(String::new());
		});
	};
	
	let remove_folder = move |id| {
		set_folders.update(|folders| {
			let Some((index, folder)) = folders.iter()
				.enumerate()
				.find(|(_, folder)| folder.id == id)
			else {
				console_warn(&format!("Tried deleting folder {id:?} but couldn't find it in folders:\n{folders:?}"));
				return;
			};
			
			folder.name.dispose();
			folders.remove(index);
		})
	};
	
	view! {
		<button class=style::sidebar_button on:click=move |_| set_sidebar_open(true)>
			<img class=style::icon src="menu.svg" alt="Sidebar" />
		</button>
		<div class=style::sidebar_container>
			<div class=sidebar_classes>
				<button class=classes!(style::sidebar_button, style::sidebar_back_button) on:click=move |_| set_sidebar_open(false)>
					<img class=style::icon src="back_arrow.svg" alt="Close" />
				</button>
				<For
					each=move || folders()
					key=|folder| folder.id.clone()
					children=move |data| {
						let id_a = data.id.clone();
						let id_b = data.id.clone();
						view! {
							<Folder
								data
								selected_folder
								select_folder=move || set_selected_folder(Some(id_a.clone()))
								delete_folder=move || remove_folder(id_b.clone())
							/>
						}
					}
				/>
				<p class=style::label>Add folder:</p>
				<div class=style::add_folder>
					<div class=style::input>
						<TextInput value=new_folder_name error=folder_name_error on_submit=create_folder />
					</div>
					<button class=style::button on:click=move |_| create_folder(())>Add</button>
				</div>
			</div>
		</div>
	}
}
