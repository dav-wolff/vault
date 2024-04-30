use leptos::*;
use stylance::import_style;
use gloo_file::FileList;

mod file;

use file::*;

import_style!(style, "file_area.scss");

#[component]
pub fn FileArea() -> impl IntoView {
	let (is_drag_target, set_is_drag_target) = create_signal(false);
	let (files, set_files) = create_signal(Vec::new());
	let input_ref: NodeRef<html::Input> = create_node_ref();
	
	let add_files = move |file_list: FileList| async move {
		let data_futures: Vec<_> = file_list.iter()
			.map(|file| gloo_file::futures::read_as_bytes(&file))
			.collect();
		
		let files_data = futures::future::join_all(data_futures).await;
		
		let new_files: Vec<_> = file_list.into_iter().zip(files_data.into_iter())
			.filter_map(|(file, data_result)| match data_result {
				Ok(data) => Some((file, data)),
				Err(error) => {
					let file_name = file.name();
					leptos_dom::error!("Error trying to read ArrayBuffer of file {file_name}:\n{error}");
					None
				}
			})
			.map(|(file, data)| {
				FileData {
					id: FileID(file.name()),
					name: file.name(),
					mime_type: file.raw_mime_type(),
					data: data.into(),
				}
			})
			.collect();
		
		set_files.update(|files| files.extend_from_slice(&new_files));
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
			<For
				each=move || files()
				key=|file| file.id.clone()
				children=|file| view! {
					<File file />
				}
			/>
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
