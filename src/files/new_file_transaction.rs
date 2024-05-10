use std::{fs::{self, File}, io::{self, Write}, path::{Path, PathBuf}, sync::RwLock};

use getrandom::getrandom;

static FILE_LOCK: RwLock<()> = RwLock::new(());

const ID_CHARACTERS: [char; 36] = [
	'0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
	'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
	'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
	'u', 'v', 'w', 'x', 'y', 'z',
];

fn create_id(length: usize) -> String {
	let mut random_data = vec![0; length];
	// TODO handle error?
	getrandom(&mut random_data).unwrap();
	
	random_data.into_iter()
		.map(|byte| ID_CHARACTERS[byte as usize % ID_CHARACTERS.len()])
		.collect()
}

pub struct NewFileTransaction {
	path: PathBuf,
	file: File,
	id: String,
	is_complete: bool,
}

impl NewFileTransaction {
	pub fn open_file(folder: &Path) -> Result<Self, io::Error> {
		let _lock = FILE_LOCK.read().unwrap();
		
		for i in 1.. {
			let id = create_id(i);
			let path = folder.join(&id);
			
			return match File::create_new(&path) {
				Err(err) if err.kind() == io::ErrorKind::AlreadyExists => continue,
				Err(err) => Err(err),
				Ok(file) => Ok(Self {
					path,
					file,
					id,
					is_complete: false,
				}),
			}
			
		}
		
		todo!()
	}
	
	pub fn id(&self) -> &str {
		&self.id
	}
	
	pub fn write_data(mut self, data: &[u8]) -> Result<(), io::Error> {
		self.file.write_all(data)?;
		
		self.is_complete = true;
		
		Ok(())
	}
}

impl Drop for NewFileTransaction {
	fn drop(&mut self) {
		if self.is_complete {
			return;
		}
		
		let _lock = FILE_LOCK.write().unwrap();
		
		if let Err(err) = fs::remove_file(&self.path) {
			eprintln!("Could not delete file: {err}");
		}
	}
}
