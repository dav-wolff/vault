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
pub struct SecretFolderName {
	name: String,
}

impl SecretFolderName {
	pub fn new(name: String) -> Self {
		Self {
			name,
		}
	}
	
	pub fn as_str(&self) -> &str {
		&self.name
	}
}

impl leptos::IntoView for SecretFolderName {
	fn into_view(self) -> leptos::View {
		self.name.into_view()
	}
}

impl From<SecretFolderName> for wasm_bindgen::JsValue {
	fn from(value: SecretFolderName) -> Self {
		value.name.into()
	}
}

#[derive(Clone)]
pub struct Vault {
	cipher: XChaCha20Poly1305,
}

impl Vault {
	pub fn new(password: Password, salt: Salt) -> Self {
		let argon2 = Argon2::default();
		let mut key: Key = Key::default();
		
		argon2.hash_password_into(password.plain_text.as_bytes(), &salt.data, &mut key).unwrap();
		
		let cipher = XChaCha20Poly1305::new(&key);
		
		Self {
			cipher,
		}
	}
	
	fn encrypt(&self, secret: &[u8]) -> Result<(Vec<u8>, Nonce), EncryptionError> {
		let mut nonce = Nonce::default();
		getrandom(&mut nonce)?;
		
		let ciphertext = self.cipher.encrypt(&nonce, secret)?;
		
		Ok((ciphertext, nonce))
	}
	
	fn decrypt(&self, nonce: &Nonce, ciphertext: &[u8]) -> Result<Vec<u8>, DecryptionError> {
		let secret = self.cipher.decrypt(nonce, ciphertext)?;
		
		Ok(secret)
	}
	
	pub fn encrypt_folder_name(&self, secret: &SecretFolderName) -> Result<CipherFolderName, EncryptionError> {
		let (ciphertext, nonce) = self.encrypt(secret.name.as_bytes())?;
		
		Ok(CipherFolderName {
			nonce,
			ciphertext,
		})
	}
	
	pub fn decrypt_folder_name(&self, cipher_name: &CipherFolderName) -> Result<SecretFolderName, DecryptionError> {
		let plain_text = self.decrypt(&cipher_name.nonce, &cipher_name.ciphertext)?;
		let name = String::from_utf8(plain_text)?;
		
		Ok(SecretFolderName {
			name,
		})
	}
}
