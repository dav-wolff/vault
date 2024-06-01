use leptos::{server_fn::error::NoCustomError, ServerFnError};

pub trait ToPrettyError {
	fn to_pretty_error(&self) -> &'static str;
}

impl ToPrettyError for ServerFnError {
	fn to_pretty_error(&self) -> &'static str {
		use ServerFnError::*;
		
		match self {
			WrappedServerError(NoCustomError) => "No Error",
			Request(_) => "Network error",
			Registration(_) | Deserialization(_) | Serialization(_) | Args(_) | MissingArg(_) => "Application error",
			Response(_) | ServerError(_) => "Server error",
		}
	}
}
