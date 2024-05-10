use std::rc::Rc;

use argon2::Argon2;
use chacha20poly1305::{aead::Aead, Key, KeyInit, XChaCha20Poly1305};
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

#[derive(Clone)]
pub struct Secret<T: CipherSecret> {
	inner: T,
}

impl<T: CipherSecret> Secret<T> {
	pub fn hide(value: T) -> Self {
		Self {
			inner: value,
		}
	}
	
	pub fn reveal_secret(&self) -> &T {
		&self.inner
	}
	
	pub fn into_revealed_secret(self) -> T {
		self.inner
	}
}

#[derive(Clone)]
pub struct Vault {
	cipher: Rc<XChaCha20Poly1305>,
}

impl Vault {
	pub fn new(password: Password, salt: Salt) -> Self {
		let argon2 = Argon2::default();
		let mut key: Key = Key::default();
		
		argon2.hash_password_into(password.plain_text.as_bytes(), &salt.data, &mut key).unwrap();
		
		let cipher = XChaCha20Poly1305::new(&key);
		
		Self {
			cipher: Rc::new(cipher),
		}
	}
	
	pub fn encrypt<T: CipherSecret>(&self, secret: &Secret<T>) -> Result<Cipher<T>, EncryptionError> {
		let secret = secret.inner.as_bytes();
		let mut nonce = Nonce::default();
		getrandom(&mut nonce)?;
		
		let ciphertext = self.cipher.encrypt(&nonce, secret.as_ref())?;
		
		Ok(Cipher::new(&nonce, &ciphertext))
	}
	
	pub fn decrypt<T: CipherSecret>(&self, cipher: &Cipher<T>) -> Result<Secret<T>, DecryptionError> {
		let secret = self.cipher.decrypt(&cipher.nonce(), cipher.ciphertext())?;
		
		Ok(Secret {
			inner: T::from_bytes(secret)?,
		})
	}
}
