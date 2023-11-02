use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::model::{Entity, Error, ModelManager, Result};
use crate::model::customer::Customer;

pub type Id = String;
pub trait DbBmc {
  const TABLE: &'static str;
}

#[derive(Debug, Deserialize, Serialize)]
struct Record<T> {
  id: Thing,
  data: T
}


pub async fn create<MC, E>(mm: &ModelManager, data: &E, id: &str) -> Result<Id>
  where
    MC: DbBmc,
    E: Entity + Serialize + Debug,
    E: for<'e> Deserialize<'e>
{
  let db = mm.db();
  let result :Vec<E> = db.create(MC::TABLE)
    .content(data).await?;

  //dbg!(&created);
  //todo!()
  //Ok(created.first().ok_or(Error::EntityCreation)?.id.id.to_string())
  Ok("4711".to_string())
}

pub async fn list<MC, E>(mm: &ModelManager) -> Result<Vec<E>>
  where
    MC: DbBmc,
    E: Entity + Serialize ,
    E: for<'e> Deserialize<'e>
{
  let db = mm.db();
  let records:Vec<E> = db.select(MC::TABLE)
    .await?;
  Ok(records)
}