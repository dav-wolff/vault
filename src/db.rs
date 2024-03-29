use std::{fs::create_dir_all, io, path::Path, sync::{Arc, Mutex}};
use leptos::use_context;
use rusqlite::Connection;
use thiserror::Error;

use crate::vault::{CipherFolderName, PasswordHash, Salt};

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
				nonce BLOB NOT NULL,
				cipher_name BLOB NOT NULL,
				user TEXT NOT NULL,
				PRIMARY KEY(nonce, cipher_name),
				FOREIGN KEY(user) REFERENCES users(name) ON UPDATE CASCADE ON DELETE CASCADE
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
	
	pub fn get_folders(&self, username: &str) -> Result<Vec<CipherFolderName>, Error> {
		let connection = self.connection.lock().unwrap();
		let mut statement = connection.prepare_cached("SELECT nonce, cipher_name FROM folders WHERE user=?1")?;
		
		let results = statement.query_map([username], |row| -> Result<CipherFolderName, _> {
			let nonce = row.get::<_, [u8; 24]>(0)?.into();
			let ciphertext = row.get(1)?;
			Ok(CipherFolderName::from_db(nonce, ciphertext, token()))
		})?;
		
		Ok(results.collect::<Result<_, _>>()?)
	}
	
	pub fn add_folder(&self, username: &str, folder_name: &CipherFolderName) -> Result<(), Error> {
		let connection = self.connection.lock().unwrap();
		let mut statement = connection.prepare_cached("INSERT INTO folders (nonce, cipher_name, user) VALUES (?1, ?2, ?3)")?;
		
		let (nonce, ciphertext) = folder_name.to_db(token());
		
		statement.execute((nonce.as_slice(), ciphertext, username))?;
		
		Ok(())
	}
}
