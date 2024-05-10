use super::*;

use std::{hash::{Hash, Hasher}, marker::PhantomData};

pub(super) trait Sealed {}

#[allow(private_bounds)]
pub trait CipherSecret : Sized + Sealed {
	fn as_bytes(&self) -> impl AsRef<[u8]>;
	fn from_bytes(bytes: Vec<u8>) -> Result<Self, DecryptionError>;
}

#[derive(Serialize, Deserialize)]
pub struct Cipher<T: CipherSecret> {
	data: Vec<u8>,
	#[serde(skip)]
	_phantom_data: PhantomData<T>,
}

impl<T: CipherSecret> Cipher<T> {
	#[cfg(feature = "hydrate")]
	pub(super) fn new(nonce: &Nonce, ciphertext: &[u8]) -> Self {
		let nonce: &[u8] = &nonce;
		let mut data = nonce.to_owned();
		data.extend_from_slice(&ciphertext);
		
		Self {
			data,
			_phantom_data: PhantomData,
		}
	}
	
	pub(super) fn nonce(&self) -> Nonce {
		Nonce::clone_from_slice(&self.data[..std::mem::size_of::<Nonce>()])
	}
	
	pub(super) fn ciphertext(&self) -> &[u8] {
		&self.data[std::mem::size_of::<Nonce>()..]
	}
	
	#[cfg(feature = "ssr")]
	pub fn as_bytes(&self) -> &[u8] {
		&self.data
	}
	
	#[cfg(feature = "ssr")]
	pub fn from_bytes(data: Vec<u8>) -> Self {
		Self {
			data,
			_phantom_data: Default::default(),
		}
	}
}

impl<T: CipherSecret> Clone for Cipher<T> {
	fn clone(&self) -> Self {
		Self {
			data: self.data.clone(),
			_phantom_data: PhantomData,
		}
	}
}

impl<T: CipherSecret> PartialEq for Cipher<T> {
	fn eq(&self, other: &Self) -> bool {
		self.data == other.data
	}
}

impl<T: CipherSecret> Eq for Cipher<T> {}

impl<T: CipherSecret> Hash for Cipher<T> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.data.hash(state);
	}
}

impl<T: CipherSecret> Debug for Cipher<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Cipher {{ nonce: {:?}, ciphertext: {:?} }}", self.nonce(), self.ciphertext())
	}
}
