use super::*;

#[derive(Clone)]
pub struct FolderName {
	pub name: String,
}

impl Sealed for FolderName {}

impl CipherSecret for FolderName {
	fn as_bytes(&self) -> impl AsRef<[u8]> {
		self.name.as_bytes()
	}
	
	fn from_bytes(bytes: Vec<u8>) -> Result<Self, DecryptionError> {
		Ok(Self {
			name: String::from_utf8(bytes).map_err(|err| err.utf8_error())?,
		})
	}
}

#[derive(Clone)]
pub struct FileInfo {
	pub name: String,
	pub mime_type: String,
}

impl Sealed for FileInfo {}

impl CipherSecret for FileInfo {
	fn as_bytes(&self) -> impl AsRef<[u8]> {
		let mut bytes = Vec::with_capacity(
			std::mem::size_of::<usize>()
				+ self.name.as_bytes().len()
				+ self.mime_type.as_bytes().len()
		);
		
		bytes.extend_from_slice(&self.name.as_bytes().len().to_le_bytes());
		bytes.extend_from_slice(self.name.as_bytes());
		bytes.extend_from_slice(self.mime_type.as_bytes());
		
		bytes
	}
	
	fn from_bytes(bytes: Vec<u8>) -> Result<Self, DecryptionError> {
		let size_len = std::mem::size_of::<usize>();
		let name_len = usize::from_le_bytes(
			bytes.get(..size_len).ok_or(DecryptionError::UnexpectedEndOfBytes)?
				.try_into().expect("size_of::<usize> should always be the correct size for from_le_bytes")
		);
		let name_end = size_len + name_len;
		
		let name = std::str::from_utf8(
			bytes.get(size_len..name_end).ok_or(DecryptionError::UnexpectedEndOfBytes)?
		)?.to_owned();
		
		let mime_type = std::str::from_utf8(
			bytes.get(name_end..).ok_or(DecryptionError::UnexpectedEndOfBytes)?
		)?.to_owned();
		
		Ok(Self {
			name,
			mime_type,
		})
	}
}

#[derive(Clone)]
pub struct FileContent {
	pub data: Vec<u8>,
}

impl Sealed for FileContent {}

impl CipherSecret for FileContent {
	fn as_bytes(&self) -> impl AsRef<[u8]> {
		&self.data
	}
	
	fn from_bytes(bytes: Vec<u8>) -> Result<Self, DecryptionError> {
		Ok(Self {
			data: bytes,
		})
	}
}
