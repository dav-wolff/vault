use std::borrow::Cow;

use leptos::ServerFnError;

pub trait ToPrettyError {
	fn to_pretty_error(&self) -> Cow<'static, str>;
}

impl<T: ToString> ToPrettyError for ServerFnError<T> {
	fn to_pretty_error(&self) -> Cow<'static, str> {
		use ServerFnError::*;
		
		match self {
			WrappedServerError(err) => return err.to_string().into(),
			Request(_) => "Network error",
			Registration(_) | Deserialization(_) | Serialization(_) | Args(_) | MissingArg(_) => "Application error",
			Response(_) | ServerError(_) => "Server error",
		}.into()
	}
}
