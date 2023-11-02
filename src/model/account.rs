use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::model::{base, Entity, ModelManager};

use crate::model::{Error, Result};
use crate::model::base::DbBmc;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Account {
  pub id: Thing,
  pub account_nr: String,
  pub account_status: AccountStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateAccount {
  pub id: String,
  pub account_nr: String,
  pub account_status: AccountStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AccountStatus {
  Active,
  Inactive,

}


impl Entity for Account{}
pub struct AccountBmc;

impl DbBmc for AccountBmc {
  const TABLE: &'static str = "account";
}

impl AccountBmc {
  pub async fn create(mm: &ModelManager, a: &Account) -> Result<()> {
    base::create::<Self,_ >(mm, a).await.unwrap();
    Ok( () )
  }
}

/*           */

