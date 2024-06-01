use leptos::*;
use stylance::import_style;
use gloo_file::FileList;

mod file;

use file::*;

use crate::{app::{folders::CurrentFolder, notify::Notify}, file_store::FileStore, vault::{FileContent, FileInfo, Secret}};

import_style!(style, "file_area.scss");

async fn parse_files(file_list: FileList) -> Vec<(Secret<FileInfo>, Secret<FileContent>)> {
	let data_futures: Vec<_> = file_list.iter()
		.map(|file| gloo_file::futures::read_as_bytes(&file))
		.collect();
	
	let files_data = futures::future::join_all(data_futures).await;
	
	file_list.into_iter().zip(files_data.into_iter())
		.filter_map(|(file, data_result)| match data_result {
			Ok(data) => Some((file, data)),
			Err(error) => {
				let file_name = file.name();
				leptos_dom::error!("Error trying to read ArrayBuffer of file {file_name}:\n{error}");
				None
			}
		})
		.map(|(file, data)| {
			let info = Secret::hide(FileInfo {
				name: file.name(),
				mime_type: file.raw_mime_type(),
			});
			
			let content = Secret::hide(FileContent {
				data,
			});
			
			(info, content)
		})
		.collect()
}

#[component]
pub fn FileArea(file_store: FileStore) -> impl IntoView {
	let (is_drag_target, set_is_drag_target) = create_signal(false);
	let input_ref: NodeRef<html::Input> = create_node_ref();
	
	let CurrentFolder(current_folder) = use_context().unwrap();
	
	let file_store = store_value(file_store);
	
	let files = move || with!(|file_store| file_store.files_in_folder_tracked(current_folder().expect("FileArea should not be shown with no folder selected")));
	
	let notify = Notify::from_context();
	
	// TODO temporary workaround for weird behavior with the effect not updating properly
	create_effect(move |_| with!(|file_store| file_store.files_in_folder_tracked(current_folder().unwrap())));
	
	let add_files = move |file_list: FileList| async move {
		let folder = current_folder.get_untracked().expect("FileArea should not be shown with no folder selected");
		let files = parse_files(file_list).await;
		
		for (file_info, _) in &files {
			notify.info(format!("Uploading {}...", file_info.reveal_secret().name));
		}
		
		let file_store = file_store.get_value();
		file_store.add_files(folder, files).await;
	};
	
	let handle_drag = move |event: ev::DragEvent| {
		event.prevent_default();
		event.stop_propagation();
		
		let data_transfer = event.data_transfer().expect("DataTransfer should always be present");
		
		if data_transfer.types().includes(&"Files".into(), 0) {
			if event.type_() == "dragenter" {
				set_is_drag_target(true);
			}
			
			data_transfer.set_drop_effect("copy");
		} else {
			data_transfer.set_drop_effect("none");
		}
	};
	
	let handle_drag_leave = move |event: ev::DragEvent| {
		event.prevent_default();
		event.stop_propagation();
		
		set_is_drag_target(false);
	};
	
	let handle_drop = move |event: ev::DragEvent| {
		event.prevent_default();
		event.stop_propagation();
		
		set_is_drag_target(false);
		
		let file_list = event.data_transfer().expect("DataTransfer should always be present")
			.files().expect("FileList should always be present for a drop event");
		
		spawn_local(add_files(file_list.into()));
	};
	
	let handle_file_input = move |event: ev::Event| {
		let Some(input) = input_ref() else {
			return;
		};
		
		event.prevent_default();
		event.stop_propagation();
		
		let file_list = input.files().expect("FileList should always be present on input of type file");
		
		spawn_local(add_files(file_list.into()));
	};
	
	view! {
		<div class=style::main on:dragenter=handle_drag on:dragover=handle_drag>
			{move || match files() {
				Some(files) => view! {
					// TODO Does this rerender everytime a file is added?
					<For
						// TODO why does this need to be cloned?
						each=move || files.clone()
						key=|file| file.id.clone()
						children=move |file| view! {
							<File file_store file />
						}
					/>
				}.into_view(),
				None => view! {
					<p>Loading...</p>
				}.into_view(),
			}}
			<label class=style::upload_button>
				<input type="file" multiple on:change=handle_file_input node_ref=input_ref />
				<img src="" alt="Upload" />
			</label>
			<Show when=is_drag_target>
				<div
					class=style::drag_queen
					on:dragenter=handle_drag
					on:dragover=handle_drag
					on:dragleave=handle_drag_leave
					on:drop=handle_drop
				/>
			</Show>
		</div>
	}
}
