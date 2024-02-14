#[cfg(feature = "hydrate")]
mod secure;

#[cfg(feature = "hydrate")]
pub use secure::*;

#[cfg(not(feature = "hydrate"))]
mod mock_secure;

#[cfg(not(feature = "hydrate"))]
pub use mock_secure::*;

use std::fmt::{self, Debug};
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

#[derive(Clone, Serialize, Deserialize)]
pub struct Salt {
	data: [u8; 32],
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasswordHash {
	#[serde(with = "BigArray")]
	data: [u8; 64],
}

impl Debug for Salt {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Salt(...)")
	}
}

impl Debug for PasswordHash {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "PasswordHash(...)")
	}
}

#[cfg(feature = "ssr")]
mod db_acces {
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
}
