use std::{fs, path::PathBuf, str::FromStr};

use leptos::{server, ServerFnError};
use thiserror::Error;

use crate::{account::{Auth, AuthError}, vault::{Cipher, FileContent, FileInfo, FolderName}};

#[allow(unused)]
use crate::db;

#[cfg(feature = "ssr")]
mod new_file_transaction;
#[cfg(feature = "ssr")]
use self::new_file_transaction::*;

#[server]
pub async fn create_folder(auth: Auth, folder_name: Cipher<FolderName>) -> Result<(), ServerFnError<FilesError>> {
	let username = auth.username()?;
	
	let db = db::use_db();
	
	db.add_folder(username, &folder_name)?;
	
	Ok(())
}

#[server]
pub async fn get_files(auth: Auth, folder: Cipher<FolderName>) -> Result<Vec<Cipher<FileInfo>>, ServerFnError<FilesError>> {
	let username = auth.username()?;
	
	let db = db::use_db();
	
	let files = db.get_files(username, &folder)?;
	
	Ok(files)
}

#[server]
pub async fn upload_file(auth: Auth, folder: Cipher<FolderName>, info: Cipher<FileInfo>, content: Cipher<FileContent>) -> Result<(), ServerFnError<FilesError>> {
	let username = auth.username()?;
	
	let db = db::use_db();
	let files_location: PathBuf = leptos::use_context().unwrap();
	
	let new_file_transaction = NewFileTransaction::open_file(&files_location).map_err(|_| ServerFnError::ServerError("Server Error".to_owned()))?;
	
	db.add_file(username, &folder, &info, new_file_transaction.id())?;
	
	new_file_transaction.write_data(content.as_bytes()).map_err(|_| ServerFnError::ServerError("Server Error".to_owned()))?;
	
	Ok(())
}

#[server]
pub async fn download_file(auth: Auth, file: Cipher<FileInfo>) -> Result<Cipher<FileContent>, ServerFnError<FilesError>> {
	let username = auth.username()?;
	
	let db = db::use_db();
	let files_location: PathBuf = leptos::use_context().unwrap();
	
	let file_id = db.get_file_id(username, &file)?;
	
	let path = files_location.join(file_id);
	let content = fs::read(path).map_err(|_| ServerFnError::ServerError("Server error".to_owned()))?;
	
	Ok(Cipher::from_bytes(content))
}

#[derive(Clone, Copy, Error, Debug)]
pub enum FilesError {
	#[error("Not Found")]
	NotFound,
	#[error("Not Authenticated")]
	NotAuthenticated,
}

impl FromStr for FilesError {
	type Err = ();
	
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"Not Found" => Ok(Self::NotFound),
			"Not Authenticated" => Ok(Self::NotAuthenticated),
			_ => Err(())
		}
	}
}

impl From<AuthError> for ServerFnError<FilesError> {
	fn from(_err: AuthError) -> Self {
		ServerFnError::WrappedServerError(FilesError::NotAuthenticated)
	}
}

#[cfg(feature = "ssr")]
impl From<db::Error> for ServerFnError<FilesError> {
	fn from(err: db::Error) -> Self {
		use db::Error::*;
		match err {
			FileError(_) | SQLiteError(_) => {
				eprintln!("Error: {err}");
				ServerFnError::ServerError("Server error".to_owned())
			},
			NotFound => ServerFnError::WrappedServerError(FilesError::NotFound)
		}
	}
}
