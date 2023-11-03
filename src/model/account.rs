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

impl From<CreateAccount> for Account {
  fn from(web_view: CreateAccount) -> Self {
    Self {
      id: Thing { tb: "customer".to_string(), id: web_view.id.into() },
      account_nr: web_view.account_nr,
      account_status: web_view.account_status,
    }
  }
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


impl Entity for Account{
  fn get_id(&self) -> Thing {
    self.id.clone()
  }
}
pub struct AccountBmc;

impl DbBmc for AccountBmc {
  const TABLE: &'static str = "account";
}

impl AccountBmc {
  pub async fn create(mm: &ModelManager, a: &Account) -> Result<()> {
    base::create::<Self,_ >(mm, a).await.unwrap();
    Ok( () )
  }

  pub async fn get(mm: &ModelManager, id: &str) -> Result<Option<Account>> {
    base::get::<Self,_ >(mm, id).await
  }

  pub async fn list(mm: &ModelManager) -> Result<Vec<Account>> {
    let list_of_customers = base::list::<Self,_ >(mm).await?;
    Ok(list_of_customers)
  }
}

/*           */

