use std::{convert::Infallible, fmt::{self, Debug}};
use super::*;

impl Salt {
	pub fn generate() -> Result<Self, Infallible> {
		panic!("Not implemented for ssr");
	}
}

pub struct Password(());

impl Password {
	pub fn new(_plain_text: String) -> Self {
		panic!("Not implemented for ssr");
	}
	
	pub fn hash(&self, _salt: &Salt) -> PasswordHash {
		panic!("Not implemented for ssr");
	}
}

impl Debug for Password {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Password(...)")
	}
}
