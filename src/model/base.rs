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

pub async fn get<MC, E>(mm: &ModelManager, id: &str) -> Result<Option<E>>
  where
    MC: DbBmc,
    E: Entity + Serialize + Debug,
    E: for<'e> Deserialize<'e>
{
  let db = mm.db();
  let result: Option<E> = db
    .select((MC::TABLE, id))
    .await?;

  Ok(result)
}
pub async fn create<MC, E>(mm: &ModelManager, data: &E) -> Result<Id>
  where
    MC: DbBmc,
    E: Entity + Serialize + Debug,
    E: for<'e> Deserialize<'e>,
{
  let db = mm.db();
  let result :Vec<E> = db.create(MC::TABLE)
    .content(data).await?;

  //dbg!(&created);
  //todo!()
  //Ok(created.first().ok_or(Error::EntityCreation)?.id.id.to_string())
  Ok("4711".to_string())
}

pub async fn relate<MC, LE, RE, V>(mm: &ModelManager, r#in: &LE, out: &RE, content: &V) -> Result<Id>
  where
    MC: DbBmc,
    V: Debug + Serialize,
    LE: Entity + Serialize + Debug,
    LE: for<'e> Deserialize<'e>,
    RE: Entity + Serialize + Debug,
    RE: for<'e> Deserialize<'e>,
{
  let db = mm.db();

  let sql = format!("RELATE (type::thing($i_table, $i_id))->{}->(type::thing($o_table, $o_id)) CONTENT  {} "
          , MC::TABLE,
          serde_json::to_string(&content).map_err(|e|Error::SQL(e.to_string()))?);
  dbg!(&sql);

  let records = db
    .query(sql)
    .bind(("i_table", r#in.get_id().tb.as_str()) )
    .bind(("i_id", r#in.get_id().id.to_raw().as_str()) )
    .bind(("o_table", out.get_id().tb.as_str()) )
    .bind(("o_id", out.get_id().id.to_raw().as_str()) )
    .await?;
  dbg!(records);

  Ok("4711".to_string())
}
pub async fn update<MC, E>(mm: &ModelManager, data: &E, id: &str) -> Result<E>
  where
    MC: DbBmc,
    E: Entity + Serialize + Debug,
    E: for<'e> Deserialize<'e>
{
  let db = mm.db();
  let result :Option<E> = db.update((MC::TABLE, id))
    .content(data).await?;

  let Some(record) = result else {
    return Err(Error::EntityCreation);
  };
  Ok(record)
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