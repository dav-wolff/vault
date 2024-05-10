use std::{marker::PhantomData, convert::Infallible};

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
pub struct Secret<T: CipherSecret> {
	_inner: PhantomData<T>,
}

impl<T: CipherSecret> Secret<T> {
	pub fn hide(_value: T) -> Self {
		panic!("Not implemented for ssr");
	}
	
	pub fn reveal_secret(&self) -> &T {
		panic!("Not implemented for ssr");
	}
	
	pub fn into_revealed_secret(self) -> T {
		panic!("Not implemented for ssr");
	}
}

#[derive(Clone)]
pub struct Vault(());

impl Vault {
	pub fn new(_password: Password, _salt: Salt) -> Self {
		panic!("Not implemented for ssr");
	}
	
	pub fn encrypt<T: CipherSecret>(&self, _secret: &Secret<T>) -> Result<Cipher<T>, EncryptionError> {
		panic!("Not implemented for ssr");
	}
	
	pub fn decrypt<T: CipherSecret>(&self, _cipher: &Cipher<T>) -> Result<Secret<T>, DecryptionError> {
		panic!("Not implemented for ssr");
	}
}
