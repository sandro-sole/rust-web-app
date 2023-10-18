use serde::Serialize;
use crate::model::store;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
  EntityNotFound { entity: &'static str, id: i64 },

  // -- Modules
  Store(store::Error),
  
  // -- Externals

}

impl From<store::Error> for Error {
  fn from(val: store::Error) -> Self {
    Self::Store(val)
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