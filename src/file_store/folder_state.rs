use super::FileData;

#[derive(Clone, Debug)]
pub enum FolderState {
	Loading(Vec<FileData>),
	Loaded(Vec<FileData>),
}

impl FolderState {
	pub fn add_local_files(&mut self, new_files: &[FileData]) {
		match self {
			Self::Loading(files) => files.extend_from_slice(new_files),
			Self::Loaded(files) => files.extend_from_slice(new_files),
		}
	}
	
	pub fn add_remote_files(&mut self, mut new_files: Vec<FileData>) {
		match self {
			Self::Loading(files) => {
				new_files.extend_from_slice(&files);
				*self = Self::Loaded(new_files);
			},
			Self::Loaded(_) => panic!("Remote files should only be added once"),
		}
	}
	
	pub fn loaded_files(self) -> Option<Vec<FileData>> {
		match self {
			Self::Loading(_) => None,
			Self::Loaded(files) => Some(files),
		}
	}
}

impl Default for FolderState {
	fn default() -> Self {
		Self::Loading(Vec::default())
	}
}
