use leptos::*;
use gloo_file::{Blob, ObjectUrl};

use crate::app::local_image::LocalImage;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct FileID(pub String);

#[derive(Clone, Debug)]
pub struct FileData {
	pub id: FileID,
	pub name: String,
	pub mime_type: String,
	pub data: Box<[u8]>,
}

#[component]
pub fn File(file: FileData) -> impl IntoView {
	let show_preview = file.mime_type.starts_with("image/");
	
	let preview_url = show_preview.then(|| {
		let blob = Blob::new(&*file.data);
		ObjectUrl::from(blob)
	});
	
	view! {
		{preview_url.map(|preview_url| view! {
			<LocalImage src=preview_url />
		})}
		<p>{file.name}</p>
	}
}
