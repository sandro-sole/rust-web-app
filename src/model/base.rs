use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::model::{Entity, Error, ModelManager, Result};

pub type Id = String;
pub trait DbBmc {
  const TABLE: &'static str;
}

#[derive(Debug, Deserialize)]
struct Record {
  id: Thing
}


pub async fn create<MC, E>(mm: &ModelManager, data: E) -> Result<Id>
  where
    MC: DbBmc,
    E: Entity + Serialize
{
  let db = mm.db();
  let created: Vec<Record> = db.create(MC::TABLE)
    .content(data).await?;

  dbg!(&created);

  Ok(created.first().ok_or(Error::EntityCreation)?.id.id.to_string())
}