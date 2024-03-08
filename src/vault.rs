#[cfg(feature = "hydrate")]
mod secure;

#[cfg(feature = "hydrate")]
pub use secure::*;

#[derive(Clone, Error, Debug)]
pub enum EncryptionError {
	#[error("Could not generate random nonce")]
	NonceGenerationError(#[from] getrandom::Error),
	#[error("Error encrypting plain text")]
	ChaChaError(#[from] chacha20poly1305::Error),
}

#[derive(Clone, Error, Debug)]
pub enum DecryptionError {
	#[error("Error decrypting ciphertext")]
	ChaChaError(#[from] chacha20poly1305::Error),
	#[error("Failed to parse plain text as UTF-8")]
	ParseUtf8Error(#[from] FromUtf8Error),
}


#[cfg(not(feature = "hydrate"))]
mod mock_secure;

#[cfg(not(feature = "hydrate"))]
pub use mock_secure::*;
use thiserror::Error;

use std::{fmt::{self, Debug}, string::FromUtf8Error};
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

type Nonce = chacha20poly1305::aead::Nonce<chacha20poly1305::XChaCha20Poly1305>;

#[derive(Clone, Serialize, Deserialize)]
pub struct Salt {
	data: [u8; 32],
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasswordHash {
	#[serde(with = "BigArray")]
	data: [u8; 64],
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct CipherFolderName {
	nonce: Nonce,
	ciphertext: Vec<u8>,
}

#[cfg(feature = "ssr")]
mod db_acces {
	use super::*;
	use crate::db;
	
	impl Salt {
		pub fn from_db(data: [u8; 32], _: db::Token) -> Self {
			Self {
				data,
			}
		}
		
		pub fn to_db(&self, _: db::Token) -> [u8; 32] {
			self.data
		}
	}
	
	impl PasswordHash {
		pub fn from_db(data: [u8; 64], _: db::Token) -> Self {
			Self {
				data,
			}
		}
		
		pub fn to_db(&self, _: db::Token) -> [u8; 64] {
			self.data
		}
	}
	
	impl CipherFolderName {
		pub fn from_db(nonce: Nonce, ciphertext: Vec<u8>, _: db::Token) -> Self {
			Self {
				nonce,
				ciphertext,
			}
		}
		
		pub fn to_db(&self, _: db::Token) -> (Nonce, &[u8]) {
			(self.nonce, &self.ciphertext)
		}
	}
}

macro_rules! hidden_debug {
	($t: ty) => {
		impl Debug for $t {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				write!(f, "{}(...)", stringify!($t))
			}
		}
	}
}

hidden_debug!(Password);
hidden_debug!(Salt);
hidden_debug!(PasswordHash);
hidden_debug!(Vault);
hidden_debug!(SecretFolderName);
