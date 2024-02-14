use std::fmt::{self, Debug};
use getrandom::getrandom;
use sha2::{Sha512, Digest};

use super::*;

impl Salt {
	pub fn generate() -> Result<Self, getrandom::Error> {
		let mut salt = Self {
			data: [0; 32],
		};
		
		getrandom(&mut salt.data)?;
		
		Ok(salt)
	}
}

pub struct Password {
	plain_text: String,
}

impl Password {
	pub fn new(plain_text: String) -> Self {
		Self {
			plain_text,
		}
	}
	
	pub fn hash(&self, salt: &Salt) -> PasswordHash {
		let mut hasher = Sha512::new();
		hasher.update(&self.plain_text);
		hasher.update(salt.data);
		
		PasswordHash {
			data: hasher.finalize().into(),
		}
	}
}

impl Debug for Password {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Password(...)")
	}
}
