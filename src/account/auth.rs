use std::time::{Duration, SystemTime};

use generic_array::{typenum::U32, GenericArray};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Copy, Error, Serialize, Deserialize, Debug)]
#[error("Authentication failed")]
pub struct AuthError;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Auth {
	username: String,
	generated_at: SystemTime,
	valid_for: Duration,
	auth_code: GenericArray<u8, U32>,
}

#[cfg(feature = "ssr")]
pub use server::*;

#[cfg(feature = "ssr")]
mod server {
	use super::*;
	
	use std::{fmt::{self, Debug}, hash::{Hash, Hasher}};
	use leptos::use_context;
	use sha2::digest::FixedOutput;
	use hmac::Mac;
	
	type Hmac = hmac::Hmac<sha2::Sha256>;
	
	impl Auth {
		pub fn username(&self) -> Result<&str, AuthError> {
			let authenticator: Authenticator = use_context().unwrap();
			
			if authenticator.validate(self) {
				Ok(&self.username)
			} else {
				Err(AuthError)
			}
		}
	}
	
	#[derive(Clone)]
	pub struct Authenticator {
		key: [u8; 64]
	}
	
	impl Authenticator {
		pub fn new(key: [u8; 64]) -> Self {
			Self {
				key,
			}
		}
		
		pub(in super::super) fn sign(&self, username: String, valid_for: Duration) -> Auth {
			let generated_at = SystemTime::now();
			
			let mut hmac = Hmac::new_from_slice(&self.key)
				.expect("HMAC takes keys of any size");
			let mut hasher = HmacHasher(&mut hmac);
			
			username.hash(&mut hasher);
			generated_at.hash(&mut hasher);
			valid_for.hash(&mut hasher);
			
			let auth_code = hmac.finalize_fixed();
			
			Auth {
				username,
				generated_at,
				valid_for,
				auth_code,
			}
		}
		
		pub(super) fn validate(&self, auth: &Auth) -> bool {
			let Ok(elapsed_time) = auth.generated_at.elapsed() else {
				return false;
			};
			
			if elapsed_time > auth.valid_for {
				return false;
			}
			
			let mut hmac = Hmac::new_from_slice(&self.key)
				.expect("HMAC takes keys of any size");
			let mut hasher = HmacHasher(&mut hmac);
			
			auth.username.hash(&mut hasher);
			auth.generated_at.hash(&mut hasher);
			auth.valid_for.hash(&mut hasher);
			
			hmac.verify(&auth.auth_code).is_ok()
		}
	}
	
	impl Debug for Authenticator {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			write!(f, "Authenticator(...)")
		}
	}
	
	struct HmacHasher<'a>(&'a mut Hmac);
	
	impl<'a> Hasher for HmacHasher<'a> {
		fn write(&mut self, bytes: &[u8]) {
			self.0.update(bytes)
		}
		
		fn finish(&self) -> u64 {
			panic!("Don't call finish on HmacHasher")
		}
	}
}
