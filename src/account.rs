// TODO: remove
#![allow(unused)]

use leptos::{server, use_context, ServerFnError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{vault::{PasswordHash, Salt}};
use crate::db;

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
pub async fn login(username: String, hash: PasswordHash) -> Result<Result<(), LoginError>, ServerFnError> {
	let db = db::use_db();
	
	let Some(correct_hash) = db.get_password_hash(&username)? else {
		return Ok(Err(LoginError::UnknownUser));
	};
	
	if hash != correct_hash {
		return Ok(Err(LoginError::IncorrectPassword));
	}
	
	Ok(Ok(()))
}

#[derive(Clone, Serialize, Deserialize, Error, Debug)]
pub enum CreateAccountError {
	#[error("Unknown user")]
	UsernameTaken,
}

#[server]
pub async fn create_account(username: String, salt: Salt, hash: PasswordHash) -> Result<Result<(), CreateAccountError>, ServerFnError> {
	let mut db = db::use_db();
	
	if db.is_user(&username)? {
		return Ok(Err(CreateAccountError::UsernameTaken));
	}
	
	db.insert_user(&username, salt, hash);
	
	Ok(Ok(()))
}
