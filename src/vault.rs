#[cfg(feature = "hydrate")]
mod secure;
#[cfg(feature = "hydrate")]
pub use secure::*;

#[cfg(not(feature = "hydrate"))]
mod mock_secure;
#[cfg(not(feature = "hydrate"))]
pub use mock_secure::*;

#[cfg(feature = "ssr")]
mod db_access;

mod cipher_secret;
pub use cipher_secret::*;

mod types;
pub use types::*;

use std::{fmt::{self, Debug}, str::Utf8Error};
use thiserror::Error;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

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
	ParseUtf8Error(#[from] Utf8Error),
	#[error("Plain text ended unexpectedly")]
	UnexpectedEndOfBytes,
}

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

impl<T: CipherSecret> Debug for Secret<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Secret<{}>(...)", std::any::type_name::<T>())
	}
}
