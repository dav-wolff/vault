use leptos::*;
use leptos_router::{use_location, use_navigate};
use stylance::{classes, import_style};
use cache_bust::asset;

use crate::{account::Auth, files, vault::{Cipher, FolderName, Secret, Vault}};

use super::input::TextInput;

mod folder;

use folder::*;

import_style!(style, "folders.css");

#[derive(Clone, Debug)]
pub struct CurrentFolder(pub Memo<Option<Cipher<FolderName>>>);

#[component]
pub fn Folders(
	vault: Vault,
	auth: Auth,
	initial_folders: Vec<Cipher<FolderName>>,
	children: Children,
) -> impl IntoView {
	let initial_folders: Vec<_> = initial_folders.into_iter()
		.enumerate()
		.map(|(index, folder_name)| {
			// TODO: handle error
			let name = vault.decrypt(&folder_name).unwrap();
			
			FolderData {
				id: folder_name,
				index: create_rw_signal(index),
				name: create_rw_signal(name),
			}
		})
		.collect();
	
	let new_folder_name = create_rw_signal(String::new());
	let folder_name_error = create_rw_signal(None);
	let (folders, set_folders) = create_signal(initial_folders);
	let (is_sidebar_open, set_sidebar_open) = create_signal(true);
	
	let location = use_location();
	
	let selected_folder = create_memo(move |_| {
		location.pathname.with(|pathname| {
			pathname.strip_prefix("/folder/").and_then(|folder_index|
				// TODO handle # and ? in url?
				folder_index.parse::<usize>().ok()
			).and_then(|index|
				folders().get(index).map(|folder_data| folder_data.id.clone())
			)
		})
	});
	
	provide_context(CurrentFolder(selected_folder));
	
	let sidebar_classes = move || classes!(
		style::sidebar,
		is_sidebar_open().then_some(style::sidebar_open)
	);
	
	let create_folder = move |()| {
		if new_folder_name.with(String::is_empty) {
			folder_name_error.set(Some("Please enter a folder name"));
		}
		
		let folder_name = Secret::hide(FolderName {
			name: new_folder_name.get_untracked(),
		});
		
		// TODO: handle error
		let cipher_folder_name = vault.encrypt(&folder_name).unwrap();
		
		let auth = auth.clone();
		
		spawn_local(async move {
			if let Err(err) = files::create_folder(auth, cipher_folder_name.clone()).await {
				// TODO: handle error
				leptos_dom::error!("Error creating folder: {err}");
				return;
			};
			
			set_folders.update(|folders| {
				folders.push(
					FolderData {
						id: cipher_folder_name,
						index: create_rw_signal(folders.len()),
						name: create_rw_signal(folder_name),
					}
				);
			});
			
			new_folder_name.set(String::new());
		});
	};
	
	let remove_folder = move |index: usize, is_selected| {
		set_folders.update(|folders| {
			folders[index].dispose();
			folders.remove(index);
			
			folders[index..].iter_mut()
				.enumerate()
				.for_each(|(i, folder)| folder.index.set(index + i));
		});
		
		if is_selected {
			let navigate = use_navigate();
			navigate("/", Default::default());
		}
	};
	
	let create_folder_cloned = create_folder.clone();
	
	view! {
		<button class=style::sidebar_button on:click=move |_| set_sidebar_open(true)>
			<img class=style::icon src=asset!("/menu.svg") alt="Sidebar" />
		</button>
		<div class=style::sidebar_container>
			<div class=sidebar_classes>
				<button class=classes!(style::sidebar_button, style::sidebar_back_button) on:click=move |_| set_sidebar_open(false)>
					<img class=style::icon src=asset!("/back_arrow.svg") alt="Close" />
				</button>
				<For
					each=move || folders()
					key=|folder| folder.id.clone()
					children=move |data| {
						let index = data.index;
						view! {
							<Folder
								data
								delete_folder=move |is_selected| remove_folder(index.get_untracked(), is_selected)
							/>
						}
					}
				/>
				<p class=style::label>Add folder:</p>
				<div class=style::add_folder>
					<div class=style::input>
						<TextInput value=new_folder_name error=folder_name_error on_submit=create_folder_cloned />
					</div>
					<button class=style::button on:click=move |_| create_folder(())>Add</button>
				</div>
			</div>
		</div>
		<div class=style::content>
			{children()}
		</div>
	}
}
