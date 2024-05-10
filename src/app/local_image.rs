use leptos::*;
use gloo_file::ObjectUrl;

#[component]
pub fn LocalImage(src: ObjectUrl) -> impl IntoView {
	let url = src.to_string();
	
	let mut src = Some(src);
	
	let loaded = move |_| {
		// keep ObjectUrl alive until img is loaded
		src.take();
	};
	
	view! {
		<img src=url on:load=loaded />
	}
}
