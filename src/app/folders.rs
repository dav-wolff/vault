use leptos::{leptos_dom::logging::console_warn, *};
use stylance::{classes, import_style};

use super::input::TextInput;

mod folder;

use folder::*;

import_style!(style, "folders.css");

#[component]
pub fn Folders() -> impl IntoView {
	let new_folder_name = create_rw_signal(String::new());
	let folder_name_error = create_rw_signal(None);
	let (folders, set_folders) = create_signal(vec![
		FolderData {
			id: FolderID(0),
			name: create_rw_signal("Folder".to_owned()),
		},
		FolderData {
			id: FolderID(1),
			name: create_rw_signal("Another folder".to_owned()),
		},
	]);
	let (selected_folder, set_selected_folder) = create_signal(None);
	
	let (is_sidebar_open, set_sidebar_open) = create_signal(true);
	
	let sidebar_classes = move || classes!(
		style::sidebar,
		is_sidebar_open().then_some(style::sidebar_open)
	);
	
	let current_id = store_value(std::cell::Cell::new(2));
	
	let add_folder = move |()| {
		if new_folder_name().is_empty() {
			folder_name_error.set(Some("Please enter a folder name"));
		}
		
		let new_id = current_id.with_value(|cell| {
			let id = cell.get();
			cell.set(id + 1);
			id
		});
		
		set_folders.update(|folders| {
			folders.push(
				FolderData {
					id: FolderID(new_id),
					name: create_rw_signal(new_folder_name()),
				}
			);
		});
		
		new_folder_name.set(String::new());
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
					key=|folder| folder.id
					children=move |data| {
						let id = data.id;
						view! {
							<Folder
								data
								selected_folder
								select_folder=move || set_selected_folder(Some(id))
								delete_folder=move || remove_folder(id)
							/>
						}
					}
				/>
				<p class=style::label>Add folder:</p>
				<div class=style::add_folder>
					<div class=style::input>
						<TextInput value=new_folder_name error=folder_name_error on_submit=add_folder />
					</div>
					<button class=style::button on:click=move |_| add_folder(())>Add</button>
				</div>
			</div>
		</div>
	}
}
