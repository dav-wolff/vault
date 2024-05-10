use leptos::*;
use gloo_file::{Blob, ObjectUrl};

use crate::{app::local_image::LocalImage, file_store::{FileData, FileStore}};

#[component]
pub fn File(file_store: StoredValue<FileStore>, file: FileData) -> impl IntoView {
	let show_preview = file.info.reveal_secret().mime_type.starts_with("image/");
	
	let file_id_cloned = file.id.clone();
	
	let preview_url = move || {
		show_preview.then(|| {
			with!(|file_store| file_store.with_file_content_tracked(file.id.clone(), |content| {
				let blob = Blob::new(&*content.reveal_secret().data);
				ObjectUrl::from(blob)
			}))
		}).flatten()
	};
	
	// TODO temporary workaround for weird behavior with the effect not updating properly
	create_effect(move |_| {
		with!(|file_store| file_store.with_file_content_tracked(file_id_cloned.clone(), |_| ()));
	});
	
	view! {
		<div>
			{move || preview_url().map(|preview_url| view! {
				<LocalImage src=preview_url />
			}.into_view())}
		</div>
		<p>{file.info.into_revealed_secret().name}</p>
	}
}
