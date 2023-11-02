mod error;

use axum::body::HttpBody;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Surreal;
pub use self::error::{Error, Result};

use surrealdb::engine::local::Db as SurrealDB;
use surrealdb::engine::local::Mem;

//pub type Db = Surreal<Client>;
pub type Db = Surreal<SurrealDB>;

pub async fn new_db_connection() -> Result<Db> {
  pub const DB_CONNECTION_STR: &'static str = "192.168.178.144:8300";
  //let db = Surreal::new::<Ws>(DB_CONNECTION_STR)
  let db = Surreal::new::<Mem>(())
    .await
    .map_err(|ex| Error::FailToCreateDbConnection(ex.to_string()))?;

  Ok(db)
}