use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use crate::model::store;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
  EntityNotFound { entity: &'static str, id: i64 },
  EntityCreation ,

  // -- Modules
  Store(store::Error),


  // -- Externals
  SurrealDb(#[serde_as(as = "DisplayFromStr")] surrealdb::Error)
}

impl From<store::Error> for Error {
  fn from(val: store::Error) -> Self {
    Self::Store(val)
  }
}

impl From<surrealdb::Error> for Error {
  fn from(val: surrealdb::Error) -> Self {
    Self::SurrealDb(val)
  }
}

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