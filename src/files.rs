use std::{fs, path::PathBuf};

use leptos::{server, use_context, ServerFnError};

use crate::{account::Auth, vault::{Cipher, FileContent, FileInfo, FolderName}};

#[allow(unused)]
use crate::db;

#[cfg(feature = "ssr")]
mod new_file_transaction;
#[cfg(feature = "ssr")]
use self::new_file_transaction::*;

#[server]
pub async fn create_folder(auth: Auth, folder_name: Cipher<FolderName>) -> Result<(), ServerFnError> {
	if !auth.is_valid() {
		todo!("Unauthorized");
	}
	
	let db = db::use_db();
	
	db.add_folder(auth.username(), &folder_name)?;
	
	Ok(())
}

#[server]
pub async fn get_files(auth: Auth, folder: Cipher<FolderName>) -> Result<Vec<Cipher<FileInfo>>, ServerFnError> {
	if !auth.is_valid() {
		todo!("Unauthorized");
	}
	
	let db = db::use_db();
	
	let files = db.get_files(&folder)?;
	
	Ok(files)
}

#[server]
pub async fn upload_file(auth: Auth, folder: Cipher<FolderName>, info: Cipher<FileInfo>, content: Cipher<FileContent>) -> Result<(), ServerFnError> {
	if !auth.is_valid() {
		todo!("Unauthorized");
	}
	
	let db = db::use_db();
	let files_location: PathBuf = use_context().unwrap();
	
	let new_file_transaction = NewFileTransaction::open_file(&files_location)?;
	
	db.add_file(&folder, &info, new_file_transaction.id())?;
	
	new_file_transaction.write_data(content.as_bytes())?;
	
	Ok(())
}

#[server]
pub async fn download_file(auth: Auth, file: Cipher<FileInfo>) -> Result<Cipher<FileContent>, ServerFnError> {
	if !auth.is_valid() {
		todo!("Unauthorized");
	}
	
	let db = db::use_db();
	let files_location: PathBuf = use_context().unwrap();
	
	let file_id = db.get_file_id(&file)?;
	
	let path = files_location.join(file_id);
	let content = fs::read(path)?;
	
	Ok(Cipher::from_bytes(content))
}
