
mod store;
mod error;

pub mod customer;
pub use self::error::{Error, Result};

use crate::model::store::{new_db_connection, Db};

#[derive(Clone)]
pub struct ModelManager {
  db: Db,
}

impl ModelManager {
  /// Constructor
  pub async fn new() -> Result<Self> {
    let db = new_db_connection().await?;

    Ok(ModelManager { db })
  }

  /// Returns the sqlx db pool reference.
  /// (Only for the model layer)
  pub(in crate::model) fn db(&self) -> &Db {
    &self.db
  }
}