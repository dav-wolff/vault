use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::vault::{Cipher, FolderName, PasswordHash, Salt};

#[allow(unused)]
use crate::db;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Auth {
	username: String,
}

#[cfg(feature = "ssr")]
impl Auth {
	fn new(username: String) -> Self {
		Self {
			username,
		}
	}
	
	pub fn is_valid(&self) -> bool {
		// TODO: authorization
		true
	}
	
	pub fn username(&self) -> &str {
		&self.username
	}
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LoginData {
	pub auth: Auth,
	pub folder_names: Vec<Cipher<FolderName>>,
}

#[derive(Clone, Serialize, Deserialize, Error, Debug)]
pub enum LoginError {
	#[error("Unknown user")]
	UnknownUser,
	#[error("Incorrect password")]
	IncorrectPassword,
}

#[server]
pub async fn get_user_salt(username: String) -> Result<Option<Salt>, ServerFnError> {
	let db = db::use_db();
	
	Ok(db.get_salt(&username)?)
}

#[server]
pub async fn login(username: String, hash: PasswordHash) -> Result<Result<LoginData, LoginError>, ServerFnError> {
	let db = db::use_db();
	
	let Some(correct_hash) = db.get_password_hash(&username)? else {
		return Ok(Err(LoginError::UnknownUser));
	};
	
	if hash != correct_hash {
		return Ok(Err(LoginError::IncorrectPassword));
	}
	
	let folder_names = db.get_folders(&username)?;
	
	Ok(Ok(LoginData {
		auth: Auth::new(username),
		folder_names,
	}))
}

#[derive(Clone, Serialize, Deserialize, Error, Debug)]
pub enum CreateAccountError {
	#[error("Unknown user")]
	UsernameTaken,
}

#[server]
pub async fn create_account(username: String, salt: Salt, hash: PasswordHash) -> Result<Result<LoginData, CreateAccountError>, ServerFnError> {
	let mut db = db::use_db();
	
	if db.is_user(&username)? {
		return Ok(Err(CreateAccountError::UsernameTaken));
	}
	
	db.insert_user(&username, salt, hash)?;
	
	Ok(Ok(LoginData {
		auth: Auth::new(username),
		folder_names: Vec::new(),
	}))
}
