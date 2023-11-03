use std::fmt::Debug;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::model::{base, Entity, ModelManager};

use crate::model::{Error, Result};
use crate::model::account::Account;
use crate::model::base::DbBmc;
use crate::model::customer::Customer;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Role {
  pub id: Thing,
  pub r#type: RoleType,
  pub created_at: NaiveDateTime,
  pub status: RoleStatus,

}

impl From<CreateRole> for Role {
  fn from(web_view: CreateRole) -> Self {
    Self {
      id: Thing { tb: RoleBmc::TABLE.to_string(), id: web_view.id.unwrap().into() },
      r#type: web_view.r#type,
      created_at: web_view.created_at,
      status: web_view.status,
    }
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateRole {
  pub id: Option<String>,
  pub r#type: RoleType,
  pub created_at: NaiveDateTime,
  pub status: RoleStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RoleStatus {
  Active,
  Inactive,

}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RoleType {
  Owner,
  Power,

}

impl Entity for Role{
  fn get_id(&self) -> Thing {
    self.id.clone()
  }
}

pub struct RoleBmc;

impl DbBmc for RoleBmc {
  const TABLE: &'static str = "role";
}

impl RoleBmc {
  pub async fn create(mm: &ModelManager, a: &Role) -> Result<()> {
    base::create::<Self, _>(mm, a).await.unwrap();
    Ok(())
  }

  pub async fn relate(mm: &ModelManager, customer: &Customer, account: &Account, role: &CreateRole) -> Result<()>
  {
    base::relate::<Self, Customer, Account, CreateRole>(mm, customer, account, role).await.unwrap();
    Ok(())
  }
}