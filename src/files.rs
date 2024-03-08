use leptos::{server, ServerFnError};

use crate::{account::Auth, vault::CipherFolderName};

#[allow(unused)]
use crate::db;

#[server]
pub async fn create_folder(auth: Auth, folder_name: CipherFolderName) -> Result<(), ServerFnError> {
	if !auth.is_valid() {
		todo!("Unauthorized");
	}
	
	let db = db::use_db();
	
	db.add_folder(auth.username(), &folder_name).inspect_err(|err| eprintln!("{err:#?}"))?;
	
	Ok(())
}
