use std::collections::HashMap;

use leptos::{create_rw_signal, spawn_local, RwSignal, SignalUpdate, SignalWith};

use crate::{account::Auth, files, vault::{Cipher, FileContent, FileInfo, FolderName, Secret, Vault}};

use self::folder_state::FolderState;

mod folder_state;

#[derive(Clone, Debug)]
pub struct FileData {
	pub id: Cipher<FileInfo>,
	pub info: Secret<FileInfo>,
}

async fn load_folder(auth: Auth, vault: Vault, folder: Cipher<FolderName>) -> Vec<FileData> {
	// TODO handle error
	let files = files::get_files(auth, folder).await.unwrap();
	
	files.into_iter()
		.map(|id| {
			// TODO handle error
			let info = vault.decrypt(&id).unwrap();
			
			FileData {
				id,
				info,
			}
		})
		.collect()
}

async fn load_file(auth: Auth, vault: Vault, file: Cipher<FileInfo>) -> Secret<FileContent> {
	// TODO handle error
	let content = files::download_file(auth, file).await.unwrap();
	
	// TODO handle error
	vault.decrypt(&content).unwrap()
}

#[derive(Clone, Debug)]
pub struct FileStore {
	vault: Vault,
	auth: Auth,
	folders: RwSignal<HashMap<Cipher<FolderName>, FolderState>>,
	files: RwSignal<HashMap<Cipher<FileInfo>, Option<Secret<FileContent>>>>,
}

impl FileStore {
	pub fn new(vault: Vault, auth: Auth) -> Self {
		Self {
			vault,
			auth,
			folders: create_rw_signal(HashMap::new()),
			files: create_rw_signal(HashMap::new()),
		}
	}
	
	pub fn files_in_folder_tracked(&self, folder: Cipher<FolderName>) -> Option<Vec<FileData>> {
		if let Some(entry) = self.folders.with(|folders| folders.get(&folder).cloned()) {
			return entry.loaded_files();
		}
		
		self.folders.update(|folders| {
			folders.insert(folder.clone(), Default::default());
		});
		
		let vault = self.vault.clone();
		let auth = self.auth.clone();
		let folders = self.folders;
		
		spawn_local(async move {
			let files = load_folder(auth, vault, folder.clone()).await;
			
			folders.update(|folders| {
				let entry = folders.get_mut(&folder).expect("None was just inserted");
				entry.add_remote_files(files);
			});
		});
		
		None
	}
	
	pub async fn add_files(&self, folder: Cipher<FolderName>, new_files: Vec<(Secret<FileInfo>, Secret<FileContent>)>) {
		// TODO display loading files
		let mut files_data = Vec::with_capacity(new_files.len());
		let contents: Vec<_> = new_files.into_iter()
			.map(|(info, content)| {
				files_data.push(FileData {
					// TODO handle errors
					id: self.vault.encrypt(&info).unwrap(),
					info,
				});
				
				content
			})
			.collect();
		
		let upload_futures = contents.iter()
			// TODO handle errors
			.map(|content| self.vault.encrypt(&content).unwrap())
			.zip(files_data.iter())
			.map(|(content, file_data)| files::upload_file(self.auth.clone(), folder.clone(), file_data.id.clone(), content));
		
		for result in futures::future::join_all(upload_futures).await {
			// TODO handle errors
			result.unwrap();
		}
		
		self.folders.update(|folders| {
			let entry = folders.get_mut(&folder).expect("Folder should have started loading before uploading files to it");
			entry.add_local_files(&files_data);
		});
		
		self.files.update(|files| {
			for (file_data, content) in files_data.into_iter().zip(contents.into_iter()) {
				files.insert(file_data.id, Some(content));
			}
		});
	}
	
	pub fn with_file_content_tracked<T>(&self, id: Cipher<FileInfo>, callback: impl Fn(&Secret<FileContent>) -> T) -> Option<T> {
		if let Some(result) = self.files.with(|files| -> Option<_> {
			let entry = files.get(&id)?;
			Some(entry.as_ref().map(|content| callback(content)))
		}) {
			return result;
		}
		
		self.files.update(|files| {
			files.insert(id.clone(), None);
		});
		
		let vault = self.vault.clone();
		let auth = self.auth.clone();
		let files = self.files;
		
		spawn_local(async move {
			let content = load_file(auth, vault, id.clone()).await;
			
			files.update(|files| {
				let entry = files.get_mut(&id).expect("None was just inserted");
				*entry = Some(content);
			});
		});
		
		None
	}
}
