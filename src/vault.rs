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

impl Salt {
	//TODO: remove
	pub fn mock_salt() -> Self {
		Self {
			data: [0; 32],
		}
	}
}

impl Debug for Salt {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Salt(...)")
	}
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasswordHash {
	#[serde(with = "BigArray")]
	data: [u8; 64],
}

impl Debug for PasswordHash {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "PasswordHash(...)")
	}
}
