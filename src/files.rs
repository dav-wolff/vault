use leptos::{server, ServerFnError};

use crate::{account::Auth, db};

#[server]
pub async fn create_folder(auth: Auth, folder_name: String) -> Result<(), ServerFnError> {
	if !auth.is_valid() {
		todo!("Unauthorized");
	}
	
	let db = db::use_db();
	
	db.add_folder(auth.username(), &folder_name).inspect_err(|err| eprintln!("{err:#?}"))?;
	
	Ok(())
}
