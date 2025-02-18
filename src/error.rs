use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use crate::model;

pub type Result<T> = core::result::Result<T, Error>;


#[derive(Debug, Serialize)]
pub enum Error {
	// -- Config

	// -- Modules
	Model(model::Error),

	//Extern

}

// region:    --- Froms
impl From<model::Error> for Error {
	fn from(val: model::Error) -> Self {
		Self::Model(val)
	}
}

// endregion: --- Froms

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
