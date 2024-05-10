use super::*;
use crate::db;

impl Salt {
	pub fn from_db(data: [u8; 32], _: db::Token) -> Self {
		Self {
			data,
		}
	}
	
	pub fn to_db(&self, _: db::Token) -> [u8; 32] {
		self.data
	}
}

impl PasswordHash {
	pub fn from_db(data: [u8; 64], _: db::Token) -> Self {
		Self {
			data,
		}
	}
	
	pub fn to_db(&self, _: db::Token) -> [u8; 64] {
		self.data
	}
}
