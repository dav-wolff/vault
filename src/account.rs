use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::vault::{PasswordHash, Salt};

#[derive(Clone, Serialize, Deserialize, Error, Debug)]
pub enum LoginError {
	#[error("Unknown user")]
	UnknownUser,
	#[error("Incorrect password")]
	IncorrectPassword,
}

#[server]
pub async fn get_user_salt(username: String) -> Result<Option<Salt>, ServerFnError> {
	Ok(Some(Salt::mock_salt()))
}

#[server]
pub async fn login(username: String, hash: PasswordHash) -> Result<Result<(), LoginError>, ServerFnError> {
	if username != "Test" {
		return Ok(Err(LoginError::UnknownUser));
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
	if username == "Test" {
		return Ok(Err(CreateAccountError::UsernameTaken));
	}
	
	Ok(Ok(()))
}
