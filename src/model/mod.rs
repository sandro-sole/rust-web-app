
mod store;
mod error;

mod base;

pub mod customer;
pub(crate) mod account;

use axum::body::HttpBody;
use surrealdb::opt::auth::Root;
pub use self::error::{Error, Result};

use crate::model::store::{Db, new_db_connection};

pub trait Entity{}

#[derive(Clone)]
pub struct ModelManager {
  db: Db,
}

impl ModelManager {
  /// Constructor
  pub async fn new() -> Result<Self> {
    let db = new_db_connection().await?;
    //db.signin(Root{username: "root", password: "root"})
    //  .await?;

    db.use_ns("customer").use_db("crm").await?;
    Ok(ModelManager { db })
  }

  /// Returns the sqlx db pool reference.
  /// (Only for the model layer)
  pub(in crate::model) fn db(&self) -> &Db {
    &self.db
  }
}
