use http::status::StatusCode;
use leptos::*;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum AppError {
	#[error("Not Found")]
	NotFound,
	#[error("No Errors")]
	NoErrors,
	#[error("Unknown Error")]
	Unknown(server_fn::error::Error),
}

impl AppError {
	pub fn status_code(&self) -> StatusCode {
		match self {
			AppError::NotFound => StatusCode::NOT_FOUND,
			AppError::NoErrors => StatusCode::INTERNAL_SERVER_ERROR,
			AppError::Unknown(_) => StatusCode::INTERNAL_SERVER_ERROR,
		}
	}
}

// A basic function to display errors served by the error boundaries.
// Feel free to do more complicated things here than just displaying the error.
#[component]
pub fn AppErrorView(
	#[prop(optional)] outside_errors: Option<Errors>,
	#[prop(optional)] errors: Option<RwSignal<Errors>>,
) -> impl IntoView {
	let errors = match outside_errors {
		Some(e) => create_rw_signal(e),
		None => match errors {
			Some(e) => e,
			None => {
				let mut errors = Errors::default();
				errors.insert_with_default_key(AppError::NoErrors);
				create_rw_signal(errors)
			},
		},
	};
	
	let errors = errors.get_untracked();
	
	let errors: Vec<AppError> = errors
		.into_iter()
		.map(|(_, err)|
			err.downcast_ref::<AppError>()
				.cloned()
				.unwrap_or(AppError::Unknown(err))
		)
		.collect();
	
	println!("Errors: {errors:#?}");
	
	#[cfg(feature = "ssr")]
	{
		use leptos_axum::ResponseOptions;
		
		if let Some(response) = use_context::<ResponseOptions>() {
			response.set_status(errors[0].status_code());
		}
	}
	
	view! {
		<h1>{if errors.len() > 1 {"Errors"} else {"Error"}}</h1>
		<For
			each=move || {errors.clone().into_iter().enumerate()}
			key=|&(index, _)| index
			children=move |(_, error)| {
				let error_code = error.status_code();
				let error_message = error.to_string();
				view! {
					<h2>{error_code.to_string()} - {error_code.canonical_reason()}</h2>
					<p>"Error: " {error_message}</p>
				}
			}
		/>
	}
}
