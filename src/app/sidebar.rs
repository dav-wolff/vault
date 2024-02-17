use leptos::*;
use stylance::{classes, import_style};

use super::input::TextInput;

import_style!(style, "sidebar.css");

#[derive(Clone, Debug)]
struct Folder {
	id: u64,
	name: String,
}

#[component]
pub fn Sidebar() -> impl IntoView {
	let new_folder_name = create_rw_signal(String::new());
	let folder_name_error = create_rw_signal(None);
	let (folders, set_folders) = create_signal(vec![
		Folder {
			id: 0,
			name: "Folder".to_owned(),
		},
		Folder {
			id: 1,
			name: "Another folder".to_owned(),
		},
	]);
	
	let (is_sidebar_open, set_sidebar_open) = create_signal(true);
	
	let sidebar_classes = move || classes!(
		style::sidebar,
		is_sidebar_open().then_some(style::sidebar_open)
	);
	
	let add_folder = move |()| {
		if new_folder_name().is_empty() {
			folder_name_error.set(Some("Please enter a folder name"));
		}
		
		set_folders.update(|folders| {
			folders.push(
				Folder {
					id: folders.len() as u64,
					name: new_folder_name(),
				}
			);
		});
		
		new_folder_name.set(String::new());
	};
	
	view! {
		<button class=style::sidebar_button on:click=move |_| set_sidebar_open(true)>
			<img class=style::icon src="" alt="Sidebar" />
		</button>
		<div class=style::sidebar_container>
			<div class=sidebar_classes>
				<button class=classes!(style::sidebar_button, style::sidebar_back_button) on:click=move |_| set_sidebar_open(false)>
					<img class=style::icon src="" alt="Close" />
				</button>
				<For
					each=move || folders()
					key=|folder| folder.id
					children=|folder| view! {<p>{folder.name}</p>}
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
