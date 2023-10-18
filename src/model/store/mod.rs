mod error;

use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Surreal;
pub use self::error::{Error, Result};


pub type Db = Surreal<Client>;

pub async fn new_db_connection() -> Result<Db> {
  let db = Surreal::new::<Ws>("192.168.178.114:8200")
    .await
    .map_err(|ex| Error::FailToCreateDbConnection(ex.to_string()));

  db
}