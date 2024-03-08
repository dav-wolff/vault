use std::convert::Infallible;
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

#[derive(Clone)]
pub struct SecretFolderName(());

impl SecretFolderName {
	pub fn new(_name: String) -> Self {
		panic!("Not implemented for ssr");
	}
	
	pub fn as_str(&self) -> &str {
		panic!("Not implemented for ssr");
	}
}

impl leptos::IntoView for SecretFolderName {
	fn into_view(self) -> leptos::View {
		panic!("Not implemented for ssr");
	}
}

impl From<SecretFolderName> for wasm_bindgen::JsValue {
	fn from(_value: SecretFolderName) -> Self {
		panic!("Not implemented for ssr");
	}
}

#[derive(Clone)]
pub struct Vault(());

impl Vault {
	pub fn new(_password: Password, _salt: Salt) -> Self {
		panic!("Not implemented for ssr");
	}
	
	pub fn encrypt_folder_name(&self, _secret: &SecretFolderName) -> Result<CipherFolderName, EncryptionError> {
		panic!("Not implemented for ssr");
	}
	
	pub fn decrypt_folder_name(&self, _cipher_name: &CipherFolderName) -> Result<SecretFolderName, DecryptionError> {
		panic!("Not implemented for ssr");
	}
}
