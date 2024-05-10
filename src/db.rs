use std::{fs::create_dir_all, io, path::Path, sync::{Arc, Mutex}};
use leptos::use_context;
use rusqlite::Connection;
use thiserror::Error;

use crate::vault::{Cipher, FileInfo, FolderName, PasswordHash, Salt};

pub struct Token(());

fn token() -> Token {
	Token(())
}

pub fn use_db() -> Database {
	use_context().unwrap()
}

#[derive(Error, Debug)]
pub enum Error {
	#[error("Error opening database file")]
	FileError(#[from] io::Error),
	#[error("SQLite error")]
	SQLiteError(#[from] rusqlite::Error),
	#[error("Not found")]
	NotFound,
}

#[derive(Clone, Debug)]
pub struct Database {
	connection: Arc<std::sync::Mutex<Connection>>,
}

impl Database {
	pub fn open(path: impl AsRef<Path>) -> Result<Self, Error> {
		let path = path.as_ref();
		
		if let Some(parent) = path.parent() {
			if !parent.exists() {
				create_dir_all(parent)?;
			}
		}
		
		let connection = Connection::open(path)?;
		
		connection.execute_batch("
			BEGIN;
			CREATE TABLE IF NOT EXISTS users (
				name TEXT NOT NULL PRIMARY KEY,
				salt BLOB NOT NULL,
				password_hash BLOB NOT NULL
			);
			CREATE TABLE IF NOT EXISTS folders (
				name BLOB NOT NULL,
				user TEXT NOT NULL,
				PRIMARY KEY(name),
				FOREIGN KEY(user) REFERENCES users(name) ON UPDATE CASCADE ON DELETE CASCADE
			);
			CREATE TABLE IF NOT EXISTS files (
				folder BLOB NOT NULL,
				info BLOB NOT NULL,
				file_id TEXT NOT NULL,
				PRIMARY KEY(info),
				FOREIGN KEY(folder) REFERENCES folders(name) ON UPDATE CASCADE ON DELETE CASCADE
			);
			COMMIT;
		")?;
		
		Ok(Self {
			connection: Arc::new(Mutex::new(connection)),
		})
	}
	
	pub fn insert_user(&mut self, username: &str, salt: Salt, password_hash: PasswordHash) -> Result<(), Error> {
		let connection = self.connection.lock().unwrap();
		let mut statement = connection.prepare_cached("INSERT INTO users (name, salt, password_hash) VALUES (?1, ?2, ?3)")?;
		
		statement.execute((username, salt.to_db(token()), password_hash.to_db(token())))?;
		
		Ok(())
	}
	
	pub fn is_user(&self, username: &str) -> Result<bool, Error> {
		let connection = self.connection.lock().unwrap();
		let mut statement = connection.prepare_cached("SELECT name FROM users WHERE name=?1")?;
		
		let mut results = statement.query_map([username], |_row|
			Ok(())
		)?;
		
		Ok(results.next().transpose()?.is_some())
	}
	
	pub fn get_salt(&self, username: &str) -> Result<Option<Salt>, Error> {
		let connection = self.connection.lock().unwrap();
		let mut statement = connection.prepare_cached("SELECT salt FROM users WHERE name=?1")?;
		
		let mut results = statement.query_map([username.to_string()], |row|
			Ok(Salt::from_db(row.get(0)?, token()))
		)?;
		
		Ok(results.next().transpose()?)
	}
	
	pub fn get_password_hash(&self, username: &str) -> Result<Option<PasswordHash>, Error> {
		let connection = self.connection.lock().unwrap();
		let mut statement = connection.prepare_cached("SELECT password_hash FROM users WHERE name=?1")?;
		
		let mut results = statement.query_map([username], |row|
			Ok(PasswordHash::from_db(row.get(0)?, token()))
		)?;
		
		Ok(results.next().transpose()?)
	}
	
	pub fn get_folders(&self, username: &str) -> Result<Vec<Cipher<FolderName>>, Error> {
		let connection = self.connection.lock().unwrap();
		let mut statement = connection.prepare_cached("SELECT name FROM folders WHERE user=?1")?;
		
		let results = statement.query_map([username], |row| -> Result<Cipher<FolderName>, _> {
			Ok(Cipher::<FolderName>::from_bytes(row.get(0)?))
		})?;
		
		Ok(results.collect::<Result<_, _>>()?)
	}
	
	pub fn add_folder(&self, username: &str, folder_name: &Cipher<FolderName>) -> Result<(), Error> {
		let connection = self.connection.lock().unwrap();
		let mut statement = connection.prepare_cached("INSERT INTO folders (name, user) VALUES (?1, ?2)")?;
		
		let name = folder_name.as_bytes();
		
		statement.execute((name, username))?;
		
		Ok(())
	}
	
	pub fn get_files(&self, folder: &Cipher<FolderName>) -> Result<Vec<Cipher<FileInfo>>, Error> {
		let connection = self.connection.lock().unwrap();
		let mut statement = connection.prepare_cached("SELECT info FROM files WHERE folder=?1")?;
		
		let folder = folder.as_bytes();
		
		let results = statement.query_map([folder], |row| {
			Ok(Cipher::<FileInfo>::from_bytes(row.get(0)?))
		})?;
		
		Ok(results.collect::<Result<_, _>>()?)
	}
	
	pub fn add_file(&self, folder: &Cipher<FolderName>, file_info: &Cipher<FileInfo>, file_id: &str) -> Result<(), Error> {
		let connection = self.connection.lock().unwrap();
		let mut statement = connection.prepare_cached("INSERT INTO files (folder, info, file_id) VALUES (?1, ?2, ?3)")?;
		
		let folder = folder.as_bytes();
		let file_info = file_info.as_bytes();
		
		statement.execute((
			folder,
			file_info,
			file_id
		))?;
		
		Ok(())
	}
	
	pub fn get_file_id(&self, file: &Cipher<FileInfo>) -> Result<String, Error> {
		let connection = self.connection.lock().unwrap();
		let mut statement = connection.prepare_cached("SELECT file_id FROM files WHERE info=?1")?;
		
		let file_info = file.as_bytes();
		
		let mut results = statement.query_map([file_info], |row| {
			row.get(0)
		})?;
		
		results.next().transpose()?.ok_or(Error::NotFound)
	}
}
