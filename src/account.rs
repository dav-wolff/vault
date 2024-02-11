use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Serialize, Deserialize, Error, Debug)]
pub enum LoginError {
	#[error("Unknown user")]
	UnknownUser,
	#[error("Incorrect password")]
	IncorrectPassword,
}

// TODO: take password hash instead of password
#[server]
pub async fn login(username: String, password: String) -> Result<Result<(), LoginError>, ServerFnError> {
	if username != "Test" {
		return Ok(Err(LoginError::UnknownUser));
	}
	
	if password != "1234" {
		return Ok(Err(LoginError::IncorrectPassword));
	}
	
	Ok(Ok(()))
}

#[derive(Clone, Serialize, Deserialize, Error, Debug)]
pub enum CreateAccountError {
	#[error("Unknown user")]
	UsernameTaken,
}

// TODO: take password hash instead of password
#[server]
pub async fn create_account(username: String) -> Result<Result<(), CreateAccountError>, ServerFnError> {
	if username == "Test" {
		return Ok(Err(CreateAccountError::UsernameTaken));
	}
	
	Ok(Ok(()))
}
